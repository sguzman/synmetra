//! Synthetic Contract V1 golden-output capture executed against the exact
//! pinned SednaLabs crate at CI time; not linked into Synmetra itself.
use ga4_mcp::contract::{self, ColumnMeta, OutputMode, TabularMeta};
use ga4_mcp::error::AnalyticsError;
use serde_json::{json, Value};

fn payload(result: rmcp::model::CallToolResult) -> Value {
    result
        .structured_content
        .expect("Pinned upstream must emit structured content")
}

#[test]
fn capture_pinned_contract_v1_goldens() {
    let mut compact = TabularMeta::rows(100, 25, vec![ColumnMeta::new("event_name")]);
    compact.output_mode = OutputMode::Compact;
    compact.summary_only = false;
    compact.next_cursor = Some("v1:abcd:25".to_owned());
    compact.query_hash = Some("abcd".to_owned());
    let compact = payload(contract::success_tabular(
        Some(json!([{"event_name":"page_view"}])),
        compact,
        8,
    ));

    let mut summary = TabularMeta::rows(10, 5, vec![ColumnMeta::new("event_name")]);
    summary.summary_only = true;
    summary.next_cursor = Some("v1:abc:5".to_owned());
    let summary = payload(contract::success_tabular(
        Some(json!([{"event_name":"page_view"}])),
        summary,
        9,
    ));

    let denied = payload(contract::error(
        AnalyticsError::policy_denied("read_only", "scratchpad_query"),
        3,
    ));
    let cursor_error = payload(contract::error(AnalyticsError::CursorQueryMismatch, 5));

    assert_eq!(compact["meta"]["row_count_total"], 100);
    assert_eq!(compact["meta"]["row_count_returned"], 25);
    assert_eq!(compact["meta"]["next_cursor"], "v1:abcd:25");
    assert_eq!(compact["meta"]["output_mode"], "compact");
    assert_eq!(compact["meta"]["elapsed_ms"], 8);
    assert_eq!(compact["data"][0]["event_name"], "page_view");
    assert!(summary["data"].is_null());
    assert_eq!(summary["meta"]["next_cursor"], "v1:abc:5");
    assert_eq!(summary["meta"]["summary_only"], true);
    assert_eq!(summary["meta"]["elapsed_ms"], 9);
    assert_eq!(denied["error"]["code"], "POLICY_DENIED");
    assert_eq!(denied["error"]["category"], "policy");
    assert_eq!(cursor_error["error"]["code"], "CURSOR_QUERY_MISMATCH");
    assert_eq!(cursor_error["error"]["reason"], "invalid_cursor");
    assert_eq!(cursor_error["error"]["category"], "cursor");

    let artifact = json!({
        "schema":"synmetra_pinned_executed_upstream_contract_v1",
        "version":1,
        "source":{
            "repository":"sednalabs/ga4-mcp",
            "commit":"039fd4620064fc4b76dc6bca641945aba52c14bc",
            "files":["src/contract.rs","tests/contract_safety_conformance.rs"],
            "execution":"cargo test --locked --test synmetra_contract_golden",
            "environment":"No Google credentials, synthetic contract values only"
        },
        "goldens":[
            {"name":"compact_source_assertions","structured_content":compact},
            {"name":"summary_only_source_assertions","structured_content":summary},
            {"name":"policy_denied_source_assertions","structured_content":denied},
            {"name":"cursor_query_mismatch_source_assertions","structured_content":cursor_error}
        ]
    });
    let output = std::env::var("SYNMETRA_GOLDEN_OUTPUT")
        .expect("CI supplies a private ephemeral JSON output path");
    std::fs::write(&output, serde_json::to_vec_pretty(&artifact).unwrap()).unwrap();
    println!("PINNED_SEDNALABS_GOLDENS={output}");
}
