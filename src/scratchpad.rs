//! Opt-in, in-memory DuckDB session manager. No Google credentials or external SQL.
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use duckdb::Connection;
use rand::{rngs::OsRng, RngCore};
use serde_json::{json, Value};
use std::{collections::HashMap, sync::{Arc, Mutex}, time::{Duration, Instant}};

#[derive(Clone)]
pub struct ScratchState(Arc<Mutex<Inner>>);

struct Inner {
    sessions: HashMap<String, Session>,
    max_sessions: usize,
}
struct Session {
    db: Connection,
    expires: Instant,
}

impl ScratchState {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(Inner { sessions: HashMap::new(), max_sessions: 4 })))
    }
    fn guard(&self) -> Result<std::sync::MutexGuard<'_, Inner>, String> {
        self.0.lock().map_err(|_| "Scratchpad lock poisoned".into())
    }
    fn expire(inner: &mut Inner) {
        let now=Instant::now();
        inner.sessions.retain(|_, session| session.expires > now);
    }
    pub fn open(&self) -> Result<Value, String> {
        let mut inner=self.guard()?;
        Self::expire(&mut inner);
        if inner.sessions.len() >= inner.max_sessions { return Err("Scratchpad session quota exceeded".into()); }
        let db=Connection::open_in_memory().map_err(|_| "Cannot open DuckDB scratchpad")?;
        db.execute_batch("SET enable_external_access=false; SET autoload_known_extensions=false; SET autoinstall_known_extensions=false;")
            .map_err(|_| "Cannot enforce DuckDB external-access restrictions")?;
        let mut random=[0u8;18];
        OsRng.fill_bytes(&mut random);
        let id=format!("sess_{}",URL_SAFE_NO_PAD.encode(random));
        inner.sessions.insert(id.clone(), Session {db, expires:Instant::now()+Duration::from_secs(1800)});
        Ok(json!({"session_id":id,"expires_in_seconds":1800}))
    }
    pub fn close(&self, id:&str) -> Result<Value,String> {
        let mut inner=self.guard()?;
        Self::expire(&mut inner);
        if inner.sessions.remove(id).is_none() {return Err("Scratchpad session not found or expired".into());}
        Ok(json!({"closed":true}))
    }
    pub fn sessions(&self) -> Result<Value,String> {
        let mut inner=self.guard()?;
        Self::expire(&mut inner);
        let mut ids:Vec<_>=inner.sessions.keys().cloned().collect();
        ids.sort();
        Ok(json!({"session_ids":ids,"count":ids.len()}))
    }
    pub fn limits(&self) -> Result<Value,String> {
        let n=self.guard()?.max_sessions;
        Ok(json!({"max_sessions":n,"max_sessions_hard":8,"max_tables_per_session":8,"max_rows_per_session":20000,"session_ttl_seconds":1800}))
    }
    pub fn set_limits(&self, max_sessions:usize) -> Result<Value,String> {
        let mut inner=self.guard()?;
        Self::expire(&mut inner);
        if !(1..=8).contains(&max_sessions) || max_sessions < inner.sessions.len() {
            return Err("Invalid scratchpad session quota".into());
        }
        inner.max_sessions=max_sessions;
        Ok(json!({"max_sessions":max_sessions}))
    }
    pub fn tables(&self, id:&str) -> Result<Value,String> {
        let mut inner=self.guard()?;
        Self::expire(&mut inner);
        let session=inner.sessions.get_mut(id).ok_or("Scratchpad session not found or expired")?;
        let mut stmt=session.db.prepare("SELECT table_name FROM information_schema.tables WHERE table_schema='main' AND table_type='BASE TABLE' ORDER BY table_name")
            .map_err(|_| "DuckDB catalog query failed")?;
        let items=stmt.query_map([],|row| row.get::<_,String>(0))
            .map_err(|_| "DuckDB catalog query failed")?
            .collect::<Result<Vec<_>,_>>().map_err(|_| "DuckDB catalog read failed")?;
        Ok(json!({"count":items.len(),"tables":items}))
    }
    pub fn drop_table(&self, id:&str, table:&str) -> Result<Value,String> {
        if table.len() > 64 || table.is_empty()
            || !table.as_bytes()[0].is_ascii_alphabetic()
            || !table.bytes().all(|c| c.is_ascii_alphanumeric() || c==b'_') {
            return Err("Invalid scratchpad table identifier".into());
        }
        let mut inner=self.guard()?;
        Self::expire(&mut inner);
        let session=inner.sessions.get_mut(id).ok_or("Scratchpad session not found or expired")?;
        session.db.execute_batch(&format!("DROP TABLE IF EXISTS \"{table}\""))
            .map_err(|_| "DuckDB table deletion failed")?;
        Ok(json!({"dropped":true,"table":table}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quota_and_lifecycle() {
        let state=ScratchState::new();
        let id=state.open().unwrap()["session_id"].as_str().unwrap().to_owned();
        assert_eq!(state.sessions().unwrap()["count"],1);
        assert!(state.set_limits(0).is_err());
        assert!(state.set_limits(9).is_err());
        state.set_limits(1).unwrap();
        assert!(state.open().is_err());
        assert_eq!(state.close(&id).unwrap()["closed"],true);
        assert!(state.close(&id).is_err());
    }
    #[test]
    fn local_tables_and_injection_guard() {
        let state=ScratchState::new();
        let id=state.open().unwrap()["session_id"].as_str().unwrap().to_owned();
        {
            let mut inner=state.guard().unwrap();
            inner.sessions.get_mut(&id).unwrap().db.execute_batch("CREATE TABLE synthetic_events (id INT)").unwrap();
        }
        assert_eq!(state.tables(&id).unwrap()["count"],1);
        assert!(state.drop_table(&id,"bad\"; DROP TABLE synthetic_events;--").is_err());
        assert_eq!(state.tables(&id).unwrap()["count"],1);
        state.drop_table(&id,"synthetic_events").unwrap();
        assert_eq!(state.tables(&id).unwrap()["count"],0);
    }
}
