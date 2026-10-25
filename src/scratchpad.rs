//! Opt-in in-memory DuckDB session manager. No Google credentials or arbitrary SQL.
use duckdb::Connection;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

const TTL_SECONDS: u64 = 1800;
const HARD_SESSIONS: usize = 8;
const HARD_TABLES: usize = 16;
const MAX_LIST_ITEMS: usize = 1024;

#[derive(Clone)]
pub struct ScratchState(Arc<Mutex<Inner>>);

struct Inner {
    sessions: HashMap<String, Session>,
    max_sessions: usize,
    max_tables_per_session: usize,
}

struct Session {
    db: Connection,
    expires: Instant,
}

fn session_id(raw: &str) -> Result<&str, String> {
    let id = raw.trim();
    if id.is_empty()
        || id.len() > 128
        || !id.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
    {
        return Err("Invalid session_id: use 1-128 ASCII letters, numbers, underscores or hyphens".into());
    }
    Ok(id)
}

fn table_name(raw: &str) -> Result<&str, String> {
    if raw.is_empty() || raw.len() > 64
        || !raw.as_bytes()[0].is_ascii_alphabetic()
        || !raw.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_')
    {
        return Err("Invalid scratchpad table_name".into());
    }
    Ok(raw)
}

fn cap(limit: Option<usize>) -> Result<usize, String> {
    match limit {
        Some(0) => Err("List limit must be positive".into()),
        Some(n) if n > MAX_LIST_ITEMS => Err("List limit exceeds 1024 entries".into()),
        Some(n) => Ok(n),
        None => Ok(MAX_LIST_ITEMS),
    }
}

impl ScratchState {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(Inner {
            sessions: HashMap::new(),
            max_sessions: 4,
            max_tables_per_session: 8,
        })))
    }

    fn guard(&self) -> Result<std::sync::MutexGuard<'_, Inner>, String> {
        self.0.lock().map_err(|_| "Scratchpad session state unavailable".into())
    }

    fn expire(inner: &mut Inner) {
        let now = Instant::now();
        inner.sessions.retain(|_, s| s.expires > now);
    }

    pub fn open(&self, raw: &str) -> Result<Value, String> {
        let id = session_id(raw)?.to_owned();
        let mut inner = self.guard()?;
        Self::expire(&mut inner);
        let refreshed = if let Some(old) = inner.sessions.get_mut(&id) {
            old.expires = Instant::now() + Duration::from_secs(TTL_SECONDS);
            true
        } else {
            if inner.sessions.len() >= inner.max_sessions {
                return Err("Scratchpad session quota exceeded".into());
            }
            let db = Connection::open_in_memory().map_err(|_| "Cannot initialize DuckDB")?;
            db.execute_batch(
                "SET enable_external_access = false;
                 SET autoload_known_extensions = false;
                 SET autoinstall_known_extensions = false;",
            ).map_err(|_| "Cannot enforce DuckDB sandbox settings")?;
            inner.sessions.insert(id.clone(), Session {
                db,
                expires: Instant::now() + Duration::from_secs(TTL_SECONDS),
            });
            false
        };
        Ok(json!({"session_id":id,"refreshed":refreshed,"ttl_seconds_remaining":TTL_SECONDS}))
    }

    pub fn close(&self, raw: &str) -> Result<Value, String> {
        let id = session_id(raw)?;
        let mut inner = self.guard()?;
        Self::expire(&mut inner);
        if inner.sessions.remove(id).is_none() {
            return Err("Scratchpad session not found or expired".into());
        }
        Ok(json!({"closed":true,"session_id":id}))
    }

    pub fn sessions(&self, limit: Option<usize>) -> Result<Value, String> {
        let max = cap(limit)?;
        let mut inner = self.guard()?;
        Self::expire(&mut inner);
        let mut ids: Vec<_> = inner.sessions.keys().cloned().collect();
        ids.sort();
        let total = ids.len();
        ids.truncate(max);
        Ok(json!({"session_ids":ids,"count":ids.len(),"total_sessions":total,"truncated":total>max}))
    }

    pub fn limits(&self) -> Result<Value, String> {
        let inner = self.guard()?;
        Ok(json!({
            "max_sessions":inner.max_sessions,
            "max_tables_per_session":inner.max_tables_per_session,
            "max_sessions_hard":HARD_SESSIONS,
            "max_tables_per_session_hard":HARD_TABLES,
            "max_rows_per_session":20000,
            "session_ttl_seconds":TTL_SECONDS
        }))
    }

    pub fn set_limits(
        &self, sessions: Option<usize>, tables: Option<usize>,
    ) -> Result<Value, String> {
        let mut inner = self.guard()?;
        Self::expire(&mut inner);
        let max_sessions = sessions.unwrap_or(inner.max_sessions);
        let max_tables = tables.unwrap_or(inner.max_tables_per_session);
        if !(1..=HARD_SESSIONS).contains(&max_sessions)
            || !(1..=HARD_TABLES).contains(&max_tables)
            || max_sessions < inner.sessions.len()
        {
            return Err("Scratchpad limits outside policy or below current usage".into());
        }
        inner.max_sessions = max_sessions;
        inner.max_tables_per_session = max_tables;
        Ok(json!({"max_sessions":max_sessions,"max_tables_per_session":max_tables}))
    }

    pub fn tables(&self, raw: &str, limit: Option<usize>) -> Result<Value, String> {
        let id = session_id(raw)?;
        let max = cap(limit)?;
        let mut inner = self.guard()?;
        Self::expire(&mut inner);
        let db = &inner.sessions.get(id).ok_or("Scratchpad session not found or expired")?.db;
        let mut stmt = db.prepare(
            "SELECT table_name FROM information_schema.tables
             WHERE table_schema='main' AND table_type='BASE TABLE'
             ORDER BY table_name",
        ).map_err(|_| "DuckDB table catalog query failed")?;
        let mut items = stmt.query_map([], |row| row.get::<_, String>(0))
            .map_err(|_| "DuckDB table catalog query failed")?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "DuckDB table catalog read failed")?;
        let total = items.len();
        items.truncate(max);
        Ok(json!({"tables":items,"count":items.len(),"total_tables":total,"truncated":total>max}))
    }

    pub fn drop_table(&self, raw: &str, name: &str, if_exists: bool) -> Result<Value, String> {
        let id = session_id(raw)?;
        let name = table_name(name)?;
        let mut inner = self.guard()?;
        Self::expire(&mut inner);
        let db = &inner.sessions.get(id).ok_or("Scratchpad session not found or expired")?.db;
        // Parameter binding protects the catalog lookup; the identifier grammar
        // independently prohibits quotes, semicolons, whitespace and path syntax.
        let count: i64 = db.query_row(
            "SELECT count(*) FROM information_schema.tables
             WHERE table_schema='main' AND table_type='BASE TABLE' AND table_name=?1",
            [name],
            |row| row.get(0),
        ).map_err(|_| "DuckDB table inventory query failed")?;
        if count == 0 {
            return if if_exists {
                Ok(json!({"table_name":name,"dropped":false,"rows_removed":0}))
            } else {
                Err("Scratchpad table not found".into())
            };
        }
        db.execute_batch(&format!("DROP TABLE \"{name}\""))
            .map_err(|_| "DuckDB table deletion failed")?;
        Ok(json!({"table_name":name,"dropped":true}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_session_refresh_and_lifecycle() {
        let state = ScratchState::new();
        assert!(state.open("../unsafe").is_err());
        assert_eq!(state.open("project_1").unwrap()["refreshed"], false);
        assert_eq!(state.open("project_1").unwrap()["refreshed"], true);
        assert_eq!(state.sessions(None).unwrap()["total_sessions"], 1);
        assert!(state.set_limits(Some(0), None).is_err());
        assert!(state.set_limits(Some(9), None).is_err());
        assert!(state.set_limits(None, Some(17)).is_err());
        state.set_limits(Some(1), Some(1)).unwrap();
        assert!(state.open("another").is_err());
        assert_eq!(state.close("project_1").unwrap()["closed"], true);
        assert!(state.close("project_1").is_err());
    }

    #[test]
    fn strict_sql_identifier_and_table_inventory() {
        let state = ScratchState::new();
        state.open("session_1").unwrap();
        {
            let mut inner = state.guard().unwrap();
            let db = &inner.sessions.get_mut("session_1").unwrap().db;
            db.execute_batch("CREATE TABLE synthetic_events (id INTEGER)").unwrap();
        }
        assert_eq!(state.tables("session_1", None).unwrap()["total_tables"], 1);
        assert!(state.tables("session_1", Some(0)).is_err());
        assert!(state.drop_table("session_1", "x\"; DROP TABLE synthetic_events;--", false).is_err());
        assert!(state.drop_table("session_1", "missing", false).is_err());
        assert_eq!(state.tables("session_1", Some(1)).unwrap()["count"], 1);
        state.drop_table("session_1", "synthetic_events", false).unwrap();
        assert_eq!(state.tables("session_1", None).unwrap()["total_tables"], 0);
        assert_eq!(
            state.drop_table("session_1", "synthetic_events", true).unwrap()["dropped"],
            false
        );
    }
}
