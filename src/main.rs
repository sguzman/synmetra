//! Native MCP stdio server. Only implemented tools are advertised.
mod auth;
mod google;
mod http;
mod tabular;
#[cfg(feature = "scratchpad")]
mod scratchpad;

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
    date_ranges: Vec<Value>,
    #[serde(default)]
    currency_code: Option<String>,
}

#[cfg(feature = "scratchpad")]
#[derive(Debug, Deserialize, JsonSchema)]
struct ScratchIngestRealtimeArgs {
    session_id: String,
    table_name: String,
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
    property_id: String,
    #[serde(default)]
    page_size: Option<u32>,
    #[serde(default)]
    max_pages: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct PropertyArgs {
    /// Positive numeric GA4 property ID or canonical properties/<id>.
    property_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct CreateAnnotationArgs {
    property_id: String,
    title: String,
    start_date: String,
    #[serde(default)]
    end_date: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default = "default_annotation_color")]
    color: String,
}

fn default_annotation_color() -> String {
    "BLUE".into()
}

#[derive(Debug, Deserialize, JsonSchema)]
struct AccountArgs {
    /// Positive GA4 account number or canonical accounts/<id>.
    account_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct FunnelArgs {
    property_id: String,
    funnel_steps: Vec<Value>,
    #[serde(default)]
    date_ranges: Option<Vec<Value>>,
    #[serde(default)]
    funnel_breakdown: Option<Value>,
    #[serde(default)]
    funnel_next_action: Option<Value>,
    #[serde(default)]
    segments: Option<Vec<Value>>,
    #[serde(default)]
    return_property_quota: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ConversionArgs {
    property_id: String,
    date_ranges: Vec<Value>,
    dimensions: Vec<String>,
    metrics: Vec<String>,
    conversion_spec: Value,
    #[serde(default)]
    dimension_filter: Option<Value>,
    #[serde(default)]
    metric_filter: Option<Value>,
    #[serde(default)]
    order_bys: Option<Vec<Value>>,
    #[serde(default)]
    limit: Option<u32>,
    #[serde(default)]
    offset: Option<u64>,
    #[serde(default)]
    currency_code: Option<String>,
    #[serde(default)]
    return_property_quota: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct PivotArgs {
    property_id: String,
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
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct BatchReportItemArgs {
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
    property_id: String,
    requests: Vec<BatchReportItemArgs>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct CompatibilityArgs {
    property_id: String,
    dimensions: Vec<String>,
    metrics: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct PropertyAccessArgs {
    property_id: String,
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
    limit: Option<u32>,
    #[serde(default)]
    offset: Option<u64>,
    #[serde(default)]
    time_zone: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct AccountAccessArgs {
    account_id: String,
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
    limit: Option<u32>,
    #[serde(default)]
    offset: Option<u64>,
    #[serde(default)]
    time_zone: Option<String>,
}

fn default_return_property_quota() -> bool {
    true
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ReportArgs {
    /// Positive numeric GA4 property ID or canonical properties/<id>.
    property_id: String,
    /// Historical date ranges, e.g. [{"startDate":"7daysAgo","endDate":"today"}].
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
    limit: Option<u32>,
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
    limit: Option<u32>,
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

fn result_with_tabular_projection(
    response: Result<Value, ApiError>,
    controls: tabular::Controls,
    realtime: bool,
) -> CallToolResult {
    match response.and_then(|provider| tabular::project(&provider, controls, realtime)) {
        Ok(projection) => {
            let payload = json!({"ok":true,"data":projection.data,"meta":projection.meta});
            let mut result =
                CallToolResult::success(vec![ContentBlock::text(payload.to_string())]);
            result.structured_content = Some(payload);
            result
        }
        Err(error) => result(Err(error)),
    }
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
        let body = google::conversion_spec(&args.conversion_spec, &args.dimensions, &args.metrics)
            .and_then(|spec| {
                if args.limit.is_some_and(|v| v == 0 || v > 250_000) {
                    return Err(ApiError::InvalidRequest);
                }
                report_request(
                    Some(args.date_ranges),
                    args.dimensions,
                    args.metrics,
                    vec![
                        ("conversionSpec", Some(spec)),
                        ("dimensionFilter", args.dimension_filter),
                        ("metricFilter", args.metric_filter),
                        ("orderBys", args.order_bys.map(|v| json!(v))),
                        ("limit", args.limit.map(|v| json!(v.to_string()))),
                        ("offset", args.offset.map(|v| json!(v.to_string()))),
                        ("currencyCode", args.currency_code.map(|v| json!(v))),
                        (
                            "returnPropertyQuota",
                            Some(json!(args.return_property_quota)),
                        ),
                    ],
                )
            });
        result(match body {
            Ok(body) => self.api.alpha_report(&args.property_id, body, false).await,
            Err(error) => Err(error),
        })
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
        let body = google::funnel_request(
            args.funnel_steps,
            args.date_ranges,
            args.funnel_breakdown,
            args.funnel_next_action,
            args.segments,
            args.return_property_quota,
        );
        result(match body {
            Ok(body) => self.api.alpha_report(&args.property_id, body, true).await,
            Err(error) => Err(error),
        })
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
        let body = if args.pivots.is_empty()
            || args.pivots.len() > 4
            || args
                .pivots
                .iter()
                .any(|p| !p.is_object() || p.as_object().is_some_and(|x| x.is_empty()))
        {
            Err(ApiError::InvalidRequest)
        } else {
            report_request(
                Some(args.date_ranges),
                args.dimensions,
                args.metrics,
                vec![
                    ("pivots", Some(json!(args.pivots))),
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
        };
        result(match body {
            Ok(body) => {
                self.api
                    .data_operation(&args.property_id, body, "runPivotReport")
                    .await
            }
            Err(error) => Err(error),
        })
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
        let request = report_request(None, args.dimensions, args.metrics, Vec::new());
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
        let request = google::access_report_request(google::AccessReportSpec {
            date_ranges: args.date_ranges,
            dimensions: args.dimensions,
            metrics: args.metrics,
            dimension_filter: args.dimension_filter,
            metric_filter: args.metric_filter,
            order_bys: args.order_bys,
            limit: args.limit,
            offset: args.offset,
            time_zone: args.time_zone,
        });
        result(match request {
            Ok(body) => self.api.access_report(&args.property_id, body, false).await,
            Err(error) => Err(error),
        })
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
        let request = google::access_report_request(google::AccessReportSpec {
            date_ranges: args.date_ranges,
            dimensions: args.dimensions,
            metrics: args.metrics,
            dimension_filter: args.dimension_filter,
            metric_filter: args.metric_filter,
            order_bys: args.order_bys,
            limit: args.limit,
            offset: args.offset,
            time_zone: args.time_zone,
        });
        result(match request {
            Ok(body) => self.api.access_report(&args.account_id, body, true).await,
            Err(error) => Err(error),
        })
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
        let validation = controls.request_limit(args.limit).and_then(|effective_limit| {
            google::property_name(&args.property_id)?;
            let args_limit = effective_limit;
            report_request(
                Some(args.date_ranges),
                args.dimensions,
                args.metrics,
                vec![
                    ("dimensionFilter", args.dimension_filter),
                    ("metricFilter", args.metric_filter),
                    ("orderBys", args.order_bys.map(|x| json!(x))),
                    ("limit", args_limit.map(|x| json!(x.to_string()))),
                    ("offset", args.offset.map(|x| json!(x.to_string()))),
                    ("currencyCode", args.currency_code.map(|x| json!(x))),
                    ("keepEmptyRows", args.keep_empty_rows.map(|x| json!(x))),
                    (
                        "returnPropertyQuota",
                        Some(json!(args.return_property_quota)),
                    ),
                ],
            )
        });
        result(validation.map(|body| json!({"request":body, "provider_called":false})))
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
        let response = match body {
            Ok(body) => self.api.report(&args.property_id, body, false).await,
            Err(error) => Err(error),
        };
        if controls.active() {
            result_with_tabular_projection(response, controls, false)
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
            result_with_tabular_projection(response, controls, true)
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
