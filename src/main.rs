//! Native MCP stdio server. Only implemented tools are advertised.
mod auth;
mod google;
mod http;
#[cfg(feature = "scratchpad")]
mod scratchpad;
mod tabular;

use google::{annotation_payload, report_request, ApiError, GoogleClient};
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::{
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
    schemars::JsonSchema,
    tool, tool_handler, tool_router,
    transport::stdio,
    ServerHandler, ServiceExt,
};
use serde::Deserialize;
use serde_json::{json, Value};

/// Match the pinned Google and SednaLabs contract: identifiers may arrive as
/// positive decimal numbers or as canonical resource-name strings.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(untagged)]
enum GaResourceIdInput {
    Number(u64),
    Text(String),
}

/// The pinned SednaLabs JSON Schema names historical dates with snake_case.
/// Runtime normalization additionally accepts Google camelCase and named ranges.
#[allow(dead_code)] // Schema-only type: runtime deserialization intentionally retains Value aliases.
#[derive(JsonSchema)]
struct ReportDateRangeInput {
    start_date: String,
    end_date: String,
}

fn deserialize_google_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let input = GaResourceIdInput::deserialize(deserializer)?;
    Ok(match input {
        GaResourceIdInput::Number(value) => value.to_string(),
        GaResourceIdInput::Text(value) => value,
    })
}

#[derive(Clone)]
struct Synmetra {
    api: GoogleClient,
    router: ToolRouter<Self>,
    writes_allowed: bool,
    #[cfg(feature = "scratchpad")]
    scratchpad: Option<scratchpad::ScratchState>,
}

impl Synmetra {
    fn new(api: GoogleClient, writes_allowed: bool) -> Self {
        let mut router = Self::tool_router();
        if !writes_allowed {
            router.disable_route("create_property_annotation");
        }
        #[cfg(feature = "scratchpad")]
        {
            let mut other = Self::scratch_tool_router();
            for tool in SCRATCHPAD_TOOL_NAMES {
                other.disable_route(tool);
            }
            router.merge(other);
        }
        Self {
            api,
            router,
            writes_allowed,
            #[cfg(feature = "scratchpad")]
            scratchpad: None,
        }
    }

    #[cfg(feature = "scratchpad")]
    fn enable_scratchpad(mut self) -> Self {
        self.scratchpad = Some(scratchpad::ScratchState::new());
        for tool in SCRATCHPAD_TOOL_NAMES {
            self.router.enable_route(tool);
        }
        self
    }
}

#[cfg(feature = "scratchpad")]
const SCRATCHPAD_TOOL_NAMES: [&str; 15] = [
    "scratchpad_open_session",
    "scratchpad_close_session",
    "scratchpad_list_sessions",
    "scratchpad_get_runtime_limits",
    "scratchpad_set_runtime_limits",
    "scratchpad_list_tables",
    "scratchpad_drop_table",
    "scratchpad_describe_table",
    "scratchpad_summarize_table",
    "scratchpad_export_evidence_bundle",
    "scratchpad_ingest_report",
    "scratchpad_ingest_realtime_report",
    "scratchpad_query",
    "scratchpad_release_regression_report",
    "scratchpad_landing_param_shift_report",
];

#[cfg(feature = "scratchpad")]
#[derive(Debug, Deserialize, JsonSchema)]
struct ScratchSessionArgs {
    session_id: String,
}

#[cfg(feature = "scratchpad")]
#[derive(Debug, Deserialize, JsonSchema)]
struct ScratchOpenArgs {
    session_id: String,
}

#[cfg(feature = "scratchpad")]
#[derive(Debug, Deserialize, JsonSchema)]
struct ScratchListArgs {
    #[serde(default)]
    limit: Option<usize>,
}

#[cfg(feature = "scratchpad")]
#[derive(Debug, Deserialize, JsonSchema)]
struct ScratchListTablesArgs {
    session_id: String,
    #[serde(default)]
    limit: Option<usize>,
}

#[cfg(feature = "scratchpad")]
#[derive(Debug, Deserialize, JsonSchema)]
struct ScratchTableArgs {
    session_id: String,
    table_name: String,
    #[serde(default)]
    if_exists: bool,
}

#[cfg(feature = "scratchpad")]
#[derive(Debug, Deserialize, JsonSchema)]
struct ScratchReadTableArgs {
    session_id: String,
    table_name: String,
}

#[cfg(feature = "scratchpad")]
#[derive(Debug, Deserialize, JsonSchema)]
struct ScratchEvidenceArgs {
    session_id: String,
    #[serde(default)]
    table_names: Option<Vec<String>>,
    #[serde(default)]
    sample_rows_per_table: Option<usize>,
}

#[cfg(feature = "scratchpad")]
#[derive(Debug, Deserialize, JsonSchema)]
struct ScratchIngestReportArgs {
    session_id: String,
    table_name: String,
    #[serde(deserialize_with = "deserialize_google_id")]
    #[schemars(with = "GaResourceIdInput")]
    property_id: String,
    dimensions: Vec<String>,
    metrics: Vec<String>,
    #[serde(default)]
    append: bool,
    #[serde(default)]
    limit: Option<u32>,
    #[serde(default)]
    offset: Option<u64>,
    #[serde(default)]
    dimension_filter: Option<Value>,
    #[serde(default)]
    metric_filter: Option<Value>,
    #[serde(default)]
    order_bys: Option<Vec<Value>>,
    #[serde(default = "default_return_property_quota")]
    return_property_quota: bool,
    #[schemars(with = "Vec<ReportDateRangeInput>")]
    date_ranges: Vec<Value>,
    #[serde(default)]
    currency_code: Option<String>,
}

#[cfg(feature = "scratchpad")]
#[derive(Debug, Deserialize, JsonSchema)]
struct ScratchIngestRealtimeArgs {
    session_id: String,
    table_name: String,
    #[serde(deserialize_with = "deserialize_google_id")]
    #[schemars(with = "GaResourceIdInput")]
    property_id: String,
    dimensions: Vec<String>,
    metrics: Vec<String>,
    #[serde(default)]
    append: bool,
    #[serde(default)]
    limit: Option<u32>,
    #[serde(default)]
    offset: Option<u64>,
    #[serde(default)]
    dimension_filter: Option<Value>,
    #[serde(default)]
    metric_filter: Option<Value>,
    #[serde(default)]
    order_bys: Option<Vec<Value>>,
    #[serde(default = "default_return_property_quota")]
    return_property_quota: bool,
}

#[cfg(feature = "scratchpad")]
#[derive(Debug, Deserialize, JsonSchema)]
struct ScratchQueryArgs {
    session_id: String,
    sql: String,
    #[serde(default)]
    max_rows: Option<usize>,
    #[serde(default)]
    max_cell_chars: Option<usize>,
    #[serde(default)]
    summary_only: bool,
    #[serde(default)]
    output_mode: Option<String>,
    #[serde(default)]
    cursor: Option<String>,
}

#[cfg(feature = "scratchpad")]
#[derive(Debug, Deserialize, JsonSchema)]
struct ScratchReleaseArgs {
    session_id: String,
    table_name: String,
    release_date: String,
    anchor_event: String,
    comparison_event: String,
    #[serde(default)]
    date_column: Option<String>,
    #[serde(default)]
    event_column: Option<String>,
    #[serde(default)]
    metric_column: Option<String>,
    #[serde(default)]
    pre_days: Option<u32>,
    #[serde(default)]
    transition_days: Option<u32>,
    #[serde(default)]
    post_days: Option<u32>,
    #[serde(default)]
    summary_only: bool,
    #[serde(default)]
    output_mode: Option<String>,
    #[serde(default)]
    cursor: Option<String>,
}

#[cfg(feature = "scratchpad")]
#[derive(Debug, Deserialize, JsonSchema)]
struct ScratchLandingArgs {
    session_id: String,
    table_name: String,
    release_date: String,
    #[serde(default)]
    date_column: Option<String>,
    #[serde(default)]
    landing_url_column: Option<String>,
    #[serde(default)]
    channel_column: Option<String>,
    #[serde(default)]
    source_medium_column: Option<String>,
    #[serde(default)]
    pre_days: Option<u32>,
    #[serde(default)]
    transition_days: Option<u32>,
    #[serde(default)]
    post_days: Option<u32>,
    #[serde(default)]
    top_n: Option<usize>,
    #[serde(default)]
    summary_only: bool,
    #[serde(default)]
    output_mode: Option<String>,
    #[serde(default)]
    cursor: Option<String>,
}

#[cfg(feature = "scratchpad")]
#[derive(Debug, Deserialize, JsonSchema)]
struct ScratchLimitArgs {
    #[serde(default)]
    max_sessions: Option<usize>,
    #[serde(default)]
    max_tables_per_session: Option<usize>,
}

#[cfg(feature = "scratchpad")]
fn local_result(value: Result<Value, String>) -> CallToolResult {
    match value {
        Ok(data) => result(Ok(data)),
        Err(message) => {
            let payload = json!({
                "ok":false,
                "error":{
                    "code":"SCRATCHPAD_ERROR",
                    "reason":"scratchpad_error",
                    "message":message,
                    "category":"local"
                },
                "meta":{"elapsed_ms":0,"elapsed_ms_measured":false}
            });
            let mut tool_result =
                CallToolResult::error(vec![ContentBlock::text(payload.to_string())]);
            tool_result.structured_content = Some(payload);
            tool_result
        }
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
struct AuthLoginCommandArgs {
    /// Optional Google account hint for browser login.
    #[serde(default)]
    account: Option<String>,
    /// Optional fixed loopback callback port for direct browser OAuth.
    #[serde(default)]
    callback_port: Option<u16>,
    /// Optional Google OAuth client id file for direct browser OAuth.
    #[serde(default)]
    client_id_file: Option<String>,
    /// Include the headless browser flag for SSH or remote environments.
    #[serde(default)]
    headless: bool,
    /// Optional quota project to include as a follow-up command.
    #[serde(default)]
    quota_project: Option<String>,
    /// Use the conventional shared gcloud ADC file instead of the server-specific file.
    #[serde(default)]
    shared_adc: Option<bool>,
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

#[derive(Debug, Deserialize, JsonSchema)]
struct AuthStatusArgs {
    /// When true, acquire a Google access token and call GA account summaries. The token is never returned.
    #[serde(default)]
    verify_token: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct CollectionArgs {
    #[serde(default)]
    page_size: Option<u32>,
    #[serde(default)]
    max_pages: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct PropertyCollectionArgs {
    #[serde(deserialize_with = "deserialize_google_id")]
    #[schemars(with = "GaResourceIdInput")]
    property_id: String,
    #[serde(default)]
    page_size: Option<u32>,
    #[serde(default)]
    max_pages: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct PropertyArgs {
    /// Positive numeric GA4 property ID or canonical properties/<id>.
    #[serde(deserialize_with = "deserialize_google_id")]
    #[schemars(with = "GaResourceIdInput")]
    property_id: String,
}

#[allow(dead_code)] // Public schema mirror; payload validation retains normalized String input.
#[derive(JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum AnnotationColorInput {
    Purple,
    Brown,
    Blue,
    Green,
    Red,
    Cyan,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct CreateAnnotationArgs {
    #[serde(deserialize_with = "deserialize_google_id")]
    #[schemars(with = "GaResourceIdInput")]
    property_id: String,
    title: String,
    start_date: String,
    #[serde(default)]
    end_date: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default = "default_annotation_color")]
    #[schemars(with = "AnnotationColorInput")]
    color: String,
}

fn default_annotation_color() -> String {
    "BLUE".into()
}

#[derive(Debug, Deserialize, JsonSchema)]
struct AccountArgs {
    /// Positive GA4 account number or canonical accounts/<id>.
    #[serde(deserialize_with = "deserialize_google_id")]
    #[schemars(with = "GaResourceIdInput")]
    account_id: String,
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[serde(deny_unknown_fields)]
struct FunnelStepInput {
    name: Option<String>,
    event: Option<String>,
    filter_expression: Option<Value>,
    is_directly_followed_by: Option<bool>,
    within_duration_from_prior_step: Option<String>,
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[serde(deny_unknown_fields)]
struct FunnelBreakdownInput {
    breakdown_dimension: String,
    limit: Option<u64>,
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[serde(deny_unknown_fields)]
struct FunnelNextActionInput {
    next_action_dimension: String,
    limit: Option<u64>,
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum FunnelVisualizationInput {
    StandardFunnel,
    TrendedFunnel,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct FunnelArgs {
    #[serde(deserialize_with = "deserialize_google_id")]
    #[schemars(with = "GaResourceIdInput")]
    property_id: String,
    #[schemars(with = "Vec<FunnelStepInput>")]
    funnel_steps: Vec<Value>,
    #[serde(default)]
    #[schemars(with = "Vec<ReportDateRangeInput>")]
    date_ranges: Option<Vec<Value>>,
    #[serde(default)]
    #[schemars(with = "Option<FunnelBreakdownInput>")]
    funnel_breakdown: Option<Value>,
    #[serde(default)]
    #[schemars(with = "Option<FunnelNextActionInput>")]
    funnel_next_action: Option<Value>,
    #[serde(default)]
    segments: Option<Vec<Value>>,
    #[serde(default)]
    return_property_quota: bool,
    #[serde(default)]
    is_open_funnel: bool,
    #[serde(default)]
    #[schemars(with = "Option<FunnelVisualizationInput>")]
    funnel_visualization_type: Option<String>,
    #[serde(default)]
    dimension_filter: Option<Value>,
    #[serde(default)]
    limit: Option<u64>,
    #[serde(default)]
    max_rows: Option<usize>,
    #[serde(default)]
    max_cell_chars: Option<usize>,
    #[serde(default)]
    summary_only: bool,
    #[serde(default)]
    output_mode: Option<tabular::OutputMode>,
}

/// Input-schema mirror of the pinned conversion specification. The runtime
/// continues accepting snake_case and Google camelCase through Value, while
/// the advertised public schema retains typed enum and action-list bounds.
#[allow(dead_code)]
#[derive(JsonSchema)]
#[serde(deny_unknown_fields)]
struct ConversionSpecInput {
    #[schemars(default)]
    attribution_model: Option<ConversionAttributionModel>,
    #[schemars(default)]
    conversion_actions: Vec<String>,
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ConversionAttributionModel {
    DataDriven,
    LastClick,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ConversionArgs {
    #[serde(deserialize_with = "deserialize_google_id")]
    #[schemars(with = "GaResourceIdInput")]
    property_id: String,
    #[schemars(with = "Vec<ReportDateRangeInput>")]
    date_ranges: Vec<Value>,
    dimensions: Vec<String>,
    metrics: Vec<String>,
    #[schemars(with = "ConversionSpecInput")]
    conversion_spec: Value,
    #[serde(default)]
    dimension_filter: Option<Value>,
    #[serde(default)]
    metric_filter: Option<Value>,
    #[serde(default)]
    order_bys: Option<Vec<Value>>,
    #[serde(default)]
    limit: Option<u64>,
    #[serde(default)]
    offset: Option<u64>,
    #[serde(default)]
    currency_code: Option<String>,
    #[serde(default)]
    return_property_quota: bool,
    #[serde(default)]
    max_rows: Option<usize>,
    #[serde(default)]
    max_cell_chars: Option<usize>,
    #[serde(default)]
    summary_only: bool,
    #[serde(default)]
    output_mode: Option<tabular::OutputMode>,
    #[serde(default)]
    cursor: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct PivotArgs {
    #[serde(deserialize_with = "deserialize_google_id")]
    #[schemars(with = "GaResourceIdInput")]
    property_id: String,
    #[schemars(with = "Vec<ReportDateRangeInput>")]
    date_ranges: Vec<Value>,
    dimensions: Vec<String>,
    metrics: Vec<String>,
    pivots: Vec<Value>,
    #[serde(default)]
    dimension_filter: Option<Value>,
    #[serde(default)]
    metric_filter: Option<Value>,
    #[serde(default)]
    order_bys: Option<Vec<Value>>,
    #[serde(default)]
    currency_code: Option<String>,
    #[serde(default)]
    keep_empty_rows: Option<bool>,
    #[serde(default = "default_return_property_quota")]
    return_property_quota: bool,
    #[serde(default)]
    max_rows: Option<usize>,
    #[serde(default)]
    max_cell_chars: Option<usize>,
    #[serde(default)]
    summary_only: bool,
    #[serde(default)]
    output_mode: Option<tabular::OutputMode>,
    #[serde(default)]
    cursor: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct BatchReportItemArgs {
    #[schemars(with = "Vec<ReportDateRangeInput>")]
    date_ranges: Vec<Value>,
    dimensions: Vec<String>,
    metrics: Vec<String>,
    #[serde(default)]
    dimension_filter: Option<Value>,
    #[serde(default)]
    metric_filter: Option<Value>,
    #[serde(default)]
    order_bys: Option<Vec<Value>>,
    #[serde(default)]
    limit: Option<u64>,
    #[serde(default)]
    offset: Option<u64>,
    #[serde(default)]
    currency_code: Option<String>,
    #[serde(default = "default_return_property_quota")]
    return_property_quota: bool,
}

impl BatchReportItemArgs {
    fn to_google_request(&self) -> Result<Value, ApiError> {
        if self
            .limit
            .is_some_and(|limit| limit == 0 || limit > 250_000)
        {
            return Err(ApiError::InvalidRequest);
        }
        report_request(
            Some(self.date_ranges.clone()),
            self.dimensions.clone(),
            self.metrics.clone(),
            vec![
                ("dimensionFilter", self.dimension_filter.clone()),
                ("metricFilter", self.metric_filter.clone()),
                ("orderBys", self.order_bys.clone().map(|x| json!(x))),
                ("limit", self.limit.map(|v| json!(v.to_string()))),
                ("offset", self.offset.map(|v| json!(v.to_string()))),
                ("currencyCode", self.currency_code.clone().map(|x| json!(x))),
                (
                    "returnPropertyQuota",
                    Some(json!(self.return_property_quota)),
                ),
            ],
        )
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
struct BatchArgs {
    #[serde(deserialize_with = "deserialize_google_id")]
    #[schemars(with = "GaResourceIdInput")]
    property_id: String,
    requests: Vec<BatchReportItemArgs>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct CompatibilityArgs {
    #[serde(deserialize_with = "deserialize_google_id")]
    #[schemars(with = "GaResourceIdInput")]
    property_id: String,
    dimensions: Vec<String>,
    metrics: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct PropertyAccessArgs {
    #[serde(deserialize_with = "deserialize_google_id")]
    #[schemars(with = "GaResourceIdInput")]
    property_id: String,
    #[schemars(with = "Vec<ReportDateRangeInput>")]
    date_ranges: Vec<Value>,
    dimensions: Vec<String>,
    metrics: Vec<String>,
    #[serde(default)]
    dimension_filter: Option<Value>,
    #[serde(default)]
    metric_filter: Option<Value>,
    #[serde(default)]
    order_bys: Option<Vec<Value>>,
    #[serde(default)]
    limit: Option<u64>,
    #[serde(default)]
    offset: Option<u64>,
    #[serde(default)]
    time_zone: Option<String>,
    #[serde(default)]
    max_rows: Option<usize>,
    #[serde(default)]
    max_cell_chars: Option<usize>,
    #[serde(default)]
    summary_only: bool,
    #[serde(default)]
    output_mode: Option<tabular::OutputMode>,
    #[serde(default)]
    cursor: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct AccountAccessArgs {
    #[serde(deserialize_with = "deserialize_google_id")]
    #[schemars(with = "GaResourceIdInput")]
    account_id: String,
    #[schemars(with = "Vec<ReportDateRangeInput>")]
    date_ranges: Vec<Value>,
    dimensions: Vec<String>,
    metrics: Vec<String>,
    #[serde(default)]
    dimension_filter: Option<Value>,
    #[serde(default)]
    metric_filter: Option<Value>,
    #[serde(default)]
    order_bys: Option<Vec<Value>>,
    #[serde(default)]
    limit: Option<u64>,
    #[serde(default)]
    offset: Option<u64>,
    #[serde(default)]
    time_zone: Option<String>,
    #[serde(default)]
    max_rows: Option<usize>,
    #[serde(default)]
    max_cell_chars: Option<usize>,
    #[serde(default)]
    summary_only: bool,
    #[serde(default)]
    output_mode: Option<tabular::OutputMode>,
    #[serde(default)]
    cursor: Option<String>,
}

fn default_return_property_quota() -> bool {
    true
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ReportArgs {
    /// Positive numeric GA4 property ID or canonical properties/<id>.
    #[serde(deserialize_with = "deserialize_google_id")]
    #[schemars(with = "GaResourceIdInput")]
    property_id: String,
    /// Historical date ranges, e.g. [{"startDate":"7daysAgo","endDate":"today"}].
    #[schemars(with = "Vec<ReportDateRangeInput>")]
    date_ranges: Vec<Value>,
    /// Names of dimensions (not Dimension objects).
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
    limit: Option<u64>,
    /// Pagination offset.
    #[serde(default)]
    offset: Option<u64>,
    /// ISO currency code.
    #[serde(default)]
    currency_code: Option<String>,
    #[serde(default)]
    keep_empty_rows: Option<bool>,
    /// Include GA4 property quota metadata in the provider response (upstream default: true).
    #[serde(default = "default_return_property_quota")]
    return_property_quota: bool,
    #[serde(default)]
    max_rows: Option<usize>,
    #[serde(default)]
    max_cell_chars: Option<usize>,
    #[serde(default)]
    summary_only: bool,
    #[serde(default)]
    output_mode: Option<tabular::OutputMode>,
    #[serde(default)]
    cursor: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct RealtimeArgs {
    #[serde(deserialize_with = "deserialize_google_id")]
    #[schemars(with = "GaResourceIdInput")]
    property_id: String,
    dimensions: Vec<String>,
    metrics: Vec<String>,
    #[serde(default)]
    dimension_filter: Option<Value>,
    #[serde(default)]
    metric_filter: Option<Value>,
    /// Google realtime supports OrderBy arrays in the v1beta request body.
    #[serde(default)]
    order_bys: Option<Vec<Value>>,
    #[serde(default)]
    limit: Option<u64>,
    /// Present for official Python signature compatibility. A nonzero value is
    /// rejected: Google's current v1beta realtime request has no offset field.
    #[serde(default)]
    offset: Option<u64>,
    #[serde(default = "default_return_property_quota")]
    return_property_quota: bool,
    #[serde(default)]
    max_rows: Option<usize>,
    #[serde(default)]
    max_cell_chars: Option<usize>,
    #[serde(default)]
    summary_only: bool,
    #[serde(default)]
    output_mode: Option<tabular::OutputMode>,
    #[serde(default)]
    cursor: Option<String>,
}

fn realtime_report_request(args: &RealtimeArgs) -> Result<Value, ApiError> {
    if args.offset.is_some_and(|offset| offset > 0) {
        return Err(ApiError::UnsupportedRealtimeOffset);
    }
    report_request(
        None,
        args.dimensions.clone(),
        args.metrics.clone(),
        vec![
            ("dimensionFilter", args.dimension_filter.clone()),
            ("metricFilter", args.metric_filter.clone()),
            ("orderBys", args.order_bys.clone().map(|x| json!(x))),
            ("limit", args.limit.map(|x| json!(x.to_string()))),
            (
                "returnPropertyQuota",
                Some(json!(args.return_property_quota)),
            ),
        ],
    )
}

fn api_error_code(error: &ApiError) -> (&'static str, &'static str) {
    match error {
        ApiError::MissingCredentials => ("AUTH_NOT_CONFIGURED", "authentication"),
        ApiError::AuthenticationFailed => ("AUTHENTICATION_FAILED", "authentication"),
        ApiError::InvalidIdentifier => ("INVALID_IDENTIFIER", "validation"),
        ApiError::InvalidRequest => ("INVALID_REQUEST", "validation"),
        ApiError::UnsupportedAuthOption => ("UNSUPPORTED_AUTH_OPTION", "validation"),
        ApiError::UnsupportedRealtimeOffset => ("UNSUPPORTED_REALTIME_OFFSET", "validation"),
        ApiError::InvalidAnnotation => ("INVALID_ANNOTATION", "validation"),
        ApiError::RequestFailed => ("UPSTREAM_UNAVAILABLE", "transport"),
        ApiError::Http(_) => ("GOOGLE_HTTP_ERROR", "upstream"),
        ApiError::ResponseTooLarge => ("RESPONSE_TOO_LARGE", "limits"),
        ApiError::CollectionTooLarge => ("COLLECTION_TOO_LARGE", "limits"),
        ApiError::TooManyPages => ("PAGINATION_LIMIT", "limits"),
        ApiError::InvalidResponse => ("INVALID_UPSTREAM_RESPONSE", "upstream"),
    }
}

fn result_with_funnel_projection(
    response: Result<Value, ApiError>,
    controls: tabular::Controls,
) -> CallToolResult {
    let output = response.and_then(|response| {
        let table = response
            .get("funnelTable")
            .ok_or(ApiError::InvalidResponse)?;
        let visualization = response
            .get("funnelVisualization")
            .ok_or(ApiError::InvalidResponse)?;
        let table = tabular::project(table, controls, false, None)?;
        let visualization = tabular::project(visualization, controls, false, None)?;
        let mut ga4 = json!({});
        for field in ["propertyQuota", "kind", "metadata"] {
            if let Some(value) = response.get(field) {
                ga4[field] = value.clone();
            }
        }
        Ok(json!({
            "ok": true,
            "data": {
                "funnel_table": table.data,
                "funnel_visualization": visualization.data
            },
            "meta": {
                "funnel_table": table.meta,
                "funnel_visualization": visualization.meta,
                "ga4": ga4,
                "elapsed_ms": 0,
                "elapsed_ms_measured": false
            }
        }))
    });
    match output {
        Ok(payload) => {
            let mut result = CallToolResult::success(vec![ContentBlock::text(payload.to_string())]);
            result.structured_content = Some(payload);
            result
        }
        Err(error) => result(Err(error)),
    }
}

fn result_with_tabular_projection(
    response: Result<Value, ApiError>,
    controls: tabular::Controls,
    realtime: bool,
    pagination: Option<&tabular::Pagination>,
) -> CallToolResult {
    match response.and_then(|provider| tabular::project(&provider, controls, realtime, pagination))
    {
        Ok(projection) => {
            let payload = json!({"ok":true,"data":projection.data,"meta":projection.meta});
            let mut result = CallToolResult::success(vec![ContentBlock::text(payload.to_string())]);
            result.structured_content = Some(payload);
            result
        }
        Err(error) => result(Err(error)),
    }
}

/// Predict the report's tabular and scratchpad column mapping without calling
/// Google. Names are descriptive only; field compatibility remains unverified.
fn preview_report_projection(dimensions: &[String], metrics: &[String]) -> Value {
    use std::collections::HashMap;

    fn next_name(base: &str, used: &mut HashMap<String, usize>) -> String {
        let count = used.entry(base.to_owned()).or_insert(0);
        *count += 1;
        if *count == 1 {
            base.to_owned()
        } else {
            format!("{base}_{count}")
        }
    }

    let mut columns = Vec::new();
    let mut ingest = Vec::new();
    let mut used = HashMap::new();
    for (index, (name, logical_type)) in dimensions
        .iter()
        .map(|name| (name, "string"))
        .chain(metrics.iter().map(|name| (name, "number")))
        .enumerate()
    {
        columns.push(json!({"name":name,"logical_type":logical_type,"nullable":true}));
        let mut base = name
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() {
                    c.to_ascii_lowercase()
                } else {
                    '_'
                }
            })
            .collect::<String>();
        base = base.trim_matches('_').to_owned();
        if base.is_empty() {
            base = format!("col_{}", index + 1);
        }
        if base.starts_with(|c: char| c.is_ascii_digit()) {
            base.insert(0, '_');
        }
        ingest.push(json!({
            "source_name":name,
            "target_name":next_name(&base, &mut used),
            "logical_type":logical_type,
            "transform":"identity"
        }));
        let canonical = name
            .chars()
            .filter(char::is_ascii_alphanumeric)
            .collect::<String>()
            .to_ascii_lowercase();
        let derived = match canonical.as_str() {
            "date" => Some(("date", "parse_ga_date")),
            "datehour" => Some(("timestamp", "parse_ga_date_hour")),
            _ => None,
        };
        if let Some((derived_type, transform)) = derived {
            ingest.push(json!({
                "source_name":name,
                "target_name":next_name(&format!("{base}_parsed"), &mut used),
                "logical_type":derived_type,
                "transform":transform
            }));
        }
    }
    json!({"tabular_columns":columns,"ingest_columns":ingest})
}

fn result(r: Result<Value, ApiError>) -> CallToolResult {
    // elapsed_ms = 0 is an explicit *unmeasured* sentinel here, not a
    // claim of zero tool latency. Per-tool timing is a separate parity gate.
    let meta = json!({"elapsed_ms": 0, "elapsed_ms_measured": false});
    match r {
        Ok(v) => {
            let payload = json!({"ok": true, "data": v, "meta": meta});
            let mut tool_result =
                CallToolResult::success(vec![ContentBlock::text(payload.to_string())]);
            tool_result.structured_content = Some(payload);
            tool_result
        }
        Err(error) => {
            let (code, category) = api_error_code(&error);
            let mut payload = json!({
                "ok": false,
                "error": {
                    "code": code,
                    "reason": code.to_ascii_lowercase(),
                    "message": error.to_string(),
                    "category": category
                },
                "meta": meta
            });
            if let ApiError::Http(status) = error {
                payload["error"]["status_code"] = json!(status);
            }
            let mut tool_result =
                CallToolResult::error(vec![ContentBlock::text(payload.to_string())]);
            tool_result.structured_content = Some(payload);
            tool_result
        }
    }
}

#[tool_router]
impl Synmetra {
    #[tool(
        description = "Explain secure one-time setup for Synmetra with Google Analytics APIs and native OAuth.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    fn ga4_get_started(&self) -> CallToolResult {
        result(Ok(json!({
            "command": "synmetra auth login --client-id-file /path/to/desktop-oauth.json",
            "requires": ["Google Analytics Admin API", "Google Analytics Data API", "GA4 property access"],
            "scope": "https://www.googleapis.com/auth/analytics.readonly",
            "python_required": false, "npm_required": false
        })))
    }

    #[tool(
        description = "Report safe Google Analytics credential status without exposing secrets, IDs, or local paths.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn ga4_auth_status(
        &self,
        Parameters(args): Parameters<AuthStatusArgs>,
    ) -> CallToolResult {
        let mut status = auth::public_status();
        if args.verify_token {
            // An explicit request may contact Google. Never include the token or
            // any returned account/property identifiers in auth diagnostics.
            let verification = self
                .api
                .account_summaries_with_options(Some(1), Some(1))
                .await;
            status["verification"] = match verification {
                Ok(_) => json!({"attempted": true, "verified": true}),
                Err(error) => {
                    let (code, category) = api_error_code(&error);
                    json!({"attempted": true, "verified": false, "code": code, "category": category})
                }
            };
        }
        result(Ok(status))
    }

    #[tool(
        description = "Return the native OAuth desktop login command; never perform interactive authorization inside an MCP tool.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    fn ga4_auth_login_command(
        &self,
        Parameters(args): Parameters<AuthLoginCommandArgs>,
    ) -> CallToolResult {
        // Native Synmetra never writes shared gcloud ADC credentials.
        // Quota-project routing requires the separate verified Google header policy.
        if args.quota_project.is_some() || args.shared_adc == Some(true) {
            return result(Err(ApiError::UnsupportedAuthOption));
        }
        let path = args
            .client_id_file
            .as_deref()
            .unwrap_or("/path/to/desktop-oauth.json");
        if path.is_empty() || path.len() > 4096 || path.chars().any(char::is_control) {
            return result(Err(ApiError::InvalidRequest));
        }
        let mut command = format!("synmetra auth login --client-id-file {}", shell_quote(path));
        if let Some(account) = args.account {
            if account.is_empty() || account.len() > 254 || account.chars().any(char::is_control) {
                return result(Err(ApiError::InvalidRequest));
            }
            command.push_str(" --account ");
            command.push_str(&shell_quote(&account));
        }
        if let Some(port) = args.callback_port {
            command.push_str(&format!(" --callback-port {port}"));
        }
        if args.headless {
            command.push_str(" --headless");
        }
        result(Ok(json!({
            "command": command,
            "interactive_authorization_required": true,
            "scope": "https://www.googleapis.com/auth/analytics.readonly",
            "credential_store": "synmetra-private"
        })))
    }

    #[tool(
        description = "WRITE: Create a GA4 reporting annotation. Hidden and blocked unless server starts with --allow-write and Google token has analytics.edit scope. Never retry automatically.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = true
        )
    )]
    async fn create_property_annotation(
        &self,
        Parameters(args): Parameters<CreateAnnotationArgs>,
    ) -> CallToolResult {
        if !self.writes_allowed {
            return result(Err(ApiError::InvalidRequest));
        }
        let body = annotation_payload(
            &args.title,
            &args.start_date,
            args.end_date.as_deref(),
            args.description.as_deref(),
            &args.color,
        );
        result(match body {
            Ok(body) => self.api.create_annotation(&args.property_id, body).await,
            Err(error) => Err(error),
        })
    }

    #[tool(
        description = "List the Google Analytics accounts and properties accessible with the configured Google token.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn get_account_summaries(
        &self,
        Parameters(args): Parameters<CollectionArgs>,
    ) -> CallToolResult {
        result(
            self.api
                .account_summaries_with_options(args.page_size, args.max_pages)
                .await,
        )
    }

    #[tool(
        description = "Read one Google Analytics 4 property, by numeric property ID or properties/<id>.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn get_property_details(
        &self,
        Parameters(args): Parameters<PropertyArgs>,
    ) -> CallToolResult {
        result(self.api.property(&args.property_id).await)
    }

    #[tool(
        description = "Get an Analytics account's data-sharing settings.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn get_account_data_sharing_settings(
        &self,
        Parameters(args): Parameters<AccountArgs>,
    ) -> CallToolResult {
        result(self.api.account_sharing(&args.account_id).await)
    }

    #[tool(
        description = "Get a GA4 property's event and user data retention settings.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn get_property_data_retention_settings(
        &self,
        Parameters(args): Parameters<PropertyArgs>,
    ) -> CallToolResult {
        result(self.api.retention_settings(&args.property_id).await)
    }

    #[tool(
        description = "List Google Ads account links for one GA4 property.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn list_google_ads_links(
        &self,
        Parameters(args): Parameters<PropertyCollectionArgs>,
    ) -> CallToolResult {
        result(
            self.api
                .ads_links_with_options(&args.property_id, args.page_size, args.max_pages)
                .await,
        )
    }

    #[tool(
        description = "List reporting annotations attached to one GA4 property (read-only).",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn list_property_annotations(
        &self,
        Parameters(args): Parameters<PropertyCollectionArgs>,
    ) -> CallToolResult {
        result(
            self.api
                .annotations_with_options(&args.property_id, args.page_size, args.max_pages)
                .await,
        )
    }

    #[tool(
        description = "List property-specific custom dimensions and metrics.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn get_custom_dimensions_and_metrics(
        &self,
        Parameters(args): Parameters<PropertyArgs>,
    ) -> CallToolResult {
        result(self.api.custom_definitions(&args.property_id).await)
    }

    #[tool(
        description = "Run Google Analytics Data API v1alpha conversion reporting with conversionSpec or conversion metrics. Requires an eligible property.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn run_conversions_report(
        &self,
        Parameters(args): Parameters<ConversionArgs>,
    ) -> CallToolResult {
        let controls = tabular::Controls {
            output_mode: args.output_mode,
            summary_only: args.summary_only,
            max_rows: args.max_rows,
            max_cell_chars: args.max_cell_chars,
            cursor_supplied: args.cursor.is_some(),
        };
        let limit = match controls.request_limit(args.limit) {
            Ok(limit) => limit,
            Err(error) => return result(Err(error)),
        };
        let body = google::conversion_spec(&args.conversion_spec, &args.dimensions, &args.metrics)
            .and_then(|spec| {
                report_request(
                    Some(args.date_ranges),
                    args.dimensions,
                    args.metrics,
                    vec![
                        ("conversionSpec", Some(spec)),
                        ("dimensionFilter", args.dimension_filter),
                        ("metricFilter", args.metric_filter),
                        ("orderBys", args.order_bys.map(|v| json!(v))),
                        ("limit", limit.map(|v| json!(v.to_string()))),
                        ("offset", args.offset.map(|v| json!(v.to_string()))),
                        ("currencyCode", args.currency_code.map(|v| json!(v))),
                        (
                            "returnPropertyQuota",
                            Some(json!(args.return_property_quota)),
                        ),
                    ],
                )
            });
        let (response, pagination) = match body {
            Ok(mut body) => {
                let page = if controls.active() {
                    match tabular::prepare_historical(
                        &mut body,
                        &args.property_id,
                        args.cursor.as_deref(),
                        args.offset,
                    ) {
                        Ok(page) => Some(page),
                        Err(error) => return result(Err(error)),
                    }
                } else {
                    None
                };
                (
                    self.api.alpha_report(&args.property_id, body, false).await,
                    page,
                )
            }
            Err(error) => (Err(error), None),
        };
        if controls.active() {
            result_with_tabular_projection(response, controls, false, pagination.as_ref())
        } else {
            result(response)
        }
    }

    #[tool(
        description = "Run Google Analytics Data API v1alpha funnel reporting with 1-10 funnel steps.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn run_funnel_report(&self, Parameters(args): Parameters<FunnelArgs>) -> CallToolResult {
        let mut controls = tabular::Controls {
            output_mode: args.output_mode,
            summary_only: args.summary_only,
            max_rows: args.max_rows,
            max_cell_chars: args.max_cell_chars,
            cursor_supplied: false,
        };
        if controls.active() && controls.max_rows.is_none() {
            // Funnel output uses a 200-row default per subreport.
            controls.max_rows = Some(200);
        }
        let limit = match controls.request_limit(args.limit) {
            Ok(value) => value,
            Err(error) => return result(Err(error)),
        };
        let request = google::funnel_request(
            args.funnel_steps,
            args.date_ranges,
            args.funnel_breakdown,
            args.funnel_next_action,
            args.segments,
            args.return_property_quota,
        )
        .and_then(|body| {
            google::funnel_request_options(
                body,
                args.is_open_funnel,
                args.funnel_visualization_type.as_deref(),
                limit,
                args.dimension_filter,
            )
        });
        let response = match request {
            Ok(body) => self.api.alpha_report(&args.property_id, body, true).await,
            Err(error) => Err(error),
        };
        if controls.active() {
            result_with_funnel_projection(response, controls)
        } else {
            result(response)
        }
    }

    #[tool(
        description = "Run a GA4 pivot report using the provider's RunPivotReportRequest JSON.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn run_pivot_report(&self, Parameters(args): Parameters<PivotArgs>) -> CallToolResult {
        let controls = tabular::Controls {
            output_mode: args.output_mode,
            summary_only: args.summary_only,
            max_rows: args.max_rows,
            max_cell_chars: args.max_cell_chars,
            cursor_supplied: args.cursor.is_some(),
        };
        // Google controls pivot offsets inside each pivot definition; never
        // pretend a flat historical report cursor can resume the pivot axes.
        if controls.cursor_supplied {
            return result(Err(ApiError::InvalidRequest));
        }
        if let Err(error) = controls.validate() {
            return result(Err(error));
        }
        let body = google::normalize_pivots(args.pivots, &args.dimensions, &args.metrics).and_then(
            |pivots| {
                report_request(
                    Some(args.date_ranges),
                    args.dimensions,
                    args.metrics,
                    vec![
                        ("pivots", Some(json!(pivots))),
                        ("dimensionFilter", args.dimension_filter),
                        ("metricFilter", args.metric_filter),
                        ("orderBys", args.order_bys.map(|x| json!(x))),
                        ("currencyCode", args.currency_code.map(|x| json!(x))),
                        ("keepEmptyRows", args.keep_empty_rows.map(|x| json!(x))),
                        (
                            "returnPropertyQuota",
                            Some(json!(args.return_property_quota)),
                        ),
                    ],
                )
            },
        );
        let response = match body {
            Ok(body) => {
                self.api
                    .data_operation(&args.property_id, body, "runPivotReport")
                    .await
            }
            Err(error) => Err(error),
        };
        if controls.active() {
            result_with_tabular_projection(response, controls, false, None)
        } else {
            result(response)
        }
    }

    #[tool(
        description = "Execute one to five GA4 historical reports with typed snake_case report arguments normalized into Google Data API batch JSON.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn batch_run_reports(&self, Parameters(args): Parameters<BatchArgs>) -> CallToolResult {
        if args.requests.is_empty() || args.requests.len() > 5 {
            return result(Err(ApiError::InvalidRequest));
        }
        let requests = args
            .requests
            .iter()
            .map(BatchReportItemArgs::to_google_request)
            .collect::<Result<Vec<_>, _>>();
        result(match requests {
            Ok(requests) => {
                self.api
                    .data_operation(
                        &args.property_id,
                        json!({"requests": requests}),
                        "batchRunReports",
                    )
                    .await
            }
            Err(error) => Err(error),
        })
    }

    #[tool(
        description = "Check GA4 Core Data API metric/dimension compatibility using CheckCompatibilityRequest JSON.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn check_report_compatibility(
        &self,
        Parameters(args): Parameters<CompatibilityArgs>,
    ) -> CallToolResult {
        let request = google::compatibility_request(args.dimensions, args.metrics);
        result(match request {
            Ok(body) => {
                self.api
                    .data_operation(&args.property_id, body, "checkCompatibility")
                    .await
            }
            Err(error) => Err(error),
        })
    }

    #[tool(
        description = "Run a GA4 property-level data access report (requires Google Analytics Administrator privileges).",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn run_property_access_report(
        &self,
        Parameters(args): Parameters<PropertyAccessArgs>,
    ) -> CallToolResult {
        let controls = tabular::Controls {
            output_mode: args.output_mode,
            summary_only: args.summary_only,
            max_rows: args.max_rows,
            max_cell_chars: args.max_cell_chars,
            cursor_supplied: args.cursor.is_some(),
        };
        let limit = match controls.request_limit(args.limit) {
            Ok(limit) => limit,
            Err(error) => return result(Err(error)),
        };
        let request = google::access_report_request(google::AccessReportSpec {
            date_ranges: args.date_ranges,
            dimensions: args.dimensions,
            metrics: args.metrics,
            dimension_filter: args.dimension_filter,
            metric_filter: args.metric_filter,
            order_bys: args.order_bys,
            limit,
            offset: args.offset,
            time_zone: args.time_zone,
        });
        let (response, pagination) = match request {
            Ok(mut body) => {
                let page = if controls.active() {
                    match tabular::prepare_access(
                        &mut body,
                        &args.property_id,
                        false,
                        args.cursor.as_deref(),
                        args.offset,
                    ) {
                        Ok(page) => Some(page),
                        Err(error) => return result(Err(error)),
                    }
                } else {
                    None
                };
                (
                    self.api.access_report(&args.property_id, body, false).await,
                    page,
                )
            }
            Err(error) => (Err(error), None),
        };
        if controls.active() {
            result_with_tabular_projection(response, controls, false, pagination.as_ref())
        } else {
            result(response)
        }
    }

    #[tool(
        description = "Run an account-level data access report (requires Google Analytics Administrator privileges).",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn run_account_access_report(
        &self,
        Parameters(args): Parameters<AccountAccessArgs>,
    ) -> CallToolResult {
        let controls = tabular::Controls {
            output_mode: args.output_mode,
            summary_only: args.summary_only,
            max_rows: args.max_rows,
            max_cell_chars: args.max_cell_chars,
            cursor_supplied: args.cursor.is_some(),
        };
        let limit = match controls.request_limit(args.limit) {
            Ok(limit) => limit,
            Err(error) => return result(Err(error)),
        };
        let request = google::access_report_request(google::AccessReportSpec {
            date_ranges: args.date_ranges,
            dimensions: args.dimensions,
            metrics: args.metrics,
            dimension_filter: args.dimension_filter,
            metric_filter: args.metric_filter,
            order_bys: args.order_bys,
            limit,
            offset: args.offset,
            time_zone: args.time_zone,
        });
        let (response, pagination) = match request {
            Ok(mut body) => {
                let page = if controls.active() {
                    match tabular::prepare_access(
                        &mut body,
                        &args.account_id,
                        true,
                        args.cursor.as_deref(),
                        args.offset,
                    ) {
                        Ok(page) => Some(page),
                        Err(error) => return result(Err(error)),
                    }
                } else {
                    None
                };
                (
                    self.api.access_report(&args.account_id, body, true).await,
                    page,
                )
            }
            Err(error) => (Err(error), None),
        };
        if controls.active() {
            result_with_tabular_projection(response, controls, false, pagination.as_ref())
        } else {
            result(response)
        }
    }

    #[tool(
        description = "Preview normalized historical report JSON offline without sending data to Google.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn preview_report_request(
        &self,
        Parameters(args): Parameters<ReportArgs>,
    ) -> CallToolResult {
        let controls = tabular::Controls {
            output_mode: args.output_mode,
            summary_only: args.summary_only,
            max_rows: args.max_rows,
            max_cell_chars: args.max_cell_chars,
            cursor_supplied: args.cursor.is_some(),
        };
        let validation = controls
            .request_limit(args.limit)
            .and_then(|effective_limit| {
                let property = google::property_name(&args.property_id)?;
                let projection = preview_report_projection(&args.dimensions, &args.metrics);
                let mut request = report_request(
                    Some(args.date_ranges),
                    args.dimensions,
                    args.metrics,
                    vec![
                        ("dimensionFilter", args.dimension_filter),
                        ("metricFilter", args.metric_filter),
                        ("orderBys", args.order_bys.map(|x| json!(x))),
                        ("limit", effective_limit.map(|x| json!(x.to_string()))),
                        ("offset", args.offset.map(|x| json!(x.to_string()))),
                        ("currencyCode", args.currency_code.map(|x| json!(x))),
                        ("keepEmptyRows", args.keep_empty_rows.map(|x| json!(x))),
                        (
                            "returnPropertyQuota",
                            Some(json!(args.return_property_quota)),
                        ),
                    ],
                )?;
                let page = tabular::prepare_historical(
                    &mut request,
                    &args.property_id,
                    args.cursor.as_deref(),
                    args.offset,
                )?;
                Ok(json!({
                    "request":request,
                    "provider_called":false,
                    "preview":{
                        "tool":"run_report",
                        "property":property,
                        "query_hash":page.query_hash,
                        "pagination":{
                            "requested_limit":args.limit,
                            "requested_offset":args.offset.unwrap_or(0),
                            "max_rows":args.max_rows,
                            "effective_limit":effective_limit,
                            "effective_offset":page.offset,
                            "cursor_supplied":args.cursor.is_some()
                        },
                        "compatibility_checked":false
                    },
                    "projection":projection,
                    "hints":[
                        "No Google Analytics API call was made.",
                        "Field names and combinations are not verified; use check_report_compatibility before large data requests."
                    ]
                }))
            });
        result(validation)
    }

    #[tool(
        description = "Run a historical Google Analytics Data API report with metric and dimension names.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn run_report(&self, Parameters(args): Parameters<ReportArgs>) -> CallToolResult {
        let controls = tabular::Controls {
            output_mode: args.output_mode,
            summary_only: args.summary_only,
            max_rows: args.max_rows,
            max_cell_chars: args.max_cell_chars,
            cursor_supplied: args.cursor.is_some(),
        };
        let effective_limit = match controls.request_limit(args.limit) {
            Ok(limit) => limit,
            Err(error) => return result(Err(error)),
        };
        let body = report_request(
            Some(args.date_ranges),
            args.dimensions,
            args.metrics,
            vec![
                ("dimensionFilter", args.dimension_filter),
                ("metricFilter", args.metric_filter),
                ("orderBys", args.order_bys.map(|x| json!(x))),
                ("limit", effective_limit.map(|x| json!(x.to_string()))),
                ("offset", args.offset.map(|x| json!(x.to_string()))),
                ("currencyCode", args.currency_code.map(|x| json!(x))),
                ("keepEmptyRows", args.keep_empty_rows.map(|x| json!(x))),
                (
                    "returnPropertyQuota",
                    Some(json!(args.return_property_quota)),
                ),
            ],
        );
        let (response, pagination) = match body {
            Ok(mut body) => {
                let page = if controls.active() {
                    match tabular::prepare_historical(
                        &mut body,
                        &args.property_id,
                        args.cursor.as_deref(),
                        args.offset,
                    ) {
                        Ok(page) => Some(page),
                        Err(error) => return result(Err(error)),
                    }
                } else {
                    None
                };
                (self.api.report(&args.property_id, body, false).await, page)
            }
            Err(error) => (Err(error), None),
        };
        if controls.active() {
            result_with_tabular_projection(response, controls, false, pagination.as_ref())
        } else {
            result(response)
        }
    }

    #[tool(
        description = "Run a GA4 realtime report with ordering and quota metadata; reject unsupported nonzero offsets explicitly rather than silently ignoring them.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn run_realtime_report(
        &self,
        Parameters(args): Parameters<RealtimeArgs>,
    ) -> CallToolResult {
        let controls = tabular::Controls {
            output_mode: args.output_mode,
            summary_only: args.summary_only,
            max_rows: args.max_rows,
            max_cell_chars: args.max_cell_chars,
            cursor_supplied: args.cursor.is_some(),
        };
        if controls.cursor_supplied {
            return result(Err(ApiError::InvalidRequest));
        }
        let effective_limit = match controls.request_limit(args.limit) {
            Ok(limit) => limit,
            Err(error) => return result(Err(error)),
        };
        let mut args = args;
        args.limit = effective_limit;
        let body = realtime_report_request(&args);
        let response = match body {
            Ok(body) => self.api.report(&args.property_id, body, true).await,
            Err(error) => Err(error),
        };
        if controls.active() {
            result_with_tabular_projection(response, controls, true, None)
        } else {
            result(response)
        }
    }
}

#[cfg(feature = "scratchpad")]
#[tool_router(router = scratch_tool_router)]
impl Synmetra {
    #[tool(
        description = "Open a private in-memory DuckDB session.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    fn scratchpad_open_session(
        &self,
        Parameters(args): Parameters<ScratchOpenArgs>,
    ) -> CallToolResult {
        local_result(
            self.scratchpad
                .as_ref()
                .expect("scratchpad capability required")
                .open(&args.session_id),
        )
    }

    #[tool(
        description = "Close a private DuckDB session and destroy its tables.",
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    fn scratchpad_close_session(
        &self,
        Parameters(args): Parameters<ScratchSessionArgs>,
    ) -> CallToolResult {
        local_result(
            self.scratchpad
                .as_ref()
                .expect("scratchpad capability required")
                .close(&args.session_id),
        )
    }

    #[tool(
        description = "List currently active local scratchpad sessions.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    fn scratchpad_list_sessions(
        &self,
        Parameters(args): Parameters<ScratchListArgs>,
    ) -> CallToolResult {
        local_result(
            self.scratchpad
                .as_ref()
                .expect("scratchpad capability required")
                .sessions(args.limit),
        )
    }

    #[tool(
        description = "Read hard-bounded scratchpad limits.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    fn scratchpad_get_runtime_limits(&self) -> CallToolResult {
        local_result(
            self.scratchpad
                .as_ref()
                .expect("scratchpad capability required")
                .limits(),
        )
    }

    #[tool(
        description = "Set the bounded local session quota.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    fn scratchpad_set_runtime_limits(
        &self,
        Parameters(args): Parameters<ScratchLimitArgs>,
    ) -> CallToolResult {
        local_result(
            self.scratchpad
                .as_ref()
                .expect("scratchpad capability required")
                .set_limits(args.max_sessions, args.max_tables_per_session),
        )
    }

    #[tool(
        description = "List DuckDB tables in one private session.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    fn scratchpad_list_tables(
        &self,
        Parameters(args): Parameters<ScratchListTablesArgs>,
    ) -> CallToolResult {
        local_result(
            self.scratchpad
                .as_ref()
                .expect("scratchpad capability required")
                .tables(&args.session_id, args.limit),
        )
    }

    #[tool(
        description = "Drop one validated local DuckDB table.",
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    fn scratchpad_drop_table(
        &self,
        Parameters(args): Parameters<ScratchTableArgs>,
    ) -> CallToolResult {
        local_result(
            self.scratchpad
                .as_ref()
                .expect("scratchpad capability required")
                .drop_table(&args.session_id, &args.table_name, args.if_exists),
        )
    }

    #[tool(
        description = "Inspect a validated local DuckDB table's schema (first 32 columns).",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    fn scratchpad_describe_table(
        &self,
        Parameters(args): Parameters<ScratchReadTableArgs>,
    ) -> CallToolResult {
        local_result(
            self.scratchpad
                .as_ref()
                .expect("scratchpad capability required")
                .describe_table(&args.session_id, &args.table_name),
        )
    }

    #[tool(
        description = "Summarize one validated local table with bounded column metadata and a row count.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    fn scratchpad_summarize_table(
        &self,
        Parameters(args): Parameters<ScratchReadTableArgs>,
    ) -> CallToolResult {
        local_result(
            self.scratchpad
                .as_ref()
                .expect("scratchpad capability required")
                .summarize_table(&args.session_id, &args.table_name),
        )
    }

    #[tool(
        description = "Export bounded private in-memory JSON and Markdown evidence from selected scratchpad tables, without touching the filesystem.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    fn scratchpad_export_evidence_bundle(
        &self,
        Parameters(args): Parameters<ScratchEvidenceArgs>,
    ) -> CallToolResult {
        local_result(
            self.scratchpad
                .as_ref()
                .expect("scratchpad capability required")
                .export_evidence_bundle(
                    &args.session_id,
                    args.table_names,
                    args.sample_rows_per_table,
                ),
        )
    }
    #[tool(
        description = "Read one bounded GA4 historical report and ingest its returned rows into a private DuckDB table. Append requires identical schema.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = true
        )
    )]
    async fn scratchpad_ingest_report(
        &self,
        Parameters(args): Parameters<ScratchIngestReportArgs>,
    ) -> CallToolResult {
        let scratch = self
            .scratchpad
            .as_ref()
            .expect("scratchpad capability required");
        if let Err(reason) = scratch.require_session(&args.session_id) {
            return local_result(Err(reason));
        }
        if args.limit.is_some_and(|v| v == 0 || v > 1000) {
            return local_result(Err("GA4 ingest limit must be 1-1000".into()));
        }
        let body = report_request(
            Some(args.date_ranges),
            args.dimensions,
            args.metrics,
            vec![
                ("dimensionFilter", args.dimension_filter),
                ("metricFilter", args.metric_filter),
                ("orderBys", args.order_bys.map(|v| json!(v))),
                ("limit", Some(json!(args.limit.unwrap_or(1000).to_string()))),
                ("offset", args.offset.map(|v| json!(v.to_string()))),
                ("currencyCode", args.currency_code.map(|v| json!(v))),
                (
                    "returnPropertyQuota",
                    Some(json!(args.return_property_quota)),
                ),
            ],
        );
        let body = match body {
            Ok(body) => body,
            Err(error) => return result(Err(error)),
        };
        match self.api.report(&args.property_id, body, false).await {
            Ok(data) => local_result(scratch.ingest_ga_response(
                &args.session_id,
                &args.table_name,
                &data,
                args.append,
            )),
            Err(error) => result(Err(error)),
        }
    }

    #[tool(
        description = "Read one bounded GA4 realtime report and ingest its returned rows into a private DuckDB table. Append requires identical schema.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = true
        )
    )]
    async fn scratchpad_ingest_realtime_report(
        &self,
        Parameters(args): Parameters<ScratchIngestRealtimeArgs>,
    ) -> CallToolResult {
        let scratch = self
            .scratchpad
            .as_ref()
            .expect("scratchpad capability required");
        if let Err(reason) = scratch.require_session(&args.session_id) {
            return local_result(Err(reason));
        }
        if args.limit.is_some_and(|v| v == 0 || v > 1000) {
            return local_result(Err("GA4 ingest limit must be 1-1000".into()));
        }
        if args.offset.is_some_and(|offset| offset > 0) {
            return result(Err(ApiError::UnsupportedRealtimeOffset));
        }
        let body = report_request(
            None,
            args.dimensions,
            args.metrics,
            vec![
                ("dimensionFilter", args.dimension_filter),
                ("metricFilter", args.metric_filter),
                ("orderBys", args.order_bys.map(|v| json!(v))),
                ("limit", Some(json!(args.limit.unwrap_or(1000).to_string()))),
                (
                    "returnPropertyQuota",
                    Some(json!(args.return_property_quota)),
                ),
            ],
        );
        let body = match body {
            Ok(body) => body,
            Err(error) => return result(Err(error)),
        };
        match self.api.report(&args.property_id, body, true).await {
            Ok(data) => local_result(scratch.ingest_ga_response(
                &args.session_id,
                &args.table_name,
                &data,
                args.append,
            )),
            Err(error) => result(Err(error)),
        }
    }
    #[tool(
        description = "Execute a restricted, identifier-validated read-only SELECT subset against one private DuckDB scratchpad session.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    fn scratchpad_query(&self, Parameters(args): Parameters<ScratchQueryArgs>) -> CallToolResult {
        if args.cursor.is_some()
            || args
                .output_mode
                .as_deref()
                .is_some_and(|mode| mode != "rows")
        {
            return local_result(Err(
                "This query profile supports rows output without pagination cursors".into(),
            ));
        }
        local_result(
            self.scratchpad
                .as_ref()
                .expect("scratchpad capability required")
                .query_bounded(
                    &args.session_id,
                    &args.sql,
                    args.max_rows,
                    args.max_cell_chars,
                    args.summary_only,
                ),
        )
    }
    #[tool(
        description = "Compare pre/post release anchor and baseline event ratios from one private DuckDB table. Heuristic only; no causal claims.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    fn scratchpad_release_regression_report(
        &self,
        Parameters(args): Parameters<ScratchReleaseArgs>,
    ) -> CallToolResult {
        if args.cursor.is_some() || args.output_mode.as_deref().is_some_and(|m| m != "rows") {
            return local_result(Err(
                "Release diagnostics do not yet support cursors or alternate output modes".into(),
            ));
        }
        let config = scratchpad::ReleaseRegressionConfig {
            session_id: args.session_id,
            table_name: args.table_name,
            release_date: args.release_date,
            anchor_event: args.anchor_event,
            comparison_event: args.comparison_event,
            date_column: args.date_column,
            event_column: args.event_column,
            metric_column: args.metric_column,
            pre_days: args.pre_days,
            transition_days: args.transition_days,
            post_days: args.post_days,
        };
        let _summary_only = args.summary_only;
        local_result(
            self.scratchpad
                .as_ref()
                .expect("scratchpad capability required")
                .release_regression(&config),
        )
    }
    #[tool(
        description = "Compare landing URL query-parameter key frequency before and after a release across optional channel and source/medium groups.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    fn scratchpad_landing_param_shift_report(
        &self,
        Parameters(args): Parameters<ScratchLandingArgs>,
    ) -> CallToolResult {
        if args.cursor.is_some() || args.output_mode.as_deref().is_some_and(|v| v != "rows") {
            return local_result(Err(
                "Landing diagnostics do not yet support cursors or alternate output modes".into(),
            ));
        }
        let config = scratchpad::LandingShiftConfig {
            session_id: args.session_id,
            table_name: args.table_name,
            release_date: args.release_date,
            date_column: args.date_column,
            landing_url_column: args.landing_url_column,
            channel_column: args.channel_column,
            source_medium_column: args.source_medium_column,
            pre_days: args.pre_days,
            transition_days: args.transition_days,
            post_days: args.post_days,
            top_n: args.top_n,
        };
        let _summary_only = args.summary_only;
        local_result(
            self.scratchpad
                .as_ref()
                .expect("scratchpad capability required")
                .landing_param_shift(&config),
        )
    }
}

#[tool_handler(
    router = self.router,
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
    let flags = if args.len() == 1 {
        &[][..]
    } else if args.get(1).is_some_and(|arg| arg == "serve") {
        &args[2..]
    } else {
        &args[1..]
    };
    let mut loopback = false;
    let mut allow_write = false;
    let mut scratchpad_requested = false;
    for flag in flags {
        match flag.as_str() {
            "--http-loopback" => loopback = true,
            "--allow-write" => allow_write = true,
            "--scratchpad" => scratchpad_requested = true,
            _ => {
                eprintln!("Usage: synmetra serve [--http-loopback] [--allow-write] [--scratchpad]");
                std::process::exit(2);
            }
        }
    }
    if args.len() > 1 && args[1] != "serve" {
        eprintln!("Usage: synmetra [serve [--http-loopback] [--allow-write] [--scratchpad]|--version|auth login --client-id-file PATH [--allow-edit]|auth status|auth logout]\nStarts an experimental read-only MCP stdio server.\nUse synmetra auth login for native read-only OAuth, or SYNMETRA_GOOGLE_ACCESS_TOKEN for a short-lived token.");
        std::process::exit(2);
    }
    // Scratchpad state is process-shared, so allowing multiple HTTP callers would
    // cross the session trust boundary. Restrict this early slice to one stdio client.
    if scratchpad_requested && loopback {
        return Err(
            "Scratchpad cannot run over HTTP until per-client session isolation is implemented"
                .into(),
        );
    }
    let server = Synmetra::new(
        GoogleClient::new(std::env::var("SYNMETRA_GOOGLE_ACCESS_TOKEN").ok()),
        allow_write,
    );
    let server = if scratchpad_requested {
        #[cfg(feature = "scratchpad")]
        {
            server.enable_scratchpad()
        }
        #[cfg(not(feature = "scratchpad"))]
        {
            return Err("Synmetra binary lacks optional scratchpad feature".into());
        }
    } else {
        server
    };
    if loopback {
        return http::serve_loopback(server).await;
    }
    server.serve(stdio()).await?.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_registered_tool_preserves_pinned_required_mcp_arguments() {
        use std::collections::BTreeSet;

        let sedna: Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let official: Value = serde_json::from_str(include_str!(
            "../spec/upstream/google-official-signatures.json"
        ))
        .unwrap();

        let baseline = Synmetra::new(GoogleClient::new(None), false);
        for tool in baseline.router.list_all() {
            let upstream = sedna["tools"]
                .as_array()
                .unwrap()
                .iter()
                .find(|entry| entry["name"].as_str() == Some(tool.name.as_ref()))
                .unwrap_or_else(|| panic!("Missing pinned upstream tool {}", tool.name));
            let expected: BTreeSet<_> = upstream["inputSchema"]["required"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect();
            let actual: BTreeSet<_> = tool
                .input_schema
                .get("required")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect();
            assert_eq!(
                actual, expected,
                "Required MCP parameters differ for {}",
                tool.name
            );
        }

        let with_write = Synmetra::new(GoogleClient::new(None), true);
        for reference in official["tools"].as_array().unwrap() {
            let name = reference["name"].as_str().unwrap();
            let tool = with_write
                .router
                .list_all()
                .into_iter()
                .find(|tool| tool.name == name)
                .unwrap_or_else(|| panic!("Missing official Google MCP tool {name}"));
            let expected: BTreeSet<_> = reference["required"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect();
            let actual: BTreeSet<_> = tool
                .input_schema
                .get("required")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect();
            assert_eq!(
                actual, expected,
                "Official Google parameters differ for {name}"
            );
        }
    }

    #[test]
    fn admin_list_page_controls_match_pinned_optional_inputs() {
        use std::collections::BTreeSet;
        let snapshot: Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let actual = Synmetra::new(GoogleClient::new(None), false)
            .router
            .list_all();
        for name in [
            "get_account_summaries",
            "list_google_ads_links",
            "list_property_annotations",
        ] {
            let upstream = snapshot["tools"]
                .as_array()
                .unwrap()
                .iter()
                .find(|item| item["name"] == name)
                .unwrap();
            let native = actual.iter().find(|tool| tool.name == name).unwrap();
            let expected: BTreeSet<_> = upstream["inputSchema"]["properties"]
                .as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect();
            let got: BTreeSet<_> = native
                .input_schema
                .get("properties")
                .and_then(Value::as_object)
                .into_iter()
                .flat_map(|items| items.keys())
                .map(String::as_str)
                .collect();
            assert_eq!(expected, got, "{name} optional input drift");
        }
    }

    #[test]
    fn only_implemented_tools_are_discovered() {
        let router = Synmetra::tool_router();
        let names: std::collections::HashSet<_> = router
            .list_all()
            .into_iter()
            .map(|tool| tool.name.to_string())
            .collect();
        assert_eq!(names.len(), 21);
        for name in [
            "ga4_get_started",
            "ga4_auth_status",
            "ga4_auth_login_command",
            "get_account_summaries",
            "get_property_details",
            "preview_report_request",
            "run_conversions_report",
            "run_funnel_report",
            "get_account_data_sharing_settings",
            "get_property_data_retention_settings",
            "list_google_ads_links",
            "list_property_annotations",
            "get_custom_dimensions_and_metrics",
            "run_report",
            "run_realtime_report",
            "run_pivot_report",
            "batch_run_reports",
            "check_report_compatibility",
            "run_property_access_report",
            "run_account_access_report",
        ] {
            assert!(names.contains(name), "missing {name}");
        }
        assert!(!Synmetra::new(GoogleClient::new(None), false)
            .router
            .has_route("create_property_annotation"));
        assert!(Synmetra::new(GoogleClient::new(None), true)
            .router
            .has_route("create_property_annotation"));
    }

    #[test]
    fn annotation_schema_and_payload_cover_default_color_and_single_or_range_dates() {
        let tool = Synmetra::tool_router()
            .list_all()
            .into_iter()
            .find(|tool| tool.name == "create_property_annotation")
            .unwrap();
        let schema = serde_json::to_value(&tool.input_schema).unwrap();
        let required = schema["required"].as_array().unwrap();
        for field in ["property_id", "title", "start_date"] {
            assert!(required.contains(&json!(field)));
        }
        for field in ["end_date", "description", "color"] {
            assert!(!required.contains(&json!(field)));
        }
        let color = &schema["properties"]["color"];
        let color_schema = if let Some(reference) = color["$ref"]
            .as_str()
            .and_then(|reference| reference.strip_prefix("#/$defs/"))
        {
            &schema["$defs"][reference]
        } else {
            color
        };
        let values = color_schema["enum"].as_array().unwrap();
        for expected in ["PURPLE", "BROWN", "BLUE", "GREEN", "RED", "CYAN"] {
            assert!(values.contains(&json!(expected)));
        }
        assert!(!values.contains(&json!("ORANGE")));
        let input: CreateAnnotationArgs = serde_json::from_value(json!({
            "property_id":42,
            "title":"Release",
            "start_date":"2026-10-25"
        }))
        .unwrap();
        assert_eq!(input.color, "BLUE");
        let single = annotation_payload(
            &input.title,
            &input.start_date,
            input.end_date.as_deref(),
            input.description.as_deref(),
            &input.color,
        )
        .unwrap();
        assert_eq!(
            single["reportingDataAnnotation"]["annotationDate"],
            json!({"year":2026,"month":10,"day":25})
        );
        assert_eq!(single["reportingDataAnnotation"]["color"], "BLUE");
        assert!(single["reportingDataAnnotation"]
            .get("annotationDateRange")
            .is_none());
        let ranged = annotation_payload(
            "Release notes",
            "2026-10-20",
            Some("2026-10-25"),
            Some("Private internal description"),
            "PURPLE",
        )
        .unwrap();
        assert_eq!(
            ranged["reportingDataAnnotation"]["annotationDateRange"]["startDate"],
            json!({"year":2026,"month":10,"day":20})
        );
        assert_eq!(
            ranged["reportingDataAnnotation"]["annotationDateRange"]["endDate"],
            json!({"year":2026,"month":10,"day":25})
        );
        assert_eq!(
            ranged["reportingDataAnnotation"]["description"],
            "Private internal description"
        );
        assert!(ranged["reportingDataAnnotation"]
            .get("annotationDate")
            .is_none());
    }

    #[test]
    fn mcp_tool_annotations_match_read_only_and_opt_in_write_effects() {
        for tool in Synmetra::tool_router().list_all() {
            let hints = tool
                .annotations
                .as_ref()
                .expect("tool annotations required");
            if tool.name == "create_property_annotation" {
                assert_eq!(hints.read_only_hint, Some(false));
                assert_eq!(hints.destructive_hint, Some(false));
                assert_eq!(hints.idempotent_hint, Some(false));
                assert_eq!(hints.open_world_hint, Some(true));
            } else {
                assert_eq!(hints.read_only_hint, Some(true), "{}", tool.name);
                assert_eq!(hints.destructive_hint, Some(false));
            }
        }
    }

    #[tokio::test]
    async fn in_memory_mcp_client_negotiates_and_calls_read_only_tools() {
        use rmcp::model::CallToolRequestParams;
        let (server_stream, client_stream) = tokio::io::duplex(65536);
        let worker = tokio::spawn(async move {
            let server = Synmetra::new(GoogleClient::new(None), false);
            server
                .serve(server_stream)
                .await
                .unwrap()
                .waiting()
                .await
                .unwrap();
        });
        let client = ().serve(client_stream).await.unwrap();
        let tools = client.list_all_tools().await.unwrap();
        assert_eq!(tools.len(), 20);
        assert!(tools
            .iter()
            .all(|tool| tool.name != "create_property_annotation"));
        let call = client
            .call_tool(CallToolRequestParams::new("get_account_summaries"))
            .await
            .unwrap();
        assert_eq!(call.is_error, Some(true));
        let message = &call.content[0].as_text().unwrap().text;
        assert!(message.contains("No Google access token"));
        assert!(!message.contains("test-token"));
        client.cancel().await.unwrap();
        worker.await.unwrap();
    }

    #[tokio::test]
    async fn write_capability_is_hidden_by_default_and_requires_scope() {
        use rmcp::model::CallToolRequestParams;
        let (server_stream, client_stream) = tokio::io::duplex(65536);
        let worker = tokio::spawn(async move {
            Synmetra::new(GoogleClient::new(None), true)
                .serve(server_stream)
                .await
                .unwrap()
                .waiting()
                .await
                .unwrap();
        });
        let client = ().serve(client_stream).await.unwrap();
        let tools = client.list_all_tools().await.unwrap();
        assert_eq!(tools.len(), 21);
        assert!(tools.iter().any(|t| t.name == "create_property_annotation"));
        let mut args = serde_json::Map::new();
        args.insert("property_id".into(), json!("42"));
        args.insert("title".into(), json!("Test"));
        args.insert("start_date".into(), json!("2026-10-25"));
        let call = client
            .call_tool(
                CallToolRequestParams::new("create_property_annotation").with_arguments(args),
            )
            .await
            .unwrap();
        assert_eq!(call.is_error, Some(true));
        client.cancel().await.unwrap();
        worker.await.unwrap();
    }

    #[tokio::test]
    async fn core_preflight_and_batch_reject_invalid_input_before_google_calls() {
        let server = Synmetra::new(GoogleClient::new(None), false);
        let bad_batch = server
            .batch_run_reports(Parameters(BatchArgs {
                property_id: "42".into(),
                requests: vec![],
            }))
            .await;
        assert_eq!(bad_batch.is_error, Some(true));
        let bad_pivot = server
            .run_pivot_report(Parameters(PivotArgs {
                property_id: "42".into(),
                date_ranges: vec![json!({"startDate":"today","endDate":"today"})],
                dimensions: vec!["country".into()],
                metrics: vec!["activeUsers".into()],
                pivots: vec![],
                dimension_filter: None,
                metric_filter: None,
                order_bys: None,
                currency_code: None,
                keep_empty_rows: None,
                return_property_quota: false,
                max_rows: None,
                max_cell_chars: None,
                summary_only: false,
                output_mode: None,
                cursor: None,
            }))
            .await;
        assert_eq!(bad_pivot.is_error, Some(true));
        let bad_compat = server
            .check_report_compatibility(Parameters(CompatibilityArgs {
                property_id: "42".into(),
                dimensions: vec!["country".into()],
                metrics: vec![],
            }))
            .await;
        assert_eq!(bad_compat.is_error, Some(true));
    }

    #[tokio::test]
    async fn offline_preview_rejects_bad_metrics_without_credentials() {
        let server = Synmetra::new(GoogleClient::new(None), false);
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
            return_property_quota: true,
            max_rows: None,
            max_cell_chars: None,
            summary_only: false,
            output_mode: None,
            cursor: None,
        };
        let result = server.preview_report_request(Parameters(bad)).await;
        assert_eq!(result.is_error, Some(true));
    }

    #[tokio::test]
    async fn offline_preview_returns_query_bound_projection_without_google_credentials() {
        let server = Synmetra::new(GoogleClient::new(None), false);
        let input = json!({
            "property_id":42,
            "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
            "dimensions":["date","dateHour","landingPagePlusQueryString"],
            "metrics":["activeUsers"],
            "max_rows":2,
            "output_mode":"compact"
        });
        let first = server
            .preview_report_request(Parameters(serde_json::from_value(input.clone()).unwrap()))
            .await;
        assert_eq!(first.is_error, Some(false));
        let data = first.structured_content.unwrap()["data"].clone();
        assert_eq!(data["provider_called"], false);
        assert_eq!(data["preview"]["property"], "properties/42");
        assert_eq!(data["preview"]["pagination"]["effective_limit"], 2);
        assert_eq!(data["request"]["limit"], "2");
        assert_eq!(data["preview"]["compatibility_checked"], false);
        assert_eq!(
            data["projection"]["tabular_columns"][0]["logical_type"],
            "string"
        );
        assert_eq!(
            data["projection"]["tabular_columns"][3]["logical_type"],
            "number"
        );
        assert_eq!(
            data["projection"]["ingest_columns"][1]["transform"],
            "parse_ga_date"
        );
        assert_eq!(
            data["projection"]["ingest_columns"][3]["transform"],
            "parse_ga_date_hour"
        );
        let second = server
            .preview_report_request(Parameters(serde_json::from_value(input.clone()).unwrap()))
            .await;
        assert_eq!(
            data["preview"]["query_hash"],
            second.structured_content.unwrap()["data"]["preview"]["query_hash"]
        );
        let mut changed = input;
        changed["metrics"] = json!(["screenPageViews"]);
        let third = server
            .preview_report_request(Parameters(serde_json::from_value(changed).unwrap()))
            .await;
        assert_ne!(
            data["preview"]["query_hash"],
            third.structured_content.unwrap()["data"]["preview"]["query_hash"]
        );
    }

    #[tokio::test]
    async fn compatibility_preflight_surfaces_incompatible_metric_without_run_report() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let responder = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buffer = [0_u8; 4096];
            let read = stream.read(&mut buffer).await.unwrap();
            let request = String::from_utf8_lossy(&buffer[..read]);
            assert!(request.contains("properties/42:checkCompatibility"));
            assert!(!request.contains(":runReport"));
            let body = json!({
                "metricCompatibilities":[{
                    "metricMetadata":{"apiName":"unsupportedCustomMetric"},
                    "compatibility":"INCOMPATIBLE"
                }],
                "dimensionCompatibilities":[{
                    "dimensionMetadata":{"apiName":"country"},
                    "compatibility":"COMPATIBLE"
                }]
            })
            .to_string();
            let reply = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(reply.as_bytes()).await.unwrap();
        });
        let client = GoogleClient::with_endpoints(
            Some("private-metadata-fixture-token".into()),
            &endpoint,
            &endpoint,
        );
        let server = Synmetra::new(client, false);
        let checked: CompatibilityArgs = serde_json::from_value(json!({
            "property_id":42,
            "dimensions":["country"],
            "metrics":["unsupportedCustomMetric"]
        }))
        .unwrap();
        let compatibility = server.check_report_compatibility(Parameters(checked)).await;
        responder.await.unwrap();
        assert_eq!(compatibility.is_error, Some(false));
        let content = compatibility.structured_content.unwrap();
        assert_eq!(
            content["data"]["metricCompatibilities"][0]["compatibility"],
            "INCOMPATIBLE"
        );
        assert!(!content
            .to_string()
            .contains("private-metadata-fixture-token"));

        let preview: ReportArgs = serde_json::from_value(json!({
            "property_id":42,
            "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
            "dimensions":["country"],
            "metrics":["unsupportedCustomMetric"]
        }))
        .unwrap();
        let offline = server.preview_report_request(Parameters(preview)).await;
        assert_eq!(offline.is_error, Some(false));
        let preview_payload = offline.structured_content.unwrap();
        assert_eq!(preview_payload["data"]["provider_called"], false);
        assert_eq!(
            preview_payload["data"]["preview"]["compatibility_checked"],
            false
        );
        assert_eq!(
            preview_payload["data"]["request"]["metrics"][0]["name"],
            "unsupportedCustomMetric"
        );
    }

    #[tokio::test]
    async fn offline_preview_rejects_oversized_report_options_with_a_typed_error() {
        let server = Synmetra::new(GoogleClient::new(None), false);
        let oversized = (0..5000)
            .map(|index| json!({"dimension":{"dimensionName":format!("field{index}")}}))
            .collect::<Vec<_>>();
        let args: ReportArgs = serde_json::from_value(json!({
            "property_id": "42",
            "date_ranges": [{"startDate": "yesterday", "endDate": "today"}],
            "dimensions": ["country"],
            "metrics": ["activeUsers"],
            "order_bys": oversized
        }))
        .unwrap();
        let reply = server.preview_report_request(Parameters(args)).await;
        assert_eq!(reply.is_error, Some(true));
        let payload = reply.structured_content.unwrap();
        assert_eq!(payload["error"]["code"], "INVALID_REQUEST");
        assert_eq!(payload["error"]["category"], "validation");
        assert!(payload["data"].is_null());
    }

    #[tokio::test]
    async fn standard_report_quota_flag_has_upstream_default_and_explicit_override() {
        let server = Synmetra::new(GoogleClient::new(None), false);
        let fields = json!({
            "property_id": "55",
            "date_ranges": [{"startDate": "2026-10-01", "endDate": "2026-10-02"}],
            "dimensions": [],
            "metrics": ["activeUsers"]
        });
        let default_args: ReportArgs = serde_json::from_value(fields.clone()).unwrap();
        assert!(default_args.return_property_quota);
        let response = server
            .preview_report_request(Parameters(default_args))
            .await;
        assert_eq!(response.is_error, Some(false));
        assert_eq!(
            response.structured_content.unwrap()["data"]["request"]["returnPropertyQuota"],
            true
        );

        let mut overridden = fields;
        overridden["return_property_quota"] = json!(false);
        let explicit_args: ReportArgs = serde_json::from_value(overridden).unwrap();
        assert!(!explicit_args.return_property_quota);
        let response = server
            .preview_report_request(Parameters(explicit_args))
            .await;
        assert_eq!(
            response.structured_content.unwrap()["data"]["request"]["returnPropertyQuota"],
            false
        );
    }

    #[test]
    fn report_date_range_mcp_schema_requires_named_start_and_end_fields() {
        let router = Synmetra::tool_router();
        for name in [
            "run_report",
            "run_pivot_report",
            "run_conversions_report",
            "batch_run_reports",
            "run_property_access_report",
        ] {
            let tool = router
                .list_all()
                .into_iter()
                .find(|item| item.name == name)
                .unwrap();
            let full = serde_json::to_value(&tool.input_schema).unwrap();
            let items = if name == "batch_run_reports" {
                let item_ref = full["properties"]["requests"]["items"]["$ref"]
                    .as_str()
                    .unwrap();
                let item_name = item_ref.strip_prefix("#/$defs/").unwrap();
                &full["$defs"][item_name]["properties"]["date_ranges"]["items"]
            } else {
                &full["properties"]["date_ranges"]["items"]
            };
            let resolved = if let Some(key) = items["$ref"]
                .as_str()
                .and_then(|path| path.strip_prefix("#/$defs/"))
            {
                &full["$defs"][key]
            } else {
                items
            };
            let required = resolved["required"].as_array().unwrap();
            assert!(required.contains(&json!("start_date")), "{name}");
            assert!(required.contains(&json!("end_date")), "{name}");
        }
    }

    #[test]
    fn resource_ids_accept_integer_or_canonical_text_in_registered_mcp_schemas() {
        let base = json!({
            "property_id":42,
            "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
            "dimensions":[],
            "metrics":["activeUsers"]
        });
        let report: ReportArgs = serde_json::from_value(base).unwrap();
        assert_eq!(report.property_id, "42");
        let prop: PropertyArgs = serde_json::from_value(json!({"property_id":42})).unwrap();
        assert_eq!(prop.property_id, "42");
        let prop: PropertyArgs =
            serde_json::from_value(json!({"property_id":"properties/42"})).unwrap();
        assert_eq!(prop.property_id, "properties/42");
        let account: AccountArgs = serde_json::from_value(json!({"account_id":97})).unwrap();
        assert_eq!(account.account_id, "97");
        for bad in [json!(-1), json!(1.5), json!(true), Value::Null] {
            assert!(serde_json::from_value::<PropertyArgs>(json!({"property_id":bad})).is_err());
        }
        let router = Synmetra::tool_router();
        for (tool_name, field_name) in [
            ("run_report", "property_id"),
            ("run_realtime_report", "property_id"),
            ("get_property_details", "property_id"),
            ("get_account_data_sharing_settings", "account_id"),
        ] {
            let tool = router
                .list_all()
                .into_iter()
                .find(|item| item.name == tool_name)
                .unwrap();
            let all = serde_json::to_value(&tool.input_schema).unwrap();
            let raw = &all["properties"][field_name];
            let schema = if let Some(key) = raw["$ref"]
                .as_str()
                .and_then(|path| path.strip_prefix("#/$defs/"))
            {
                &all["$defs"][key]
            } else {
                raw
            };
            assert!(
                schema.get("anyOf").is_some()
                    || schema.get("oneOf").is_some()
                    || schema["type"]
                        .as_array()
                        .is_some_and(|types| types.len() >= 2),
                "{tool_name} must advertise numeric or text IDs"
            );
        }
    }

    #[tokio::test]
    async fn report_limit_width_matches_pinned_schema_and_rejects_over_limit_offline() {
        let server = Synmetra::new(GoogleClient::new(None), false);
        let pinned: Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let router = Synmetra::tool_router();
        for name in ["run_report", "run_realtime_report"] {
            let expected = pinned["tools"]
                .as_array()
                .unwrap()
                .iter()
                .find(|entry| entry["name"] == name)
                .unwrap();
            let actual = router
                .list_all()
                .into_iter()
                .find(|tool| tool.name == name)
                .unwrap();
            assert_eq!(
                actual.input_schema["properties"]["limit"]["format"],
                expected["inputSchema"]["properties"]["limit"]["format"],
                "{name}"
            );
            assert_eq!(
                actual.input_schema["properties"]["limit"]["format"],
                "uint64"
            );
        }

        let huge = json!({
            "property_id":"42",
            "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
            "dimensions":["country"],
            "metrics":["activeUsers"],
            "limit":4294967296_u64
        });
        let preview = server
            .preview_report_request(Parameters(serde_json::from_value(huge.clone()).unwrap()))
            .await;
        assert_eq!(preview.is_error, Some(true));
        assert_eq!(
            preview.structured_content.unwrap()["error"]["code"],
            "INVALID_REQUEST"
        );

        let realtime: RealtimeArgs = serde_json::from_value(json!({
            "property_id":"42",
            "dimensions":[],
            "metrics":["activeUsers"],
            "limit":4294967296_u64
        }))
        .unwrap();
        let result = server.run_realtime_report(Parameters(realtime)).await;
        assert_eq!(result.is_error, Some(true));
        assert_eq!(
            result.structured_content.unwrap()["error"]["code"],
            "INVALID_REQUEST"
        );
    }

    #[test]
    fn realtime_official_optional_arguments_are_declared_and_ordering_is_forwarded() {
        let official: Value = serde_json::from_str(include_str!(
            "../spec/upstream/google-official-signatures.json"
        ))
        .unwrap();
        let signature = official["tools"]
            .as_array()
            .unwrap()
            .iter()
            .find(|tool| tool["name"] == "run_realtime_report")
            .unwrap();
        let declared = Synmetra::tool_router()
            .list_all()
            .into_iter()
            .find(|tool| tool.name == "run_realtime_report")
            .unwrap();
        let properties = declared.input_schema.get("properties").unwrap();
        for arg in signature["parameters"].as_array().unwrap() {
            let name = arg["name"].as_str().unwrap();
            assert!(
                properties.get(name).is_some(),
                "Missing official argument {name}"
            );
        }

        let args: RealtimeArgs = serde_json::from_value(json!({
            "property_id": "42",
            "dimensions": ["country"],
            "metrics": ["activeUsers"],
            "order_bys": [{"dimension": {"dimensionName": "country"}}],
            "offset": 0
        }))
        .unwrap();
        let body = realtime_report_request(&args).unwrap();
        assert_eq!(body["orderBys"][0]["dimension"]["dimensionName"], "country");
        assert_eq!(body["returnPropertyQuota"], true);
        assert!(body.get("offset").is_none());
    }

    #[tokio::test]
    async fn compatibility_probe_uses_property_zero_without_a_report_call() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let responder = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buffer = [0_u8; 4096];
            let n = stream.read(&mut buffer).await.unwrap();
            let request = String::from_utf8_lossy(&buffer[..n]);
            assert!(request.starts_with("POST /v1beta/properties/0:checkCompatibility "));
            assert!(!request.contains(":runReport"));
            assert!(request.contains(r#""dimensions":[]"#));
            assert!(request.contains(r#""metrics":[]"#));
            let body = json!({
                "dimensionCompatibilities":[{"dimensionMetadata":{"apiName":"country"},"compatibility":"COMPATIBLE"}],
                "metricCompatibilities":[{"metricMetadata":{"apiName":"activeUsers"},"compatibility":"COMPATIBLE"}]
            })
            .to_string();
            let reply = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(reply.as_bytes()).await.unwrap();
        });
        let server = Synmetra::new(
            GoogleClient::with_endpoints(Some("secret-fixture-token".into()), &endpoint, &endpoint),
            false,
        );
        let args: CompatibilityArgs = serde_json::from_value(json!({
            "property_id":0,
            "dimensions":[],
            "metrics":[]
        }))
        .unwrap();
        let response = server.check_report_compatibility(Parameters(args)).await;
        responder.await.unwrap();
        assert_eq!(response.is_error, Some(false));
        let payload = response.structured_content.unwrap();
        assert_eq!(
            payload["data"]["dimensionCompatibilities"][0]["compatibility"],
            "COMPATIBLE"
        );
        assert_eq!(
            payload["data"]["metricCompatibilities"][0]["compatibility"],
            "COMPATIBLE"
        );
        assert!(!payload.to_string().contains("secret-fixture-token"));

        let invalid = Synmetra::new(GoogleClient::new(None), false)
            .check_report_compatibility(Parameters(CompatibilityArgs {
                property_id: "0".into(),
                dimensions: vec!["bad\nname".into()],
                metrics: vec![],
            }))
            .await;
        assert_eq!(invalid.is_error, Some(true));
        assert_eq!(
            invalid.structured_content.unwrap()["error"]["code"],
            "INVALID_REQUEST"
        );
    }

    #[test]
    fn all_compatible_ga_report_mcp_tools_advertise_four_tabular_output_modes() {
        let upstream: Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let router = Synmetra::tool_router();
        for name in [
            "run_report",
            "run_realtime_report",
            "run_conversions_report",
            "run_pivot_report",
            "run_funnel_report",
            "run_property_access_report",
            "run_account_access_report",
        ] {
            let original = upstream["tools"]
                .as_array()
                .unwrap()
                .iter()
                .find(|item| item["name"] == name)
                .unwrap();
            assert!(original["inputSchema"]["properties"]["output_mode"].is_object());
            let tool = router
                .list_all()
                .into_iter()
                .find(|item| item.name == name)
                .unwrap();
            assert!(
                tool.input_schema["properties"]["output_mode"].is_object(),
                "{name} is missing tabular output_mode"
            );
        }
        for name in ["rows", "tuples", "scalar", "compact"] {
            let mode: tabular::OutputMode = serde_json::from_value(json!(name)).unwrap();
            assert!(matches!(
                mode,
                tabular::OutputMode::Rows
                    | tabular::OutputMode::Tuples
                    | tabular::OutputMode::Scalar
                    | tabular::OutputMode::Compact
            ));
        }
        for invalid in ["csv", "jsonl", "table", "scalar-list"] {
            assert!(serde_json::from_value::<tabular::OutputMode>(json!(invalid)).is_err());
        }
    }

    #[test]
    fn upstream_parameter_audit_preserves_complete_pinned_source_fields() {
        use std::collections::BTreeSet;

        let audit: Value =
            serde_json::from_str(include_str!("../spec/audits/upstream-parameter-union.json"))
                .unwrap();
        let sedna: Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let google: Value = serde_json::from_str(include_str!(
            "../spec/upstream/google-official-signatures.json"
        ))
        .unwrap();
        let records = audit["tools"].as_array().unwrap();
        assert_eq!(records.len(), 36);
        let names: BTreeSet<_> = records
            .iter()
            .map(|record| record["name"].as_str().unwrap())
            .collect();
        assert_eq!(names.len(), records.len());

        for tool in sedna["tools"].as_array().unwrap() {
            let name = tool["name"].as_str().unwrap();
            let record = records
                .iter()
                .find(|record| record["name"] == name)
                .unwrap();
            assert_eq!(
                record["sednalabs_defs"],
                tool["inputSchema"]
                    .get("$defs")
                    .cloned()
                    .unwrap_or_else(|| json!({})),
                "nested schemas differ for {name}"
            );
            let pinned_fields = tool["inputSchema"]["properties"].as_object().unwrap();
            let captured = record["sednalabs_parameters"].as_array().unwrap();
            assert_eq!(captured.len(), pinned_fields.len(), "{name} field count");
            for (field, schema) in pinned_fields {
                let reported = captured
                    .iter()
                    .find(|item| item["name"] == field.as_str())
                    .unwrap();
                assert_eq!(reported["raw_schema"], *schema, "{name}.{field}");
                let required = tool["inputSchema"]["required"]
                    .as_array()
                    .is_some_and(|set| set.contains(&json!(field)));
                assert_eq!(reported["required"], required, "{name}.{field}");
            }
        }
        for tool in google["tools"].as_array().unwrap() {
            let name = tool["name"].as_str().unwrap();
            let record = records
                .iter()
                .find(|record| record["name"] == name)
                .unwrap();
            let captured = record["google_signature_parameters"].as_array().unwrap();
            let source = tool["parameters"].as_array().unwrap();
            assert_eq!(captured.len(), source.len(), "{name} field count");
            for field in source {
                let field_name = field["name"].as_str().unwrap();
                let reported = captured
                    .iter()
                    .find(|item| item["name"] == field_name)
                    .unwrap();
                assert_eq!(
                    reported["required"],
                    tool["required"]
                        .as_array()
                        .unwrap()
                        .contains(&json!(field_name)),
                    "{name}.{field_name}"
                );
            }
        }
        assert_eq!(audit["summary"]["shared_tools"], 9);
        assert_eq!(audit["summary"]["google_only_tools"], 1);
    }

    #[test]
    fn core_reporting_inputs_cover_pinned_contract_without_untracked_fields() {
        use std::collections::BTreeSet;

        let pinned: Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let router = Synmetra::tool_router();
        for name in [
            "run_report",
            "run_realtime_report",
            "run_conversions_report",
        ] {
            let upstream = pinned["tools"]
                .as_array()
                .unwrap()
                .iter()
                .find(|tool| tool["name"] == name)
                .unwrap();
            let local = router
                .list_all()
                .into_iter()
                .find(|tool| tool.name == name)
                .unwrap();
            let expected: BTreeSet<_> = upstream["inputSchema"]["properties"]
                .as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect();
            let actual: BTreeSet<_> = local.input_schema["properties"]
                .as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect();
            assert!(
                expected.is_subset(&actual),
                "Missing pinned reporting fields: {name}"
            );
            let extras: Vec<_> = actual.difference(&expected).copied().collect();
            if name == "run_report" {
                assert_eq!(extras, ["keep_empty_rows"]);
            } else {
                assert!(
                    extras.is_empty(),
                    "Unexpected reporting fields for {name}: {extras:?}"
                );
            }

            let expected_required: BTreeSet<_> = upstream["inputSchema"]["required"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(Value::as_str)
                .collect();
            let actual_required: BTreeSet<_> = local.input_schema["required"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(Value::as_str)
                .collect();
            assert_eq!(
                actual_required, expected_required,
                "Required fields differ for {name}"
            );
            assert_eq!(
                local.input_schema["properties"]["limit"]["format"],
                "uint64"
            );
            assert_eq!(
                local.input_schema["properties"]["return_property_quota"]["default"],
                upstream["inputSchema"]["properties"]["return_property_quota"]["default"]
            );
        }
    }

    #[test]
    fn conversion_spec_schema_advertises_pinned_action_and_attribution_fields() {
        let router = Synmetra::tool_router();
        let tool = router
            .list_all()
            .into_iter()
            .find(|tool| tool.name == "run_conversions_report")
            .unwrap();
        let all = serde_json::to_value(&tool.input_schema).unwrap();
        let source = &all["properties"]["conversion_spec"];
        let schema = if let Some(name) = source["$ref"]
            .as_str()
            .and_then(|reference| reference.strip_prefix("#/$defs/"))
        {
            &all["$defs"][name]
        } else {
            source
        };
        assert!(schema["properties"]["conversion_actions"]["items"].is_object());
        assert!(schema["properties"]["attribution_model"].is_object());
        assert_eq!(
            schema["properties"]["conversion_actions"]["default"],
            json!([])
        );
    }

    #[tokio::test]
    async fn conversion_report_opt_in_projection_uses_alpha_endpoint_and_bounded_response() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let responder = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buffer = [0u8; 4096];
            let len = stream.read(&mut buffer).await.unwrap();
            let request = String::from_utf8_lossy(&buffer[..len]);
            assert!(request.starts_with("POST /v1alpha/properties/42:runReport "));
            assert!(request.contains(r#""conversionSpec":{"attributionModel":"LAST_CLICK""#));
            assert!(request.contains(r#""limit":"1""#));
            let body = json!({
                "dimensionHeaders":[{"name":"country"}],
                "metricHeaders":[{"name":"allConversionsByInteractionDate"}],
                "rows":[{"dimensionValues":[{"value":"Mexico"}],"metricValues":[{"value":"4"}]}],
                "rowCount":2
            })
            .to_string();
            let reply = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(reply.as_bytes()).await.unwrap();
        });
        let server = Synmetra::new(
            GoogleClient::with_endpoints(Some("mock-private-token".into()), &endpoint, &endpoint),
            false,
        );
        let args: ConversionArgs = serde_json::from_value(json!({
            "property_id":42,
            "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
            "dimensions":["country"],
            "metrics":["allConversionsByInteractionDate"],
            "conversion_spec":{"attribution_model":"LAST_CLICK"},
            "max_rows":1,
            "output_mode":"rows",
            "summary_only":false
        }))
        .unwrap();
        let response = server.run_conversions_report(Parameters(args)).await;
        responder.await.unwrap();
        assert_eq!(response.is_error, Some(false));
        let payload = response.structured_content.unwrap();
        assert_eq!(payload["data"][0]["country"], "Mexico");
        assert_eq!(payload["meta"]["row_count_total"], 2);
        assert_eq!(payload["meta"]["row_count_returned"], 1);
        assert!(payload["meta"]["next_cursor"].as_str().is_some());
        assert!(!payload.to_string().contains("mock-private-token"));
    }

    #[test]
    fn pivot_input_fields_and_required_sets_match_pinned_contract() {
        use std::collections::BTreeSet;

        let pinned: Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let source = pinned["tools"]
            .as_array()
            .unwrap()
            .iter()
            .find(|tool| tool["name"] == "run_pivot_report")
            .unwrap();
        let local = Synmetra::tool_router()
            .list_all()
            .into_iter()
            .find(|tool| tool.name == "run_pivot_report")
            .unwrap();
        let fields: BTreeSet<_> = local.input_schema["properties"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        let upstream: BTreeSet<_> = source["inputSchema"]["properties"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(fields, upstream);
        let required: BTreeSet<_> = local.input_schema["required"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(Value::as_str)
            .collect();
        let expected: BTreeSet<_> = source["inputSchema"]["required"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(Value::as_str)
            .collect();
        assert_eq!(required, expected);
        assert_eq!(
            local.input_schema["properties"]["return_property_quota"]["default"],
            source["inputSchema"]["properties"]["return_property_quota"]["default"]
        );
    }

    #[test]
    fn funnel_input_schema_covers_pinned_fields_and_structured_steps() {
        use std::collections::BTreeSet;

        let pinned: Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let upstream = pinned["tools"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["name"] == "run_funnel_report")
            .unwrap();
        let local = Synmetra::tool_router()
            .list_all()
            .into_iter()
            .find(|tool| tool.name == "run_funnel_report")
            .unwrap();
        let native: BTreeSet<_> = local.input_schema["properties"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        let expected: BTreeSet<_> = upstream["inputSchema"]["properties"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(native, expected);
        let native_required: BTreeSet<_> = local.input_schema["required"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(Value::as_str)
            .collect();
        let pinned_required: BTreeSet<_> = upstream["inputSchema"]["required"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(Value::as_str)
            .collect();
        assert_eq!(native_required, pinned_required);
        assert_eq!(
            local.input_schema["properties"]["limit"]["format"],
            "uint64"
        );
        let schema = serde_json::to_value(&local.input_schema).unwrap();
        assert!(schema["properties"]["funnel_steps"]["items"]["$ref"].is_string());
    }

    #[tokio::test]
    async fn funnel_subreports_are_projected_independently_with_open_funnel_options() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let responder = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buffer = [0u8; 4096];
            let n = stream.read(&mut buffer).await.unwrap();
            let request = String::from_utf8_lossy(&buffer[..n]);
            assert!(request.contains("properties/42:runFunnelReport"));
            assert!(request.contains(r#""isOpenFunnel":true"#));
            assert!(request.contains(r#""funnelVisualizationType":"TRENDED_FUNNEL""#));
            assert!(request.contains(r#""limit":"1""#));
            let subreport = json!({
                "dimensionHeaders":[{"name":"funnelStepName"}],
                "metricHeaders":[{"name":"activeUsers"}],
                "rows":[{"dimensionValues":[{"value":"1. Viewed"}],
                         "metricValues":[{"value":"9"}]}],
                "rowCount":2
            });
            let body = json!({
                "funnelTable": subreport,
                "funnelVisualization": subreport,
                "propertyQuota":{"concurrentRequests":{"remaining":2}}
            })
            .to_string();
            let reply = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(reply.as_bytes()).await.unwrap();
        });
        let server = Synmetra::new(
            GoogleClient::with_endpoints(Some("private-funnel-token".into()), &endpoint, &endpoint),
            false,
        );
        let args: FunnelArgs = serde_json::from_value(json!({
            "property_id":42,
            "funnel_steps":[{"name":"Viewed","event":"page_view"}],
            "is_open_funnel":true,
            "funnel_visualization_type":"TRENDED_FUNNEL",
            "dimension_filter":"country==US",
            "limit":10,
            "max_rows":1,
            "output_mode":"rows"
        }))
        .unwrap();
        let result = server.run_funnel_report(Parameters(args)).await;
        responder.await.unwrap();
        assert_eq!(result.is_error, Some(false));
        let payload = result.structured_content.unwrap();
        assert_eq!(
            payload["data"]["funnel_table"][0]["funnelStepName"],
            "1. Viewed"
        );
        assert_eq!(
            payload["data"]["funnel_visualization"][0]["activeUsers"],
            "9"
        );
        assert_eq!(payload["meta"]["funnel_table"]["row_count_total"], 2);
        assert_eq!(
            payload["meta"]["funnel_visualization"]["row_count_returned"],
            1
        );
        assert_eq!(
            payload["meta"]["ga4"]["propertyQuota"]["concurrentRequests"]["remaining"],
            2
        );
        assert!(!payload.to_string().contains("private-funnel-token"));
    }

    #[tokio::test]
    async fn funnel_invalid_arguments_and_google_permission_denials_are_redacted() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        for invalid in [
            json!({"funnel_steps":[{"event":"page_view","filter_expression":{}}]}),
            json!({"funnel_steps":[{"event":"page_view"}],"limit":0}),
            json!({"funnel_steps":[{"event":"page_view"}],"funnel_visualization_type":"INVALID"}),
        ] {
            let mut request = invalid;
            request["property_id"] = json!(42);
            let args: FunnelArgs = serde_json::from_value(request).unwrap();
            let reply = Synmetra::new(GoogleClient::new(None), false)
                .run_funnel_report(Parameters(args))
                .await;
            assert_eq!(reply.is_error, Some(true));
            assert_eq!(
                reply.structured_content.unwrap()["error"]["code"],
                "INVALID_REQUEST"
            );
        }

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let responder = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buffer = [0u8; 4096];
            let n = stream.read(&mut buffer).await.unwrap();
            assert!(String::from_utf8_lossy(&buffer[..n]).contains("properties/42:runFunnelReport"));
            let body = r#"{"error":{"message":"private-funnel-denial"}}"#;
            let response = format!(
                "HTTP/1.1 403 Forbidden\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        });
        let server = Synmetra::new(
            GoogleClient::with_endpoints(
                Some("private-funnel-bearer".into()),
                &endpoint,
                &endpoint,
            ),
            false,
        );
        let args: FunnelArgs = serde_json::from_value(json!({
            "property_id":42,
            "funnel_steps":[{"event":"page_view"}],
            "max_rows":1
        }))
        .unwrap();
        let reply = server.run_funnel_report(Parameters(args)).await;
        responder.await.unwrap();
        assert_eq!(reply.is_error, Some(true));
        let payload = reply.structured_content.unwrap();
        assert_eq!(payload["error"]["code"], "GOOGLE_HTTP_ERROR");
        assert_eq!(payload["error"]["status_code"], 403);
        assert!(!payload.to_string().contains("private-funnel-denial"));
        assert!(!payload.to_string().contains("private-funnel-bearer"));
    }

    #[test]
    fn funnel_summary_only_keeps_both_total_counts_without_rows() {
        let subreport = json!({
            "dimensionHeaders":[{"name":"funnelStepName"}],
            "metricHeaders":[{"name":"activeUsers"}],
            "rows":[{"dimensionValues":[{"value":"1. Viewed"}],
                     "metricValues":[{"value":"9"}]}],
            "rowCount":4
        });
        let controls = tabular::Controls {
            output_mode: Some(tabular::OutputMode::Tuples),
            summary_only: true,
            max_rows: Some(1),
            max_cell_chars: None,
            cursor_supplied: false,
        };
        let reply = result_with_funnel_projection(
            Ok(json!({"funnelTable":subreport,"funnelVisualization":subreport})),
            controls,
        );
        assert_eq!(reply.is_error, Some(false));
        let payload = reply.structured_content.unwrap();
        assert!(payload["data"]["funnel_table"].is_null());
        assert!(payload["data"]["funnel_visualization"].is_null());
        assert_eq!(payload["meta"]["funnel_table"]["row_count_total"], 4);
        assert_eq!(
            payload["meta"]["funnel_visualization"]["row_count_total"],
            4
        );
        assert!(payload["meta"]["funnel_table"]["next_cursor"].is_null());
    }

    #[tokio::test]
    async fn pivot_report_provider_denial_excludes_http_body_and_bearer() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let responder = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buffer = [0u8; 4096];
            let n = stream.read(&mut buffer).await.unwrap();
            assert!(String::from_utf8_lossy(&buffer[..n]).contains("properties/42:runPivotReport"));
            let body = r#"{"error":{"message":"private-pivot-body"}}"#;
            let response = format!(
                "HTTP/1.1 403 Forbidden\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        });
        let server = Synmetra::new(
            GoogleClient::with_endpoints(Some("private-pivot-bearer".into()), &endpoint, &endpoint),
            false,
        );
        let args: PivotArgs = serde_json::from_value(json!({
            "property_id":42,
            "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
            "dimensions":["country"],
            "metrics":["activeUsers"],
            "pivots":[{"fieldNames":["country"],"limit":"2"}]
        }))
        .unwrap();
        let result = server.run_pivot_report(Parameters(args)).await;
        responder.await.unwrap();
        assert_eq!(result.is_error, Some(true));
        let content = result.structured_content.unwrap();
        assert_eq!(content["error"]["code"], "GOOGLE_HTTP_ERROR");
        assert_eq!(content["error"]["status_code"], 403);
        assert!(!content.to_string().contains("private-pivot-body"));
        assert!(!content.to_string().contains("private-pivot-bearer"));
    }

    #[test]
    fn admin_access_mcp_inputs_cover_pinned_fields_and_required_sets() {
        use std::collections::BTreeSet;

        let pinned: Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let router = Synmetra::tool_router();
        for name in ["run_property_access_report", "run_account_access_report"] {
            let upstream = pinned["tools"]
                .as_array()
                .unwrap()
                .iter()
                .find(|tool| tool["name"] == name)
                .unwrap();
            let local = router
                .list_all()
                .into_iter()
                .find(|tool| tool.name == name)
                .unwrap();
            for field in ["properties", "required"] {
                let collect = |value: &Value| -> BTreeSet<String> {
                    if field == "required" {
                        value["required"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .filter_map(Value::as_str)
                            .map(ToOwned::to_owned)
                            .collect()
                    } else {
                        value["properties"]
                            .as_object()
                            .unwrap()
                            .keys()
                            .cloned()
                            .collect()
                    }
                };
                let native = serde_json::to_value(&local.input_schema).unwrap();
                assert_eq!(
                    collect(&native),
                    collect(&upstream["inputSchema"]),
                    "{name} {field}"
                );
            }
            assert_eq!(
                local.input_schema["properties"]["limit"]["format"],
                "uint64"
            );
        }
    }

    #[tokio::test]
    async fn admin_access_reports_project_named_fields_and_redact_google_denials() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        for account in [false, true] {
            for denied in [false, true] {
                let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                let endpoint = format!("http://{}", listener.local_addr().unwrap());
                let responder = tokio::spawn(async move {
                    let (mut stream, _) = listener.accept().await.unwrap();
                    let mut buffer = [0u8; 4096];
                    let read = stream.read(&mut buffer).await.unwrap();
                    let request = String::from_utf8_lossy(&buffer[..read]);
                    let resource = if account { "accounts" } else { "properties" };
                    assert!(request.contains(&format!("{resource}/42:runAccessReport")));
                    assert!(request.contains(r#""dimensionName":"userEmail""#));
                    assert!(request.contains(r#""metricName":"accessCount""#));
                    assert!(request.contains(r#""limit":"1""#));
                    let status = if denied { "403 Forbidden" } else { "200 OK" };
                    let body = if denied {
                        json!({"error":{"message":"private-access-body"}})
                    } else {
                        json!({
                            "dimensionHeaders":[{"dimensionName":"userEmail"}],
                            "metricHeaders":[{"metricName":"accessCount"}],
                            "rows":[{"dimensionValues":[{"value":"test@example.invalid"}],
                                     "metricValues":[{"value":"3"}]}],
                            "rowCount":2
                        })
                    }
                    .to_string();
                    let response = format!(
                        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        body.len(),
                        body
                    );
                    stream.write_all(response.as_bytes()).await.unwrap();
                });
                let server = Synmetra::new(
                    GoogleClient::with_endpoints(
                        Some("private-access-token".into()),
                        &endpoint,
                        &endpoint,
                    ),
                    false,
                );
                let mut request = json!({
                    "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
                    "dimensions":["userEmail"],
                    "metrics":["accessCount"],
                    "max_rows":1,
                    "output_mode":"rows"
                });
                request[if account { "account_id" } else { "property_id" }] = json!(42);
                let reply = if account {
                    server
                        .run_account_access_report(Parameters(
                            serde_json::from_value(request).unwrap(),
                        ))
                        .await
                } else {
                    server
                        .run_property_access_report(Parameters(
                            serde_json::from_value(request).unwrap(),
                        ))
                        .await
                };
                responder.await.unwrap();
                assert_eq!(reply.is_error, Some(denied));
                let payload = reply.structured_content.unwrap();
                assert!(!payload.to_string().contains("private-access-token"));
                assert!(!payload.to_string().contains("private-access-body"));
                if denied {
                    assert_eq!(payload["error"]["code"], "GOOGLE_HTTP_ERROR");
                    assert_eq!(payload["error"]["status_code"], 403);
                } else {
                    assert_eq!(payload["data"][0]["userEmail"], "test@example.invalid");
                    assert_eq!(payload["data"][0]["accessCount"], "3");
                    assert_eq!(payload["meta"]["row_count_total"], 2);
                    assert!(payload["meta"]["next_cursor"].as_str().is_some());
                }
            }
        }
    }

    #[tokio::test]
    async fn pivot_report_projection_keeps_pivot_header_metadata_without_fake_cursor() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let responder = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buffer = [0u8; 4096];
            let n = stream.read(&mut buffer).await.unwrap();
            let request = String::from_utf8_lossy(&buffer[..n]);
            assert!(request.contains("properties/42:runPivotReport"));
            assert!(request.contains(r#""pivots":[{"fieldNames":["country"],"limit":"2"}]"#));
            let body = json!({
                "dimensionHeaders":[{"name":"country"}],
                "metricHeaders":[{"name":"activeUsers"}],
                "pivotHeaders":[{"pivotDimensionHeaders":[],"rowCount":2}],
                "rows":[{"dimensionValues":[{"value":"US"}],"metricValues":[{"value":"3"}]}],
                "rowCount":2
            })
            .to_string();
            let reply = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(reply.as_bytes()).await.unwrap();
        });
        let server = Synmetra::new(
            GoogleClient::with_endpoints(Some("pivot-fixture-token".into()), &endpoint, &endpoint),
            false,
        );
        let base = json!({
            "property_id":42,
            "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
            "dimensions":["country"],
            "metrics":["activeUsers"],
            "pivots":[{"fieldNames":["country"],"limit":"2"}],
            "max_rows":1,
            "output_mode":"rows"
        });
        let args: PivotArgs = serde_json::from_value(base.clone()).unwrap();
        let response = server.run_pivot_report(Parameters(args)).await;
        responder.await.unwrap();
        assert_eq!(response.is_error, Some(false));
        let payload = response.structured_content.unwrap();
        assert_eq!(payload["data"][0]["country"], "US");
        assert_eq!(payload["meta"]["ga4"]["pivotHeaders"][0]["rowCount"], 2);
        assert_eq!(payload["meta"]["row_count_total"], 2);
        assert!(payload["meta"]["next_cursor"].is_null());
        assert!(!payload.to_string().contains("pivot-fixture-token"));

        let mut invalid = base;
        invalid["cursor"] = json!("v1:wrong:1");
        let rejected = Synmetra::new(GoogleClient::new(None), false)
            .run_pivot_report(Parameters(serde_json::from_value(invalid).unwrap()))
            .await;
        assert_eq!(rejected.is_error, Some(true));
        assert_eq!(
            rejected.structured_content.unwrap()["error"]["code"],
            "INVALID_REQUEST"
        );
    }

    #[tokio::test]
    async fn conversion_report_denial_and_invalid_spec_fail_without_sensitive_output() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let fields = json!({
            "property_id":42,
            "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
            "dimensions":["country"],
            "metrics":["allConversionsByInteractionDate"],
            "conversion_spec":{"attribution_model":"LAST_CLICK"}
        });
        let mut bad = fields.clone();
        bad["conversion_spec"] = json!({"conversion_actions":["invalid-name"]});
        let offline = Synmetra::new(GoogleClient::new(None), false)
            .run_conversions_report(Parameters(serde_json::from_value(bad).unwrap()))
            .await;
        assert_eq!(offline.is_error, Some(true));
        assert_eq!(
            offline.structured_content.unwrap()["error"]["code"],
            "INVALID_REQUEST"
        );

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let responder = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buffer = [0u8; 4096];
            let n = stream.read(&mut buffer).await.unwrap();
            assert!(String::from_utf8_lossy(&buffer[..n]).contains("properties/42:runReport"));
            let body = r#"{"error":{"message":"sensitive-fixture-detail"}}"#;
            let response = format!(
                "HTTP/1.1 403 Forbidden\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        });
        let server = Synmetra::new(
            GoogleClient::with_endpoints(
                Some("sensitive-fixture-token".into()),
                &endpoint,
                &endpoint,
            ),
            false,
        );
        let result = server
            .run_conversions_report(Parameters(serde_json::from_value(fields).unwrap()))
            .await;
        responder.await.unwrap();
        assert_eq!(result.is_error, Some(true));
        let payload = result.structured_content.unwrap();
        assert_eq!(payload["error"]["code"], "GOOGLE_HTTP_ERROR");
        assert_eq!(payload["error"]["status_code"], 403);
        assert!(!payload.to_string().contains("sensitive-fixture-token"));
        assert!(!payload.to_string().contains("sensitive-fixture-detail"));
    }

    #[tokio::test]
    async fn conversion_eligibility_and_permission_errors_remain_typed_and_private() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        for (status, code) in [("400 Bad Request", 400), ("403 Forbidden", 403)] {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let endpoint = format!("http://{}", listener.local_addr().unwrap());
            let responder = tokio::spawn(async move {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut buffer = [0_u8; 4096];
                let count = socket.read(&mut buffer).await.unwrap();
                let request = String::from_utf8_lossy(&buffer[..count]);
                assert!(request.starts_with("POST /v1alpha/properties/42:runReport "));
                assert!(request.contains(r#""conversionSpec":{"attributionModel":"LAST_CLICK""#));
                let body = r#"{"error":{"message":"private-ineligible-property-fixture"}}"#;
                let response = format!(
                    "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                socket.write_all(response.as_bytes()).await.unwrap();
            });
            let server = Synmetra::new(
                GoogleClient::with_endpoints(
                    Some("private-conversion-token".into()),
                    &endpoint,
                    &endpoint,
                ),
                false,
            );
            let args: ConversionArgs = serde_json::from_value(json!({
                "property_id":42,
                "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
                "dimensions":["country"],
                "metrics":["allConversionsByInteractionDate"],
                "conversion_spec":{"attribution_model":"LAST_CLICK"}
            }))
            .unwrap();
            let result = server.run_conversions_report(Parameters(args)).await;
            responder.await.unwrap();
            assert_eq!(result.is_error, Some(true));
            let payload = result.structured_content.unwrap();
            assert_eq!(payload["error"]["code"], "GOOGLE_HTTP_ERROR");
            assert_eq!(payload["error"]["status_code"], code);
            assert_eq!(payload["error"]["category"], "upstream");
            assert!(!payload.to_string().contains("private-conversion-token"));
            assert!(!payload
                .to_string()
                .contains("private-ineligible-property-fixture"));
        }
    }

    #[tokio::test]
    async fn historical_and_realtime_report_denials_are_redacted_and_stable() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        for realtime in [false, true] {
            for (http_status, code) in [("403 Forbidden", 403), ("429 Too Many Requests", 429)] {
                let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                let endpoint = format!("http://{}", listener.local_addr().unwrap());
                let responder = tokio::spawn(async move {
                    let (mut stream, _) = listener.accept().await.unwrap();
                    let mut buffer = [0_u8; 4096];
                    let read = stream.read(&mut buffer).await.unwrap();
                    let request = String::from_utf8_lossy(&buffer[..read]);
                    if realtime {
                        assert!(request.contains("properties/42:runRealtimeReport"));
                    } else {
                        assert!(request.contains("properties/42:runReport"));
                    }
                    let body = r#"{"error":{"code":403,"message":"private upstream secret"}}"#;
                    let response = format!(
                        "HTTP/1.1 {http_status}\r\nContent-Type: application/json\r\nRetry-After: 99\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        body.len(),
                        body
                    );
                    stream.write_all(response.as_bytes()).await.unwrap();
                });
                let server = Synmetra::new(
                    GoogleClient::with_endpoints(
                        Some("private-report-access-token".into()),
                        &endpoint,
                        &endpoint,
                    ),
                    false,
                );
                let reply = if realtime {
                    let args: RealtimeArgs = serde_json::from_value(json!({
                        "property_id":42,
                        "dimensions":["country"],
                        "metrics":["activeUsers"]
                    }))
                    .unwrap();
                    server.run_realtime_report(Parameters(args)).await
                } else {
                    let args: ReportArgs = serde_json::from_value(json!({
                        "property_id":42,
                        "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
                        "dimensions":["country"],
                        "metrics":["activeUsers"]
                    }))
                    .unwrap();
                    server.run_report(Parameters(args)).await
                };
                responder.await.unwrap();
                assert_eq!(reply.is_error, Some(true));
                let payload = reply.structured_content.unwrap();
                assert_eq!(payload["error"]["code"], "GOOGLE_HTTP_ERROR");
                assert_eq!(payload["error"]["category"], "upstream");
                assert_eq!(payload["error"]["status_code"], code);
                assert!(!payload.to_string().contains("private-report-access-token"));
                assert!(!payload.to_string().contains("private upstream secret"));
            }
        }
    }

    #[tokio::test]
    async fn historical_cursor_pages_remain_query_bound_and_never_repeat_rows() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let responder = tokio::spawn(async move {
            for (page, country) in ["Mexico", "Canada"].into_iter().enumerate() {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut buffer = [0_u8; 4096];
                let received = stream.read(&mut buffer).await.unwrap();
                let request = String::from_utf8_lossy(&buffer[..received]);
                assert!(request.contains("properties/42:runReport"));
                assert!(request.contains(r#""limit":"1""#));
                if page == 1 {
                    assert!(request.contains(r#""offset":"1""#));
                } else {
                    assert!(!request.contains(r#""offset":"1""#));
                }
                let body = json!({
                    "dimensionHeaders":[{"name":"country"}],
                    "metricHeaders":[{"name":"activeUsers"}],
                    "rows":[{
                        "dimensionValues":[{"value":country}],
                        "metricValues":[{"value":"12"}]
                    }],
                    "rowCount":2
                })
                .to_string();
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(response.as_bytes()).await.unwrap();
            }
        });
        let server = Synmetra::new(
            GoogleClient::with_endpoints(Some("private-test-token".into()), &endpoint, &endpoint),
            false,
        );
        let mut fields = json!({
            "property_id":"42",
            "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
            "dimensions":["country"],
            "metrics":["activeUsers"],
            "output_mode":"rows",
            "max_rows":1
        });
        let first = server
            .run_report(Parameters(serde_json::from_value(fields.clone()).unwrap()))
            .await;
        assert_eq!(first.is_error, Some(false));
        let first = first.structured_content.unwrap();
        assert_eq!(first["data"][0]["country"], "Mexico");
        let cursor = first["meta"]["next_cursor"].as_str().unwrap().to_owned();
        fields["cursor"] = json!(cursor);
        let second = server
            .run_report(Parameters(serde_json::from_value(fields.clone()).unwrap()))
            .await;
        responder.await.unwrap();
        assert_eq!(second.is_error, Some(false));
        let second = second.structured_content.unwrap();
        assert_eq!(second["data"][0]["country"], "Canada");
        assert!(second["meta"]["next_cursor"].is_null());
        assert_eq!(second["meta"]["row_count_total"], 2);
        assert!(!second.to_string().contains("private-test-token"));

        fields["metrics"] = json!(["screenPageViews"]);
        let rejected = server
            .run_report(Parameters(serde_json::from_value(fields).unwrap()))
            .await;
        assert_eq!(rejected.is_error, Some(true));
        assert_eq!(
            rejected.structured_content.unwrap()["error"]["code"],
            "INVALID_REQUEST"
        );
    }

    #[tokio::test]
    async fn historical_report_opt_in_projection_preserves_metadata_and_previews_offline() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let fields = json!({
            "property_id":"42",
            "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
            "dimensions":["country"],
            "metrics":["activeUsers"],
            "output_mode":"rows",
            "max_rows":1,
            "max_cell_chars":3
        });
        let preview = Synmetra::new(GoogleClient::new(None), false)
            .preview_report_request(Parameters(serde_json::from_value(fields.clone()).unwrap()))
            .await;
        assert_eq!(preview.is_error, Some(false));
        let preview_payload = preview.structured_content.unwrap();
        assert_eq!(preview_payload["data"]["provider_called"], false);
        assert_eq!(preview_payload["data"]["request"]["limit"], "1");

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let api = GoogleClient::with_endpoints(
            Some("private-tabular-fixture-token".into()),
            &endpoint,
            &endpoint,
        );
        let responder = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buffer = [0u8; 4096];
            let received = stream.read(&mut buffer).await.unwrap();
            let request = String::from_utf8_lossy(&buffer[..received]);
            assert!(request.contains("properties/42:runReport"));
            assert!(request.contains(r#""limit":"1""#));
            let body = json!({
                "dimensionHeaders":[{"name":"country"}],
                "metricHeaders":[{"name":"activeUsers","type":"TYPE_INTEGER"}],
                "rows":[{"dimensionValues":[{"value":"México"}],"metricValues":[{"value":"17"}]}],
                "rowCount":3,
                "propertyQuota":{"concurrentRequests":{"remaining":9}}
            })
            .to_string();
            let reply = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(reply.as_bytes()).await.unwrap();
        });
        let server = Synmetra::new(api, false);
        let report = server
            .run_report(Parameters(serde_json::from_value(fields).unwrap()))
            .await;
        responder.await.unwrap();
        assert_eq!(report.is_error, Some(false));
        let payload = report.structured_content.unwrap();
        assert_eq!(payload["data"][0]["country"], "Méx");
        assert_eq!(payload["meta"]["row_count_total"], 3);
        assert_eq!(payload["meta"]["row_count_returned"], 1);
        assert_eq!(payload["meta"]["truncated"], true);
        let cursor = payload["meta"]["next_cursor"].as_str().unwrap();
        assert!(cursor.starts_with("v1:"));
        assert!(cursor.ends_with(":1"));
        assert_eq!(
            payload["meta"]["ga4"]["propertyQuota"]["concurrentRequests"]["remaining"],
            9
        );
        assert!(!payload
            .to_string()
            .contains("private-tabular-fixture-token"));

        let unsupported_cursor = Synmetra::new(GoogleClient::new(None), false)
            .preview_report_request(Parameters(
                serde_json::from_value(json!({
                    "property_id":"42",
                    "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
                    "dimensions":["country"],
                    "metrics":["activeUsers"],
                    "cursor":"v1:pending:5"
                }))
                .unwrap(),
            ))
            .await;
        assert_eq!(unsupported_cursor.is_error, Some(true));
        assert_eq!(
            unsupported_cursor.structured_content.unwrap()["error"]["code"],
            "INVALID_REQUEST"
        );
    }

    #[tokio::test]
    async fn batch_reports_normalize_separate_requests_and_preserve_distinct_results() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let api = GoogleClient::with_endpoints(
            Some("private-batch-fixture-token".into()),
            &endpoint,
            &endpoint,
        );
        let responder = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buffer = vec![0u8; 8192];
            let read = stream.read(&mut buffer).await.unwrap();
            let request = String::from_utf8_lossy(&buffer[..read]);
            assert!(request.starts_with("POST /v1beta/properties/42:batchRunReports "));
            assert!(
                request.contains(r#""dateRanges":[{"endDate":"today","startDate":"yesterday"}]"#)
            );
            assert!(request.contains(r#""returnPropertyQuota":true"#));
            assert!(request.contains(r#""name":"activeUsers""#));
            assert!(request.contains(r#""name":"screenPageViews""#));
            assert!(!request.contains("date_ranges"));
            let body = json!({
                "reports": [
                    {"rowCount":1,"rows":[{"metricValues":[{"value":"17"}]}]},
                    {"rowCount":1,"rows":[{"metricValues":[{"value":"23"}]}]}
                ]
            })
            .to_string();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        });
        let args: BatchArgs = serde_json::from_value(json!({
            "property_id":"42",
            "requests":[
                {
                    "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
                    "dimensions":["country"],
                    "metrics":["activeUsers"]
                },
                {
                    "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
                    "dimensions":["country"],
                    "metrics":["screenPageViews"],
                    "return_property_quota":false
                }
            ]
        }))
        .unwrap();
        let server = Synmetra::new(api, false);
        let response = server.batch_run_reports(Parameters(args)).await;
        responder.await.unwrap();
        assert_eq!(response.is_error, Some(false));
        let body = response.structured_content.unwrap();
        assert_eq!(
            body["data"]["reports"][0]["rows"][0]["metricValues"][0]["value"],
            "17"
        );
        assert_eq!(
            body["data"]["reports"][1]["rows"][0]["metricValues"][0]["value"],
            "23"
        );
        assert!(!body.to_string().contains("private-batch-fixture-token"));
    }

    #[test]
    fn batch_input_contract_requires_only_property_and_report_requests() {
        use std::collections::BTreeSet;

        let pinned: Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let upstream = pinned["tools"]
            .as_array()
            .unwrap()
            .iter()
            .find(|tool| tool["name"] == "batch_run_reports")
            .unwrap();
        let native = Synmetra::tool_router()
            .list_all()
            .into_iter()
            .find(|tool| tool.name == "batch_run_reports")
            .unwrap();
        let expected: BTreeSet<_> = upstream["inputSchema"]["properties"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        let actual: BTreeSet<_> = native.input_schema["properties"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(actual, expected);
        assert_eq!(actual, BTreeSet::from(["property_id", "requests"]));
        let required: BTreeSet<_> = native.input_schema["required"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(Value::as_str)
            .collect();
        assert_eq!(required, actual);
    }

    #[tokio::test]
    async fn batch_report_permission_and_quota_denials_do_not_leak_provider_details() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        for (status, expected) in [("403 Forbidden", 403), ("429 Too Many Requests", 429)] {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let endpoint = format!("http://{}", listener.local_addr().unwrap());
            let responder = tokio::spawn(async move {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut buffer = [0u8; 4096];
                let n = stream.read(&mut buffer).await.unwrap();
                assert!(
                    String::from_utf8_lossy(&buffer[..n]).contains("properties/42:batchRunReports")
                );
                let body = r#"{"error":{"message":"provider-private-error"}}"#;
                let reply = format!(
                    "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nRetry-After: 99\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(reply.as_bytes()).await.unwrap();
            });
            let server = Synmetra::new(
                GoogleClient::with_endpoints(
                    Some("private-batch-token".into()),
                    &endpoint,
                    &endpoint,
                ),
                false,
            );
            let args: BatchArgs = serde_json::from_value(json!({
                "property_id":42,
                "requests":[{
                    "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
                    "dimensions":[],
                    "metrics":["activeUsers"]
                }]
            }))
            .unwrap();
            let response = server.batch_run_reports(Parameters(args)).await;
            responder.await.unwrap();
            assert_eq!(response.is_error, Some(true));
            let payload = response.structured_content.unwrap();
            assert_eq!(payload["error"]["code"], "GOOGLE_HTTP_ERROR");
            assert_eq!(payload["error"]["status_code"], expected);
            assert!(!payload.to_string().contains("private-batch-token"));
            assert!(!payload.to_string().contains("provider-private-error"));
        }
    }

    #[tokio::test]
    async fn batch_report_five_item_boundary_is_enforced_before_google_credentials() {
        let server = Synmetra::new(GoogleClient::new(None), false);
        let item = json!({
            "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
            "dimensions":[],
            "metrics":["activeUsers"]
        });
        for (count, expected_code) in [
            (0, "INVALID_REQUEST"),
            (5, "AUTH_NOT_CONFIGURED"),
            (6, "INVALID_REQUEST"),
        ] {
            let args: BatchArgs = serde_json::from_value(json!({
                "property_id":"42",
                "requests":vec![item.clone();count]
            }))
            .unwrap();
            let result = server.batch_run_reports(Parameters(args)).await;
            assert_eq!(result.is_error, Some(true));
            assert_eq!(
                result.structured_content.unwrap()["error"]["code"],
                expected_code
            );
        }
    }

    #[test]
    fn batch_arguments_reject_unimplemented_output_controls_and_bad_items() {
        for invalid in [
            json!({
                "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
                "dimensions":[],
                "metrics":["activeUsers"],
                "cursor":"unimplemented"
            }),
            json!({
                "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
                "dimensions":[],
                "metrics":["activeUsers"],
                "limit":250001
            }),
            json!({
                "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
                "dimensions":[],
                "metrics":[]
            }),
        ] {
            let item = serde_json::from_value::<BatchReportItemArgs>(invalid);
            if let Ok(parsed) = item {
                assert_eq!(parsed.to_google_request(), Err(ApiError::InvalidRequest));
            }
        }
    }

    #[tokio::test]
    async fn admin_handlers_preserve_golden_google_json_and_reject_403_without_leakage() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        for name in [
            "get_account_summaries",
            "get_property_details",
            "get_account_data_sharing_settings",
            "get_property_data_retention_settings",
            "list_google_ads_links",
            "list_property_annotations",
            "get_custom_dimensions_and_metrics",
        ] {
            let routes: Vec<(&str, Value)> = match name {
                "get_account_summaries" => vec![(
                    "/v1beta/accountSummaries",
                    json!({"accountSummaries":[{"name":"accounts/42"}]}),
                )],
                "get_property_details" => vec![(
                    "/v1beta/properties/42",
                    json!({"name":"properties/42","displayName":"Fixture property"}),
                )],
                "get_account_data_sharing_settings" => vec![(
                    "/v1beta/accounts/42/dataSharingSettings",
                    json!({"name":"accounts/42/dataSharingSettings","sharingWithGoogleSupportEnabled":false}),
                )],
                "get_property_data_retention_settings" => vec![(
                    "/v1beta/properties/42/dataRetentionSettings",
                    json!({"name":"properties/42/dataRetentionSettings","eventDataRetention":"FOURTEEN_MONTHS"}),
                )],
                "list_google_ads_links" => vec![(
                    "/v1beta/properties/42/googleAdsLinks",
                    json!({"googleAdsLinks":[{"name":"properties/42/googleAdsLinks/1"}]}),
                )],
                "list_property_annotations" => vec![(
                    "/v1alpha/properties/42/reportingDataAnnotations",
                    json!({"reportingDataAnnotations":[{"name":"properties/42/reportingDataAnnotations/1"}]}),
                )],
                "get_custom_dimensions_and_metrics" => vec![
                    (
                        "/v1beta/properties/42/customDimensions",
                        json!({"customDimensions":[{"name":"properties/42/customDimensions/1"}]}),
                    ),
                    (
                        "/v1beta/properties/42/customMetrics",
                        json!({"customMetrics":[{"name":"properties/42/customMetrics/1"}]}),
                    ),
                ],
                _ => unreachable!(),
            };
            for denied in [false, true] {
                let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                let endpoint = format!("http://{}", listener.local_addr().unwrap());
                let api = GoogleClient::with_endpoints(
                    Some("private-admin-fixture-token".into()),
                    &endpoint,
                    &endpoint,
                );
                let active = if denied {
                    routes.iter().take(1).cloned().collect::<Vec<_>>()
                } else {
                    routes.clone()
                };
                let responder = tokio::spawn(async move {
                    for (path, expected) in active {
                        let (mut stream, _) = listener.accept().await.unwrap();
                        let mut buffer = [0_u8; 4096];
                        let read = stream.read(&mut buffer).await.unwrap();
                        let request = String::from_utf8_lossy(&buffer[..read]);
                        assert!(request.starts_with("GET "), "{request}");
                        assert!(request.contains(path), "{request}");
                        let status = if denied { "403 Forbidden" } else { "200 OK" };
                        let body = if denied {
                            json!({"error":{"code":403,"message":"Private fixture denial"}})
                        } else {
                            expected
                        };
                        let text = body.to_string();
                        let response = format!(
                            "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            text.len(),
                            text
                        );
                        stream.write_all(response.as_bytes()).await.unwrap();
                    }
                });
                let server = Synmetra::new(api, false);
                let actual = match name {
                    "get_account_summaries" => {
                        server
                            .get_account_summaries(Parameters(CollectionArgs {
                                page_size: None,
                                max_pages: None,
                            }))
                            .await
                    }
                    "get_property_details" => {
                        server
                            .get_property_details(Parameters(PropertyArgs {
                                property_id: "42".into(),
                            }))
                            .await
                    }
                    "get_account_data_sharing_settings" => {
                        server
                            .get_account_data_sharing_settings(Parameters(AccountArgs {
                                account_id: "42".into(),
                            }))
                            .await
                    }
                    "get_property_data_retention_settings" => {
                        server
                            .get_property_data_retention_settings(Parameters(PropertyArgs {
                                property_id: "42".into(),
                            }))
                            .await
                    }
                    "list_google_ads_links" => {
                        server
                            .list_google_ads_links(Parameters(PropertyCollectionArgs {
                                property_id: "42".into(),
                                page_size: None,
                                max_pages: None,
                            }))
                            .await
                    }
                    "list_property_annotations" => {
                        server
                            .list_property_annotations(Parameters(PropertyCollectionArgs {
                                property_id: "42".into(),
                                page_size: None,
                                max_pages: None,
                            }))
                            .await
                    }
                    "get_custom_dimensions_and_metrics" => {
                        server
                            .get_custom_dimensions_and_metrics(Parameters(PropertyArgs {
                                property_id: "42".into(),
                            }))
                            .await
                    }
                    _ => unreachable!(),
                };
                responder.await.unwrap();
                assert_eq!(actual.is_error, Some(denied), "{name} denied={denied}");
                let payload = actual.structured_content.unwrap();
                assert!(!payload.to_string().contains("private-admin-fixture-token"));
                if denied {
                    assert_eq!(payload["error"]["code"], "GOOGLE_HTTP_ERROR", "{name}");
                    assert_eq!(payload["error"]["status_code"], 403, "{name}");
                    assert_eq!(payload["error"]["category"], "upstream", "{name}");
                } else if name == "get_custom_dimensions_and_metrics" {
                    assert_eq!(
                        payload["data"]["custom_dimensions"],
                        json!([{"name":"properties/42/customDimensions/1"}])
                    );
                    assert_eq!(
                        payload["data"]["custom_metrics"],
                        json!([{"name":"properties/42/customMetrics/1"}])
                    );
                } else {
                    assert_eq!(payload["data"], routes[0].1, "{name}");
                }
            }
        }
    }

    #[tokio::test]
    async fn realtime_mcp_retains_provider_quota_without_fabricating_a_next_cursor() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let api = GoogleClient::with_endpoints(
            Some("private-realtime-fixture-token".into()),
            &endpoint,
            &endpoint,
        );
        let responder = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buffer = [0_u8; 4096];
            let count = stream.read(&mut buffer).await.unwrap();
            let request = String::from_utf8_lossy(&buffer[..count]);
            assert!(request.contains("runRealtimeReport"));
            let body = r#"{"rowCount":154,"rows":[{"dimensionValues":[{"value":"US"}],"metricValues":[{"value":"42"}]}],"propertyQuota":{"concurrentRequests":{"consumed":1,"remaining":9}}}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        });
        let server = Synmetra::new(api, false);
        let args: RealtimeArgs = serde_json::from_value(json!({
            "property_id": "42",
            "dimensions": ["country"],
            "metrics": ["activeUsers"],
            "limit": 1,
            "return_property_quota": true
        }))
        .unwrap();
        let response = server.run_realtime_report(Parameters(args)).await;
        responder.await.unwrap();
        assert_eq!(response.is_error, Some(false));
        let structured = response.structured_content.unwrap();
        assert_eq!(structured["data"]["rowCount"], 154);
        assert_eq!(structured["data"]["rows"].as_array().unwrap().len(), 1);
        assert_eq!(
            structured["data"]["propertyQuota"]["concurrentRequests"]["remaining"],
            9
        );
        assert!(structured["meta"].get("next_cursor").is_none());
        assert!(structured["data"].get("next_cursor").is_none());
        assert!(!structured
            .to_string()
            .contains("private-realtime-fixture-token"));
    }

    #[test]
    fn realtime_nonzero_offset_has_stable_preflight_error() {
        let args: RealtimeArgs = serde_json::from_value(json!({
            "property_id": "42",
            "dimensions": [],
            "metrics": ["activeUsers"],
            "offset": 7
        }))
        .unwrap();
        assert_eq!(
            realtime_report_request(&args),
            Err(ApiError::UnsupportedRealtimeOffset)
        );
        assert_eq!(
            api_error_code(&ApiError::UnsupportedRealtimeOffset),
            ("UNSUPPORTED_REALTIME_OFFSET", "validation")
        );
    }

    #[test]
    fn property_quota_schema_defaults_match_pinned_upstream_tools() {
        let snapshot: Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let server = Synmetra::new(GoogleClient::new(None), false);
        #[cfg(feature = "scratchpad")]
        let server = server.enable_scratchpad();
        let tools = server.router.list_all();
        for name in [
            "run_report",
            "run_realtime_report",
            "run_pivot_report",
            "run_conversions_report",
            "run_funnel_report",
            "scratchpad_ingest_report",
            "scratchpad_ingest_realtime_report",
        ] {
            #[cfg(not(feature = "scratchpad"))]
            if name.starts_with("scratchpad_") {
                continue;
            }
            let expected = snapshot["tools"]
                .as_array()
                .unwrap()
                .iter()
                .find(|entry| entry["name"] == name)
                .unwrap();
            let native = tools.iter().find(|tool| tool.name == name).unwrap();
            let pinned = &expected["inputSchema"]["properties"]["return_property_quota"];
            let declared = native
                .input_schema
                .get("properties")
                .and_then(|props| props.get("return_property_quota"))
                .unwrap_or_else(|| panic!("{name} omitted return_property_quota"));
            assert_eq!(declared["type"], pinned["type"], "{name} type drift");
            assert_eq!(
                declared["default"], pinned["default"],
                "{name} default drift"
            );
        }
    }

    #[test]
    fn pivot_and_realtime_default_to_upstream_property_quota_reporting() {
        let pivot: PivotArgs = serde_json::from_value(json!({
            "property_id": "42",
            "date_ranges": [{"startDate": "2026-10-01", "endDate": "2026-10-02"}],
            "dimensions": [],
            "metrics": ["activeUsers"],
            "pivots": [{"fieldNames": ["country"], "limit": 10}]
        }))
        .unwrap();
        assert!(pivot.return_property_quota);
        let realtime: RealtimeArgs = serde_json::from_value(json!({
            "property_id": "42",
            "dimensions": [],
            "metrics": ["activeUsers"]
        }))
        .unwrap();
        assert!(realtime.return_property_quota);
    }

    #[cfg(feature = "scratchpad")]
    #[test]
    fn scratchpad_ingestion_defaults_to_upstream_property_quota_reporting() {
        let data = json!({
            "session_id": "safe-session",
            "table_name": "events",
            "property_id": "42",
            "date_ranges": [{"startDate": "2026-10-01", "endDate": "2026-10-02"}],
            "dimensions": [],
            "metrics": ["activeUsers"]
        });
        let historic: ScratchIngestReportArgs = serde_json::from_value(data.clone()).unwrap();
        assert!(historic.return_property_quota);
        let mut realtime_request = data;
        realtime_request
            .as_object_mut()
            .unwrap()
            .remove("date_ranges");
        let realtime: ScratchIngestRealtimeArgs = serde_json::from_value(realtime_request).unwrap();
        assert!(realtime.return_property_quota);
    }

    #[cfg(feature = "scratchpad")]
    #[test]
    fn scratchpad_inputs_match_pinned_sednalabs_mcp_fields() {
        use std::collections::BTreeSet;
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let server = Synmetra::new(GoogleClient::new(None), false).enable_scratchpad();
        let advertised = server.router.list_all();
        for name in SCRATCHPAD_TOOL_NAMES.iter().take(7).copied() {
            let reference = fixture["tools"]
                .as_array()
                .unwrap()
                .iter()
                .find(|tool| tool["name"].as_str() == Some(name))
                .unwrap();
            let native = advertised.iter().find(|tool| tool.name == name).unwrap();
            let native_fields: BTreeSet<_> = native
                .input_schema
                .get("properties")
                .and_then(serde_json::Value::as_object)
                .into_iter()
                .flat_map(|props| props.keys())
                .map(String::as_str)
                .collect();
            let expected_fields: BTreeSet<_> = reference["inputSchema"]["properties"]
                .as_object()
                .into_iter()
                .flat_map(|props| props.keys())
                .map(String::as_str)
                .collect();
            assert_eq!(
                native_fields, expected_fields,
                "MCP input properties drifted for {name}"
            );
            let native_required: BTreeSet<_> = native
                .input_schema
                .get("required")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(serde_json::Value::as_str)
                .collect();
            let expected_required: BTreeSet<_> = reference["inputSchema"]["required"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(serde_json::Value::as_str)
                .collect();
            assert_eq!(
                native_required, expected_required,
                "MCP required fields drifted for {name}"
            );
        }
    }

    #[cfg(feature = "scratchpad")]
    #[test]
    fn full_capability_profile_matches_exact_pinned_36_tool_union() {
        use std::collections::BTreeSet;

        let inventory: Value =
            serde_json::from_str(include_str!("../docs/tool-inventory.json")).unwrap();
        let expected: BTreeSet<_> = inventory["union"]["tools"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| item["name"].as_str().unwrap().to_owned())
            .collect();
        assert_eq!(expected.len(), 36);

        let all_enabled = Synmetra::new(GoogleClient::new(None), true).enable_scratchpad();
        let actual: BTreeSet<_> = all_enabled
            .router
            .list_all()
            .into_iter()
            .map(|tool| tool.name.to_string())
            .collect();

        assert_eq!(
            actual, expected,
            "Full tool union drifted from pinned inventory"
        );
        assert_eq!(actual.len(), 36);
        if std::env::var_os("SYNMETRA_CAPTURE_INPUT_SCHEMAS").is_some() {
            let mut native = all_enabled.router.list_all();
            native.sort_by(|left, right| left.name.cmp(&right.name));
            for tool in native {
                let snapshot = json!({
                    "name": tool.name.to_string(),
                    "inputSchema": &*tool.input_schema,
                });
                println!("SYNMETRA_NATIVE_TOOL_SCHEMA {}", snapshot);
            }
        }
        assert!(all_enabled.router.has_route("create_property_annotation"));
        assert!(all_enabled
            .router
            .has_route("scratchpad_landing_param_shift_report"));

        // Without explicit write capability, the same process must never advertise
        // the official mutation tool even when local DuckDB analysis is enabled.
        let read_only = Synmetra::new(GoogleClient::new(None), false).enable_scratchpad();
        let tools = read_only.router.list_all();
        assert_eq!(tools.len(), 35);
        assert!(!tools
            .iter()
            .any(|tool| tool.name == "create_property_annotation"));
    }

    #[cfg(feature = "scratchpad")]
    #[tokio::test]
    async fn every_pinned_tool_name_dispatches_through_real_mcp_transport() {
        use rmcp::model::CallToolRequestParams;
        use std::collections::BTreeSet;

        fn fixture(name: &str) -> serde_json::Map<String, Value> {
            let historical = json!({
                "property_id":42,
                "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
                "dimensions":["country"],
                "metrics":["activeUsers"]
            });
            let property = json!({"property_id":42});
            let account = json!({"account_id":42});
            let scratch = json!({"session_id":"dispatch-missing-session"});
            let raw = match name {
                "ga4_get_started"
                | "ga4_auth_status"
                | "ga4_auth_login_command"
                | "get_account_summaries"
                | "scratchpad_get_runtime_limits"
                | "scratchpad_list_sessions"
                | "scratchpad_set_runtime_limits" => json!({}),
                "get_account_data_sharing_settings" => account,
                "get_property_data_retention_settings"
                | "get_property_details"
                | "get_custom_dimensions_and_metrics"
                | "list_google_ads_links"
                | "list_property_annotations" => property,
                "create_property_annotation" => json!({
                    "property_id":42,
                    "title":"Synthetic routing fixture",
                    "start_date":"2026-10-25"
                }),
                "check_report_compatibility" => json!({
                    "property_id":0,"dimensions":[],"metrics":[]
                }),
                "run_realtime_report" => json!({
                    "property_id":42,"dimensions":["country"],"metrics":["activeUsers"]
                }),
                "run_report" | "preview_report_request" => historical,
                "run_conversions_report" => {
                    let mut obj = historical;
                    obj["conversion_spec"] = json!({"attribution_model":"LAST_CLICK"});
                    obj
                }
                "run_pivot_report" => {
                    let mut obj = historical;
                    obj["pivots"] = json!([]);
                    obj
                }
                "run_funnel_report" => json!({
                    "property_id":42,"funnel_steps":[]
                }),
                "batch_run_reports" => json!({"property_id":42,"requests":[]}),
                "run_property_access_report" => historical,
                "run_account_access_report" => json!({
                    "account_id":42,
                    "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
                    "dimensions":["userEmail"],"metrics":["accessCount"]
                }),
                "scratchpad_open_session" => json!({"session_id":"dispatch-new-session"}),
                "scratchpad_close_session"
                | "scratchpad_list_tables"
                | "scratchpad_export_evidence_bundle" => scratch,
                "scratchpad_describe_table" | "scratchpad_summarize_table" => json!({
                    "session_id":"dispatch-missing-session","table_name":"events"
                }),
                "scratchpad_drop_table" => json!({
                    "session_id":"dispatch-missing-session","table_name":"events",
                    "if_exists":true
                }),
                "scratchpad_query" => json!({
                    "session_id":"dispatch-missing-session","sql":"SELECT 1"
                }),
                "scratchpad_ingest_report" => json!({
                    "session_id":"dispatch-missing-session","table_name":"events",
                    "property_id":42,
                    "date_ranges":[{"start_date":"yesterday","end_date":"today"}],
                    "dimensions":["country"],"metrics":["activeUsers"]
                }),
                "scratchpad_ingest_realtime_report" => json!({
                    "session_id":"dispatch-missing-session","table_name":"events",
                    "property_id":42,"dimensions":["country"],"metrics":["activeUsers"]
                }),
                "scratchpad_release_regression_report" => json!({
                    "session_id":"dispatch-missing-session","table_name":"events",
                    "release_date":"2026-10-25",
                    "anchor_event":"session_start","comparison_event":"purchase"
                }),
                "scratchpad_landing_param_shift_report" => json!({
                    "session_id":"dispatch-missing-session","table_name":"events",
                    "release_date":"2026-10-25"
                }),
                unknown => panic!("Pinned 36-name catalog gained an unhandled fixture: {unknown}"),
            };
            serde_json::from_value(raw).unwrap()
        }

        let (server_stream, client_stream) = tokio::io::duplex(1_048_576);
        let worker = tokio::spawn(async move {
            Synmetra::new(GoogleClient::new(None), true)
                .enable_scratchpad()
                .serve(server_stream)
                .await
                .unwrap()
                .waiting()
                .await
                .unwrap();
        });
        let client = ().serve(client_stream).await.unwrap();
        let inventory: Value =
            serde_json::from_str(include_str!("../docs/tool-inventory.json")).unwrap();
        let pinned: BTreeSet<_> = inventory["union"]["tools"]
            .as_array()
            .unwrap()
            .iter()
            .map(|entry| entry["name"].as_str().unwrap().to_owned())
            .collect();
        let advertised = client.list_all_tools().await.unwrap();
        let actual: BTreeSet<_> = advertised.iter().map(|t| t.name.to_string()).collect();
        assert_eq!(actual, pinned);
        assert_eq!(actual.len(), 36);
        for name in actual {
            let request = CallToolRequestParams::new(name.clone()).with_arguments(fixture(&name));
            let call =
                tokio::time::timeout(std::time::Duration::from_secs(8), client.call_tool(request))
                    .await
                    .unwrap_or_else(|_| panic!("MCP tool dispatch timed out: {name}"))
                    .unwrap_or_else(|error| {
                        panic!("MCP transport/argument dispatch failed for {name}: {error}")
                    });
            let structured = call
                .structured_content
                .unwrap_or_else(|| panic!("MCP tool {name} returned no structured envelope"));
            assert!(
                structured["ok"].is_boolean(),
                "Tool {name} omitted the Contract V1 ok flag: {structured}"
            );
        }
        client.cancel().await.unwrap();
        worker.await.unwrap();
    }

    #[cfg(feature = "scratchpad")]
    #[test]
    fn scratchpad_tools_are_hidden_without_explicit_capability() {
        let base = Synmetra::new(GoogleClient::new(None), false);
        assert_eq!(base.router.list_all().len(), 20);
        let enabled = base.enable_scratchpad();
        assert_eq!(enabled.router.list_all().len(), 35);
        assert!(enabled.router.has_route("scratchpad_drop_table"));
        assert!(!enabled.router.has_route("create_property_annotation"));
    }

    #[cfg(feature = "scratchpad")]
    #[test]
    fn landing_shift_required_fields_match_pinned_upstream_contract() {
        use std::collections::BTreeSet;
        let upstream: Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let expected_tool = upstream["tools"]
            .as_array()
            .unwrap()
            .iter()
            .find(|t| t["name"] == "scratchpad_landing_param_shift_report")
            .unwrap();
        let server = Synmetra::new(GoogleClient::new(None), false).enable_scratchpad();
        let native = server
            .router
            .list_all()
            .into_iter()
            .find(|tool| tool.name == "scratchpad_landing_param_shift_report")
            .unwrap();
        let expected: BTreeSet<_> = expected_tool["inputSchema"]["required"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect();
        let actual: BTreeSet<_> = native
            .input_schema
            .get("required")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect();
        assert_eq!(expected, actual);
        assert_eq!(
            native.annotations.as_ref().unwrap().read_only_hint,
            Some(true)
        );
    }

    #[cfg(feature = "scratchpad")]
    #[test]
    fn release_regression_has_upstream_required_inputs_and_read_only_hint() {
        use std::collections::BTreeSet;
        let upstream: Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let reference = upstream["tools"]
            .as_array()
            .unwrap()
            .iter()
            .find(|x| x["name"] == "scratchpad_release_regression_report")
            .unwrap();
        let server = Synmetra::new(GoogleClient::new(None), false).enable_scratchpad();
        let tool = server
            .router
            .list_all()
            .into_iter()
            .find(|x| x.name == "scratchpad_release_regression_report")
            .unwrap();
        let expected: BTreeSet<_> = reference["inputSchema"]["required"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect();
        let actual: BTreeSet<_> = tool
            .input_schema
            .get("required")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect();
        assert_eq!(expected, actual);
        assert_eq!(
            tool.annotations.as_ref().unwrap().read_only_hint,
            Some(true)
        );
    }

    #[cfg(feature = "scratchpad")]
    #[test]
    fn read_only_query_preserves_pinned_required_args_and_annotations() {
        use std::collections::BTreeSet;
        let reference: Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let upstream = reference["tools"]
            .as_array()
            .unwrap()
            .iter()
            .find(|tool| tool["name"] == "scratchpad_query")
            .unwrap();
        let enabled = Synmetra::new(GoogleClient::new(None), false).enable_scratchpad();
        let tool = enabled
            .router
            .list_all()
            .into_iter()
            .find(|tool| tool.name == "scratchpad_query")
            .unwrap();
        let expected: BTreeSet<_> = upstream["inputSchema"]["required"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect();
        let actual: BTreeSet<_> = tool
            .input_schema
            .get("required")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect();
        assert_eq!(actual, expected);
        assert_eq!(
            tool.annotations.as_ref().unwrap().read_only_hint,
            Some(true)
        );
    }

    #[cfg(feature = "scratchpad")]
    #[test]
    fn historical_and_realtime_ingest_have_upstream_required_mcp_fields() {
        use std::collections::BTreeSet;
        let reference: Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let enabled = Synmetra::new(GoogleClient::new(None), false).enable_scratchpad();
        for name in [
            "scratchpad_ingest_report",
            "scratchpad_ingest_realtime_report",
        ] {
            let upstream = reference["tools"]
                .as_array()
                .unwrap()
                .iter()
                .find(|tool| tool["name"] == name)
                .unwrap();
            let tool = enabled
                .router
                .list_all()
                .into_iter()
                .find(|tool| tool.name == name)
                .unwrap();
            let expected: BTreeSet<_> = upstream["inputSchema"]["required"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect();
            let actual: BTreeSet<_> = tool
                .input_schema
                .get("required")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect();
            assert_eq!(actual, expected, "{name} required-field schema drift");
        }
    }

    #[cfg(feature = "scratchpad")]
    #[tokio::test]
    async fn ingest_mcp_preflights_local_session_before_google_api() {
        let server = Synmetra::new(GoogleClient::new(None), false).enable_scratchpad();
        let result = server
            .scratchpad_ingest_realtime_report(Parameters(ScratchIngestRealtimeArgs {
                session_id: "missing".into(),
                table_name: "events".into(),
                property_id: "42".into(),
                dimensions: vec!["country".into()],
                metrics: vec!["activeUsers".into()],
                append: false,
                limit: Some(10),
                offset: None,
                dimension_filter: None,
                metric_filter: None,
                order_bys: None,
                return_property_quota: false,
            }))
            .await;
        assert_eq!(result.is_error, Some(true));
        assert!(result.content[0]
            .as_text()
            .unwrap()
            .text
            .contains("session not found"));
    }

    #[test]
    fn get_started_matches_pinned_empty_input_schema_and_native_login_guidance() {
        let pinned: Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let reference = pinned["tools"]
            .as_array()
            .unwrap()
            .iter()
            .find(|item| item["name"] == "ga4_get_started")
            .unwrap();
        let tool = Synmetra::tool_router()
            .list_all()
            .into_iter()
            .find(|item| item.name == "ga4_get_started")
            .unwrap();
        assert_eq!(
            tool.input_schema.get("properties"),
            reference["inputSchema"].get("properties")
        );
        assert!(tool
            .input_schema
            .get("required")
            .and_then(Value::as_array)
            .is_none_or(Vec::is_empty));

        let server = Synmetra::new(GoogleClient::new(None), false);
        let result = server.ga4_get_started();
        let payload = result.structured_content.unwrap();
        assert_eq!(payload["ok"], true);
        let data = &payload["data"];
        assert!(data["command"]
            .as_str()
            .unwrap()
            .starts_with("synmetra auth login --client-id-file"));
        assert_eq!(
            data["scope"],
            "https://www.googleapis.com/auth/analytics.readonly"
        );
        assert_eq!(data["python_required"], false);
        assert_eq!(data["npm_required"], false);
    }

    #[test]
    fn auth_login_command_supports_pinned_optional_fields_without_shell_injection() {
        use std::collections::BTreeSet;

        let pinned: Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let upstream = pinned["tools"]
            .as_array()
            .unwrap()
            .iter()
            .find(|item| item["name"] == "ga4_auth_login_command")
            .unwrap();
        let route = Synmetra::tool_router()
            .list_all()
            .into_iter()
            .find(|item| item.name == "ga4_auth_login_command")
            .unwrap();
        let expected: BTreeSet<_> = upstream["inputSchema"]["properties"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        let actual: BTreeSet<_> = route.input_schema["properties"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(expected, actual);
        assert!(route
            .input_schema
            .get("required")
            .and_then(Value::as_array)
            .is_none_or(Vec::is_empty));
        for field in expected {
            let pinned = &upstream["inputSchema"]["properties"][field];
            let native = &route.input_schema["properties"][field];
            for attribute in [
                "type",
                "default",
                "description",
                "format",
                "minimum",
                "maximum",
            ] {
                assert_eq!(
                    native.get(attribute),
                    pinned.get(attribute),
                    "{field} {attribute} differs from pinned input schema"
                );
            }
        }
        assert_eq!(
            shell_quote("/tmp/it's an OAuth file.json"),
            "'/tmp/it'\"'\"'s an OAuth file.json'"
        );
    }

    #[test]
    fn auth_login_command_refuses_shared_adc_and_unimplemented_quota_routing() {
        let server = Synmetra::new(GoogleClient::new(None), false);
        for options in [
            json!({"shared_adc": true}),
            json!({"quota_project": "project-fixture"}),
        ] {
            let args: AuthLoginCommandArgs = serde_json::from_value(options).unwrap();
            let reply = server.ga4_auth_login_command(Parameters(args));
            assert_eq!(reply.is_error, Some(true));
            assert_eq!(
                reply.structured_content.unwrap()["error"]["code"],
                "UNSUPPORTED_AUTH_OPTION"
            );
        }
    }

    #[test]
    fn auth_login_command_renders_supported_safe_native_flags() {
        let server = Synmetra::new(GoogleClient::new(None), false);
        let args: AuthLoginCommandArgs = serde_json::from_value(json!({
            "client_id_file": "/home/user/My OAuth's client.json",
            "account": "sample@example.com",
            "callback_port": 39900,
            "headless": true
        }))
        .unwrap();
        let result = server.ga4_auth_login_command(Parameters(args));
        assert_eq!(result.is_error, Some(false));
        let payload = result.structured_content.unwrap();
        let cmd = payload["data"]["command"].as_str().unwrap();
        assert!(cmd.contains("--client-id-file '/home/user/My OAuth'\"'\"'s client.json'"));
        assert!(cmd.contains("--account 'sample@example.com'"));
        assert!(cmd.contains("--callback-port 39900 --headless"));
    }

    #[test]
    fn auth_status_schema_advertises_optional_upstream_verification_flag() {
        let tool = Synmetra::tool_router()
            .list_all()
            .into_iter()
            .find(|tool| tool.name == "ga4_auth_status")
            .expect("registered auth status tool");
        let schema = serde_json::to_value(&tool.input_schema).unwrap();
        let pinned: Value = serde_json::from_str(include_str!(
            "../spec/upstream/sednalabs-tool-schema.v1.json"
        ))
        .unwrap();
        let reference = pinned["tools"]
            .as_array()
            .unwrap()
            .iter()
            .find(|item| item["name"] == "ga4_auth_status")
            .unwrap();
        let expected = &reference["inputSchema"]["properties"]["verify_token"];
        let actual = &schema["properties"]["verify_token"];
        assert_eq!(actual["type"], expected["type"]);
        assert_eq!(actual["default"], expected["default"]);
        assert_eq!(actual["description"], expected["description"]);
        assert!(!schema["required"]
            .as_array()
            .is_some_and(|required| required.iter().any(|v| v == "verify_token")));
    }

    #[tokio::test]
    async fn auth_status_only_contacts_google_when_explicitly_requested() {
        let server = Synmetra::new(GoogleClient::new(None), false);
        let reply = server
            .ga4_auth_status(Parameters(AuthStatusArgs {
                verify_token: false,
            }))
            .await;
        let payload = reply.structured_content.expect("structured auth status");
        assert_eq!(payload["ok"], true);
        assert!(payload["data"].get("verification").is_none());
        assert!(payload["data"].get("access_token").is_none());
    }

    #[tokio::test]
    async fn opt_in_auth_verification_excludes_google_account_data_and_tokens() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let api =
            GoogleClient::with_endpoints(Some("fixture-only-token".into()), &endpoint, &endpoint);
        let responder = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buffer = [0_u8; 4096];
            let received = stream.read(&mut buffer).await.unwrap();
            let request = String::from_utf8_lossy(&buffer[..received]);
            assert!(request.contains("accountSummaries"));
            let body = r#"{"accountSummaries":[{"name":"accounts/fixture-private"}]}"#;
            let reply = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(reply.as_bytes()).await.unwrap();
        });
        let server = Synmetra::new(api, false);
        let result = server
            .ga4_auth_status(Parameters(AuthStatusArgs { verify_token: true }))
            .await;
        responder.await.unwrap();
        let payload = result.structured_content.unwrap();
        assert_eq!(payload["ok"], true);
        assert_eq!(payload["data"]["verification"]["attempted"], true);
        assert_eq!(payload["data"]["verification"]["verified"], true);
        let public = payload.to_string();
        assert!(!public.contains("fixture-only-token"));
        assert!(!public.contains("accounts/fixture-private"));
    }

    #[test]
    fn mcp_text_envelopes_have_stable_error_categories_and_explicit_unmeasured_timing() {
        let success = result(Ok(json!({"rows":[]})));
        let output: Value =
            serde_json::from_str(&success.content[0].as_text().unwrap().text).unwrap();
        assert_eq!(output["ok"], true);
        assert!(output["data"]["rows"].is_array());
        assert_eq!(output["meta"]["elapsed_ms"], 0);
        assert_eq!(output["meta"]["elapsed_ms_measured"], false);
        assert_eq!(success.structured_content.as_ref(), Some(&output));

        let invalid = result(Err(ApiError::InvalidIdentifier));
        let failure: Value =
            serde_json::from_str(&invalid.content[0].as_text().unwrap().text).unwrap();
        assert_eq!(invalid.is_error, Some(true));
        assert_eq!(failure["error"]["code"], "INVALID_IDENTIFIER");
        assert_eq!(failure["error"]["category"], "validation");
        assert_eq!(invalid.structured_content.as_ref(), Some(&failure));
        assert!(failure["error"]["message"]
            .as_str()
            .unwrap()
            .contains("identifier"));
        assert!(failure["error"].get("status_code").is_none());

        let status = result(Err(ApiError::Http(429)));
        let failure: Value =
            serde_json::from_str(&status.content[0].as_text().unwrap().text).unwrap();
        assert_eq!(failure["error"]["status_code"], 429);
        assert_eq!(failure["error"]["code"], "GOOGLE_HTTP_ERROR");

        #[cfg(feature = "scratchpad")]
        {
            let local = local_result(Err("Scratchpad session expired".into()));
            let failure: Value =
                serde_json::from_str(&local.content[0].as_text().unwrap().text).unwrap();
            assert_eq!(failure["error"]["code"], "SCRATCHPAD_ERROR");
            assert_eq!(failure["error"]["category"], "local");
            assert_eq!(local.structured_content.as_ref(), Some(&failure));
        }
    }

    #[test]
    fn tool_errors_are_mcp_tool_errors() {
        let result = super::result(Err(ApiError::InvalidIdentifier));
        assert_eq!(result.is_error, Some(true));
        assert!(!result.content[0].as_text().unwrap().text.contains("Bearer"));
    }
}
