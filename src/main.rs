//! Native MCP stdio server. Only implemented tools are advertised.
mod auth;
mod google;

use google::{report_request, ApiError, GoogleClient};
use rmcp::{
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
    schemars::JsonSchema,
    tool, tool_handler, tool_router, ServerHandler, ServiceExt,
    transport::stdio,
};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Clone)]
struct Synmetra {
    api: GoogleClient,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct PropertyArgs {
    /// Positive numeric GA4 property ID or canonical properties/<id>.
    property_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct AccountArgs {
    /// Positive GA4 account number or canonical accounts/<id>.
    account_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct RawPropertyRequestArgs {
    /// Positive GA4 property number or canonical properties/<id>.
    property_id: String,
    /// Provider-compatible Google API request JSON. Maximum 64 KiB.
    request: Value,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct RawAccountRequestArgs {
    /// Positive GA4 account number or canonical accounts/<id>.
    account_id: String,
    /// Provider-compatible Google Admin API access-report request JSON. Maximum 64 KiB.
    request: Value,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ReportArgs {
    /// Positive numeric GA4 property ID or canonical properties/<id>.
    property_id: String,
    /// Historical date ranges, e.g. [{"startDate":"7daysAgo","endDate":"today"}].
    date_ranges: Vec<Value>,
    /// Names of dimensions (not Dimension objects).
    #[serde(default)]
    dimensions: Vec<String>,
    /// Names of metrics (not Metric objects).
    metrics: Vec<String>,
    /// Optional Google FilterExpression.
    #[serde(default)]
    dimension_filter: Option<Value>,
    /// Optional Google FilterExpression.
    #[serde(default)]
    metric_filter: Option<Value>,
    /// Optional Google OrderBy structures.
    #[serde(default)]
    order_bys: Option<Vec<Value>>,
    /// Limit rows returned.
    #[serde(default)]
    limit: Option<u32>,
    /// Pagination offset.
    #[serde(default)]
    offset: Option<u64>,
    /// ISO currency code.
    #[serde(default)]
    currency_code: Option<String>,
    #[serde(default)]
    keep_empty_rows: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct RealtimeArgs {
    property_id: String,
    #[serde(default)]
    dimensions: Vec<String>,
    metrics: Vec<String>,
    #[serde(default)]
    dimension_filter: Option<Value>,
    #[serde(default)]
    metric_filter: Option<Value>,
    #[serde(default)]
    limit: Option<u32>,
}

fn result(r: Result<Value, ApiError>) -> CallToolResult {
    match r {
        Ok(v) => CallToolResult::success(vec![ContentBlock::text(json!({"ok": true, "data": v}).to_string())]),
        Err(error) => CallToolResult::error(vec![ContentBlock::text(
            json!({"ok": false, "error": {"reason": error.to_string()}}).to_string()
        )]),
    }
}

#[tool_router]
impl Synmetra {
    #[tool(description = "List the Google Analytics accounts and properties accessible with the configured Google token.")]
    async fn get_account_summaries(&self) -> CallToolResult {
        result(self.api.account_summaries().await)
    }

    #[tool(description = "Read one Google Analytics 4 property, by numeric property ID or properties/<id>.")]
    async fn get_property_details(&self, Parameters(args): Parameters<PropertyArgs>) -> CallToolResult {
        result(self.api.property(&args.property_id).await)
    }

    #[tool(description = "Get an Analytics account's data-sharing settings.")]
    async fn get_account_data_sharing_settings(&self, Parameters(args): Parameters<AccountArgs>) -> CallToolResult {
        result(self.api.account_sharing(&args.account_id).await)
    }

    #[tool(description = "Get a GA4 property's event and user data retention settings.")]
    async fn get_property_data_retention_settings(&self, Parameters(args): Parameters<PropertyArgs>) -> CallToolResult {
        result(self.api.retention_settings(&args.property_id).await)
    }

    #[tool(description = "List Google Ads account links for one GA4 property.")]
    async fn list_google_ads_links(&self, Parameters(args): Parameters<PropertyArgs>) -> CallToolResult {
        result(self.api.ads_links(&args.property_id).await)
    }

    #[tool(description = "List reporting annotations attached to one GA4 property (read-only).")]
    async fn list_property_annotations(&self, Parameters(args): Parameters<PropertyArgs>) -> CallToolResult {
        result(self.api.annotations(&args.property_id).await)
    }

    #[tool(description = "List property-specific custom dimensions and metrics.")]
    async fn get_custom_dimensions_and_metrics(&self, Parameters(args): Parameters<PropertyArgs>) -> CallToolResult {
        result(self.api.custom_definitions(&args.property_id).await)
    }

    #[tool(description = "Run Google Analytics Data API v1alpha conversion reporting with conversionSpec or conversion metrics. Requires an eligible property.")]
    async fn run_conversions_report(&self, Parameters(args): Parameters<RawPropertyRequestArgs>) -> CallToolResult {
        result(self.api.alpha_report(&args.property_id, args.request, false).await)
    }

    #[tool(description = "Run Google Analytics Data API v1alpha funnel reporting with 1-10 funnel steps.")]
    async fn run_funnel_report(&self, Parameters(args): Parameters<RawPropertyRequestArgs>) -> CallToolResult {
        result(self.api.alpha_report(&args.property_id, args.request, true).await)
    }

    #[tool(description = "Run a GA4 pivot report using the provider's RunPivotReportRequest JSON.")]
    async fn run_pivot_report(&self, Parameters(args): Parameters<RawPropertyRequestArgs>) -> CallToolResult {
        result(self.api.data_operation(&args.property_id, args.request, "runPivotReport").await)
    }

    #[tool(description = "Execute one to five GA4 historical reports in a single batch using a requests JSON array.")]
    async fn batch_run_reports(&self, Parameters(args): Parameters<RawPropertyRequestArgs>) -> CallToolResult {
        result(self.api.data_operation(&args.property_id, args.request, "batchRunReports").await)
    }

    #[tool(description = "Check GA4 Core Data API metric/dimension compatibility using CheckCompatibilityRequest JSON.")]
    async fn check_report_compatibility(&self, Parameters(args): Parameters<RawPropertyRequestArgs>) -> CallToolResult {
        result(self.api.data_operation(&args.property_id, args.request, "checkCompatibility").await)
    }

    #[tool(description = "Run a GA4 property-level data access report (requires Google Analytics Administrator privileges).")]
    async fn run_property_access_report(&self, Parameters(args): Parameters<RawPropertyRequestArgs>) -> CallToolResult {
        result(self.api.access_report(&args.property_id, args.request, false).await)
    }

    #[tool(description = "Run an account-level data access report (requires Google Analytics Administrator privileges).")]
    async fn run_account_access_report(&self, Parameters(args): Parameters<RawAccountRequestArgs>) -> CallToolResult {
        result(self.api.access_report(&args.account_id, args.request, true).await)
    }

    #[tool(description = "Preview normalized historical report JSON offline without sending data to Google.")]
    async fn preview_report_request(&self, Parameters(args): Parameters<ReportArgs>) -> CallToolResult {
        let validation = google::property_name(&args.property_id).and_then(|_| report_request(
            Some(args.date_ranges),
            args.dimensions,
            args.metrics,
            vec![
                ("dimensionFilter", args.dimension_filter),
                ("metricFilter", args.metric_filter),
                ("orderBys", args.order_bys.map(|x| json!(x))),
                ("limit", args.limit.map(|x| json!(x.to_string()))),
                ("offset", args.offset.map(|x| json!(x.to_string()))),
                ("currencyCode", args.currency_code.map(|x| json!(x))),
                ("keepEmptyRows", args.keep_empty_rows.map(|x| json!(x)))
            ],
        ));
        result(validation.map(|body| json!({"request":body, "provider_called":false})))
    }

    #[tool(description = "Run a historical Google Analytics Data API report with metric and dimension names.")]
    async fn run_report(&self, Parameters(args): Parameters<ReportArgs>) -> CallToolResult {
        let body = report_request(
            Some(args.date_ranges),
            args.dimensions,
            args.metrics,
            vec![
                ("dimensionFilter", args.dimension_filter),
                ("metricFilter", args.metric_filter),
                ("orderBys", args.order_bys.map(|x| json!(x))),
                ("limit", args.limit.map(|x| json!(x.to_string()))),
                ("offset", args.offset.map(|x| json!(x.to_string()))),
                ("currencyCode", args.currency_code.map(|x| json!(x))),
                ("keepEmptyRows", args.keep_empty_rows.map(|x| json!(x)))
            ],
        );
        result(match body {
            Ok(body) => self.api.report(&args.property_id, body, false).await,
            Err(e) => Err(e),
        })
    }

    #[tool(description = "Run a realtime GA4 report. Realtime reports cannot use offsets or page cursors.")]
    async fn run_realtime_report(&self, Parameters(args): Parameters<RealtimeArgs>) -> CallToolResult {
        let body = report_request(
            None,
            args.dimensions,
            args.metrics,
            vec![
                ("dimensionFilter", args.dimension_filter),
                ("metricFilter", args.metric_filter),
                ("limit", args.limit.map(|x| json!(x.to_string())))
            ],
        );
        result(match body {
            Ok(body) => self.api.report(&args.property_id, body, true).await,
            Err(e) => Err(e),
        })
    }
}

#[tool_handler(
    name = "synmetra",
    version = "0.0.1",
    instructions = "Read-only Google Analytics MCP server. Use synmetra auth login for native OAuth token refresh, or SYNMETRA_GOOGLE_ACCESS_TOKEN for a short-lived analytics.readonly token. Only implemented tools are listed; no write or scratchpad tools are enabled."
)]
impl ServerHandler for Synmetra {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 && args[1] == "auth" {
        return auth::command(&args[2..]).await.map_err(|e| e.into());
    }
    if args.len() == 2 && (args[1] == "--version" || args[1] == "-V") {
        println!("synmetra 0.0.1 (experimental read-only MCP)");
        return Ok(());
    }
    if args.len() != 1 && !(args.len() == 2 && args[1] == "serve") {
        eprintln!("Usage: synmetra [serve|--version|auth login --client-id-file PATH|auth status|auth logout]\nStarts an experimental read-only MCP stdio server.\nUse synmetra auth login for native read-only OAuth, or SYNMETRA_GOOGLE_ACCESS_TOKEN for a short-lived token.");
        std::process::exit(2);
    }
    let server = Synmetra {
        api: GoogleClient::new(std::env::var("SYNMETRA_GOOGLE_ACCESS_TOKEN").ok()),
    };
    server.serve(stdio()).await?.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_implemented_tools_are_discovered() {
        let router = Synmetra::tool_router();
        let names: std::collections::HashSet<_> = router
            .list_all()
            .into_iter()
            .map(|tool| tool.name.to_string())
            .collect();
        assert_eq!(names.len(), 17);
        for name in ["get_account_summaries", "get_property_details", "preview_report_request", "run_conversions_report", "run_funnel_report", "get_account_data_sharing_settings", "get_property_data_retention_settings", "list_google_ads_links", "list_property_annotations", "get_custom_dimensions_and_metrics", "run_report", "run_realtime_report", "run_pivot_report", "batch_run_reports", "check_report_compatibility", "run_property_access_report", "run_account_access_report"] {
            assert!(names.contains(name), "missing {name}");
        }
        assert!(!names.contains("create_property_annotation"));
    }

    #[tokio::test]
    async fn in_memory_mcp_client_negotiates_and_calls_read_only_tools() {
        use rmcp::model::CallToolRequestParams;
        let (server_stream, client_stream) = tokio::io::duplex(65536);
        let worker = tokio::spawn(async move {
            let server = Synmetra { api: GoogleClient::new(None) };
            server.serve(server_stream).await.unwrap().waiting().await.unwrap();
        });
        let client = ().serve(client_stream).await.unwrap();
        let tools = client.list_all_tools().await.unwrap();
        assert_eq!(tools.len(), 17);
        assert!(tools.iter().all(|tool| tool.name != "create_property_annotation"));
        let call = client.call_tool(CallToolRequestParams::new("get_account_summaries")).await.unwrap();
        assert_eq!(call.is_error, Some(true));
        let message = &call.content[0].as_text().unwrap().text;
        assert!(message.contains("No Google access token"));
        assert!(!message.contains("test-token"));
        client.cancel().await.unwrap();
        worker.await.unwrap();
    }

    #[tokio::test]
    async fn offline_preview_rejects_bad_metrics_without_credentials() {
        let server = Synmetra { api: GoogleClient::new(None) };
        let bad = ReportArgs {
            property_id: "55".into(),
            date_ranges: vec![json!({"startDate":"7daysAgo","endDate":"today"})],
            dimensions: vec![],
            metrics: vec![],
            dimension_filter: None,
            metric_filter: None,
            order_bys: None,
            limit: None,
            offset: None,
            currency_code: None,
            keep_empty_rows: None,
        };
        let result = server.preview_report_request(Parameters(bad)).await;
        assert_eq!(result.is_error, Some(true));
    }

    #[test]
    fn tool_errors_are_mcp_tool_errors() {
        let result = super::result(Err(ApiError::InvalidIdentifier));
        assert_eq!(result.is_error, Some(true));
        assert!(!result.content[0].as_text().unwrap().text.contains("Bearer"));
    }
}
