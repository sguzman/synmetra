//! Bounded and least-privilege Google Analytics HTTP client.
//! Credentials are never included in logs or response error strings.

use chrono::{Datelike, NaiveDate};
use futures_util::StreamExt;
use serde_json::{json, Map, Value};
use std::{collections::HashSet, fmt, time::Duration};

const ADMIN: &str = "https://analyticsadmin.googleapis.com";
const DATA: &str = "https://analyticsdata.googleapis.com";
const MAX_RESPONSE_BYTES: usize = 4 * 1024 * 1024;
const MAX_COLLECTION_BYTES: usize = 16 * 1024 * 1024;
const MAX_COLLECTION_ITEMS: usize = 10_000;
const MAX_PAGE_TOKEN_BYTES: usize = 1024;
const MAX_PAGES: usize = 100;

#[derive(Clone)]
pub struct GoogleClient {
    http: reqwest::Client,
    bearer: Option<String>,
    admin_base: String,
    data_base: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    MissingCredentials,
    InvalidIdentifier,
    InvalidRequest,
    UnsupportedAuthOption,
    UnsupportedRealtimeOffset,
    RequestFailed,
    Http(u16),
    ResponseTooLarge,
    InvalidResponse,
    TooManyPages,
    CollectionTooLarge,
    AuthenticationFailed,
    InvalidAnnotation,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCredentials => write!(f, "No Google access token configured. Run synmetra auth login --client-id-file PATH or set SYNMETRA_GOOGLE_ACCESS_TOKEN to a scoped, short-lived token."),
            Self::InvalidIdentifier => write!(f, "Invalid Google Analytics property identifier: expected positive decimal digits or properties/<digits>."),
            Self::InvalidRequest => write!(f, "Invalid Google Analytics request: report fields must be a JSON object."),
            Self::UnsupportedAuthOption => write!(f, "Requested OAuth command option is not supported: shared gcloud ADC and quota-project routing are not enabled."),
            Self::UnsupportedRealtimeOffset => write!(f, "Realtime report offset is unavailable in the current Google Analytics v1beta request. Use ordering and limit, or a historical report for offset pagination."),
            Self::RequestFailed => write!(f, "Google Analytics request failed (network or timeout)."),
            Self::Http(status) => write!(f, "Google Analytics API returned HTTP {status}. Check property access, scopes, quotas, and the request."),
            Self::ResponseTooLarge => write!(f, "Google Analytics response exceeded the 4 MiB safety limit."),
            Self::InvalidResponse => write!(f, "Google Analytics response was not valid JSON or had an invalid pagination contract."),
            Self::TooManyPages => write!(f, "Google Analytics pagination exceeded the 100-page safety limit."),
            Self::CollectionTooLarge => write!(f, "Google Analytics collection exceeded the 10,000-item or 16 MiB safety limit."),
            Self::AuthenticationFailed => write!(f, "Synmetra OAuth credentials could not be loaded or refreshed. Run synmetra auth status, then login again if necessary."),
            Self::InvalidAnnotation => write!(f, "Invalid annotation: check title, dates, description and allowed color."),
        }
    }
}

fn scoped_name(input: &str, scope: &str) -> Result<String, ApiError> {
    let prefix = format!("{scope}/");
    let digits = input.strip_prefix(&prefix).unwrap_or(input);
    if digits.is_empty()
        || digits.starts_with('0')
        || !digits.bytes().all(|c| c.is_ascii_digit())
        || digits.parse::<u64>().is_err()
    {
        return Err(ApiError::InvalidIdentifier);
    }
    Ok(format!("{scope}/{digits}"))
}

pub fn property_name(input: &str) -> Result<String, ApiError> {
    scoped_name(input, "properties")
}

pub fn account_name(input: &str) -> Result<String, ApiError> {
    scoped_name(input, "accounts")
}

/// Validate a single optional Google Analytics reporting annotation before any write.
pub fn annotation_payload(
    title: &str,
    start_date: &str,
    end_date: Option<&str>,
    description: Option<&str>,
    color: &str,
) -> Result<Value, ApiError> {
    let title = title.trim();
    if title.is_empty()
        || title.chars().count() > 60
        || description.is_some_and(|d| d.chars().count() > 150)
    {
        return Err(ApiError::InvalidAnnotation);
    }
    let color = color.trim().to_ascii_uppercase();
    if !["PURPLE", "BROWN", "BLUE", "GREEN", "RED", "CYAN"].contains(&color.as_str()) {
        // Orange is reserved for system-generated Google Analytics annotations.
        return Err(ApiError::InvalidAnnotation);
    }
    let parsed_start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
        .map_err(|_| ApiError::InvalidAnnotation)?;
    let date_object = |d: NaiveDate| {
        json!({
            "year": d.year(), "month": d.month(), "day": d.day()
        })
    };
    let mut annotation = json!({"title": title, "color": color});
    if let Some(description) = description {
        annotation["description"] = json!(description);
    }
    if let Some(end_date) = end_date {
        let parsed_end = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
            .map_err(|_| ApiError::InvalidAnnotation)?;
        if parsed_end < parsed_start {
            return Err(ApiError::InvalidAnnotation);
        }
        annotation["annotationDateRange"] = json!({
            "startDate": date_object(parsed_start), "endDate": date_object(parsed_end)
        });
    } else {
        annotation["annotationDate"] = date_object(parsed_start);
    }
    Ok(json!({"reportingDataAnnotation": annotation}))
}

impl GoogleClient {
    pub fn new(bearer: Option<String>) -> Self {
        Self::with_endpoints(bearer, ADMIN, DATA)
    }

    pub(crate) fn with_endpoints(bearer: Option<String>, admin: &str, data: &str) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(20))
            .connect_timeout(Duration::from_secs(8))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("TLS-enabled HTTP client must construct");
        Self {
            http,
            bearer: bearer.filter(|t| !t.trim().is_empty()),
            admin_base: admin.to_owned(),
            data_base: data.to_owned(),
        }
    }

    async fn token(&self) -> Result<String, ApiError> {
        if let Some(explicit) = &self.bearer {
            return Ok(explicit.clone());
        }
        match crate::auth::access_token().await {
            Ok(Some(token)) => Ok(token),
            Ok(None) => Err(ApiError::MissingCredentials),
            Err(_) => Err(ApiError::AuthenticationFailed),
        }
    }

    async fn send(&self, req: reqwest::RequestBuilder) -> Result<Value, ApiError> {
        let bearer = self.token().await?;
        let response = req
            .bearer_auth(&bearer)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await
            .map_err(|_| ApiError::RequestFailed)?;
        let status = response.status();
        if !status.is_success() {
            // Do not reflect response body or URL: both may contain secrets.
            return Err(ApiError::Http(status.as_u16()));
        }
        if response
            .content_length()
            .is_some_and(|n| n > MAX_RESPONSE_BYTES as u64)
        {
            return Err(ApiError::ResponseTooLarge);
        }
        let mut data = Vec::new();
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|_| ApiError::RequestFailed)?;
            if chunk.len() > MAX_RESPONSE_BYTES - data.len() {
                return Err(ApiError::ResponseTooLarge);
            }
            data.extend_from_slice(&chunk);
        }
        serde_json::from_slice(&data).map_err(|_| ApiError::InvalidResponse)
    }

    async fn get(
        &self,
        base: &str,
        path: &str,
        page_token: Option<&str>,
    ) -> Result<Value, ApiError> {
        let req = self.http.get(format!("{base}{path}"));
        let req = if let Some(token) = page_token {
            req.query(&[("pageToken", token)])
        } else {
            req
        };
        self.send(req).await
    }

    pub async fn collection(
        &self,
        path: &str,
        field: &str,
        alpha: bool,
    ) -> Result<Value, ApiError> {
        self.collection_with_options(path, field, alpha, None, None)
            .await
    }

    pub async fn collection_with_options(
        &self,
        path: &str,
        field: &str,
        alpha: bool,
        page_size: Option<u32>,
        max_pages: Option<u32>,
    ) -> Result<Value, ApiError> {
        if page_size.is_some_and(|n| n == 0 || n > 200)
            || max_pages.is_some_and(|n| n == 0 || n > MAX_PAGES as u32)
        {
            return Err(ApiError::InvalidRequest);
        }
        let base = &self.admin_base;
        let path = format!("/{}{path}", if alpha { "v1alpha" } else { "v1beta" });
        let mut output = Vec::new();
        let mut next: Option<String> = None;
        let mut tokens = HashSet::new();
        let mut payload_bytes = 0usize;
        let page_cap = max_pages.map_or(MAX_PAGES, |n| n as usize);
        for page_number in 0..page_cap {
            let page = if let Some(size) = page_size {
                let mut req = self
                    .http
                    .get(format!("{base}{path}"))
                    .query(&[("pageSize", size.to_string())]);
                if let Some(token) = next.as_deref() {
                    req = req.query(&[("pageToken", token)]);
                }
                self.send(req).await?
            } else {
                self.get(base, &path, next.as_deref()).await?
            };
            if let Some(items) = page.get(field) {
                let items = items.as_array().ok_or(ApiError::InvalidResponse)?;
                let bytes = serde_json::to_vec(items)
                    .map_err(|_| ApiError::InvalidResponse)?
                    .len();
                if output.len().saturating_add(items.len()) > MAX_COLLECTION_ITEMS
                    || payload_bytes.saturating_add(bytes) > MAX_COLLECTION_BYTES
                {
                    return Err(ApiError::CollectionTooLarge);
                }
                payload_bytes += bytes;
                output.extend_from_slice(items);
            }
            next = page
                .get("nextPageToken")
                .and_then(Value::as_str)
                .filter(|s| !s.is_empty())
                .map(ToOwned::to_owned);
            if let Some(token) = next.as_ref() {
                if token.len() > MAX_PAGE_TOKEN_BYTES || !tokens.insert(token.clone()) {
                    return Err(ApiError::InvalidResponse);
                }
            } else {
                return Ok(json!({field: output}));
            }
            if max_pages.is_some() && page_number + 1 == page_cap {
                let mut response = serde_json::Map::new();
                response.insert(field.to_owned(), json!(output));
                response.insert("nextPageToken".into(), json!(next));
                response.insert("truncated".into(), json!(true));
                return Ok(Value::Object(response));
            }
        }
        Err(ApiError::TooManyPages)
    }

    #[cfg(test)]
    pub async fn account_summaries(&self) -> Result<Value, ApiError> {
        self.collection("/accountSummaries", "accountSummaries", false)
            .await
    }

    pub async fn account_summaries_with_options(
        &self,
        page_size: Option<u32>,
        max_pages: Option<u32>,
    ) -> Result<Value, ApiError> {
        self.collection_with_options(
            "/accountSummaries",
            "accountSummaries",
            false,
            page_size,
            max_pages,
        )
        .await
    }

    pub async fn property(&self, property: &str) -> Result<Value, ApiError> {
        let path = format!("/v1beta/{}", property_name(property)?);
        self.get(&self.admin_base, &path, None).await
    }

    pub async fn account_sharing(&self, account: &str) -> Result<Value, ApiError> {
        let path = format!("/v1beta/{}/dataSharingSettings", account_name(account)?);
        self.get(&self.admin_base, &path, None).await
    }

    pub async fn retention_settings(&self, property: &str) -> Result<Value, ApiError> {
        let path = format!("/v1beta/{}/dataRetentionSettings", property_name(property)?);
        self.get(&self.admin_base, &path, None).await
    }

    #[cfg(test)]
    pub async fn ads_links(&self, property: &str) -> Result<Value, ApiError> {
        let path = format!("/{}/googleAdsLinks", property_name(property)?);
        self.collection(&path, "googleAdsLinks", false).await
    }

    pub async fn ads_links_with_options(
        &self,
        property: &str,
        page_size: Option<u32>,
        max_pages: Option<u32>,
    ) -> Result<Value, ApiError> {
        let path = format!("/{}/googleAdsLinks", property_name(property)?);
        self.collection_with_options(&path, "googleAdsLinks", false, page_size, max_pages)
            .await
    }

    pub async fn create_annotation(&self, property: &str, body: Value) -> Result<Value, ApiError> {
        let path = property_name(property)?;
        if !body.is_object() || body["reportingDataAnnotation"].is_null() {
            return Err(ApiError::InvalidAnnotation);
        }
        let url = format!(
            "{}/v1alpha/{path}/reportingDataAnnotations",
            self.admin_base
        );
        // Mutating requests are never retried automatically.
        self.send(self.http.post(url).json(&body)).await
    }

    pub async fn annotations_with_options(
        &self,
        property: &str,
        page_size: Option<u32>,
        max_pages: Option<u32>,
    ) -> Result<Value, ApiError> {
        let path = format!("/{}/reportingDataAnnotations", property_name(property)?);
        self.collection_with_options(
            &path,
            "reportingDataAnnotations",
            true,
            page_size,
            max_pages,
        )
        .await
    }

    pub async fn custom_definitions(&self, property: &str) -> Result<Value, ApiError> {
        let path = property_name(property)?;
        let dimensions = self
            .collection(
                &format!("/{path}/customDimensions"),
                "customDimensions",
                false,
            )
            .await?;
        let metrics = self
            .collection(&format!("/{path}/customMetrics"), "customMetrics", false)
            .await?;
        Ok(json!({
            "custom_dimensions": dimensions["customDimensions"],
            "custom_metrics": metrics["customMetrics"]
        }))
    }

    pub(crate) fn validate_body(request: &Value) -> Result<(), ApiError> {
        if !request.is_object()
            || serde_json::to_vec(request).map_or(true, |bytes| bytes.len() > 64 * 1024)
        {
            return Err(ApiError::InvalidRequest);
        }
        Ok(())
    }

    pub async fn data_operation(
        &self,
        property: &str,
        request: Value,
        method: &str,
    ) -> Result<Value, ApiError> {
        let property =
            if method == "checkCompatibility" && (property == "0" || property == "properties/0") {
                "properties/0".to_owned()
            } else {
                property_name(property)?
            };
        Self::validate_body(&request)?;
        if ![
            "runReport",
            "runRealtimeReport",
            "runPivotReport",
            "batchRunReports",
            "checkCompatibility",
        ]
        .contains(&method)
        {
            return Err(ApiError::InvalidRequest);
        }
        if method == "batchRunReports"
            && request["requests"]
                .as_array()
                .is_none_or(|list| list.is_empty() || list.len() > 5)
        {
            return Err(ApiError::InvalidRequest);
        }
        let url = format!("{}/v1beta/{property}:{method}", self.data_base);
        let response = self.send(self.http.post(url).json(&request)).await?;
        match method {
            "runReport" | "runRealtimeReport" | "runPivotReport" => {
                validate_google_report_response(&response)?;
            }
            "batchRunReports" => {
                let reports = response["reports"]
                    .as_array()
                    .ok_or(ApiError::InvalidResponse)?;
                if reports.len()
                    != request["requests"]
                        .as_array()
                        .ok_or(ApiError::InvalidRequest)?
                        .len()
                {
                    return Err(ApiError::InvalidResponse);
                }
                for report in reports {
                    validate_google_report_response(report)?;
                }
            }
            _ => {}
        }
        Ok(response)
    }

    pub async fn alpha_report(
        &self,
        property: &str,
        request: Value,
        funnel: bool,
    ) -> Result<Value, ApiError> {
        let property = property_name(property)?;
        Self::validate_body(&request)?;
        if funnel {
            let steps = request["funnel"]["steps"]
                .as_array()
                .ok_or(ApiError::InvalidRequest)?;
            if steps.is_empty() || steps.len() > 10 {
                return Err(ApiError::InvalidRequest);
            }
        } else if request.get("conversionSpec").is_none()
            && !request["metrics"].as_array().is_some_and(|metrics| {
                metrics.iter().any(|metric| {
                    metric["name"]
                        .as_str()
                        .is_some_and(|name| name.to_lowercase().contains("conversion"))
                })
            })
        {
            return Err(ApiError::InvalidRequest);
        }
        let method = if funnel {
            "runFunnelReport"
        } else {
            "runReport"
        };
        let url = format!("{}/v1alpha/{property}:{method}", self.data_base);
        self.send(self.http.post(url).json(&request)).await
    }

    pub async fn access_report(
        &self,
        entity: &str,
        request: Value,
        account: bool,
    ) -> Result<Value, ApiError> {
        let entity = if account {
            account_name(entity)?
        } else {
            property_name(entity)?
        };
        Self::validate_body(&request)?;
        // This Admin API endpoint requires Administrator privileges on the selected entity.
        let url = format!("{}/v1beta/{entity}:runAccessReport", self.admin_base);
        self.send(self.http.post(url).json(&request)).await
    }

    pub async fn report(
        &self,
        property: &str,
        request: Value,
        realtime: bool,
    ) -> Result<Value, ApiError> {
        self.data_operation(
            property,
            request,
            if realtime {
                "runRealtimeReport"
            } else {
                "runReport"
            },
        )
        .await
    }
}

/// Reject malformed successful GA4 tabular responses before exposing them to
/// either native provider-shaped tools or the optional Contract V1 projector.
fn validate_google_report_response(response: &Value) -> Result<(), ApiError> {
    let root = response.as_object().ok_or(ApiError::InvalidResponse)?;
    let count = if let Some(raw) = root.get("rowCount") {
        let count = raw
            .as_u64()
            .or_else(|| raw.as_str().and_then(|text| text.parse::<u64>().ok()));
        Some(count.ok_or(ApiError::InvalidResponse)?)
    } else {
        None
    };
    for header in ["dimensionHeaders", "metricHeaders"] {
        if let Some(value) = root.get(header) {
            let items = value.as_array().ok_or(ApiError::InvalidResponse)?;
            if items.iter().any(|item| !item.is_object()) {
                return Err(ApiError::InvalidResponse);
            }
        }
    }
    if let Some(raw_rows) = root.get("rows") {
        let rows = raw_rows.as_array().ok_or(ApiError::InvalidResponse)?;
        if count.is_some_and(|count| count < rows.len() as u64) {
            return Err(ApiError::InvalidResponse);
        }
        for row in rows {
            let row = row.as_object().ok_or(ApiError::InvalidResponse)?;
            for field in ["dimensionValues", "metricValues"] {
                if let Some(value) = row.get(field) {
                    let items = value.as_array().ok_or(ApiError::InvalidResponse)?;
                    if items.iter().any(|item| !item.is_object()) {
                        return Err(ApiError::InvalidResponse);
                    }
                }
            }
        }
    }
    Ok(())
}

/// Normalize and validate Google's v1alpha conversion report specification.
/// Schema differs from historical GA4 Core Data reports.
pub fn conversion_spec(
    spec: &Value,
    dimensions: &[String],
    metrics: &[String],
) -> Result<Value, ApiError> {
    const DIMS: &[&str] = &[
        "campaignName",
        "continent",
        "country",
        "defaultChannelGroup",
        "deviceCategory",
        "medium",
        "platform",
        "primaryChannelGroup",
        "source",
        "sourceMedium",
        "sourcePlatform",
        "subcontinent",
    ];
    const METRICS: &[&str] = &[
        "advertiserAdClicks",
        "advertiserAdCost",
        "advertiserAdCostPerAllConversionsByConversionDate",
        "advertiserAdCostPerAllConversionsByInteractionDate",
        "advertiserAdCostPerClick",
        "advertiserAdImpressions",
        "allConversionsByConversionDate",
        "allConversionsByInteractionDate",
        "returnOnAdSpendByConversionDate",
        "returnOnAdSpendByInteractionDate",
        "totalRevenueByConversionDate",
        "totalRevenueByInteractionDate",
    ];
    if !spec.is_object()
        || dimensions.iter().any(|s| !DIMS.contains(&s.as_str()))
        || metrics.is_empty()
        || metrics.iter().any(|s| !METRICS.contains(&s.as_str()))
    {
        return Err(ApiError::InvalidRequest);
    }
    let actions = spec
        .get("conversion_actions")
        .or_else(|| spec.get("conversionActions"));
    let actions = match actions {
        Some(value) => value.as_array().ok_or(ApiError::InvalidRequest)?.clone(),
        None => Vec::new(),
    };
    if actions.len() > 50
        || actions.iter().any(|action| {
            action.as_str().is_none_or(|s| {
                let Some(id) = s.strip_prefix("conversionActions/") else {
                    return true;
                };
                id.is_empty() || id.bytes().any(|c| !c.is_ascii_digit())
            })
        })
    {
        return Err(ApiError::InvalidRequest);
    }
    let model = spec
        .get("attribution_model")
        .or_else(|| spec.get("attributionModel"));
    if model.is_some_and(|v| !["DATA_DRIVEN", "LAST_CLICK"].contains(&v.as_str().unwrap_or(""))) {
        return Err(ApiError::InvalidRequest);
    }
    let mut result = json!({"conversionActions":actions});
    if let Some(model) = model {
        result["attributionModel"] = model.clone();
    }
    Ok(result)
}

/// Map official funnel step arguments into Google Data API v1alpha REST JSON.
pub fn funnel_request(
    steps: Vec<Value>,
    dates: Option<Vec<Value>>,
    breakdown: Option<Value>,
    next_action: Option<Value>,
    segments: Option<Vec<Value>>,
    quota: bool,
) -> Result<Value, ApiError> {
    if steps.is_empty()
        || steps.len() > 10
        || dates.as_ref().is_some_and(|d| d.len() > 4)
        || segments.as_ref().is_some_and(|s| {
            s.len() > 4
                || s.iter()
                    .any(|x| !x.is_object() || x.as_object().is_some_and(|obj| obj.is_empty()))
        })
    {
        return Err(ApiError::InvalidRequest);
    }
    let mut mapped = Vec::new();
    for (index, step) in steps.into_iter().enumerate() {
        let object = step.as_object().ok_or(ApiError::InvalidRequest)?;
        let event = object.get("event");
        let filter = object
            .get("filter_expression")
            .or_else(|| object.get("filterExpression"));
        if event.is_some() == filter.is_some() {
            return Err(ApiError::InvalidRequest);
        }
        let name = object
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("")
            .trim()
            .to_owned();
        let name = if name.is_empty() {
            format!("Step {}", index + 1)
        } else {
            name
        };
        if name.len() > 100 {
            return Err(ApiError::InvalidRequest);
        }
        let expression = if let Some(event) = event {
            let event = event.as_str().ok_or(ApiError::InvalidRequest)?;
            if event.is_empty() || event.len() > 128 {
                return Err(ApiError::InvalidRequest);
            }
            json!({"funnelEventFilter":{"eventName":event}})
        } else {
            let value = filter.ok_or(ApiError::InvalidRequest)?;
            validate_bounded_filter(value)?;
            value.clone()
        };
        mapped.push(json!({"name":name,"filterExpression":expression}));
    }
    let dates = match dates {
        Some(dates) => normalize_date_ranges(dates)?,
        None => Vec::new(),
    };
    let mut request = json!({
        "funnel":{"steps":mapped},
        "dateRanges":dates,
        "returnPropertyQuota":quota
    });
    if let Some(breakdown) = breakdown {
        let field = breakdown
            .get("breakdown_dimension")
            .or_else(|| breakdown.get("breakdownDimension"))
            .and_then(Value::as_str)
            .ok_or(ApiError::InvalidRequest)?;
        if field.trim().is_empty() {
            return Err(ApiError::InvalidRequest);
        }
        request["funnelBreakdown"] = json!({"breakdownDimension":{"name":field}});
    }
    if let Some(next_action) = next_action {
        let field = next_action
            .get("next_action_dimension")
            .or_else(|| next_action.get("nextActionDimension"))
            .and_then(Value::as_str)
            .ok_or(ApiError::InvalidRequest)?;
        if field.trim().is_empty() {
            return Err(ApiError::InvalidRequest);
        }
        let mut action = json!({"nextActionDimension":{"name":field}});
        if let Some(limit) = next_action.get("limit") {
            let amount = limit.as_u64().ok_or(ApiError::InvalidRequest)?;
            if amount == 0 || amount > 1000 {
                return Err(ApiError::InvalidRequest);
            }
            action["limit"] = json!(amount.to_string());
        }
        request["funnelNextAction"] = action;
    }
    if let Some(segments) = segments {
        request["segments"] = json!(segments);
    }
    if serde_json::to_vec(&request).map_or(true, |bytes| bytes.len() > 64 * 1024) {
        return Err(ApiError::InvalidRequest);
    }
    Ok(request)
}

/// Normalize Google Analytics Admin API access-report inputs.
/// AccessDimension/AccessMetric use dimensionName/metricName (not "name").
pub struct AccessReportSpec {
    pub date_ranges: Vec<Value>,
    pub dimensions: Vec<String>,
    pub metrics: Vec<String>,
    pub dimension_filter: Option<Value>,
    pub metric_filter: Option<Value>,
    pub order_bys: Option<Vec<Value>>,
    pub limit: Option<u32>,
    pub offset: Option<u64>,
    pub time_zone: Option<String>,
}

pub fn access_report_request(spec: AccessReportSpec) -> Result<Value, ApiError> {
    let AccessReportSpec {
        date_ranges,
        dimensions,
        metrics,
        dimension_filter,
        metric_filter,
        order_bys,
        limit,
        offset,
        time_zone,
    } = spec;
    if date_ranges.is_empty()
        || date_ranges.len() > 4
        || metrics.is_empty()
        || metrics.len() > 10
        || dimensions.len() > 9
        || dimensions
            .iter()
            .chain(metrics.iter())
            .any(|s| s.trim().is_empty())
        || limit.is_some_and(|v| v == 0 || v > 250_000)
    {
        return Err(ApiError::InvalidRequest);
    }
    let dates = normalize_date_ranges(date_ranges)?;
    let mut req = json!({
        "dateRanges":dates,
        "dimensions":dimensions.iter().map(|name| json!({"dimensionName":name})).collect::<Vec<_>>(),
        "metrics":metrics.iter().map(|name| json!({"metricName":name})).collect::<Vec<_>>()
    });
    if let Some(filter) = dimension_filter {
        validate_bounded_filter(&filter)?;
        req["dimensionFilter"] = filter;
    }
    if let Some(filter) = metric_filter {
        validate_bounded_filter(&filter)?;
        req["metricFilter"] = filter;
    }
    if let Some(order_bys) = order_bys {
        req["orderBys"] = json!(order_bys);
    }
    if let Some(limit) = limit {
        req["limit"] = json!(limit.to_string());
    }
    if let Some(offset) = offset {
        req["offset"] = json!(offset.to_string());
    }
    if let Some(zone) = time_zone {
        if zone.is_empty()
            || zone.len() > 80
            || zone
                .chars()
                .any(|c| !c.is_ascii_alphanumeric() && !"/_+-".contains(c))
        {
            return Err(ApiError::InvalidRequest);
        }
        req["timeZone"] = json!(zone);
    }
    if serde_json::to_vec(&req).map_or(true, |bytes| bytes.len() > 64 * 1024) {
        return Err(ApiError::InvalidRequest);
    }
    Ok(req)
}

/// Canonicalize relative and absolute GA4 date ranges without silently
/// passing malformed dates or unknown fields to the provider.
pub fn normalize_date_ranges(dates: Vec<Value>) -> Result<Vec<Value>, ApiError> {
    if dates.is_empty() || dates.len() > 4 {
        return Err(ApiError::InvalidRequest);
    }
    fn validate_date(value: &str) -> Result<Option<NaiveDate>, ApiError> {
        if value == "today" || value == "yesterday" {
            return Ok(None);
        }
        if let Some(days) = value.strip_suffix("daysAgo") {
            if days.is_empty()
                || days.len() > 5
                || !days.bytes().all(|ch| ch.is_ascii_digit())
                || days.parse::<u32>().map_or(true, |n| n > 36500)
            {
                return Err(ApiError::InvalidRequest);
            }
            return Ok(None);
        }
        NaiveDate::parse_from_str(value, "%Y-%m-%d")
            .map(Some)
            .map_err(|_| ApiError::InvalidRequest)
    }
    dates
        .into_iter()
        .map(|date| {
            let data = date.as_object().ok_or(ApiError::InvalidRequest)?;
            if data.keys().any(|key| {
                !["startDate", "endDate", "start_date", "end_date", "name"].contains(&key.as_str())
            }) {
                return Err(ApiError::InvalidRequest);
            }
            let start = data
                .get("startDate")
                .or_else(|| data.get("start_date"))
                .and_then(Value::as_str)
                .ok_or(ApiError::InvalidRequest)?;
            let end = data
                .get("endDate")
                .or_else(|| data.get("end_date"))
                .and_then(Value::as_str)
                .ok_or(ApiError::InvalidRequest)?;
            if data.contains_key("startDate") && data.contains_key("start_date")
                || data.contains_key("endDate") && data.contains_key("end_date")
            {
                return Err(ApiError::InvalidRequest);
            }
            let start_abs = validate_date(start)?;
            let end_abs = validate_date(end)?;
            if let (Some(start), Some(end)) = (start_abs, end_abs) {
                if start > end {
                    return Err(ApiError::InvalidRequest);
                }
            }
            let mut range = json!({"startDate":start,"endDate":end});
            if let Some(name) = data.get("name") {
                let name = name.as_str().ok_or(ApiError::InvalidRequest)?;
                if name.trim().is_empty() || name.len() > 64 {
                    return Err(ApiError::InvalidRequest);
                }
                range["name"] = json!(name);
            }
            Ok(range)
        })
        .collect()
}

/// Accept Google FilterExpression objects, JSON-encoded objects, or an
/// explicit equality shorthand. Reject ambiguous and lossily coerced values.
pub fn normalize_report_filter(input: Value, metric: bool) -> Result<Value, ApiError> {
    let value = match input {
        Value::Object(_) => input,
        Value::String(raw) => {
            let raw = raw.trim();
            if raw.len() > 1024 || raw.is_empty() {
                return Err(ApiError::InvalidRequest);
            }
            if raw.starts_with('{') {
                serde_json::from_str::<Value>(raw).map_err(|_| ApiError::InvalidRequest)?
            } else {
                let (field, raw_value) = raw.split_once("==").ok_or(ApiError::InvalidRequest)?;
                let field = field.trim();
                let raw_value = raw_value.trim();
                if field.is_empty()
                    || field.len() > 64
                    || !field.as_bytes()[0].is_ascii_alphabetic()
                    || !field
                        .bytes()
                        .all(|c| c.is_ascii_alphanumeric() || c == b'_')
                    || raw_value.is_empty()
                    || raw_value.len() > 256
                {
                    return Err(ApiError::InvalidRequest);
                }
                if metric {
                    // Numeric filters must carry exact integer or finite decimal
                    // values, not invented string-to-number coercions.
                    if raw_value.starts_with('"') || raw_value.contains("==") {
                        return Err(ApiError::InvalidRequest);
                    }
                    let digits = raw_value.strip_prefix('-').unwrap_or(raw_value);
                    let numeric =
                        if !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit()) {
                            // Never silently convert an out-of-range integer to f64.
                            let value = raw_value
                                .parse::<i64>()
                                .map_err(|_| ApiError::InvalidRequest)?;
                            json!({"int64Value":value.to_string()})
                        } else {
                            let value = raw_value
                                .parse::<f64>()
                                .map_err(|_| ApiError::InvalidRequest)?;
                            if !value.is_finite() {
                                return Err(ApiError::InvalidRequest);
                            }
                            json!({"doubleValue":value})
                        };
                    json!({"filter":{"fieldName":field,"numericFilter":{"operation":"EQUAL","value":numeric}}})
                } else {
                    let parsed = if raw_value.starts_with('"') {
                        serde_json::from_str::<String>(raw_value)
                            .map_err(|_| ApiError::InvalidRequest)?
                    } else {
                        if raw_value
                            .chars()
                            .any(|c| c.is_whitespace() || c.is_control())
                            || raw_value.contains("==")
                            || raw_value.contains('"')
                        {
                            return Err(ApiError::InvalidRequest);
                        }
                        raw_value.to_owned()
                    };
                    if parsed.is_empty() || parsed.len() > 256 {
                        return Err(ApiError::InvalidRequest);
                    }
                    json!({"filter":{"fieldName":field,"stringFilter":{"matchType":"EXACT","value":parsed}}})
                }
            }
        }
        _ => return Err(ApiError::InvalidRequest),
    };
    validate_bounded_filter(&value)?;
    Ok(value)
}

/// Validate bounded, structured GA4 FilterExpression input without coercing values.
/// The Google API remains responsible for deciding valid field/operator semantics.
pub fn validate_bounded_filter(filter: &Value) -> Result<(), ApiError> {
    if !filter.is_object()
        || serde_json::to_vec(filter).map_or(true, |bytes| bytes.len() > 16 * 1024)
    {
        return Err(ApiError::InvalidRequest);
    }
    let mut nodes = 0usize;
    let mut stack = vec![(filter, 0usize)];
    while let Some((value, depth)) = stack.pop() {
        nodes += 1;
        if nodes > 256 || depth > 12 {
            return Err(ApiError::InvalidRequest);
        }
        match value {
            Value::Object(map) => {
                if map.is_empty() || map.len() > 64 {
                    return Err(ApiError::InvalidRequest);
                }
                for (name, nested) in map {
                    if name.len() > 128 {
                        return Err(ApiError::InvalidRequest);
                    }
                    stack.push((nested, depth + 1));
                }
            }
            Value::Array(items) => {
                if items.len() > 64 {
                    return Err(ApiError::InvalidRequest);
                }
                for nested in items {
                    stack.push((nested, depth + 1));
                }
            }
            Value::String(value) if value.len() > 1024 => {
                return Err(ApiError::InvalidRequest);
            }
            _ => {}
        }
    }
    Ok(())
}

/// Prepare a compatibility-only probe without inventing a report request. Google
/// accepts property 0 to check universally available dimensions and metrics.
pub fn compatibility_request(
    dimensions: Vec<String>,
    metrics: Vec<String>,
) -> Result<Value, ApiError> {
    if dimensions.len() > 9
        || metrics.len() > 10
        || dimensions
            .iter()
            .chain(metrics.iter())
            .any(|name| name.is_empty() || name.len() > 128 || name.chars().any(char::is_control))
    {
        return Err(ApiError::InvalidRequest);
    }
    let body = json!({
        "dimensions": dimensions.into_iter().map(|name| json!({"name":name})).collect::<Vec<_>>(),
        "metrics": metrics.into_iter().map(|name| json!({"name":name})).collect::<Vec<_>>()
    });
    GoogleClient::validate_body(&body)?;
    Ok(body)
}

/// Google REST APIs accept array entries as objects, not raw field names.
pub fn report_request(
    dates: Option<Vec<Value>>,
    dimensions: Vec<String>,
    metrics: Vec<String>,
    extras: Vec<(&str, Option<Value>)>,
) -> Result<Value, ApiError> {
    if metrics.is_empty()
        || metrics.len() > 10
        || dimensions.len() > 9
        || metrics
            .iter()
            .chain(dimensions.iter())
            .any(|s| s.trim().is_empty())
    {
        return Err(ApiError::InvalidRequest);
    }

    let mut request = Map::new();
    if let Some(dates) = dates {
        request.insert(
            "dateRanges".to_owned(),
            Value::Array(normalize_date_ranges(dates)?),
        );
    }
    request.insert(
        "dimensions".to_owned(),
        Value::Array(
            dimensions
                .into_iter()
                .map(|name| json!({"name": name}))
                .collect(),
        ),
    );
    request.insert(
        "metrics".to_owned(),
        Value::Array(
            metrics
                .into_iter()
                .map(|name| json!({"name": name}))
                .collect(),
        ),
    );
    for (name, val) in extras {
        if let Some(val) = val {
            let val = match name {
                "dimensionFilter" => normalize_report_filter(val, false)?,
                "metricFilter" => normalize_report_filter(val, true)?,
                _ => val,
            };
            request.insert(name.to_owned(), val);
        }
    }
    let body = Value::Object(request);
    GoogleClient::validate_body(&body)?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };

    #[test]
    fn conversion_specs_enforce_alpha_metric_dimension_and_action_limits() {
        let dim = vec!["country".to_owned()];
        let metric = vec!["allConversionsByInteractionDate".to_owned()];
        let converted = conversion_spec(
            &json!({"conversion_actions":["conversionActions/42"],"attribution_model":"LAST_CLICK"}),
            &dim, &metric,
        ).unwrap();
        assert_eq!(converted["conversionActions"][0], "conversionActions/42");
        assert_eq!(converted["attributionModel"], "LAST_CLICK");
        assert!(conversion_spec(&json!({}), &["invalid".into()], &metric).is_err());
        assert!(conversion_spec(&json!({}), &dim, &["activeUsers".into()]).is_err());
        assert!(
            conversion_spec(&json!({"conversion_actions":["../unsafe"]}), &dim, &metric).is_err()
        );
        assert!(conversion_spec(
            &json!({"conversion_actions":vec!["conversionActions/5";51]}),
            &dim,
            &metric
        )
        .is_err());
        assert!(
            conversion_spec(&json!({"attribution_model":"NOT_A_MODEL"}), &dim, &metric).is_err()
        );
    }

    #[test]
    fn funnels_normalize_events_and_reject_unsafe_step_shapes() {
        let request = funnel_request(
            vec![
                json!({"event":"page_view"}),
                json!({"name":"Signup","event":"sign_up"}),
            ],
            Some(vec![json!({"startDate":"7daysAgo","endDate":"today"})]),
            Some(json!({"breakdown_dimension":"deviceCategory"})),
            Some(json!({"next_action_dimension":"eventName","limit":5})),
            None,
            false,
        )
        .unwrap();
        assert_eq!(request["funnel"]["steps"][0]["name"], "Step 1");
        assert_eq!(
            request["funnel"]["steps"][0]["filterExpression"]["funnelEventFilter"]["eventName"],
            "page_view"
        );
        assert_eq!(
            request["funnelBreakdown"]["breakdownDimension"]["name"],
            "deviceCategory"
        );
        assert_eq!(request["funnelNextAction"]["limit"], "5");
        for bad in [
            vec![],
            vec![json!({})],
            vec![json!({"event":"page_view","filter_expression":{}})],
            vec![json!({"event":""})],
            vec![json!({"filter_expression":{}})],
            vec![json!({"event":"page_view"}); 11],
        ] {
            assert!(funnel_request(bad, None, None, None, None, false).is_err());
        }
    }

    #[test]
    fn report_filter_shorthand_normalizes_without_ambiguous_coercions() {
        let country = normalize_report_filter(json!("country==Mexico"), false).unwrap();
        assert_eq!(country["filter"]["fieldName"], "country");
        assert_eq!(country["filter"]["stringFilter"]["value"], "Mexico");
        let channel =
            normalize_report_filter(json!("sessionDefaultChannelGroup==\"Paid Other\""), false)
                .unwrap();
        assert_eq!(channel["filter"]["stringFilter"]["value"], "Paid Other");
        let count = normalize_report_filter(json!("sessions==7"), true).unwrap();
        assert_eq!(count["filter"]["numericFilter"]["value"]["int64Value"], "7");
        let rate = normalize_report_filter(json!("engagementRate==0.5"), true).unwrap();
        assert_eq!(rate["filter"]["numericFilter"]["value"]["doubleValue"], 0.5);
        let json_encoded = normalize_report_filter(
            json!(r#"{"filter":{"fieldName":"country","stringFilter":{"value":"MX","matchType":"EXACT"}}}"#),
            false,
        ).unwrap();
        assert_eq!(json_encoded["filter"]["stringFilter"]["value"], "MX");

        for raw in [
            "country=Mexico",
            "country==Paid Other",
            "country==",
            "country-name==Mexico",
            "country==Mexico==US",
            "country==\"unterminated",
            "country==\"x\"y",
        ] {
            assert!(normalize_report_filter(json!(raw), false).is_err(), "{raw}");
        }
        assert!(normalize_report_filter(json!("sessions==\"7\""), true).is_err());
        assert!(normalize_report_filter(json!("sessions==NaN"), true).is_err());
        assert!(normalize_report_filter(json!("sessions==infinity"), true).is_err());
    }

    #[test]
    fn numeric_metric_filter_preserves_i64_edges_and_rejects_overflow() {
        for (literal, canonical) in [
            ("9223372036854775807", "9223372036854775807"),
            ("-9223372036854775808", "-9223372036854775808"),
            ("0", "0"),
        ] {
            let value =
                normalize_report_filter(json!(format!("sessions=={literal}")), true).unwrap();
            assert_eq!(
                value["filter"]["numericFilter"]["value"]["int64Value"],
                canonical
            );
        }
        for invalid in [
            "sessions==9223372036854775808",
            "sessions==-9223372036854775809",
            "sessions==18446744073709551616",
            "sessions==999999999999999999999999999999999",
        ] {
            assert_eq!(
                normalize_report_filter(json!(invalid), true),
                Err(ApiError::InvalidRequest),
                "{invalid}"
            );
        }
        let decimal = normalize_report_filter(json!("engagementRate==1e20"), true).unwrap();
        assert_eq!(
            decimal["filter"]["numericFilter"]["value"]["doubleValue"],
            1e20
        );
    }

    #[test]
    fn nested_ga_filter_preserved_and_excessive_structures_rejected() {
        let valid = json!({
            "andGroup": { "expressions": [
                { "filter": { "fieldName": "country", "stringFilter": {
                    "matchType": "EXACT", "value": "Mexico"
                }}},
                { "notExpression": { "filter": { "fieldName": "eventName", "stringFilter": {
                    "matchType": "EXACT", "value": "test_event"
                }}}}
            ]}
        });
        assert!(validate_bounded_filter(&valid).is_ok());
        let request = report_request(
            Some(vec![json!({"startDate":"7daysAgo","endDate":"today"})]),
            vec!["country".into()],
            vec!["activeUsers".into()],
            vec![("dimensionFilter", Some(valid.clone()))],
        )
        .unwrap();
        assert_eq!(request["dimensionFilter"], valid);

        let mut deep = json!({"filter":{"fieldName":"country"}});
        for _ in 0..14 {
            deep = json!({"notExpression":deep});
        }
        assert!(validate_bounded_filter(&deep).is_err());
        assert!(validate_bounded_filter(&json!("country==Mexico")).is_err());
        assert!(validate_bounded_filter(&json!({"value":"x".repeat(1025)})).is_err());
        assert!(validate_bounded_filter(&json!({"expressions":vec![true;65]})).is_err());
        assert!(report_request(
            None,
            vec![],
            vec!["activeUsers".into()],
            vec![("metricFilter", Some(deep))]
        )
        .is_err());
    }

    #[test]
    fn date_ranges_preserve_relative_dates_and_normalize_snake_case() {
        let result = normalize_date_ranges(vec![
            json!({"startDate":"7daysAgo","endDate":"today"}),
            json!({"start_date":"2026-10-01","end_date":"2026-10-25","name":"baseline"}),
        ])
        .unwrap();
        assert_eq!(result[0]["startDate"], "7daysAgo");
        assert_eq!(result[1]["startDate"], "2026-10-01");
        assert_eq!(result[1]["name"], "baseline");
        assert_eq!(result.len(), 2);
        assert!(report_request(
            Some(vec![json!({"start_date":"yesterday","end_date":"today"})]),
            vec!["country".into()],
            vec!["activeUsers".into()],
            vec![]
        )
        .is_ok());
    }

    #[test]
    fn date_ranges_reject_invalid_dates_and_unknown_fields() {
        for invalid in [
            json!({"startDate":"2026-02-30","endDate":"today"}),
            json!({"startDate":"2026-10-26","endDate":"2026-10-25"}),
            json!({"startDate":"999999daysAgo","endDate":"today"}),
            json!({"startDate":"-2daysAgo","endDate":"today"}),
            json!({"startDate":"7dayAgo","endDate":"today"}),
            json!({"startDate":"today","endDate":"today","other":"unrecognized"}),
            json!({"startDate":"today","start_date":"today","endDate":"today"}),
            json!({"startDate":"today"}),
            json!({"startDate":5,"endDate":"today"}),
        ] {
            assert_eq!(
                normalize_date_ranges(vec![invalid]).unwrap_err(),
                ApiError::InvalidRequest
            );
        }
        assert!(normalize_date_ranges(vec![]).is_err());
        assert!(
            normalize_date_ranges(vec![json!({"startDate":"today","endDate":"today"}); 5]).is_err()
        );
    }

    #[test]
    fn four_historical_date_ranges_round_trip_and_a_fifth_is_rejected() {
        let ranges = vec![
            json!({"start_date":"7daysAgo","end_date":"yesterday","name":"Recent"}),
            json!({"startDate":"2026-01-01","endDate":"2026-01-31","name":"January"}),
            json!({"startDate":"today","endDate":"today"}),
            json!({"startDate":"2024-02-29","endDate":"2024-03-01","name":"Leap day"}),
        ];
        let normalized = report_request(
            Some(ranges.clone()),
            vec!["country".into()],
            vec!["activeUsers".into()],
            vec![],
        )
        .unwrap();
        let output = normalized["dateRanges"].as_array().unwrap();
        assert_eq!(output.len(), 4);
        assert_eq!(
            output[0],
            json!({"startDate":"7daysAgo","endDate":"yesterday","name":"Recent"})
        );
        assert_eq!(output[1]["name"], "January");
        assert_eq!(output[2]["startDate"], "today");
        assert_eq!(output[3]["startDate"], "2024-02-29");

        let mut too_many = ranges;
        too_many.push(json!({"startDate":"yesterday","endDate":"today"}));
        assert_eq!(
            report_request(
                Some(too_many),
                vec!["country".into()],
                vec!["activeUsers".into()],
                vec![],
            ),
            Err(ApiError::InvalidRequest)
        );

        for invalid in [
            json!({"startDate":"2026-03-02","endDate":"2026-03-01"}),
            json!({"startDate":"2026-02-30","endDate":"2026-03-01"}),
            json!({"startDate":"2026-03-01","start_date":"2026-03-01","endDate":"2026-03-02"}),
            json!({"startDate":"2026-03-01","endDate":"2026-03-02","name":""}),
        ] {
            assert_eq!(
                normalize_date_ranges(vec![invalid]),
                Err(ApiError::InvalidRequest)
            );
        }
    }

    #[test]
    fn access_reports_use_admin_field_names_and_reject_invalid_ranges() {
        let request = access_report_request(AccessReportSpec {
            date_ranges: vec![json!({"start_date":"2026-10-20","end_date":"2026-10-25"})],
            dimensions: vec!["userEmail".into()],
            metrics: vec!["accessCount".into()],
            dimension_filter: None,
            metric_filter: None,
            order_bys: None,
            limit: Some(200),
            offset: Some(0),
            time_zone: Some("America/Mexico_City".into()),
        })
        .unwrap();
        assert_eq!(request["dateRanges"][0]["startDate"], "2026-10-20");
        assert_eq!(request["dimensions"][0]["dimensionName"], "userEmail");
        assert_eq!(request["metrics"][0]["metricName"], "accessCount");
        assert_eq!(request["limit"], "200");
        assert!(access_report_request(AccessReportSpec {
            date_ranges: vec![],
            dimensions: vec![],
            metrics: vec!["accessCount".into()],
            dimension_filter: None,
            metric_filter: None,
            order_bys: None,
            limit: None,
            offset: None,
            time_zone: None,
        })
        .is_err());
        assert!(access_report_request(AccessReportSpec {
            date_ranges: vec![json!({})],
            dimensions: vec![],
            metrics: vec!["accessCount".into()],
            dimension_filter: None,
            metric_filter: None,
            order_bys: None,
            limit: None,
            offset: None,
            time_zone: None,
        })
        .is_err());
        assert!(access_report_request(AccessReportSpec {
            date_ranges: vec![json!({"startDate":"today","endDate":"today"})],
            dimensions: vec![],
            metrics: vec!["accessCount".into()],
            dimension_filter: None,
            metric_filter: None,
            order_bys: None,
            limit: None,
            offset: None,
            time_zone: Some("bad zone".into()),
        })
        .is_err());
    }

    #[test]
    fn annotation_validates_dates_colors_and_lengths() {
        let single = annotation_payload(" Release ", "2026-10-25", None, None, "BLUE").unwrap();
        assert_eq!(
            single["reportingDataAnnotation"]["annotationDate"]["month"],
            10
        );
        assert_eq!(single["reportingDataAnnotation"]["title"], "Release");
        assert!(single["reportingDataAnnotation"]
            .get("annotationDateRange")
            .is_none());
        let range = annotation_payload(
            "Release",
            "2026-10-25",
            Some("2026-10-27"),
            Some("v1"),
            "CYAN",
        )
        .unwrap();
        assert_eq!(
            range["reportingDataAnnotation"]["annotationDateRange"]["endDate"]["day"],
            27
        );
        for (start, end, color) in [
            ("2026-02-30", None, "BLUE"),
            ("2026-10-25", Some("2026-10-24"), "BLUE"),
            ("2026-10-25", None, "ORANGE"),
            ("2026-10-25", None, "COLOR_UNSPECIFIED"),
        ] {
            assert_eq!(
                annotation_payload("Release", start, end, None, color).unwrap_err(),
                ApiError::InvalidAnnotation
            );
        }
        assert!(annotation_payload(&"a".repeat(61), "2026-10-25", None, None, "BLUE").is_err());
        assert!(
            annotation_payload("x", "2026-10-25", None, Some(&"d".repeat(151)), "BLUE").is_err()
        );
    }

    #[tokio::test]
    async fn annotation_post_targets_admin_v1alpha_without_retry() {
        let (url, handle) = once(r#"{"name":"properties/42/reportingDataAnnotations/1"}"#).await;
        let c = GoogleClient::with_endpoints(Some("test-token".into()), &url, &url);
        let body = annotation_payload("Release", "2026-10-25", None, None, "BLUE").unwrap();
        assert_eq!(
            c.create_annotation("42", body).await.unwrap()["name"],
            "properties/42/reportingDataAnnotations/1"
        );
        let request = handle.await.unwrap();
        assert!(
            request.starts_with("POST /v1alpha/properties/42/reportingDataAnnotations HTTP/1.1")
        );
    }

    #[test]
    fn rejects_property_path_and_zero() {
        for bad in [
            "",
            "0",
            "001",
            "-1",
            "../x",
            "properties/2/ads",
            "https://example.org",
            "123?x=1",
            " 21",
            "18446744073709551616",
        ] {
            assert!(property_name(bad).is_err(), "{bad}");
        }
        assert_eq!(property_name("321").unwrap(), "properties/321");
        assert_eq!(property_name("properties/321").unwrap(), "properties/321");
        assert_eq!(account_name("123").unwrap(), "accounts/123");
        assert_eq!(account_name("accounts/123").unwrap(), "accounts/123");
        assert!(account_name("properties/5").is_err());
    }

    #[test]
    fn report_response_validator_rejects_inconsistent_provider_row_shapes() {
        assert!(validate_google_report_response(
            &json!({"rowCount":2,"rows":[{"metricValues":[{"value":"7"}]}]})
        )
        .is_ok());
        assert!(validate_google_report_response(&json!({"rowCount":"2","rows":[]})).is_ok());
        for invalid in [
            json!(null),
            json!({"rowCount":-1}),
            json!({"rowCount":0,"rows":[{}]}),
            json!({"dimensionHeaders":"wrong"}),
            json!({"metricHeaders":[17]}),
            json!({"rows":{}}),
            json!({"rows":[null]}),
            json!({"rows":[{"metricValues":[7]}]}),
            json!({"rows":[{"dimensionValues":"not an array"}]}),
        ] {
            assert_eq!(
                validate_google_report_response(&invalid),
                Err(ApiError::InvalidResponse)
            );
        }
    }

    #[test]
    fn compatibility_requests_allow_inventory_or_metric_only_without_report_dates() {
        let inventory = compatibility_request(vec![], vec![]).unwrap();
        assert_eq!(inventory, json!({"dimensions":[],"metrics":[]}));
        assert!(inventory.get("dateRanges").is_none());

        let dimensions = compatibility_request(vec!["country".into()], vec![]).unwrap();
        assert_eq!(dimensions["dimensions"][0]["name"], "country");
        let metric = compatibility_request(vec![], vec!["activeUsers".into()]).unwrap();
        assert_eq!(metric["metrics"][0]["name"], "activeUsers");

        assert_eq!(
            compatibility_request(vec!["bad\nname".into()], vec![]),
            Err(ApiError::InvalidRequest)
        );
        assert_eq!(
            compatibility_request(vec![], vec!["activeUsers".into(); 11]),
            Err(ApiError::InvalidRequest)
        );
    }

    #[test]
    fn historical_report_request_rejects_oversized_options_and_deep_filters() {
        // This check must run *before* network access, including offline previews.
        let huge_order_bys = (0..5000)
            .map(|index| json!({"dimension":{"dimensionName":format!("field{index}")}}))
            .collect::<Vec<_>>();
        let large = report_request(
            Some(vec![json!({"startDate":"yesterday","endDate":"today"})]),
            vec!["country".into()],
            vec!["activeUsers".into()],
            vec![("orderBys", Some(json!(huge_order_bys)))],
        );
        assert_eq!(large, Err(ApiError::InvalidRequest));

        let mut nested = json!({"filter":{"fieldName":"country","stringFilter":{"value":"US"}}});
        for _ in 0..16 {
            nested = json!({"notExpression":nested});
        }
        let rejected = report_request(
            Some(vec![json!({"startDate":"yesterday","endDate":"today"})]),
            vec!["country".into()],
            vec!["activeUsers".into()],
            vec![("dimensionFilter", Some(nested))],
        );
        assert_eq!(rejected, Err(ApiError::InvalidRequest));

        let too_wide = json!({"orGroup":{"expressions":vec![json!({"filter":{"fieldName":"country","stringFilter":{"value":"US"}}});65]}});
        assert_eq!(
            validate_bounded_filter(&too_wide),
            Err(ApiError::InvalidRequest)
        );
    }

    #[test]
    fn report_inputs_are_typed_and_bounded() {
        let req = report_request(
            Some(vec![json!({"startDate":"7daysAgo","endDate":"today"})]),
            vec!["country".into()],
            vec!["activeUsers".into()],
            vec![("limit", Some(json!("10")))],
        )
        .unwrap();
        assert_eq!(req["dimensions"][0]["name"], "country");
        assert_eq!(req["metrics"][0]["name"], "activeUsers");
        assert!(report_request(None, vec![], vec![], vec![]).is_err());
        assert!(report_request(Some(vec![]), vec![], vec!["activeUsers".into()], vec![]).is_err());
    }

    async fn once(response: &'static str) -> (String, tokio::task::JoinHandle<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut data = vec![0u8; 8192];
            let size = socket.read(&mut data).await.unwrap();
            let received = String::from_utf8_lossy(&data[..size]).into_owned();
            let reply = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",response.len(),response);
            socket.write_all(reply.as_bytes()).await.unwrap();
            received
        });
        (format!("http://{addr}"), handle)
    }

    #[tokio::test]
    async fn collection_page_size_and_explicit_page_cap_are_bounded() {
        let (url, handle) =
            once(r#"{"accountSummaries":[{"name":"accounts/1"}],"nextPageToken":"next-page"}"#)
                .await;
        let client = GoogleClient::with_endpoints(Some("mock".into()), &url, &url);
        let result = client
            .account_summaries_with_options(Some(25), Some(1))
            .await
            .unwrap();
        assert_eq!(result["accountSummaries"][0]["name"], "accounts/1");
        assert_eq!(result["nextPageToken"], "next-page");
        assert_eq!(result["truncated"], true);
        let request = handle.await.unwrap();
        assert!(request.starts_with("GET /v1beta/accountSummaries?pageSize=25 HTTP/1.1"));
        assert!(client
            .account_summaries_with_options(Some(0), Some(1))
            .await
            .is_err());
        assert!(client
            .account_summaries_with_options(Some(201), Some(1))
            .await
            .is_err());
        assert!(client
            .account_summaries_with_options(Some(25), Some(101))
            .await
            .is_err());
    }

    #[tokio::test]
    async fn account_pagination_collects_pages_and_rejects_replayed_tokens() {
        async fn serve(pages: Vec<&'static str>) -> (String, tokio::task::JoinHandle<Vec<String>>) {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let address = listener.local_addr().unwrap();
            let handle = tokio::spawn(async move {
                let mut requests = Vec::new();
                for response in pages {
                    let (mut socket, _) = listener.accept().await.unwrap();
                    let mut raw = [0u8; 8192];
                    let count = socket.read(&mut raw).await.unwrap();
                    requests.push(String::from_utf8_lossy(&raw[..count]).into_owned());
                    let reply = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        response.len(), response
                    );
                    socket.write_all(reply.as_bytes()).await.unwrap();
                }
                requests
            });
            (format!("http://{address}"), handle)
        }
        let (url, handle) = serve(vec![
            r#"{"accountSummaries":[{"name":"accounts/1"}],"nextPageToken":"cursor2"}"#,
            r#"{"accountSummaries":[{"name":"accounts/2"}]}"#,
        ])
        .await;
        let client = GoogleClient::with_endpoints(Some("mock".into()), &url, &url);
        let response = client.account_summaries().await.unwrap();
        assert_eq!(response["accountSummaries"].as_array().unwrap().len(), 2);
        let requests = handle.await.unwrap();
        assert!(requests[0].starts_with("GET /v1beta/accountSummaries HTTP/1.1"));
        assert!(requests[1].starts_with("GET /v1beta/accountSummaries?pageToken=cursor2 HTTP/1.1"));

        let (url, handle) = serve(vec![
            r#"{"accountSummaries":[],"nextPageToken":"cycle"}"#,
            r#"{"accountSummaries":[],"nextPageToken":"cycle"}"#,
        ])
        .await;
        let client = GoogleClient::with_endpoints(Some("mock".into()), &url, &url);
        assert_eq!(
            client.account_summaries().await.unwrap_err(),
            ApiError::InvalidResponse
        );
        assert_eq!(handle.await.unwrap().len(), 2);

        let (url, handle) = serve(vec![
            r#"{"googleAdsLinks":[{"name":"properties/42/googleAdsLinks/1"}],"nextPageToken":"next"}"#,
            r#"{"googleAdsLinks":[{"name":"properties/42/googleAdsLinks/2"}]}"#,
        ]).await;
        let client = GoogleClient::with_endpoints(Some("mock".into()), &url, &url);
        assert_eq!(
            client.ads_links("42").await.unwrap()["googleAdsLinks"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
        let requests = handle.await.unwrap();
        assert!(requests[0].starts_with("GET /v1beta/properties/42/googleAdsLinks HTTP/1.1"));
        assert!(requests[1].contains("pageToken=next"));
    }

    #[tokio::test]
    async fn account_collection_rejects_unbounded_item_count() {
        let payload =
            json!({"accountSummaries":vec![json!({"name":"accounts/1"});10_001]}).to_string();
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buf = [0u8; 4096];
            let received = socket.read(&mut buf).await.unwrap();
            assert!(received > 0);
            let reply = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                payload.len(), payload
            );
            socket.write_all(reply.as_bytes()).await.unwrap();
        });
        let url = format!("http://{address}");
        let client = GoogleClient::with_endpoints(Some("mock".into()), &url, &url);
        assert_eq!(
            client.account_summaries().await.unwrap_err(),
            ApiError::CollectionTooLarge
        );
        handle.await.unwrap();
    }

    #[tokio::test]
    async fn account_summaries_use_scoped_bearer_and_safe_host() {
        let (url, handle) = once(r#"{"accountSummaries":[{"name":"accounts/1"}]}"#).await;
        let client = GoogleClient::with_endpoints(Some("test-token".into()), &url, &url);
        let response = client.account_summaries().await.unwrap();
        assert_eq!(response["accountSummaries"][0]["name"], "accounts/1");
        let request = handle.await.unwrap();
        assert!(request.starts_with("GET /v1beta/accountSummaries HTTP/1.1"));
        assert!(request
            .to_lowercase()
            .contains("authorization: bearer test-token"));
        assert!(!format!("{:?}", ApiError::Http(403)).contains("test-token"));
    }

    #[tokio::test]
    async fn report_posts_to_correct_data_api_endpoint() {
        let (url, handle) = once(r#"{"rowCount":1}"#).await;
        let client = GoogleClient::with_endpoints(Some("token".into()), &url, &url);
        let response = client
            .report(
                "properties/42",
                json!({"metrics":[{"name":"activeUsers"}]}),
                false,
            )
            .await
            .unwrap();
        assert_eq!(response["rowCount"], 1);
        let request = handle.await.unwrap();
        assert!(request.starts_with("POST /v1beta/properties/42:runReport HTTP/1.1"));
    }

    #[tokio::test]
    async fn missing_credentials_reject_before_http() {
        let client = GoogleClient::new(None);
        assert_eq!(
            client.account_summaries().await.unwrap_err(),
            ApiError::MissingCredentials
        );
    }

    #[tokio::test]
    async fn account_sharing_and_retention_use_correct_paths() {
        let (url, handle) = once(r#"{"sharingWithOthersEnabled":false}"#).await;
        let c = GoogleClient::with_endpoints(Some("token".into()), &url, &url);
        let value = c.account_sharing("accounts/87").await.unwrap();
        assert_eq!(value["sharingWithOthersEnabled"], false);
        assert!(handle
            .await
            .unwrap()
            .starts_with("GET /v1beta/accounts/87/dataSharingSettings HTTP/1.1"));

        let (url, handle) = once(r#"{"eventDataRetention":"FOURTEEN_MONTHS"}"#).await;
        let c = GoogleClient::with_endpoints(Some("token".into()), &url, &url);
        let value = c.retention_settings("42").await.unwrap();
        assert_eq!(value["eventDataRetention"], "FOURTEEN_MONTHS");
        assert!(handle
            .await
            .unwrap()
            .starts_with("GET /v1beta/properties/42/dataRetentionSettings HTTP/1.1"));
    }

    #[tokio::test]
    async fn advanced_reports_use_read_only_api_paths() {
        for method in ["runPivotReport", "checkCompatibility", "batchRunReports"] {
            let response = match method {
                "runPivotReport" => r#"{"rowCount":0}"#,
                "checkCompatibility" => {
                    r#"{"dimensionCompatibilities":[],"metricCompatibilities":[]}"#
                }
                "batchRunReports" => r#"{"reports":[{"rowCount":0}]}"#,
                _ => unreachable!(),
            };
            let (url, handle) = once(response).await;
            let c = GoogleClient::with_endpoints(Some("token".into()), &url, &url);
            let body = if method == "batchRunReports" {
                json!({"requests":[{"metrics":[{"name":"activeUsers"}]}]})
            } else {
                json!({"metrics":[{"name":"activeUsers"}]})
            };
            let parsed = c
                .data_operation("properties/77", body, method)
                .await
                .unwrap();
            match method {
                "runPivotReport" => assert_eq!(parsed["rowCount"], 0),
                "checkCompatibility" => assert!(parsed["metricCompatibilities"].is_array()),
                "batchRunReports" => assert_eq!(parsed["reports"][0]["rowCount"], 0),
                _ => unreachable!(),
            }
            assert!(handle
                .await
                .unwrap()
                .starts_with(&format!("POST /v1beta/properties/77:{method} HTTP/1.1")));
        }
    }

    #[tokio::test]
    async fn batch_reports_reject_missing_or_extra_provider_results() {
        for payload in [
            r#"{\"reports\":[]}"#,
            r#"{\"reports\":[{\"rowCount\":0},{\"rowCount\":0}]}"#,
            r#"{\"reports\":[{\"rowCount\":-1}]}"#,
            r#"{\"reports\":\"invalid\"}"#,
        ] {
            let (url, handle) = once(payload).await;
            let client = GoogleClient::with_endpoints(Some("fixture-token".into()), &url, &url);
            let request = json!({
                "requests": [{
                    "metrics": [{"name":"activeUsers"}],
                    "dateRanges": [{"startDate":"yesterday","endDate":"today"}]
                }]
            });
            assert_eq!(
                client
                    .data_operation("42", request, "batchRunReports")
                    .await,
                Err(ApiError::InvalidResponse)
            );
            assert!(handle
                .await
                .unwrap()
                .contains("properties/42:batchRunReports"));
        }
    }

    #[tokio::test]
    async fn access_reports_are_scoped_to_account_or_property() {
        for (id, account) in [("accounts/16", true), ("properties/82", false)] {
            let (url, handle) = once(r#"{"rowCount":0}"#).await;
            let c = GoogleClient::with_endpoints(Some("token".into()), &url, &url);
            assert_eq!(
                c.access_report(id, json!({"dimensions":[]}), account)
                    .await
                    .unwrap()["rowCount"],
                0
            );
            assert!(handle
                .await
                .unwrap()
                .starts_with(&format!("POST /v1beta/{id}:runAccessReport HTTP/1.1")));
        }
    }

    #[tokio::test]
    async fn alpha_conversions_and_funnel_use_expected_methods() {
        for (funnel, method, request) in [
            (
                true,
                "runFunnelReport",
                json!({"funnel":{"steps":[{"name":"Home","filterExpression":{}}]}}),
            ),
            (
                false,
                "runReport",
                json!({"conversionSpec":{"attributionModel":"LAST_CLICK"}}),
            ),
        ] {
            let (url, handle) = once(r#"{"rowCount":0}"#).await;
            let c = GoogleClient::with_endpoints(Some("token".into()), &url, &url);
            assert_eq!(
                c.alpha_report("42", request, funnel).await.unwrap()["rowCount"],
                0
            );
            assert!(handle
                .await
                .unwrap()
                .starts_with(&format!("POST /v1alpha/properties/42:{method} HTTP/1.1")));
        }
    }

    #[tokio::test]
    async fn rejects_invalid_funnel_and_missing_conversion_spec() {
        let c = GoogleClient::new(Some("token".into()));
        for request in [
            json!({}),
            json!({"funnel":{"steps":[]}}),
            json!({"funnel":{"steps":vec![json!({});11]}}),
        ] {
            assert_eq!(
                c.alpha_report("42", request, true).await.unwrap_err(),
                ApiError::InvalidRequest
            );
        }
        assert_eq!(
            c.alpha_report("42", json!({"metrics":[{"name":"activeUsers"}]}), false)
                .await
                .unwrap_err(),
            ApiError::InvalidRequest
        );
    }

    #[tokio::test]
    async fn rejects_invalid_batch_without_http_request() {
        let c = GoogleClient::new(Some("token".into()));
        for bad in [
            json!({}),
            json!({"requests":[]}),
            json!({"requests":[{},{},{},{},{},{}]}),
        ] {
            assert_eq!(
                c.data_operation("99", bad, "batchRunReports")
                    .await
                    .unwrap_err(),
                ApiError::InvalidRequest
            );
        }
        assert_eq!(
            c.data_operation("99", json!({}), "notARealMethod")
                .await
                .unwrap_err(),
            ApiError::InvalidRequest
        );
    }
}
