//! Native MCP stdio server. Only implemented tools are advertised.
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
    instructions = "Read-only Google Analytics MCP server. Set SYNMETRA_GOOGLE_ACCESS_TOKEN to a short-lived access token scoped to analytics.readonly. Only implemented tools are listed; no write or scratchpad tools are enabled."
)]
impl ServerHandler for Synmetra {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && (args[1] == "--version" || args[1] == "-V") {
        println!("synmetra 0.0.1 (experimental read-only MCP)");
        return Ok(());
    }
    if args.len() != 1 && !(args.len() == 2 && args[1] == "serve") {
        eprintln!("Usage: synmetra [serve|--version]\nStarts an experimental read-only MCP stdio server.\nSet SYNMETRA_GOOGLE_ACCESS_TOKEN to a short-lived Google OAuth token with analytics.readonly scope.");
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
        assert_eq!(names.len(), 7);
        for name in ["get_account_summaries", "get_property_details", "list_google_ads_links", "list_property_annotations", "get_custom_dimensions_and_metrics", "run_report", "run_realtime_report"] {
            assert!(names.contains(name), "missing {name}");
        }
        assert!(!names.contains("create_property_annotation"));
    }

    #[test]
    fn tool_errors_are_mcp_tool_errors() {
        let result = super::result(Err(ApiError::InvalidIdentifier));
        assert_eq!(result.is_error, Some(true));
        assert!(!result.content[0].as_text().unwrap().text.contains("Bearer"));
    }
}
