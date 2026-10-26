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

/// Strict bounded release-window analysis inputs. Every SQL identifier is checked
/// against DuckDB's main schema before a generated read-only projection is used.
pub struct ReleaseRegressionConfig {
    pub session_id: String,
    pub table_name: String,
    pub release_date: String,
    pub anchor_event: String,
    pub comparison_event: String,
    pub date_column: Option<String>,
    pub event_column: Option<String>,
    pub metric_column: Option<String>,
    pub pre_days: Option<u32>,
    pub transition_days: Option<u32>,
    pub post_days: Option<u32>,
}

fn release_windows(
    date: &str,
    pre: Option<u32>,
    transition: Option<u32>,
    post: Option<u32>,
) -> Result<
    (
        chrono::NaiveDate,
        chrono::NaiveDate,
        chrono::NaiveDate,
        chrono::NaiveDate,
    ),
    String,
> {
    use chrono::{Duration, NaiveDate};
    let release = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|_| "release_date must be YYYY-MM-DD")?;
    let pre = pre.unwrap_or(7);
    let transition = transition.unwrap_or(1);
    let post = post.unwrap_or(7);
    if !(1..=90).contains(&pre) || transition > 30 || !(1..=90).contains(&post) {
        return Err("Release windows require pre/post 1-90 days and transition 0-30".into());
    }
    let baseline = release
        .checked_sub_signed(Duration::days(i64::from(pre)))
        .ok_or("Invalid pre-release date")?;
    let follow = release
        .checked_add_signed(Duration::days(i64::from(transition)))
        .ok_or("Invalid transition window")?;
    let end = follow
        .checked_add_signed(Duration::days(i64::from(post)))
        .ok_or("Invalid post-release date")?;
    Ok((baseline, release, follow, end))
}

fn parse_event_date(value: &str) -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .or_else(|_| chrono::NaiveDate::parse_from_str(value, "%Y%m%d"))
        .ok()
}

/// Bounded pre/post landing-page query-parameter comparison.
pub struct LandingShiftConfig {
    pub session_id: String,
    pub table_name: String,
    pub release_date: String,
    pub date_column: Option<String>,
    pub landing_url_column: Option<String>,
    pub channel_column: Option<String>,
    pub source_medium_column: Option<String>,
    pub pre_days: Option<u32>,
    pub transition_days: Option<u32>,
    pub post_days: Option<u32>,
    pub top_n: Option<usize>,
}

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

    /// Limited SELECT-only query. The caller's SQL is parsed into a tightly
    /// defined grammar; only a new, identifier-quoted projection is executed.
    pub fn query_bounded(
        &self,
        raw: &str,
        sql: &str,
        max_rows: Option<usize>,
        max_cell_chars: Option<usize>,
        summary_only: bool,
    ) -> Result<Value, String> {
        let id = session_id(raw)?;
        let row_cap = max_rows.unwrap_or(100);
        let cell_cap = max_cell_chars.unwrap_or(1024);
        if row_cap == 0 || row_cap > 1000 || cell_cap == 0 || cell_cap > 2048 {
            return Err("Query bounds must be 1-1000 rows and 1-2048 cell characters".into());
        }
        let input = sql.trim();
        if input.is_empty() || input.len() > 2048 {
            return Err("SQL query must be 1-2048 bytes".into());
        }
        let statement = input.strip_suffix(';').unwrap_or(input).trim();
        if !statement
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b.is_ascii_whitespace() || b"_*,()".contains(&b))
        {
            return Err("Only a restricted SELECT grammar is allowed".into());
        }
        let tokens: Vec<&str> = statement.split_ascii_whitespace().collect();
        if tokens.len() < 4 || !tokens[0].eq_ignore_ascii_case("SELECT") {
            return Err("Query must start with SELECT columns FROM table".into());
        }
        let from = tokens
            .iter()
            .position(|s| s.eq_ignore_ascii_case("FROM"))
            .ok_or("SELECT requires FROM")?;
        if from < 2 || from + 1 >= tokens.len() {
            return Err("SELECT requires columns and a table".into());
        }
        let table = table_name(tokens[from + 1])?;
        // The owned vector below is necessary because SQL projection tokens
        // may contain whitespace between comma-separated column names.
        let projections: Vec<String> = tokens[1..from]
            .join(" ")
            .split(',')
            .map(|x| x.trim().to_owned())
            .collect();
        if projections.is_empty()
            || projections.len() > 32
            || projections.iter().any(|s| s.is_empty())
        {
            return Err("SELECT projection must have 1-32 columns".into());
        }
        let count = projections.len() == 1 && projections[0].eq_ignore_ascii_case("COUNT(*)");
        let wildcard = projections.len() == 1 && projections[0] == "*";
        if !count && !wildcard {
            for column in &projections {
                table_name(column)?;
            }
        }
        let mut order: Option<(String, &'static str)> = None;
        let mut sql_limit: Option<usize> = None;
        let mut offset = 0usize;
        let mut cursor = from + 2;
        while cursor < tokens.len() {
            if tokens[cursor].eq_ignore_ascii_case("ORDER") {
                if count
                    || order.is_some()
                    || cursor + 2 >= tokens.len()
                    || !tokens[cursor + 1].eq_ignore_ascii_case("BY")
                {
                    return Err("Invalid ORDER BY clause".into());
                }
                let name = table_name(tokens[cursor + 2])?.to_owned();
                cursor += 3;
                let mut dir = "ASC";
                if cursor < tokens.len() && tokens[cursor].eq_ignore_ascii_case("DESC") {
                    dir = "DESC";
                    cursor += 1;
                } else if cursor < tokens.len() && tokens[cursor].eq_ignore_ascii_case("ASC") {
                    cursor += 1;
                }
                order = Some((name, dir));
            } else if tokens[cursor].eq_ignore_ascii_case("LIMIT") {
                if sql_limit.is_some() || cursor + 1 >= tokens.len() {
                    return Err("Invalid LIMIT clause".into());
                }
                let limit = tokens[cursor + 1]
                    .parse::<usize>()
                    .map_err(|_| "Invalid LIMIT number")?;
                if limit == 0 || limit > 1000 {
                    return Err("LIMIT must be between 1 and 1000".into());
                }
                sql_limit = Some(limit);
                cursor += 2;
            } else if tokens[cursor].eq_ignore_ascii_case("OFFSET") {
                if offset != 0 || cursor + 1 >= tokens.len() {
                    return Err("Invalid OFFSET clause".into());
                }
                offset = tokens[cursor + 1]
                    .parse::<usize>()
                    .map_err(|_| "Invalid OFFSET number")?;
                if offset > 20000 {
                    return Err("OFFSET must be at most 20000".into());
                }
                cursor += 2;
            } else {
                return Err(
                    "Unsupported SQL clause: SELECT, ORDER BY, LIMIT and OFFSET only".into(),
                );
            }
        }
        let mut inner = self.guard()?;
        Self::expire(&mut inner);
        let db = &inner
            .sessions
            .get(id)
            .ok_or("Scratchpad session not found or expired")?
            .db;
        let mut schema = db
            .prepare(
                "SELECT column_name FROM information_schema.columns
             WHERE table_schema='main' AND table_name=?1
             ORDER BY ordinal_position LIMIT 33",
            )
            .map_err(|_| "Query schema inspection failed")?;
        let actual = schema
            .query_map([table], |row| row.get::<_, String>(0))
            .map_err(|_| "Query schema inspection failed")?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Query schema inspection failed")?;
        if actual.is_empty() || actual.len() > 32 {
            return Err("Table not found or exceeds 32-column query limit".into());
        }
        let columns: Vec<String> = if count {
            vec!["row_count".to_owned()]
        } else if wildcard {
            actual.clone()
        } else {
            for col in &projections {
                if !actual.iter().any(|a| a.eq_ignore_ascii_case(col)) {
                    return Err("Unknown SELECT column".into());
                }
            }
            projections.clone()
        };
        if let Some((column, _)) = &order {
            if !actual.iter().any(|a| a.eq_ignore_ascii_case(column)) {
                return Err("Unknown ORDER BY column".into());
            }
        }
        let projection = if count {
            "CAST(COUNT(*) AS VARCHAR) AS \"row_count\"".to_owned()
        } else {
            columns
                .iter()
                .map(|col| {
                    let quoted = col.replace('"', "\"\"");
                    format!("substr(CAST(\"{quoted}\" AS VARCHAR), 1, {cell_cap}) AS \"{quoted}\"")
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        let extra = row_cap.min(sql_limit.unwrap_or(row_cap));
        let table_sql = table.replace('"', "\"\"");
        let mut constructed = format!("SELECT {projection} FROM \"{table_sql}\"");
        if let Some((column, direction)) = &order {
            let safe = column.replace('"', "\"\"");
            constructed.push_str(&format!(" ORDER BY \"{safe}\" {direction}"));
        }
        let fetch = if count { 1 } else { extra.saturating_add(1) };
        constructed.push_str(&format!(" LIMIT {fetch} OFFSET {offset}"));
        let mut stmt = db
            .prepare(&constructed)
            .map_err(|_| "Read-only SELECT preparation failed")?;
        let mut rows = stmt
            .query_map([], |row| {
                let mut result = serde_json::Map::new();
                for (i, col) in columns.iter().enumerate() {
                    let value: Option<String> = row.get(i)?;
                    result.insert(col.clone(), value.map_or(Value::Null, |v| json!(v)));
                }
                Ok(Value::Object(result))
            })
            .map_err(|_| "Read-only SELECT failed")?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Read-only SELECT returned invalid values")?;
        let truncated = !count && rows.len() > extra;
        rows.truncate(extra);
        let returned = rows.len();
        if summary_only {
            rows.clear();
        }
        let output = json!({
            "session_id":id,"columns":columns,"rows":rows,
            "returned_rows":returned,"truncated":truncated,
            "summary_only":summary_only,"source":"local_scratchpad",
            "sql_subset":"SELECT columns FROM table [ORDER BY column] [LIMIT n] [OFFSET n]"
        });
        if serde_json::to_vec(&output).map_or(true, |v| v.len() > 256 * 1024) {
            return Err("Query response exceeds 256 KiB output policy".into());
        }
        Ok(output)
    }

    /// Aggregate bounded pre/post event ratios with conservative confidence flags.
    pub fn release_regression(&self, config: &ReleaseRegressionConfig) -> Result<Value, String> {
        let id = session_id(&config.session_id)?;
        let table = table_name(&config.table_name)?;
        let date_column = table_name(config.date_column.as_deref().unwrap_or("date"))?;
        let event_column = table_name(config.event_column.as_deref().unwrap_or("event_name"))?;
        let metric_column = match config.metric_column.as_deref() {
            Some(name) => Some(table_name(name)?),
            None => None,
        };
        let anchor = config.anchor_event.trim();
        let comparison = config.comparison_event.trim();
        if anchor.is_empty()
            || comparison.is_empty()
            || anchor == comparison
            || anchor.len() > 128
            || comparison.len() > 128
        {
            return Err("Anchor and comparison events must be distinct bounded names".into());
        }
        let (start, release, post_start, end) = release_windows(
            &config.release_date,
            config.pre_days,
            config.transition_days,
            config.post_days,
        )?;
        let mut inner = self.guard()?;
        Self::expire(&mut inner);
        let db = &inner
            .sessions
            .get(id)
            .ok_or("Scratchpad session not found or expired")?
            .db;
        let mut schema = db
            .prepare(
                "SELECT column_name FROM information_schema.columns
             WHERE table_schema='main' AND table_name=?1
             ORDER BY ordinal_position LIMIT 33",
            )
            .map_err(|_| "Release schema lookup failed")?;
        let actual = schema
            .query_map([table], |row| row.get::<_, String>(0))
            .map_err(|_| "Release schema lookup failed")?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Release schema lookup failed")?;
        let check = |name: &str| actual.iter().any(|a| a.eq_ignore_ascii_case(name));
        if actual.is_empty()
            || actual.len() > 32
            || !check(date_column)
            || !check(event_column)
            || metric_column.is_some_and(|name| !check(name))
        {
            return Err("Release diagnostic columns are absent from local table".into());
        }
        let metric_expr = metric_column.map_or("'1'".to_owned(), |name| {
            format!("substr(CAST(\"{name}\" AS VARCHAR),1,65)")
        });
        let sql = format!(
            "SELECT substr(CAST(\"{date_column}\" AS VARCHAR),1,33),
             substr(CAST(\"{event_column}\" AS VARCHAR),1,513),
             {metric_expr} FROM \"{table}\" LIMIT 20001"
        );
        let mut stmt = db
            .prepare(&sql)
            .map_err(|_| "Release SELECT preparation failed")?;
        let samples = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, Option<String>>(0)?,
                    row.get::<_, Option<String>>(1)?,
                    row.get::<_, Option<String>>(2)?,
                ))
            })
            .map_err(|_| "Release SELECT failed")?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Release SELECT result invalid")?;
        if samples.len() > 20000 {
            return Err("Release diagnostic scan exceeds 20000 rows".into());
        }
        let mut pre_anchor = 0.0f64;
        let mut pre_comparison = 0.0f64;
        let mut post_anchor = 0.0f64;
        let mut post_comparison = 0.0f64;
        let mut ignored = 0usize;
        for (date, event, weight) in samples {
            let Some(date) = date.as_deref().and_then(parse_event_date) else {
                ignored += 1;
                continue;
            };
            let Some(event) = event else {
                ignored += 1;
                continue;
            };
            if date < start || date >= end || (date >= release && date < post_start) {
                continue;
            }
            let weight: f64 = weight
                .as_deref()
                .unwrap_or("1")
                .parse()
                .map_err(|_| "Metric values must be finite nonnegative numbers")?;
            if !weight.is_finite() || weight < 0.0 || weight > 1e12 {
                return Err("Metric values must be finite nonnegative numbers".into());
            }
            if date < release {
                if event == anchor {
                    pre_anchor += weight;
                }
                if event == comparison {
                    pre_comparison += weight;
                }
            } else {
                if event == anchor {
                    post_anchor += weight;
                }
                if event == comparison {
                    post_comparison += weight;
                }
            }
        }
        let pre_ratio = if pre_comparison > 0.0 {
            Some(pre_anchor / pre_comparison)
        } else {
            None
        };
        let post_ratio = if post_comparison > 0.0 {
            Some(post_anchor / post_comparison)
        } else {
            None
        };
        let relative = match (pre_ratio, post_ratio) {
            (Some(a), Some(b)) if a > 0.0 => Some((b / a - 1.0) * 100.0),
            _ => None,
        };
        let confidence = match relative {
            None => "insufficient_baseline",
            Some(_) if pre_anchor < 10.0 || pre_comparison < 10.0 || post_comparison < 10.0 => {
                "low_sample"
            }
            Some(change) if change <= -30.0 => "possible_instrumentation_regression",
            Some(_) => "no_large_ratio_drop_detected",
        };
        Ok(json!({
            "session_id":id,"table_name":table,
            "release_date":config.release_date,
            "baseline_start":start.to_string(),
            "post_start":post_start.to_string(),
            "post_end_exclusive":end.to_string(),
            "pre_anchor":pre_anchor,"pre_comparison":pre_comparison,
            "post_anchor":post_anchor,"post_comparison":post_comparison,
            "pre_ratio":pre_ratio,"post_ratio":post_ratio,
            "relative_ratio_change_pct":relative,
            "assessment":confidence,"ignored_invalid_date_rows":ignored,
            "caution":"Heuristic association only; cannot establish causality or a definite instrumentation fault."
        }))
    }

    /// Compare query-parameter key frequency before and after a release.
    /// No user SQL or external URL fetch is performed.
    pub fn landing_param_shift(&self, config: &LandingShiftConfig) -> Result<Value, String> {
        use std::collections::{HashMap, HashSet};
        let id = session_id(&config.session_id)?;
        let table = table_name(&config.table_name)?;
        let date_column = table_name(config.date_column.as_deref().unwrap_or("date_parsed"))?;
        let url_column = table_name(
            config
                .landing_url_column
                .as_deref()
                .unwrap_or("landingpageplusquerystring"),
        )?;
        let channel_column = config
            .channel_column
            .as_deref()
            .map(table_name)
            .transpose()?;
        let source_column = config
            .source_medium_column
            .as_deref()
            .map(table_name)
            .transpose()?;
        let top_n = config.top_n.unwrap_or(100);
        if top_n == 0 || top_n > 200 {
            return Err("top_n must be between 1 and 200".into());
        }
        let (start, release, post_start, end) = release_windows(
            &config.release_date,
            config.pre_days,
            config.transition_days,
            config.post_days,
        )?;
        let mut inner = self.guard()?;
        Self::expire(&mut inner);
        let db = &inner
            .sessions
            .get(id)
            .ok_or("Scratchpad session not found or expired")?
            .db;
        let mut stmt = db
            .prepare(
                "SELECT column_name FROM information_schema.columns
             WHERE table_schema='main' AND table_name=?1
             ORDER BY ordinal_position LIMIT 33",
            )
            .map_err(|_| "Landing schema lookup failed")?;
        let fields = stmt
            .query_map([table], |row| row.get::<_, String>(0))
            .map_err(|_| "Landing schema lookup failed")?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Landing schema lookup failed")?;
        let exists = |name: &str| fields.iter().any(|field| field.eq_ignore_ascii_case(name));
        if fields.is_empty()
            || fields.len() > 32
            || !exists(date_column)
            || !exists(url_column)
            || channel_column.is_some_and(|name| !exists(name))
            || source_column.is_some_and(|name| !exists(name))
        {
            return Err("Landing diagnostic columns are absent".into());
        }
        let cast = |name: &str, max: usize| format!("substr(CAST(\"{name}\" AS VARCHAR),1,{max})");
        let channel_sql = channel_column.map_or("'all'".to_owned(), |name| cast(name, 129));
        let source_sql = source_column.map_or("'all'".to_owned(), |name| cast(name, 129));
        let sql = format!(
            "SELECT {},{},{channel_sql},{source_sql} FROM \"{table}\" LIMIT 20001",
            cast(date_column, 33),
            cast(url_column, 2049)
        );
        let mut statement = db.prepare(&sql).map_err(|_| "Landing projection failed")?;
        let rows = statement
            .query_map([], |row| {
                Ok((
                    row.get::<_, Option<String>>(0)?,
                    row.get::<_, Option<String>>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, Option<String>>(3)?,
                ))
            })
            .map_err(|_| "Landing projection failed")?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Landing projection returned invalid rows")?;
        if rows.len() > 20000 {
            return Err("Landing analysis scan exceeds 20000 rows".into());
        }
        let mut groups: HashMap<(String, String, String), (usize, usize)> = HashMap::new();
        let mut pre_rows = 0usize;
        let mut post_rows = 0usize;
        let mut ignored = 0usize;
        for (date, url, channel, source) in rows {
            let Some(date) = date.as_deref().and_then(parse_event_date) else {
                ignored += 1;
                continue;
            };
            let phase = if date >= start && date < release {
                0
            } else if date >= post_start && date < end {
                1
            } else {
                continue;
            };
            let Some(url) = url else {
                ignored += 1;
                continue;
            };
            if url.len() > 2048 {
                return Err("Landing URL exceeds 2048-byte limit".into());
            }
            let channel = channel.unwrap_or_else(|| "unknown".into());
            let source = source.unwrap_or_else(|| "unknown".into());
            if channel.len() > 128 || source.len() > 128 {
                return Err("Channel/source segment exceeds 128-byte limit".into());
            }
            if phase == 0 {
                pre_rows += 1;
            } else {
                post_rows += 1;
            }
            let Some((_, query)) = url.split_once('?') else {
                continue;
            };
            let query = query.split('#').next().unwrap_or("");
            let mut seen = HashSet::new();
            for field in query.split('&').take(33) {
                let key = field.split('=').next().unwrap_or("").trim();
                if key.is_empty() {
                    continue;
                }
                if key.len() > 64
                    || !key
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
                {
                    continue;
                }
                if !seen.insert(key.to_owned()) {
                    continue;
                }
                if groups.len() >= 2000
                    && !groups.contains_key(&(key.to_owned(), channel.clone(), source.clone()))
                {
                    return Err("Landing parameter group cap of 2000 exceeded".into());
                }
                let entry = groups
                    .entry((key.to_owned(), channel.clone(), source.clone()))
                    .or_insert((0, 0));
                if phase == 0 {
                    entry.0 += 1;
                } else {
                    entry.1 += 1;
                }
            }
        }
        let mut ranked: Vec<_> = groups
            .into_iter()
            .map(|((name, channel, source), (pre, post))| {
                let delta = post as i64 - pre as i64;
                json!({"parameter":name,"channel":channel,"source_medium":source,
                   "pre_occurrences":pre,"post_occurrences":post,"delta":delta})
            })
            .collect();
        ranked.sort_by(|a, b| {
            b["delta"]
                .as_i64()
                .unwrap_or(0)
                .unsigned_abs()
                .cmp(&a["delta"].as_i64().unwrap_or(0).unsigned_abs())
                .then_with(|| a["parameter"].as_str().cmp(&b["parameter"].as_str()))
                .then_with(|| a["channel"].as_str().cmp(&b["channel"].as_str()))
                .then_with(|| {
                    a["source_medium"]
                        .as_str()
                        .cmp(&b["source_medium"].as_str())
                })
        });
        let total = ranked.len();
        ranked.truncate(top_n);
        let output = json!({
            "session_id":id,"table_name":table,
            "release_date":config.release_date,
            "baseline_start":start.to_string(),
            "post_start":post_start.to_string(),"post_end_exclusive":end.to_string(),
            "pre_rows":pre_rows,"post_rows":post_rows,
            "invalid_date_or_url_rows":ignored,
            "groups_total":total,"truncated":total>ranked.len(),
            "shifts":ranked,
            "caution":"Parameter-key counts measure observed URL prevalence, not causal attribution."
        });
        if serde_json::to_vec(&output).map_or(true, |v| v.len() > 256 * 1024) {
            return Err("Landing shift report exceeds 256 KiB".into());
        }
        Ok(output)
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
    fn aggregate_evidence_bundle_rejects_oversized_output_without_altering_tables() {
        let state = ScratchState::new();
        state.open("bounded_bundle").unwrap();
        {
            let mut inner = state.guard().unwrap();
            let db = &inner.sessions.get_mut("bounded_bundle").unwrap().db;
            for index in 0..8 {
                db.execute_batch(&format!(
                    "CREATE TABLE sample_{index} (value VARCHAR);
                     INSERT INTO sample_{index}
                     SELECT repeat('x', 1900) FROM range(20);"
                ))
                .unwrap();
            }
        }
        assert_eq!(
            state.tables("bounded_bundle", None).unwrap()["total_tables"],
            8
        );
        let too_large = state
            .export_evidence_bundle("bounded_bundle", None, Some(20))
            .unwrap_err();
        assert!(too_large.contains("256 KiB"), "{too_large}");
        let minimal = state
            .export_evidence_bundle("bounded_bundle", None, Some(0))
            .unwrap();
        assert_eq!(minimal["tables"].as_array().unwrap().len(), 8);
        assert!(minimal["tables"]
            .as_array()
            .unwrap()
            .iter()
            .all(|table| table["sample_rows"].as_array().unwrap().is_empty()));
        let bounded = state
            .export_evidence_bundle("bounded_bundle", Some(vec!["sample_0".into()]), Some(1))
            .unwrap();
        assert_eq!(bounded["tables"][0]["row_count"], 20);
        assert_eq!(
            bounded["tables"][0]["sample_rows"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        assert!(serde_json::to_vec(&bounded).unwrap().len() <= 256 * 1024);
        assert_eq!(
            state.summarize_table("bounded_bundle", "sample_0").unwrap()["row_count"],
            20
        );
    }

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
    fn session_quota_expiry_and_limit_reconfiguration_are_isolated() {
        let state = ScratchState::new();
        assert_eq!(state.limits().unwrap()["max_sessions"], 4);
        assert_eq!(state.limits().unwrap()["max_tables_per_session"], 8);
        assert_eq!(
            state.set_limits(Some(2), Some(1)).unwrap(),
            json!({"max_sessions":2,"max_tables_per_session":1})
        );
        assert_eq!(state.open("alpha").unwrap()["refreshed"], false);
        assert_eq!(state.open("alpha").unwrap()["refreshed"], true);
        assert_eq!(state.open("beta").unwrap()["refreshed"], false);
        assert!(state.open("gamma").unwrap_err().contains("quota"));
        assert_eq!(
            state.sessions(Some(1)).unwrap()["session_ids"],
            json!(["alpha"])
        );
        assert_eq!(state.sessions(Some(1)).unwrap()["total_sessions"], 2);
        assert_eq!(state.sessions(Some(1)).unwrap()["truncated"], true);
        assert_eq!(
            state.sessions(None).unwrap()["session_ids"],
            json!(["alpha", "beta"])
        );
        assert!(state.sessions(Some(0)).is_err());
        assert!(state.sessions(Some(MAX_LIST_ITEMS + 1)).is_err());
        assert!(state.set_limits(Some(1), None).is_err());
        assert!(state.set_limits(Some(HARD_SESSIONS + 1), None).is_err());
        assert!(state.set_limits(None, Some(HARD_TABLES + 1)).is_err());
        assert_eq!(state.limits().unwrap()["max_tables_per_session"], 1);

        {
            let mut guard = state.guard().unwrap();
            guard.sessions.get_mut("alpha").unwrap().expires =
                Instant::now() - Duration::from_secs(1);
        }
        assert!(state.require_session("alpha").is_err());
        assert_eq!(
            state.sessions(None).unwrap()["session_ids"],
            json!(["beta"])
        );
        assert_eq!(state.open("alpha").unwrap()["refreshed"], false);
        assert_eq!(state.close("beta").unwrap()["closed"], true);
        assert!(state.close("beta").is_err());
        assert_eq!(state.close("alpha").unwrap()["session_id"], "alpha");
        assert_eq!(state.sessions(None).unwrap()["total_sessions"], 0);
    }

    #[test]
    fn table_quota_and_private_catalog_survive_rejected_operations() {
        let state = ScratchState::new();
        state.set_limits(Some(2), Some(1)).unwrap();
        state.open("primary").unwrap();
        state.open("secondary").unwrap();
        let payload = json!({
            "dimensionHeaders":[{"name":"country"}],
            "metricHeaders":[{"name":"activeUsers"}],
            "rows":[
                {"dimensionValues":[{"value":"Mexico"}],
                 "metricValues":[{"value":"7"}]},
                {"dimensionValues":[{"value":"Canada"}],
                 "metricValues":[{"value":"3"}]}
            ]
        });
        assert_eq!(
            state
                .ingest_ga_response("primary", "events", &payload, false)
                .unwrap()["rows_ingested"],
            2
        );
        assert!(state
            .ingest_ga_response("primary", "second", &payload, false)
            .is_err());
        assert_eq!(
            state.tables("primary", None).unwrap()["tables"],
            json!(["events"])
        );
        assert!(state.tables("secondary", None).unwrap()["tables"]
            .as_array()
            .unwrap()
            .is_empty());
        assert!(state.describe_table("secondary", "events").is_err());
        assert_eq!(
            state.describe_table("primary", "events").unwrap()["columns"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            state.summarize_table("primary", "events").unwrap()["row_count"],
            2
        );
        assert_eq!(
            state
                .query_bounded("primary", "SELECT COUNT(*) FROM events", None, None, false)
                .unwrap()["rows"][0]["row_count"],
            "2"
        );
        assert!(state.drop_table("secondary", "events", false).is_err());
        assert!(state.drop_table("primary", "bad;DROP", true).is_err());
        assert_eq!(
            state.drop_table("primary", "events", false).unwrap()["dropped"],
            true
        );
        assert_eq!(
            state.drop_table("primary", "events", true).unwrap()["dropped"],
            false
        );
        assert!(state.drop_table("primary", "events", false).is_err());
        assert_eq!(state.tables("primary", None).unwrap()["total_tables"], 0);
        assert_eq!(
            state
                .ingest_ga_response("primary", "again", &payload, false)
                .unwrap()["rows_ingested"],
            2
        );
        assert_eq!(
            state.summarize_table("primary", "again").unwrap()["row_count"],
            2
        );
        state.close("primary").unwrap();
        assert!(state.tables("primary", None).is_err());
        assert!(state
            .query_bounded("secondary", "SELECT * FROM again", None, None, false)
            .is_err());
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
    fn landing_shift_finds_bounded_pre_post_parameter_changes() {
        let state = ScratchState::new();
        state.open("landing_test").unwrap();
        let payload = json!({
            "dimensionHeaders":[
                {"name":"date_parsed"},
                {"name":"landingpageplusquerystring"},
                {"name":"channel"},
                {"name":"source_medium"}
            ],
            "metricHeaders":[],
            "rows":[
                {"dimensionValues":[
                    {"value":"2026-10-15"},{"value":"/home?utm_campaign=old&utm_source=search"},
                    {"value":"Organic"},{"value":"google/organic"}],"metricValues":[]},
                {"dimensionValues":[
                    {"value":"2026-10-16"},{"value":"/home?utm_campaign=old"},
                    {"value":"Organic"},{"value":"google/organic"}],"metricValues":[]},
                {"dimensionValues":[
                    {"value":"2026-10-28"},{"value":"/home?gclid=123&utm_source=search"},
                    {"value":"Organic"},{"value":"google/organic"}],"metricValues":[]}
            ]
        });
        state
            .ingest_ga_response("landing_test", "landing", &payload, false)
            .unwrap();
        let mut config = LandingShiftConfig {
            session_id: "landing_test".into(),
            table_name: "landing".into(),
            release_date: "2026-10-25".into(),
            date_column: None,
            landing_url_column: None,
            channel_column: Some("channel".into()),
            source_medium_column: Some("source_medium".into()),
            pre_days: Some(14),
            transition_days: Some(1),
            post_days: Some(7),
            top_n: Some(10),
        };
        let report = state.landing_param_shift(&config).unwrap();
        assert_eq!(report["pre_rows"], 2);
        assert_eq!(report["post_rows"], 1);
        assert!(report["shifts"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s["parameter"] == "utm_campaign" && s["delta"] == -2));
        assert!(report["shifts"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s["parameter"] == "gclid" && s["delta"] == 1));
        config.top_n = Some(0);
        assert!(state.landing_param_shift(&config).is_err());
        config.top_n = Some(10);
        config.channel_column = Some("not-a-column".into());
        assert!(state.landing_param_shift(&config).is_err());
        assert_eq!(
            state.summarize_table("landing_test", "landing").unwrap()["row_count"],
            3
        );
    }

    #[test]
    fn release_diagnostic_distinguishes_ratio_drop_and_low_sample() {
        let state = ScratchState::new();
        state.open("release_test").unwrap();
        {
            let mut inner = state.guard().unwrap();
            let db = &inner.sessions.get_mut("release_test").unwrap().db;
            db.execute_batch(
                "CREATE TABLE events(date VARCHAR,event_name VARCHAR,weight VARCHAR);
                 INSERT INTO events VALUES
                 ('2026-10-15','anchor','80'),('2026-10-15','comparison','100'),
                 ('2026-10-27','anchor','10'),('2026-10-27','comparison','100');",
            )
            .unwrap();
        }
        let mut config = ReleaseRegressionConfig {
            session_id: "release_test".into(),
            table_name: "events".into(),
            release_date: "2026-10-25".into(),
            anchor_event: "anchor".into(),
            comparison_event: "comparison".into(),
            date_column: None,
            event_column: None,
            metric_column: Some("weight".into()),
            pre_days: Some(14),
            transition_days: Some(1),
            post_days: Some(7),
        };
        let report = state.release_regression(&config).unwrap();
        assert_eq!(report["pre_anchor"], 80.0);
        assert_eq!(report["post_comparison"], 100.0);
        assert_eq!(report["assessment"], "possible_instrumentation_regression");
        config.anchor_event = "comparison".into();
        assert!(state.release_regression(&config).is_err());
        config.anchor_event = "anchor".into();
        config.date_column = Some("bad\";DROP TABLE events;--".into());
        assert!(state.release_regression(&config).is_err());
        config.date_column = None;
        config.pre_days = Some(0);
        assert!(state.release_regression(&config).is_err());
        assert_eq!(
            state.summarize_table("release_test", "events").unwrap()["row_count"],
            4
        );
    }

    #[test]
    fn restricted_select_query_is_bounded_and_never_executes_user_sql() {
        let state = ScratchState::new();
        state.open("query_test").unwrap();
        let payload = json!({
            "dimensionHeaders":[{"name":"country"}],
            "metricHeaders":[{"name":"users"}],
            "rows":[
                {"dimensionValues":[{"value":"Mexico"}],"metricValues":[{"value":"7"}]},
                {"dimensionValues":[{"value":"Canada"}],"metricValues":[{"value":"3"}]}
            ]
        });
        state
            .ingest_ga_response("query_test", "events", &payload, false)
            .unwrap();
        let results = state
            .query_bounded(
                "query_test",
                "SELECT country, users FROM events ORDER BY country ASC LIMIT 1",
                Some(1),
                Some(50),
                false,
            )
            .unwrap();
        assert_eq!(results["rows"][0]["country"], "Canada");
        assert_eq!(results["returned_rows"], 1);
        let count = state
            .query_bounded(
                "query_test",
                "SELECT COUNT(*) FROM events",
                None,
                None,
                false,
            )
            .unwrap();
        assert_eq!(count["rows"][0]["row_count"], "2");
        let summary = state
            .query_bounded("query_test", "SELECT * FROM events", Some(1), None, true)
            .unwrap();
        assert!(summary["rows"].as_array().unwrap().is_empty());
        assert_eq!(summary["returned_rows"], 1);
        assert_eq!(summary["truncated"], true);
        for bad in [
            "DROP TABLE events",
            "WITH x AS (SELECT * FROM events) SELECT * FROM x",
            "SELECT * FROM events; DROP TABLE events",
            "SELECT read_csv_auto('/etc/passwd') FROM events",
            "SELECT * FROM events WHERE country = Mexico",
            "SELECT * FROM events UNION SELECT * FROM events",
            "SELECT unknown FROM events",
            "SELECT * FROM events LIMIT 1001",
            "SELECT * FROM events OFFSET 20001",
            "SELECT * FROM events -- comment",
        ] {
            assert!(
                state
                    .query_bounded("query_test", bad, None, None, false)
                    .is_err(),
                "{bad}"
            );
        }
        assert_eq!(
            state.summarize_table("query_test", "events").unwrap()["row_count"],
            2
        );
    }

    #[test]
    fn duckdb_engine_denies_external_file_access_beyond_restricted_sql_parser() {
        let state = ScratchState::new();
        state.open("external_gate").unwrap();
        let mut inner = state.guard().unwrap();
        let db = &mut inner.sessions.get_mut("external_gate").unwrap().db;
        let external: bool = db
            .query_row(
                "SELECT current_setting('enable_external_access')",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(!external, "DuckDB must keep external access disabled");
        for hostile in [
            "SELECT * FROM read_csv_auto('/etc/passwd')",
            "SELECT * FROM read_text('/etc/hostname')",
            "COPY (SELECT 'synthetic') TO '/dev/null'",
            "SET enable_external_access=true",
        ] {
            assert!(
                db.execute_batch(hostile).is_err(),
                "DuckDB accepted external operation despite sandboxing"
            );
        }
        drop(inner);
        assert!(state
            .query_bounded(
                "external_gate",
                "SELECT * FROM read_csv('/etc/passwd')",
                None,
                None,
                false
            )
            .is_err());
        assert_eq!(state.tables("external_gate", None).unwrap()["total_tables"], 0);
    }

    #[test]
    fn hostile_sql_corpus_and_deterministic_mutations_never_execute() {
        let state = ScratchState::new();
        state.open("hostile_sql").unwrap();
        let payload = json!({
            "dimensionHeaders":[{"name":"country"}],
            "metricHeaders":[{"name":"users"}],
            "rows":[
                {"dimensionValues":[{"value":"Mexico"}],"metricValues":[{"value":"7"}]},
                {"dimensionValues":[{"value":"Canada"}],"metricValues":[{"value":"3"}]}
            ]
        });
        state
            .ingest_ga_response("hostile_sql", "events", &payload, false)
            .unwrap();

        let attempts = [
            "ATTACH '/tmp/evil.db' AS secret",
            "INSTALL httpfs",
            "LOAD httpfs",
            "COPY events TO '/tmp/private.csv'",
            "EXPORT DATABASE '/tmp/private'",
            "PRAGMA enable_external_access=true",
            "SET enable_external_access=true",
            "DELETE FROM events",
            "UPDATE events SET users=999",
            "INSERT INTO events VALUES ('Fake',999)",
            "CREATE TABLE destroyed AS SELECT * FROM events",
            "DROP TABLE events",
            "ALTER TABLE events RENAME TO destroyed",
            "TRUNCATE events",
            "SELECT * FROM read_csv_auto('/etc/passwd')",
            "SELECT * FROM read_parquet('/tmp/file.parquet')",
            "SELECT * FROM httpfs_read('https://invalid.example')",
            "SELECT * FROM events; DROP TABLE events;",
            "SELECT * FROM events /* sneaky */",
            "SELECT * FROM events -- suspicious",
            "SELECT * FROM events UNION ALL SELECT * FROM events",
            "SELECT * FROM events JOIN events e2 ON true",
            "SELECT * FROM events WHERE 1=1",
            "SELECT * FROM events ORDER BY country; DELETE FROM events",
            "SELECT * FROM events LIMIT 1 OFFSET 999999",
            "SELECT * FROM events LIMIT 0",
            "SELECT * FROM events LIMIT 1001",
            "SELECT * FROM events OFFSET 20001",
            "SELECT * FROM events ORDER BY random()",
            "SELECT * FROM events ORDER BY country DESC DESC",
            "SELECT * FROM events OFFSET -1",
            "SELECT * FROM events LIMIT 18446744073709551616",
            "SELECT * FROM events\x00DROP",
            "SELECT * FROM events\u{202e}DROP",
        ];
        for (index, sql) in attempts.into_iter().enumerate() {
            assert!(
                state
                    .query_bounded("hostile_sql", sql, None, None, false)
                    .is_err(),
                "Hostile SQL fixture {index} unexpectedly accepted"
            );
        }

        // Deterministic mutation corpus (no random seed, clocks, or environmental input).
        // Each variant tries to append a forbidden SQL expression or source escape
        // after syntactically valid SELECT, ORDER BY, or LIMIT clauses.
        let prefixes = [
            "SELECT * FROM events",
            "SELECT country, users FROM events ORDER BY country ASC",
            "SELECT COUNT(*) FROM events",
            "SELECT users FROM events LIMIT 1",
        ];
        let payloads = [
            ";DROP TABLE events",
            "' OR '1'='1",
            "\";COPY events TO '/tmp/leak.csv'",
            " /*LOAD httpfs*/",
            " -- bypass",
            " UNION SELECT * FROM events",
            " WHERE country=Mexico",
            " JOIN events USING(country)",
            " ATTACH '/tmp/x.db'",
            " \u{202e}INSERT",
            "\x00 DELETE FROM events",
        ];
        let mut cases = 0;
        for prefix in prefixes {
            for suffix in payloads {
                let mutated = format!("{prefix}{suffix}");
                assert!(
                    state
                        .query_bounded("hostile_sql", &mutated, Some(10), Some(32), false)
                        .is_err(),
                    "Mutated SQL unexpectedly accepted at case {cases}"
                );
                cases += 1;
            }
        }
        assert_eq!(cases, 44);
        assert_eq!(
            state.summarize_table("hostile_sql", "events").unwrap()["row_count"],
            2
        );
        assert_eq!(
            state
                .query_bounded(
                    "hostile_sql",
                    "SELECT COUNT(*) FROM events",
                    None,
                    None,
                    false
                )
                .unwrap()["rows"][0]["row_count"],
            "2"
        );
        assert_eq!(
            state.tables("hostile_sql", None).unwrap()["total_tables"],
            1
        );
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
