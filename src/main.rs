//! Native MCP stdio server. Only implemented tools are advertised.
mod auth;
mod google;
mod http;
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
    #[serde(default)]
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
    #[serde(default)]
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
        Err(message) => CallToolResult::error(vec![ContentBlock::text(
            json!({
                "ok":false,
                "error":{
                    "code":"SCRATCHPAD_ERROR",
                    "reason":"scratchpad_error",
                    "message":message,
                    "category":"local"
                },
                "meta":{"elapsed_ms":0,"elapsed_ms_measured":false}
            })
            .to_string(),
        )]),
    }
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
    #[serde(default)]
    return_property_quota: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct BatchArgs {
    property_id: String,
    requests: Vec<Value>,
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
    #[serde(default)]
    limit: Option<u32>,
}

fn api_error_code(error: &ApiError) -> (&'static str, &'static str) {
    match error {
        ApiError::MissingCredentials => ("AUTH_NOT_CONFIGURED", "authentication"),
        ApiError::AuthenticationFailed => ("AUTHENTICATION_FAILED", "authentication"),
        ApiError::InvalidIdentifier => ("INVALID_IDENTIFIER", "validation"),
        ApiError::InvalidRequest => ("INVALID_REQUEST", "validation"),
        ApiError::InvalidAnnotation => ("INVALID_ANNOTATION", "validation"),
        ApiError::RequestFailed => ("UPSTREAM_UNAVAILABLE", "transport"),
        ApiError::Http(_) => ("GOOGLE_HTTP_ERROR", "upstream"),
        ApiError::ResponseTooLarge => ("RESPONSE_TOO_LARGE", "limits"),
        ApiError::CollectionTooLarge => ("COLLECTION_TOO_LARGE", "limits"),
        ApiError::TooManyPages => ("PAGINATION_LIMIT", "limits"),
        ApiError::InvalidResponse => ("INVALID_UPSTREAM_RESPONSE", "upstream"),
    }
}

fn result(r: Result<Value, ApiError>) -> CallToolResult {
    // elapsed_ms = 0 is an explicit *unmeasured* sentinel here, not a
    // claim of zero tool latency. Per-tool timing is a separate parity gate.
    let meta = json!({"elapsed_ms": 0, "elapsed_ms_measured": false});
    match r {
        Ok(v) => CallToolResult::success(vec![ContentBlock::text(
            json!({"ok": true, "data": v, "meta": meta}).to_string(),
        )]),
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
            CallToolResult::error(vec![ContentBlock::text(payload.to_string())])
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
    fn ga4_auth_status(&self) -> CallToolResult {
        result(Ok(auth::public_status()))
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
    fn ga4_auth_login_command(&self) -> CallToolResult {
        result(Ok(json!({
            "command": "synmetra auth login --client-id-file /path/to/desktop-oauth.json",
            "interactive_authorization_required": true,
            "scope": "https://www.googleapis.com/auth/analytics.readonly"
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
        description = "Execute one to five GA4 historical reports in a single batch using a requests JSON array.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn batch_run_reports(&self, Parameters(args): Parameters<BatchArgs>) -> CallToolResult {
        result(
            self.api
                .data_operation(
                    &args.property_id,
                    json!({"requests": args.requests}),
                    "batchRunReports",
                )
                .await,
        )
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
        let validation = google::property_name(&args.property_id).and_then(|_| {
            report_request(
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
                    ("keepEmptyRows", args.keep_empty_rows.map(|x| json!(x))),
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
                ("keepEmptyRows", args.keep_empty_rows.map(|x| json!(x))),
            ],
        );
        result(match body {
            Ok(body) => self.api.report(&args.property_id, body, false).await,
            Err(e) => Err(e),
        })
    }

    #[tool(
        description = "Run a realtime GA4 report. Realtime reports cannot use offsets or page cursors.",
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
        let body = report_request(
            None,
            args.dimensions,
            args.metrics,
            vec![
                ("dimensionFilter", args.dimension_filter),
                ("metricFilter", args.metric_filter),
                ("limit", args.limit.map(|x| json!(x.to_string()))),
            ],
        );
        result(match body {
            Ok(body) => self.api.report(&args.property_id, body, true).await,
            Err(e) => Err(e),
        })
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
        let body = report_request(
            None,
            args.dimensions,
            args.metrics,
            vec![
                ("dimensionFilter", args.dimension_filter),
                ("metricFilter", args.metric_filter),
                ("orderBys", args.order_bys.map(|v| json!(v))),
                ("limit", Some(json!(args.limit.unwrap_or(1000).to_string()))),
                ("offset", args.offset.map(|v| json!(v.to_string()))),
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
        eprintln!("Usage: synmetra [serve [--http-loopback] [--allow-write] [--scratchpad]|--version|auth login --client-id-file PATH|auth status|auth logout]\nStarts an experimental read-only MCP stdio server.\nUse synmetra auth login for native read-only OAuth, or SYNMETRA_GOOGLE_ACCESS_TOKEN for a short-lived token.");
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
        };
        let result = server.preview_report_request(Parameters(bad)).await;
        assert_eq!(result.is_error, Some(true));
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
    fn mcp_text_envelopes_have_stable_error_categories_and_explicit_unmeasured_timing() {
        let success = result(Ok(json!({"rows":[]})));
        let output: Value =
            serde_json::from_str(&success.content[0].as_text().unwrap().text).unwrap();
        assert_eq!(output["ok"], true);
        assert!(output["data"]["rows"].is_array());
        assert_eq!(output["meta"]["elapsed_ms"], 0);
        assert_eq!(output["meta"]["elapsed_ms_measured"], false);

        let invalid = result(Err(ApiError::InvalidIdentifier));
        let failure: Value =
            serde_json::from_str(&invalid.content[0].as_text().unwrap().text).unwrap();
        assert_eq!(invalid.is_error, Some(true));
        assert_eq!(failure["error"]["code"], "INVALID_IDENTIFIER");
        assert_eq!(failure["error"]["category"], "validation");
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
        }
    }

    #[test]
    fn tool_errors_are_mcp_tool_errors() {
        let result = super::result(Err(ApiError::InvalidIdentifier));
        assert_eq!(result.is_error, Some(true));
        assert!(!result.content[0].as_text().unwrap().text.contains("Bearer"));
    }
}
