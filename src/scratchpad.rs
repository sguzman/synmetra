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
        || !id
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
    {
        return Err(
            "Invalid session_id: use 1-128 ASCII letters, numbers, underscores or hyphens".into(),
        );
    }
    Ok(id)
}

fn table_name(raw: &str) -> Result<&str, String> {
    if raw.is_empty()
        || raw.len() > 64
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
        self.0
            .lock()
            .map_err(|_| "Scratchpad session state unavailable".into())
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
            )
            .map_err(|_| "Cannot enforce DuckDB sandbox settings")?;
            inner.sessions.insert(
                id.clone(),
                Session {
                    db,
                    expires: Instant::now() + Duration::from_secs(TTL_SECONDS),
                },
            );
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
        Ok(
            json!({"session_ids":ids,"count":ids.len(),"total_sessions":total,"truncated":total>max}),
        )
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
        &self,
        sessions: Option<usize>,
        tables: Option<usize>,
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
        let db = &inner
            .sessions
            .get(id)
            .ok_or("Scratchpad session not found or expired")?
            .db;
        let mut stmt = db
            .prepare(
                "SELECT table_name FROM information_schema.tables
             WHERE table_schema='main' AND table_type='BASE TABLE'
             ORDER BY table_name",
            )
            .map_err(|_| "DuckDB table catalog query failed")?;
        let mut items = stmt
            .query_map([], |row| row.get::<_, String>(0))
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
        let db = &inner
            .sessions
            .get(id)
            .ok_or("Scratchpad session not found or expired")?
            .db;
        // Parameter binding protects the catalog lookup; the identifier grammar
        // independently prohibits quotes, semicolons, whitespace and path syntax.
        let count: i64 = db
            .query_row(
                "SELECT count(*) FROM information_schema.tables
             WHERE table_schema='main' AND table_type='BASE TABLE' AND table_name=?1",
                [name],
                |row| row.get(0),
            )
            .map_err(|_| "DuckDB table inventory query failed")?;
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

    pub fn describe_table(&self, raw: &str, table: &str) -> Result<Value, String> {
        let id = session_id(raw)?;
        let table = table_name(table)?;
        let mut inner = self.guard()?;
        Self::expire(&mut inner);
        let db = &inner
            .sessions
            .get(id)
            .ok_or("Scratchpad session not found or expired")?
            .db;
        let mut stmt = db
            .prepare(
                "SELECT column_name, data_type, is_nullable
             FROM information_schema.columns
             WHERE table_schema='main' AND table_name=?1
             ORDER BY ordinal_position LIMIT 33",
            )
            .map_err(|_| "Could not prepare DuckDB schema inspection")?;
        let mut columns = stmt
            .query_map([table], |row| {
                Ok(json!({
                    "name": row.get::<_, String>(0)?,
                    "data_type": row.get::<_, String>(1)?,
                    "nullable": row.get::<_, String>(2)? == "YES"
                }))
            })
            .map_err(|_| "DuckDB schema lookup failed")?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "DuckDB schema inspection failed")?;
        if columns.is_empty() {
            return Err("Scratchpad table not found".into());
        }
        let truncated = columns.len() > 32;
        columns.truncate(32);
        Ok(json!({
            "session_id":id, "table_name":table,
            "columns":columns,"column_count_returned":columns.len(),
            "columns_truncated":truncated
        }))
    }

    /// Export bounded in-memory JSON and Markdown evidence. Never writes files.
    pub fn export_evidence_bundle(
        &self,
        raw: &str,
        requested_tables: Option<Vec<String>>,
        sample_rows_per_table: Option<usize>,
    ) -> Result<Value, String> {
        use sha2::{Digest, Sha256};
        use std::collections::BTreeSet;

        let id = session_id(raw)?;
        let sample_limit = sample_rows_per_table.unwrap_or(20);
        if sample_limit > 20 {
            return Err("Evidence sample_rows_per_table must be at most 20".into());
        }
        let mut inner = self.guard()?;
        Self::expire(&mut inner);
        let db = &inner
            .sessions
            .get(id)
            .ok_or("Scratchpad session not found or expired")?
            .db;
        let selected: Vec<String> = if let Some(tables) = requested_tables {
            if tables.len() > 8 {
                return Err("Evidence bundle may include at most eight tables".into());
            }
            let mut unique = BTreeSet::new();
            for name in &tables {
                table_name(name)?;
                if !unique.insert(name) {
                    return Err("Evidence bundle table names must be unique".into());
                }
            }
            tables
        } else {
            let mut statement = db
                .prepare(
                    "SELECT table_name FROM information_schema.tables
                     WHERE table_schema='main' AND table_type='BASE TABLE'
                     ORDER BY table_name LIMIT 9",
                )
                .map_err(|_| "Evidence catalog query failed")?;
            let tables = statement
                .query_map([], |row| row.get::<_, String>(0))
                .map_err(|_| "Evidence catalog query failed")?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| "Evidence catalog read failed")?;
            if tables.len() > 8 {
                return Err("More than eight tables: explicitly select a bounded subset".into());
            }
            tables
        };

        let mut evidence = Vec::new();
        let mut markdown = String::from(
            "# Synmetra scratchpad evidence\n\n| Table | Rows | Sampled |\n|---|---:|---:|\n",
        );
        for name in selected {
            let name = table_name(&name)?;
            let exists: i64 = db
                .query_row(
                    "SELECT COUNT(*) FROM information_schema.tables
                     WHERE table_schema='main' AND table_type='BASE TABLE' AND table_name=?1",
                    [name],
                    |row| row.get(0),
                )
                .map_err(|_| "Evidence catalog validation failed")?;
            if exists != 1 {
                return Err("Selected evidence table does not exist".into());
            }
            let row_count: i64 = db
                .query_row(&format!("SELECT COUNT(*) FROM \"{name}\""), [], |row| {
                    row.get(0)
                })
                .map_err(|_| "Evidence row count query failed")?;
            // No DuckDB JSON extension: gather a bounded, escaped SQL projection
            // and encode preview cells in Rust. Column names come from DuckDB's
            // catalog, but must still be identifier-quoted to prevent injection.
            let mut schema_stmt = db
                .prepare(
                    "SELECT column_name FROM information_schema.columns
                     WHERE table_schema='main' AND table_name=?1
                     ORDER BY ordinal_position LIMIT 33",
                )
                .map_err(|_| "Evidence schema lookup failed")?;
            let columns = schema_stmt
                .query_map([name], |row| row.get::<_, String>(0))
                .map_err(|_| "Evidence schema lookup failed")?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| "Evidence schema read failed")?;
            if columns.is_empty() || columns.len() > 32 {
                return Err("Evidence preview requires between one and 32 columns".into());
            }
            let projection = columns
                .iter()
                .map(|column| {
                    let quoted = column.replace('"', "\"\"");
                    format!("substr(CAST(\"{quoted}\" AS VARCHAR), 1, 2049)")
                })
                .collect::<Vec<_>>()
                .join(", ");
            let query = format!("SELECT {projection} FROM \"{name}\" LIMIT {sample_limit}");
            let samples: Vec<Value> = if sample_limit == 0 {
                Vec::new()
            } else {
                let mut statement = db
                    .prepare(&query)
                    .map_err(|_| "Evidence sampling query failed")?;
                statement
                    .query_map([], |row| {
                        let mut map = serde_json::Map::new();
                        for (index, column) in columns.iter().enumerate() {
                            let cell: Option<String> = row.get(index)?;
                            map.insert(column.clone(), cell.map_or(Value::Null, |s| json!(s)));
                        }
                        Ok(Value::Object(map))
                    })
                    .map_err(|_| "Evidence sampling query failed")?
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| "Evidence sampling returned invalid rows")?
                    .into_iter()
                    .map(|row| {
                        if serde_json::to_vec(&row).map_or(true, |bytes| bytes.len() > 2048) {
                            Err("An evidence sample row exceeds 2048 bytes".to_owned())
                        } else {
                            Ok(row)
                        }
                    })
                    .collect::<Result<Vec<_>, String>>()?
            };
            markdown.push_str(&format!("| {name} | {row_count} | {} |\n", samples.len()));
            evidence.push(json!({
                "table_name":name,
                "row_count":row_count,
                "sample_rows":samples,
                "sample_query_sha256":format!("{:x}", Sha256::digest(query.as_bytes())),
            }));
        }
        let result = json!({
            "session_id":id,
            "tables":evidence,
            "sample_rows_per_table":sample_limit,
            "markdown":markdown,
            "source":"local_scratchpad",
            "provider_called":false,
            "sample_cell_encoding":"bounded_text",
        });
        if serde_json::to_vec(&result).map_or(true, |data| data.len() > 256 * 1024) {
            return Err("Evidence bundle exceeds 256 KiB output policy".into());
        }
        Ok(result)
    }

    /// Ingest a single bounded GA4 Data API response with parameter-bound values.
    /// Existing tables are never silently overwritten; append requires identical schema.
    pub fn ingest_ga_response(
        &self,
        raw: &str,
        target: &str,
        response: &Value,
        append: bool,
    ) -> Result<Value, String> {
        use std::collections::HashSet;
        let id = session_id(raw)?;
        let table = table_name(target)?;
        if serde_json::to_vec(response).map_or(true, |data| data.len() > 2 * 1024 * 1024) {
            return Err("GA4 response exceeds 2 MiB ingest policy".into());
        }
        let dim = response["dimensionHeaders"]
            .as_array()
            .ok_or("Missing dimension headers")?;
        let met = response["metricHeaders"]
            .as_array()
            .ok_or("Missing metric headers")?;
        let headers = dim
            .iter()
            .chain(met.iter())
            .map(|v| {
                v["name"]
                    .as_str()
                    .ok_or_else(|| "Invalid GA4 header".to_owned())
            })
            .collect::<Result<Vec<_>, _>>()?;
        if headers.is_empty() || headers.len() > 16 {
            return Err("GA4 ingest supports 1-16 columns".into());
        }
        let mut unique = HashSet::new();
        for name in &headers {
            table_name(name)?;
            if !unique.insert(name.to_ascii_lowercase()) {
                return Err("GA4 headers must be unique, ignoring case".into());
            }
        }
        let response_rows = response["rows"]
            .as_array()
            .ok_or("GA4 response is missing rows")?;
        if response_rows.len() > 1_000 {
            return Err("GA4 ingest accepts at most 1000 rows per request".into());
        }
        let mut parsed_rows = Vec::with_capacity(response_rows.len());
        for row in response_rows {
            let dimensions = row["dimensionValues"]
                .as_array()
                .ok_or("Missing dimension values")?;
            let metrics = row["metricValues"]
                .as_array()
                .ok_or("Missing metric values")?;
            if dimensions.len() != dim.len() || metrics.len() != met.len() {
                return Err("GA4 values do not match response headers".into());
            }
            let values = dimensions
                .iter()
                .chain(metrics.iter())
                .map(|v| {
                    let text = v["value"].as_str().ok_or("Invalid GA4 cell value")?;
                    if text.len() > 2_048 {
                        return Err("GA4 cell exceeds 2048-byte limit");
                    }
                    Ok(text.to_owned())
                })
                .collect::<Result<Vec<_>, &str>>()?;
            parsed_rows.push(values);
        }

        let mut inner = self.guard()?;
        Self::expire(&mut inner);
        let max_tables = inner.max_tables_per_session;
        let db = &inner
            .sessions
            .get(id)
            .ok_or("Scratchpad session not found or expired")?
            .db;
        let mut existing = db
            .prepare(
                "SELECT table_name FROM information_schema.tables
             WHERE table_schema='main' AND table_type='BASE TABLE' ORDER BY table_name",
            )
            .map_err(|_| "DuckDB catalog inspection failed")?;
        let table_names = existing
            .query_map([], |r| r.get::<_, String>(0))
            .map_err(|_| "DuckDB catalog inspection failed")?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "DuckDB catalog inspection failed")?;
        drop(existing);
        let exists = table_names
            .iter()
            .any(|name| name.eq_ignore_ascii_case(table));
        if exists && !append {
            return Err("Table already exists; append=true is required".into());
        }
        if !exists && table_names.len() >= max_tables {
            return Err("Scratchpad table quota exceeded".into());
        }
        let mut total_rows: i64 = 0;
        for name in &table_names {
            let safe = name.replace('"', "\"\"");
            let count: i64 = db
                .query_row(&format!("SELECT COUNT(*) FROM \"{safe}\""), [], |row| {
                    row.get(0)
                })
                .map_err(|_| "Scratchpad row quota inspection failed")?;
            total_rows = total_rows.saturating_add(count);
        }
        if total_rows.saturating_add(parsed_rows.len() as i64) > 20_000 {
            return Err("Scratchpad session row cap of 20000 would be exceeded".into());
        }
        if exists {
            let mut stmt = db
                .prepare(
                    "SELECT column_name, data_type FROM information_schema.columns
                 WHERE table_schema='main' AND table_name=?1 ORDER BY ordinal_position",
                )
                .map_err(|_| "DuckDB schema validation failed")?;
            let actual = stmt
                .query_map([table], |row| {
                    Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
                })
                .map_err(|_| "DuckDB schema validation failed")?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| "DuckDB schema validation failed")?;
            if actual.len() != headers.len()
                || actual
                    .iter()
                    .zip(&headers)
                    .any(|((name, ty), want)| !name.eq_ignore_ascii_case(want) || ty != "VARCHAR")
            {
                return Err(
                    "Append requires exactly matching column names and VARCHAR types".into(),
                );
            }
        }
        let columns = headers
            .iter()
            .map(|x| format!("\"{x}\""))
            .collect::<Vec<_>>()
            .join(", ");
        let create = format!(
            "CREATE TABLE \"{table}\" ({})",
            headers
                .iter()
                .map(|x| format!("\"{x}\" VARCHAR"))
                .collect::<Vec<_>>()
                .join(", ")
        );
        let placeholders = vec!["?"; headers.len()].join(", ");
        let insert = format!("INSERT INTO \"{table}\" ({columns}) VALUES ({placeholders})");
        db.execute_batch("BEGIN TRANSACTION")
            .map_err(|_| "DuckDB transaction failed")?;
        let action = (|| -> Result<(), String> {
            if !exists {
                db.execute_batch(&create)
                    .map_err(|_| "DuckDB table creation failed")?;
            }
            {
                let mut stmt = db
                    .prepare(&insert)
                    .map_err(|_| "DuckDB insert prepare failed")?;
                for row in &parsed_rows {
                    stmt.execute(duckdb::params_from_iter(row.iter()))
                        .map_err(|_| "DuckDB parameterized insert failed")?;
                }
            }
            db.execute_batch("COMMIT")
                .map_err(|_| "DuckDB ingest commit failed")?;
            Ok(())
        })();
        if action.is_err() {
            let _ = db.execute_batch("ROLLBACK");
        }
        action?;
        Ok(json!({
            "session_id":id, "table_name":table,
            "rows_ingested":parsed_rows.len(),"columns":headers,
            "append":append,"provider_result_rows":response_rows.len()
        }))
    }

    /// Preflight local-session existence before issuing any Google API request.
    pub fn require_session(&self, raw: &str) -> Result<(), String> {
        let id = session_id(raw)?;
        let mut inner = self.guard()?;
        Self::expire(&mut inner);
        if inner.sessions.contains_key(id) {
            Ok(())
        } else {
            Err("Scratchpad session not found or expired".into())
        }
    }

    pub fn summarize_table(&self, raw: &str, table: &str) -> Result<Value, String> {
        // Catalog validation precedes the only dynamically constructed SQL statement.
        let schema = self.describe_table(raw, table)?;
        let id = session_id(raw)?;
        let table = table_name(table)?;
        let mut inner = self.guard()?;
        Self::expire(&mut inner);
        let db = &inner
            .sessions
            .get(id)
            .ok_or("Scratchpad session not found or expired")?
            .db;
        let row_count: i64 = db
            .query_row(&format!("SELECT COUNT(*) FROM \"{table}\""), [], |row| {
                row.get(0)
            })
            .map_err(|_| "DuckDB table summary failed")?;
        Ok(json!({
            "session_id":id,"table_name":table,"row_count":row_count,
            "columns":schema["columns"],
            "column_count_returned":schema["column_count_returned"],
            "columns_truncated":schema["columns_truncated"]
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidence_export_handles_quoted_columns_nulls_and_bounded_text_cells() {
        let state = ScratchState::new();
        state.open("private_test").unwrap();
        {
            let mut inner = state.guard().unwrap();
            let db = &inner.sessions.get_mut("private_test").unwrap().db;
            db.execute_batch(
                "CREATE TABLE quoted (\"odd\"\"label\" VARCHAR, qty INTEGER, note VARCHAR);
                 INSERT INTO quoted VALUES ('value', 7, NULL);
                 CREATE TABLE too_long (value VARCHAR);
                 INSERT INTO too_long VALUES (repeat('x', 3000));",
            )
            .unwrap();
        }
        let evidence = state
            .export_evidence_bundle("private_test", Some(vec!["quoted".into()]), Some(1))
            .unwrap();
        let sample = &evidence["tables"][0]["sample_rows"][0];
        assert_eq!(sample["odd\"label"], "value");
        assert_eq!(sample["qty"], "7");
        assert!(sample["note"].is_null());
        assert_eq!(evidence["sample_cell_encoding"], "bounded_text");
        assert!(state
            .export_evidence_bundle("private_test", Some(vec!["too_long".into()]), Some(1))
            .is_err());
        let empty = state
            .export_evidence_bundle("private_test", Some(vec!["quoted".into()]), Some(0))
            .unwrap();
        assert_eq!(
            empty["tables"][0]["sample_rows"].as_array().unwrap().len(),
            0
        );
    }

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
    fn ingest_ga4_response_is_transactional_bounded_and_sql_safe() {
        let state = ScratchState::new();
        state.open("ga4_ingest").unwrap();
        let response = json!({
            "dimensionHeaders":[{"name":"country"}],
            "metricHeaders":[{"name":"activeUsers"}],
            "rows":[
                {"dimensionValues":[{"value":"Mexico"}],"metricValues":[{"value":"42"}]},
                {"dimensionValues":[{"value":"O'Brien'); DROP TABLE metrics;--"}],
                 "metricValues":[{"value":"1"}]}
            ]
        });
        assert_eq!(
            state
                .ingest_ga_response("ga4_ingest", "metrics", &response, false)
                .unwrap()["rows_ingested"],
            2
        );
        assert_eq!(
            state.summarize_table("ga4_ingest", "metrics").unwrap()["row_count"],
            2
        );
        assert!(state
            .ingest_ga_response("ga4_ingest", "metrics", &response, false)
            .is_err());
        assert_eq!(
            state
                .ingest_ga_response("ga4_ingest", "metrics", &response, true)
                .unwrap()["rows_ingested"],
            2
        );
        assert_eq!(
            state.summarize_table("ga4_ingest", "metrics").unwrap()["row_count"],
            4
        );
        let bad = json!({
            "dimensionHeaders":[{"name":"country"}], "metricHeaders":[{"name":"activeUsers"}],
            "rows":[{"dimensionValues":[{"value":"Mexico"}], "metricValues":[]}]
        });
        assert!(state
            .ingest_ga_response("ga4_ingest", "another", &bad, false)
            .is_err());
        assert_eq!(state.tables("ga4_ingest", None).unwrap()["total_tables"], 1);
        let malicious = json!({
            "dimensionHeaders":[{"name":"x\"; DROP TABLE metrics;--"}],
            "metricHeaders":[], "rows":[]
        });
        assert!(state
            .ingest_ga_response("ga4_ingest", "bad", &malicious, false)
            .is_err());
        assert_eq!(
            state.summarize_table("ga4_ingest", "metrics").unwrap()["row_count"],
            4
        );
        assert!(state
            .ingest_ga_response("missing", "new", &response, false)
            .is_err());
    }

    #[test]
    fn exports_private_bounded_json_and_markdown_evidence() {
        let state = ScratchState::new();
        state.open("analysis_1").unwrap();
        {
            let mut inner = state.guard().unwrap();
            let db = &inner.sessions.get_mut("analysis_1").unwrap().db;
            db.execute_batch(
                "CREATE TABLE sample (event_name VARCHAR, count INTEGER);
                 INSERT INTO sample VALUES ('page_view', 3), ('purchase', 1);",
            )
            .unwrap();
        }
        let export = state
            .export_evidence_bundle("analysis_1", Some(vec!["sample".into()]), Some(2))
            .unwrap();
        assert_eq!(export["tables"][0]["row_count"], 2);
        assert_eq!(
            export["tables"][0]["sample_rows"][0]["event_name"],
            "page_view"
        );
        assert!(export["markdown"]
            .as_str()
            .unwrap()
            .contains("| sample | 2 | 2 |"));
        assert_eq!(export["provider_called"], false);
        assert!(state
            .export_evidence_bundle("analysis_1", None, Some(21))
            .is_err());
        assert!(state
            .export_evidence_bundle(
                "analysis_1",
                Some(vec!["sample".into(), "sample".into()]),
                None
            )
            .is_err());
        assert!(state
            .export_evidence_bundle(
                "analysis_1",
                Some(vec!["bad\";DROP TABLE sample;--".into()]),
                None
            )
            .is_err());
        assert_eq!(
            state.summarize_table("analysis_1", "sample").unwrap()["row_count"],
            2
        );
    }

    #[test]
    fn describes_and_summarizes_bounded_local_tables() {
        let state = ScratchState::new();
        state.open("test_analysis").unwrap();
        {
            let mut inner = state.guard().unwrap();
            let db = &inner.sessions.get_mut("test_analysis").unwrap().db;
            db.execute_batch(
                "CREATE TABLE sample (id INTEGER, event_name VARCHAR);
                 INSERT INTO sample VALUES (1,'page_view'),(2,'login');",
            )
            .unwrap();
        }
        let desc = state.describe_table("test_analysis", "sample").unwrap();
        assert_eq!(desc["columns"][0]["name"], "id");
        assert_eq!(desc["column_count_returned"], 2);
        assert_eq!(
            state.summarize_table("test_analysis", "sample").unwrap()["row_count"],
            2
        );
        assert!(state.describe_table("test_analysis", "unknown").is_err());
        assert!(state
            .summarize_table("test_analysis", "x\";DROP TABLE sample;--")
            .is_err());
        assert_eq!(
            state.summarize_table("test_analysis", "sample").unwrap()["row_count"],
            2
        );
    }

    #[test]
    fn strict_sql_identifier_and_table_inventory() {
        let state = ScratchState::new();
        state.open("session_1").unwrap();
        {
            let mut inner = state.guard().unwrap();
            let db = &inner.sessions.get_mut("session_1").unwrap().db;
            db.execute_batch("CREATE TABLE synthetic_events (id INTEGER)")
                .unwrap();
        }
        assert_eq!(state.tables("session_1", None).unwrap()["total_tables"], 1);
        assert!(state.tables("session_1", Some(0)).is_err());
        assert!(state
            .drop_table("session_1", "x\"; DROP TABLE synthetic_events;--", false)
            .is_err());
        assert!(state.drop_table("session_1", "missing", false).is_err());
        assert_eq!(state.tables("session_1", Some(1)).unwrap()["count"], 1);
        state
            .drop_table("session_1", "synthetic_events", false)
            .unwrap();
        assert_eq!(state.tables("session_1", None).unwrap()["total_tables"], 0);
        assert_eq!(
            state
                .drop_table("session_1", "synthetic_events", true)
                .unwrap()["dropped"],
            false
        );
    }
}
