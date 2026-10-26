//! Bounded and least-privilege Google Analytics HTTP client.
//! Credentials are never included in logs or response error strings.

use chrono::{Datelike, NaiveDate};
use futures_util::StreamExt;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, HashSet},
    fmt,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

const ADMIN: &str = "https://analyticsadmin.googleapis.com";
const DATA: &str = "https://analyticsdata.googleapis.com";
const MAX_RESPONSE_BYTES: usize = 4 * 1024 * 1024;
const MAX_COLLECTION_BYTES: usize = 16 * 1024 * 1024;
const MAX_COLLECTION_ITEMS: usize = 10_000;
const MAX_PAGE_TOKEN_BYTES: usize = 1024;
const MAX_PAGES: usize = 100;
const COMPATIBILITY_CACHE_TTL: Duration = Duration::from_secs(30);
const MAX_COMPATIBILITY_CACHE_ENTRIES: usize = 64;
const MAX_COMPATIBILITY_CACHE_RESPONSE_BYTES: usize = 128 * 1024;
type CompatibilityCache = HashMap<[u8; 32], (Instant, Value)>;

#[derive(Clone)]
pub struct GoogleClient {
    http: reqwest::Client,
    bearer: Option<String>,
    quota_project: Result<Option<String>, ApiError>,
    admin_base: String,
    data_base: String,
    compatibility_cache: Arc<Mutex<CompatibilityCache>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    MissingCredentials,
    InvalidIdentifier,
    InvalidRequest,
    CursorQueryMismatch,
    UnsupportedAuthOption,
    InvalidQuotaProject,
    UnsupportedRealtimeOffset,
    RequestFailed,
    Http(u16),
    ResponseTooLarge,
    InvalidResponse,
    TooManyPages,
    CollectionTooLarge,
    AuthenticationFailed,
    InvalidAnnotation,
    EditScopeRequired,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCredentials => write!(f, "No Google access token configured. Run synmetra auth login --client-id-file PATH or set SYNMETRA_GOOGLE_ACCESS_TOKEN to a scoped, short-lived token."),
            Self::InvalidIdentifier => write!(f, "Invalid Google Analytics property identifier: expected positive decimal digits or properties/<digits>."),
            Self::InvalidRequest => write!(f, "Invalid Google Analytics request: report fields must be a JSON object."),
            Self::CursorQueryMismatch => write!(f, "Report cursor belongs to another query or Google Analytics resource; restart pagination with the current property and filters."),
            Self::UnsupportedAuthOption => write!(f, "Requested OAuth command option is not supported: shared gcloud ADC or per-command quota-project selection are not enabled."),
            Self::InvalidQuotaProject => write!(f, "SYNMETRA_GOOGLE_QUOTA_PROJECT must name a valid Google Cloud project ID or number; no Google request was made."),
            Self::UnsupportedRealtimeOffset => write!(f, "Realtime report offset is unavailable in the current Google Analytics v1beta request. Use ordering and limit, or a historical report for offset pagination."),
            Self::RequestFailed => write!(f, "Google Analytics request failed (network or timeout)."),
            Self::Http(status) => match status {
                401 => write!(
                    f,
                    "Google Analytics HTTP 401: credentials were rejected or expired. Check synmetra auth status/doctor, then reauthorize with the account owner's consent if needed."
                ),
                403 => write!(
                    f,
                    "Google Analytics HTTP 403: verify both Google Cloud API enablement and the Google Analytics account/property role and requested OAuth scope; Synmetra will not silently escalate permissions."
                ),
                429 => write!(
                    f,
                    "Google Analytics HTTP 429: rate or quota limit exceeded. Reduce request volume or check the project's Analytics API quotas; read-only retries are bounded."
                ),
                500..=599 => write!(
                    f,
                    "Google Analytics HTTP {status}: upstream service unavailable. Read-only retries are bounded, and annotation mutations are never automatically replayed."
                ),
                _ => write!(
                    f,
                    "Google Analytics HTTP {status}: check request validation, enabled APIs and resource access."
                ),
            },
            Self::ResponseTooLarge => write!(f, "Google Analytics response exceeded the 4 MiB safety limit."),
            Self::InvalidResponse => write!(f, "Google Analytics response was not valid JSON or had an invalid pagination contract."),
            Self::TooManyPages => write!(f, "Google Analytics pagination exceeded the 100-page safety limit."),
            Self::CollectionTooLarge => write!(f, "Google Analytics collection exceeded the 10,000-item or 16 MiB safety limit."),
            Self::AuthenticationFailed => write!(f, "Synmetra OAuth credentials could not be loaded or refreshed. Run synmetra auth status, then login again if necessary."),
            Self::InvalidAnnotation => write!(f, "Invalid annotation: check title, dates, description and allowed color."),
            Self::EditScopeRequired => write!(f, "Annotation writes require a locally verified Google analytics.edit OAuth scope. Reauthorize via synmetra auth login --client-id-file PATH --allow-edit. Unverifiable external bearer overrides are read-only for mutations; Google property editor permission is separately required."),
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

/// Bounded automatic retry is restricted to read-only Google methods.
/// If a provider's Retry-After exceeds the five-second retry budget, return
/// the original HTTP error instead of violating the requested backoff.
fn readonly_retry_delay(
    status: reqwest::StatusCode,
    headers: &reqwest::header::HeaderMap,
    attempt: usize,
) -> Option<Duration> {
    if !matches!(status.as_u16(), 429 | 500 | 502 | 503 | 504) {
        return None;
    }
    const MAX_WAIT_MS: u64 = 5_000;
    let minimum_ms = if let Some(raw) = headers.get(reqwest::header::RETRY_AFTER) {
        let seconds = raw.to_str().ok()?.parse::<u64>().ok()?;
        let delay = seconds.checked_mul(1_000)?;
        if delay > MAX_WAIT_MS {
            return None;
        }
        delay
    } else {
        200_u64.checked_mul(1_u64.checked_shl(attempt as u32)?)?
    };
    let jitter_ms = (rand::random::<u8>() % 101) as u64;
    Some(Duration::from_millis(
        (minimum_ms + jitter_ms).min(MAX_WAIT_MS),
    ))
}

/// Quota attribution is explicitly configured by the local operator, not
/// chosen from any untrusted MCP tool argument or upstream response.
fn valid_quota_project(value: &str) -> bool {
    if value.bytes().all(|byte| byte.is_ascii_digit()) {
        return (6..=20).contains(&value.len());
    }
    let bytes = value.as_bytes();
    (6..=30).contains(&bytes.len())
        && bytes[0].is_ascii_lowercase()
        && bytes[bytes.len() - 1].is_ascii_alphanumeric()
        && bytes
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
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
            quota_project: std::env::var_os("SYNMETRA_GOOGLE_QUOTA_PROJECT")
                .map(|value| {
                    value
                        .into_string()
                        .map_err(|_| ApiError::InvalidQuotaProject)
                })
                .transpose(),
            admin_base: admin.to_owned(),
            data_base: data.to_owned(),
            compatibility_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn validated_quota_project(&self) -> Result<Option<&str>, ApiError> {
        let quota = self
            .quota_project
            .as_ref()
            .map_err(Clone::clone)?
            .as_deref();
        if quota.is_some_and(|project| !valid_quota_project(project)) {
            return Err(ApiError::InvalidQuotaProject);
        }
        Ok(quota)
    }

    fn with_quota_header(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::RequestBuilder, ApiError> {
        if let Some(project) = self.validated_quota_project()? {
            return Ok(request.header("x-goog-user-project", project));
        }
        Ok(request)
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

    /// Fail closed *before* sending a Google mutation. Only the private native
    /// OAuth store carries scope provenance that this process can verify.
    /// Bearer overrides have no locally attested scope and remain read-only.
    /// A matching scope never implies a sufficient Google property role.
    pub(crate) fn ensure_annotation_edit_scope(&self) -> Result<(), ApiError> {
        if self.bearer.is_some() {
            return Err(ApiError::EditScopeRequired);
        }
        match crate::auth::native_edit_scope_granted() {
            Ok(Some(true)) => Ok(()),
            Ok(Some(false)) => Err(ApiError::EditScopeRequired),
            Ok(None) => Err(ApiError::MissingCredentials),
            Err(_) => Err(ApiError::AuthenticationFailed),
        }
    }

    async fn send(&self, req: reqwest::RequestBuilder) -> Result<Value, ApiError> {
        // Single attempt only. This path is reserved for mutations; never replay a
        // Google annotation write after a timeout, 429, or ambiguous 5xx response.
        let req = self.with_quota_header(req)?;
        let bearer = self.token().await?;
        let response = req
            .bearer_auth(&bearer)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await
            .map_err(|_| ApiError::RequestFailed)?;
        Self::decode_response(response).await
    }

    async fn send_readonly(&self, req: reqwest::RequestBuilder) -> Result<Value, ApiError> {
        // Fail closed before refreshing OAuth; attach the header exactly once
        // in send_readonly_as, which also handles controlled retries.
        let _ = self.validated_quota_project()?;
        let bearer = self.token().await?;
        self.send_readonly_as(req, &bearer).await
    }

    async fn send_readonly_as(
        &self,
        req: reqwest::RequestBuilder,
        bearer: &str,
    ) -> Result<Value, ApiError> {
        let mut req = self.with_quota_header(req)?;
        for attempt in 0..3 {
            // A request carrying a streaming or unclonable body is not retried.
            let replay = req.try_clone();
            let response = req
                .bearer_auth(bearer)
                .header(reqwest::header::ACCEPT, "application/json")
                .send()
                .await
                .map_err(|_| ApiError::RequestFailed)?;
            let status = response.status();
            if attempt < 2 {
                if let (Some(next), Some(delay)) = (
                    replay,
                    readonly_retry_delay(status, response.headers(), attempt),
                ) {
                    tokio::time::sleep(delay).await;
                    req = next;
                    continue;
                }
            }
            return Self::decode_response(response).await;
        }
        Err(ApiError::RequestFailed)
    }

    async fn decode_response(response: reqwest::Response) -> Result<Value, ApiError> {
        let status = response.status();
        if !status.is_success() {
            // Never reflect response body, endpoint URL, or request headers.
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
        self.send_readonly(req).await
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
                self.send_readonly(req).await?
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
        if method == "checkCompatibility" {
            // A cached compatibility result is caller-specific and property-specific.
            // The same bearer is used to authorize the actual Google request and
            // construct its cache key, so refresh races cannot cross identities.
            let bearer = self.token().await?;
            let bytes = serde_json::to_vec(&request).map_err(|_| ApiError::InvalidRequest)?;
            let mut digest = Sha256::new();
            for part in [
                b"synmetra-compat-v1".as_slice(),
                self.data_base.as_bytes(),
                property.as_bytes(),
                bearer.as_bytes(),
                bytes.as_slice(),
            ] {
                digest.update((part.len() as u64).to_be_bytes());
                digest.update(part);
            }
            let key: [u8; 32] = digest.finalize().into();
            {
                let cache = self.compatibility_cache.lock().await;
                if let Some((created, cached)) = cache.get(&key) {
                    if created.elapsed() <= COMPATIBILITY_CACHE_TTL {
                        return Ok(cached.clone());
                    }
                }
            }
            let response = self
                .send_readonly_as(self.http.post(url).json(&request), &bearer)
                .await?;
            // Only bounded, successful, structurally plausible metadata is cached.
            // Quota snapshots and failed authorization are never cached.
            let is_compatible_payload = response.is_object()
                && (response["metricCompatibilities"].is_array()
                    || response["dimensionCompatibilities"].is_array());
            if is_compatible_payload
                && response.get("propertyQuota").is_none()
                && serde_json::to_vec(&response)
                    .is_ok_and(|body| body.len() <= MAX_COMPATIBILITY_CACHE_RESPONSE_BYTES)
            {
                let mut cache = self.compatibility_cache.lock().await;
                cache.retain(|_, (created, _)| created.elapsed() <= COMPATIBILITY_CACHE_TTL);
                if cache.len() >= MAX_COMPATIBILITY_CACHE_ENTRIES {
                    if let Some(oldest) = cache
                        .iter()
                        .min_by_key(|(_, (created, _))| *created)
                        .map(|(key, _)| *key)
                    {
                        cache.remove(&oldest);
                    }
                }
                cache.insert(key, (Instant::now(), response.clone()));
            }
            return Ok(response);
        }
        let response = self
            .send_readonly(self.http.post(url).json(&request))
            .await?;
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
        self.send_readonly(self.http.post(url).json(&request)).await
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
        self.send_readonly(self.http.post(url).json(&request)).await
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
        if object.keys().any(|field| {
            !matches!(
                field.as_str(),
                "event"
                    | "filter_expression"
                    | "filterExpression"
                    | "name"
                    | "is_directly_followed_by"
                    | "isDirectlyFollowedBy"
                    | "within_duration_from_prior_step"
                    | "withinDurationFromPriorStep"
            )
        }) || (object.contains_key("filter_expression")
            && object.contains_key("filterExpression"))
            || (object.contains_key("is_directly_followed_by")
                && object.contains_key("isDirectlyFollowedBy"))
            || (object.contains_key("within_duration_from_prior_step")
                && object.contains_key("withinDurationFromPriorStep"))
        {
            return Err(ApiError::InvalidRequest);
        }
        // The pinned schema permits null for optional event and filter fields.
        let event = object.get("event").filter(|value| !value.is_null());
        let filter = object
            .get("filter_expression")
            .or_else(|| object.get("filterExpression"))
            .filter(|value| !value.is_null());
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
        let mut serialized = json!({"name":name,"filterExpression":expression});
        if let Some(follows) = object
            .get("is_directly_followed_by")
            .or_else(|| object.get("isDirectlyFollowedBy"))
        {
            let followed = follows.as_bool().ok_or(ApiError::InvalidRequest)?;
            if index == 0 && followed {
                return Err(ApiError::InvalidRequest);
            }
            serialized["isDirectlyFollowedBy"] = json!(followed);
        }
        if let Some(duration) = object
            .get("within_duration_from_prior_step")
            .or_else(|| object.get("withinDurationFromPriorStep"))
        {
            if index == 0 {
                return Err(ApiError::InvalidRequest);
            }
            let duration = duration.as_str().ok_or(ApiError::InvalidRequest)?;
            let numeric = duration.strip_suffix('s').ok_or(ApiError::InvalidRequest)?;
            let parts = numeric.split('.').collect::<Vec<_>>();
            if parts.is_empty()
                || parts.len() > 2
                || parts[0].is_empty()
                || parts[0].parse::<u64>().is_err()
                || (parts.len() == 2
                    && (parts[1].is_empty()
                        || parts[1].len() > 9
                        || !parts[1].bytes().all(|byte| byte.is_ascii_digit())))
            {
                return Err(ApiError::InvalidRequest);
            }
            serialized["withinDurationFromPriorStep"] = json!(duration);
        }
        mapped.push(serialized);
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
        let details = breakdown.as_object().ok_or(ApiError::InvalidRequest)?;
        if details.keys().any(|field| {
            !matches!(
                field.as_str(),
                "breakdown_dimension" | "breakdownDimension" | "limit"
            )
        }) || (details.contains_key("breakdown_dimension")
            && details.contains_key("breakdownDimension"))
        {
            return Err(ApiError::InvalidRequest);
        }
        let field = breakdown
            .get("breakdown_dimension")
            .or_else(|| breakdown.get("breakdownDimension"))
            .and_then(Value::as_str)
            .ok_or(ApiError::InvalidRequest)?;
        if field.trim().is_empty() {
            return Err(ApiError::InvalidRequest);
        }
        let mut spec = json!({"breakdownDimension":{"name":field}});
        if let Some(limit) = breakdown.get("limit") {
            let limit = limit.as_u64().ok_or(ApiError::InvalidRequest)?;
            if limit == 0 || limit > 15 {
                return Err(ApiError::InvalidRequest);
            }
            spec["limit"] = json!(limit.to_string());
        }
        request["funnelBreakdown"] = spec;
    }
    if let Some(next_action) = next_action {
        let details = next_action.as_object().ok_or(ApiError::InvalidRequest)?;
        if details.keys().any(|field| {
            !matches!(
                field.as_str(),
                "next_action_dimension" | "nextActionDimension" | "limit"
            )
        }) || (details.contains_key("next_action_dimension")
            && details.contains_key("nextActionDimension"))
        {
            return Err(ApiError::InvalidRequest);
        }
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
            if amount == 0 || amount > 5 {
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

/// Attach official v1alpha funnel controls after step/segment normalization.
pub fn funnel_request_options(
    mut request: Value,
    is_open_funnel: bool,
    visualization: Option<&str>,
    limit: Option<u64>,
    dimension_filter: Option<Value>,
) -> Result<Value, ApiError> {
    if is_open_funnel {
        request["funnel"]["isOpenFunnel"] = json!(true);
    }
    if let Some(kind) = visualization {
        if ![
            "STANDARD_FUNNEL",
            "TRENDED_FUNNEL",
            "FUNNEL_VISUALIZATION_TYPE_UNSPECIFIED",
        ]
        .contains(&kind)
        {
            return Err(ApiError::InvalidRequest);
        }
        request["funnelVisualizationType"] = json!(kind);
    }
    if let Some(limit) = limit {
        if limit == 0 || limit > 250_000 {
            return Err(ApiError::InvalidRequest);
        }
        request["limit"] = json!(limit.to_string());
    }
    if let Some(filter) = dimension_filter {
        request["dimensionFilter"] = normalize_report_filter(filter, false)?;
    }
    GoogleClient::validate_body(&request)?;
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
    pub limit: Option<u64>,
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

/// Derive a conservative historical/Core report preflight verdict from the
/// provider's exact compatibility metadata without replacing that metadata.
/// Compatibility is unknown if the provider omits a requested field or returns
/// COMPATIBILITY_UNSPECIFIED. Realtime has independent compatibility rules.
pub fn compatibility_verdict(
    requested_dimensions: &[String],
    requested_metrics: &[String],
    provider: &Value,
) -> Result<Value, ApiError> {
    let object = provider.as_object().ok_or(ApiError::InvalidResponse)?;
    if !object.contains_key("dimensionCompatibilities")
        && !object.contains_key("metricCompatibilities")
    {
        return Err(ApiError::InvalidResponse);
    }
    let mut incompatible = Vec::new();
    let mut unchecked = Vec::new();
    for (kind, requested, key, metadata_key) in [
        (
            "dimension",
            requested_dimensions,
            "dimensionCompatibilities",
            "dimensionMetadata",
        ),
        (
            "metric",
            requested_metrics,
            "metricCompatibilities",
            "metricMetadata",
        ),
    ] {
        let entries = match object.get(key) {
            Some(value) => value.as_array().ok_or(ApiError::InvalidResponse)?,
            None => {
                for name in requested {
                    unchecked.push(json!({"kind":kind,"name":name}));
                }
                continue;
            }
        };
        let mut statuses = HashMap::new();
        for entry in entries {
            let name = entry[metadata_key]["apiName"]
                .as_str()
                .filter(|name| !name.is_empty())
                .ok_or(ApiError::InvalidResponse)?;
            let status = entry["compatibility"]
                .as_str()
                .ok_or(ApiError::InvalidResponse)?;
            if !["COMPATIBLE", "INCOMPATIBLE", "COMPATIBILITY_UNSPECIFIED"].contains(&status)
                || statuses.insert(name, status).is_some()
            {
                return Err(ApiError::InvalidResponse);
            }
        }
        for name in requested {
            match statuses.get(name.as_str()).copied() {
                Some("COMPATIBLE") => {}
                Some("INCOMPATIBLE") => {
                    incompatible.push(json!({"kind":kind,"name":name}));
                }
                _ => unchecked.push(json!({"kind":kind,"name":name})),
            }
        }
    }
    let inventory = requested_dimensions.is_empty() && requested_metrics.is_empty();
    let safe_to_run = !inventory && incompatible.is_empty() && unchecked.is_empty();
    Ok(json!({
        "scope": "historical_core_reports",
        "inventory_only": inventory,
        "complete": !inventory && unchecked.is_empty(),
        "safe_to_run": safe_to_run,
        "status": if inventory {
            "inventory"
        } else if !incompatible.is_empty() {
            "incompatible"
        } else if !unchecked.is_empty() {
            "incomplete"
        } else {
            "compatible"
        },
        "incompatible_fields": incompatible,
        "unchecked_fields": unchecked
    }))
}

/// Prepare a compatibility-only probe without inventing a report request. Google
/// accepts property 0 to check universally available dimensions and metrics.
pub fn compatibility_request(
    dimensions: Vec<String>,
    metrics: Vec<String>,
) -> Result<Value, ApiError> {
    if dimensions.len() > 9
        || metrics.len() > 10
        || dimensions.iter().collect::<HashSet<_>>().len() != dimensions.len()
        || metrics.iter().collect::<HashSet<_>>().len() != metrics.len()
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
/// Normalize Google v1beta Pivot objects without accepting unbounded pivot
/// cardinality. The product of pivot limits must be at most 250,000.
pub fn normalize_pivots(
    pivots: Vec<Value>,
    dimensions: &[String],
    metrics: &[String],
) -> Result<Vec<Value>, ApiError> {
    if pivots.is_empty() || pivots.len() > 4 {
        return Err(ApiError::InvalidRequest);
    }
    let mut cardinality = 1_u64;
    let mut result = Vec::with_capacity(pivots.len());
    for pivot in pivots {
        let mut object = pivot.as_object().ok_or(ApiError::InvalidRequest)?.clone();
        let fields = object
            .get("fieldNames")
            .and_then(Value::as_array)
            .ok_or(ApiError::InvalidRequest)?;
        if fields.is_empty() || fields.len() > 9 {
            return Err(ApiError::InvalidRequest);
        }
        let mut seen = HashSet::new();
        for field in fields {
            let field = field.as_str().ok_or(ApiError::InvalidRequest)?;
            if !seen.insert(field.to_owned())
                || (field != "dateRange" && !dimensions.iter().any(|name| name == field))
            {
                return Err(ApiError::InvalidRequest);
            }
        }
        let number = |value: &Value, allow_zero: bool| -> Result<u64, ApiError> {
            let integer = if let Some(number) = value.as_u64() {
                number
            } else if let Some(raw) = value.as_str() {
                if raw.is_empty() || !raw.bytes().all(|byte| byte.is_ascii_digit()) {
                    return Err(ApiError::InvalidRequest);
                }
                raw.parse::<u64>().map_err(|_| ApiError::InvalidRequest)?
            } else {
                return Err(ApiError::InvalidRequest);
            };
            if !allow_zero && integer == 0 {
                return Err(ApiError::InvalidRequest);
            }
            Ok(integer)
        };
        let limit = number(object.get("limit").ok_or(ApiError::InvalidRequest)?, false)?;
        cardinality = cardinality
            .checked_mul(limit)
            .ok_or(ApiError::InvalidRequest)?;
        if cardinality > 250_000 {
            return Err(ApiError::InvalidRequest);
        }
        object.insert("limit".to_owned(), json!(limit.to_string()));
        if let Some(offset) = object.get("offset") {
            let offset = number(offset, true)?;
            if offset > 10_000_000 {
                return Err(ApiError::InvalidRequest);
            }
            object.insert("offset".to_owned(), json!(offset.to_string()));
        }
        if let Some(order_bys) = object.get("orderBys") {
            let order_bys = order_bys.as_array().ok_or(ApiError::InvalidRequest)?;
            if order_bys.len() > 10 {
                return Err(ApiError::InvalidRequest);
            }
            for order in order_bys {
                let order = order.as_object().ok_or(ApiError::InvalidRequest)?;
                let selectors = ["dimension", "metric", "pivot"]
                    .iter()
                    .filter(|selector| order.contains_key(**selector))
                    .count();
                if selectors != 1 {
                    return Err(ApiError::InvalidRequest);
                }
                if let Some(dimension) = order.get("dimension") {
                    let field = dimension
                        .get("dimensionName")
                        .and_then(Value::as_str)
                        .ok_or(ApiError::InvalidRequest)?;
                    if !seen.contains(field) {
                        return Err(ApiError::InvalidRequest);
                    }
                }
                if let Some(metric) = order.get("metric") {
                    let field = metric
                        .get("metricName")
                        .and_then(Value::as_str)
                        .ok_or(ApiError::InvalidRequest)?;
                    if !metrics.iter().any(|name| name == field) {
                        return Err(ApiError::InvalidRequest);
                    }
                }
            }
        }
        result.push(Value::Object(object));
    }
    Ok(result)
}

pub fn report_request(
    dates: Option<Vec<Value>>,
    dimensions: Vec<String>,
    metrics: Vec<String>,
    extras: Vec<(&str, Option<Value>)>,
) -> Result<Value, ApiError> {
    if metrics.is_empty()
        || metrics.len() > 10
        || dimensions.len() > 9
        || metrics.iter().chain(dimensions.iter()).any(|name| {
            name.is_empty()
                || name.len() > 128
                || name.trim() != name
                || name.chars().any(char::is_control)
        })
        || metrics.iter().collect::<HashSet<_>>().len() != metrics.len()
        || dimensions.iter().collect::<HashSet<_>>().len() != dimensions.len()
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
    fn quota_project_ids_and_numbers_are_strictly_validated() {
        for valid in ["quota-project-123", "example2", "123456789012"] {
            assert!(valid_quota_project(valid), "Valid project fixture: {valid}");
        }
        for invalid in [
            "",
            "short",
            "-leading-hyphen",
            "trailing-hyphen-",
            "MixedCase",
            "quota-project\r\nX-Extra: injected",
            "quota project",
            "quota/project",
            "01234",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ] {
            assert!(
                !valid_quota_project(invalid),
                "Invalid quota project passed"
            );
        }
    }

    #[tokio::test]
    async fn opt_in_quota_header_is_kept_separate_from_google_bearer_on_reads_and_writes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let host = format!("http://{}", listener.local_addr().unwrap());
        let capture = tokio::spawn(async move {
            let mut observed = Vec::new();
            for (method, path, body) in [
                (
                    "GET",
                    "/v1beta/accountSummaries",
                    r#"{"accountSummaries":[]}"#,
                ),
                (
                    "POST",
                    "/v1alpha/properties/42/reportingDataAnnotations",
                    r#"{"name":"properties/42/reportingDataAnnotations/9"}"#,
                ),
            ] {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut buffer = [0u8; 8192];
                let length = stream.read(&mut buffer).await.unwrap();
                let request = String::from_utf8_lossy(&buffer[..length]).to_string();
                let headers = request.to_ascii_lowercase();
                assert!(request.starts_with(&format!("{method} {path} HTTP/1.1")));
                assert!(headers.contains("x-goog-user-project: quota-project-123"));
                assert_eq!(headers.matches("x-goog-user-project:").count(), 1);
                assert!(headers.contains("authorization: bearer synthetic-google-token"));
                assert!(!headers.contains("synthetic-inbound-mcp-token"));
                let reply = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(reply.as_bytes()).await.unwrap();
                observed.push(request);
            }
            observed
        });
        let mut client =
            GoogleClient::with_endpoints(Some("synthetic-google-token".into()), &host, &host);
        client.quota_project = Ok(Some("quota-project-123".into()));
        assert_eq!(
            client.account_summaries().await.unwrap()["accountSummaries"],
            json!([])
        );
        assert_eq!(
            client
                .create_annotation("42", json!({"reportingDataAnnotation":{"title":"fixture"}}))
                .await
                .unwrap()["name"],
            "properties/42/reportingDataAnnotations/9"
        );
        assert_eq!(capture.await.unwrap().len(), 2);

        let mut invalid = GoogleClient::with_endpoints(
            Some("synthetic-google-token".into()),
            "http://127.0.0.1:9",
            "http://127.0.0.1:9",
        );
        invalid.quota_project = Ok(Some("bad\nheader".into()));
        assert_eq!(
            invalid.account_summaries().await,
            Err(ApiError::InvalidQuotaProject)
        );
    }

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
    fn funnel_nullable_optionals_and_unknown_fields_have_explicit_semantics() {
        let request = funnel_request(
            vec![
                json!({"name":null,"event":"page_view","filter_expression":null}),
                json!({"event":null,"filter_expression":{"funnelEventFilter":{"eventName":"purchase"}}}),
            ],
            None,
            None,
            None,
            None,
            false,
        )
        .unwrap();
        assert_eq!(request["funnel"]["steps"][0]["name"], "Step 1");
        assert_eq!(
            request["funnel"]["steps"][1]["filterExpression"]["funnelEventFilter"]["eventName"],
            "purchase"
        );
        for invalid_step in [
            json!({"event":"page_view","limit":50}),
            json!({"event":"page_view","filterExpression":null,"filter_expression":null}),
            json!({"event":"page_view","isDirectlyFollowedBy":false,"is_directly_followed_by":false}),
            json!({"event":null,"filter_expression":null}),
        ] {
            assert_eq!(
                funnel_request(vec![invalid_step], None, None, None, None, false),
                Err(ApiError::InvalidRequest)
            );
        }
        for invalid in [
            json!({"breakdown_dimension":"country","limt":3}),
            json!({"breakdown_dimension":"country","breakdownDimension":"deviceCategory"}),
        ] {
            assert_eq!(
                funnel_request(
                    vec![json!({"event":"page_view"})],
                    None,
                    Some(invalid),
                    None,
                    None,
                    false
                ),
                Err(ApiError::InvalidRequest)
            );
        }
        for invalid in [
            json!({"next_action_dimension":"eventName","unknown":true}),
            json!({"next_action_dimension":"eventName","nextActionDimension":"pagePath"}),
        ] {
            assert_eq!(
                funnel_request(
                    vec![json!({"event":"page_view"})],
                    None,
                    None,
                    Some(invalid),
                    None,
                    false
                ),
                Err(ApiError::InvalidRequest)
            );
        }
    }

    #[test]
    fn complex_funnel_steps_segments_breakdown_and_trended_options_round_trip() {
        let segments = vec![
            json!({"name":"Purchasers","userSegment":{"userInclusionCriteria":{}}}),
            json!({"name":"All sessions","sessionSegment":{"sessionInclusionCriteria":{}}}),
        ];
        let body = funnel_request(
            vec![
                json!({"name":"Browse","event":"page_view"}),
                json!({
                    "name":"Add to cart",
                    "filter_expression":{"funnelEventFilter":{"eventName":"add_to_cart"}},
                    "is_directly_followed_by":true,
                    "within_duration_from_prior_step":"3600.5s"
                }),
                json!({
                    "name":"Purchase",
                    "event":"purchase",
                    "within_duration_from_prior_step":"86400s"
                }),
            ],
            Some(vec![json!({"start_date":"30daysAgo","end_date":"today"})]),
            Some(json!({"breakdown_dimension":"deviceCategory","limit":15})),
            Some(json!({"next_action_dimension":"eventName","limit":5})),
            Some(segments.clone()),
            true,
        )
        .unwrap();
        let final_request = funnel_request_options(
            body,
            true,
            Some("TRENDED_FUNNEL"),
            Some(150),
            Some(json!("country==MX")),
        )
        .unwrap();
        assert_eq!(
            final_request["funnel"]["steps"].as_array().unwrap().len(),
            3
        );
        assert_eq!(
            final_request["funnel"]["steps"][1]["filterExpression"]["funnelEventFilter"]
                ["eventName"],
            "add_to_cart"
        );
        assert_eq!(
            final_request["funnel"]["steps"][1]["isDirectlyFollowedBy"],
            true
        );
        assert_eq!(
            final_request["funnel"]["steps"][1]["withinDurationFromPriorStep"],
            "3600.5s"
        );
        assert_eq!(
            final_request["funnel"]["steps"][2]["withinDurationFromPriorStep"],
            "86400s"
        );
        assert_eq!(final_request["funnel"]["isOpenFunnel"], true);
        assert_eq!(final_request["funnelVisualizationType"], "TRENDED_FUNNEL");
        assert_eq!(final_request["funnelBreakdown"]["limit"], "15");
        assert_eq!(final_request["funnelNextAction"]["limit"], "5");
        assert_eq!(final_request["segments"], json!(segments));
        assert_eq!(final_request["returnPropertyQuota"], true);
        assert_eq!(final_request["limit"], "150");
        assert_eq!(
            final_request["dimensionFilter"]["filter"]["fieldName"],
            "country"
        );

        for bad in [
            json!({"event":"page_view","is_directly_followed_by":true}),
            json!({"event":"page_view","within_duration_from_prior_step":"60s"}),
        ] {
            assert_eq!(
                funnel_request(vec![bad], None, None, None, None, false),
                Err(ApiError::InvalidRequest)
            );
        }
        assert!(funnel_request(
            vec![json!({"event":"page_view"})],
            None,
            Some(json!({"breakdown_dimension":"deviceCategory","limit":16})),
            None,
            None,
            false,
        )
        .is_err());
        assert!(funnel_request(
            vec![json!({"event":"page_view"})],
            None,
            None,
            Some(json!({"next_action_dimension":"eventName","limit":6})),
            None,
            false,
        )
        .is_err());
        assert!(funnel_request(
            vec![json!({"event":"page_view"})],
            None,
            None,
            None,
            Some(vec![json!({"name":"segment"}); 5]),
            false,
        )
        .is_err());
    }

    #[test]
    fn multi_axis_pivot_ordering_and_cardinality_are_bounded() {
        let dims = vec!["country".into(), "browser".into()];
        let metrics = vec!["sessions".into()];
        let pivots = vec![
            json!({
                "fieldNames":["country"],
                "limit":500,
                "offset":0,
                "orderBys":[{"dimension":{"dimensionName":"country"},"desc":true}]
            }),
            json!({
                "fieldNames":["browser"],
                "limit":"500",
                "offset":"10",
                "orderBys":[{"metric":{"metricName":"sessions"},"desc":true}]
            }),
        ];
        let normalized = normalize_pivots(pivots.clone(), &dims, &metrics).unwrap();
        assert_eq!(normalized[0]["limit"], "500");
        assert_eq!(normalized[0]["offset"], "0");
        assert_eq!(normalized[1]["limit"], "500");
        assert_eq!(normalized[1]["offset"], "10");
        assert_eq!(
            normalized[0]["orderBys"][0]["dimension"]["dimensionName"],
            "country"
        );
        assert_eq!(
            normalized[1]["orderBys"][0]["metric"]["metricName"],
            "sessions"
        );
        let mut too_many = pivots;
        too_many[1]["limit"] = json!(501);
        assert_eq!(
            normalize_pivots(too_many, &dims, &metrics),
            Err(ApiError::InvalidRequest)
        );
        for invalid in [
            json!({"fieldNames":["country"]}),
            json!({"fieldNames":["country"],"limit":0}),
            json!({"fieldNames":["notRequested"],"limit":2}),
            json!({"fieldNames":["country","country"],"limit":2}),
            json!({"fieldNames":["country"],"limit":2,"offset":-1}),
            json!({"fieldNames":["country"],"limit":2,"offset":"1.5"}),
            json!({"fieldNames":["country"],"limit":2,
                   "orderBys":[{"dimension":{"dimensionName":"browser"}}]}),
            json!({"fieldNames":["country"],"limit":2,
                   "orderBys":[{"metric":{"metricName":"notRequested"}}]}),
            json!({"fieldNames":["country"],"limit":2,
                   "orderBys":[{"metric":{},"dimension":{}}]}),
        ] {
            assert_eq!(
                normalize_pivots(vec![invalid], &dims, &metrics),
                Err(ApiError::InvalidRequest)
            );
        }
    }

    #[test]
    fn report_currency_request_remains_exact_and_numeric_filter_is_not_rounded() {
        let request = report_request(
            Some(vec![json!({"start_date":"yesterday","end_date":"today"})]),
            vec!["country".into()],
            vec!["purchaseRevenue".into()],
            vec![
                ("currencyCode", Some(json!("MXN"))),
                ("metricFilter", Some(json!("purchaseRevenue==0.000000001"))),
            ],
        )
        .unwrap();
        assert_eq!(request["currencyCode"], "MXN");
        assert_eq!(request["metrics"][0]["name"], "purchaseRevenue");
        assert_eq!(
            request["metricFilter"]["filter"]["numericFilter"]["value"]["doubleValue"],
            0.000000001
        );
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
    fn normalized_report_body_preserves_valid_inputs_and_rejects_ambiguous_values() {
        let nested = json!({
            "andGroup": {
                "expressions": [
                    {"filter":{"fieldName":"country","stringFilter":{"matchType":"EXACT","value":"MX"}}},
                    {"notExpression":{"filter":{"fieldName":"deviceCategory","stringFilter":{"value":"tablet"}}}}
                ]
            }
        });
        let ordering = json!([{"dimension":{"dimensionName":"country"},"desc":true}]);
        let normalized = report_request(
            Some(vec![
                json!({"start_date":"7daysAgo","end_date":"yesterday","name":"Week"}),
                json!({"startDate":"2024-02-29","endDate":"2024-03-01"}),
            ]),
            vec!["country".into()],
            vec!["sessions".into()],
            vec![
                ("dimensionFilter", Some(nested.clone())),
                ("metricFilter", Some(json!("sessions==9223372036854775807"))),
                ("orderBys", Some(ordering.clone())),
                ("limit", Some(json!("250000"))),
                ("offset", Some(json!("0"))),
            ],
        )
        .unwrap();
        assert_eq!(normalized["dateRanges"][0]["startDate"], "7daysAgo");
        assert_eq!(normalized["dateRanges"][0]["name"], "Week");
        assert_eq!(normalized["dateRanges"][1]["startDate"], "2024-02-29");
        assert_eq!(normalized["dimensions"], json!([{"name":"country"}]));
        assert_eq!(normalized["metrics"], json!([{"name":"sessions"}]));
        assert_eq!(normalized["dimensionFilter"], nested);
        assert_eq!(normalized["orderBys"], ordering);
        assert_eq!(
            normalized["metricFilter"]["filter"]["numericFilter"]["value"]["int64Value"],
            "9223372036854775807"
        );
        assert_eq!(normalized["limit"], "250000");
        assert_eq!(normalized["offset"], "0");

        for invalid in [
            json!("sessions==9223372036854775808"),
            json!("sessions==NaN"),
            json!("sessions==1e309"),
            json!(true),
            Value::Null,
        ] {
            assert_eq!(
                report_request(
                    Some(vec![json!({"startDate":"today","endDate":"today"})]),
                    vec![],
                    vec!["sessions".into()],
                    vec![("metricFilter", Some(invalid))]
                ),
                Err(ApiError::InvalidRequest)
            );
        }
        assert_eq!(
            report_request(
                Some(vec![json!({
                    "startDate":"today",
                    "start_date":"today",
                    "endDate":"today"
                })]),
                vec![],
                vec!["sessions".into()],
                vec![]
            ),
            Err(ApiError::InvalidRequest)
        );
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
        assert_eq!(request["timeZone"], "America/Mexico_City");
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
    fn readonly_retry_policy_honors_backoff_limits_and_never_retries_forbidden() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::RETRY_AFTER, "1".parse().unwrap());
        assert!(readonly_retry_delay(reqwest::StatusCode::FORBIDDEN, &headers, 0).is_none());
        assert!(readonly_retry_delay(reqwest::StatusCode::BAD_REQUEST, &headers, 0).is_none());
        let delayed =
            readonly_retry_delay(reqwest::StatusCode::TOO_MANY_REQUESTS, &headers, 0).unwrap();
        assert!(delayed >= Duration::from_secs(1));
        assert!(delayed <= Duration::from_millis(1100));
        headers.insert(reqwest::header::RETRY_AFTER, "60".parse().unwrap());
        assert!(
            readonly_retry_delay(reqwest::StatusCode::TOO_MANY_REQUESTS, &headers, 0).is_none()
        );
        headers.insert(reqwest::header::RETRY_AFTER, "invalid".parse().unwrap());
        assert!(
            readonly_retry_delay(reqwest::StatusCode::SERVICE_UNAVAILABLE, &headers, 0).is_none()
        );
        headers.clear();
        let delayed =
            readonly_retry_delay(reqwest::StatusCode::SERVICE_UNAVAILABLE, &headers, 1).unwrap();
        assert!(delayed >= Duration::from_millis(400));
        assert!(delayed <= Duration::from_millis(500));
    }

    #[tokio::test(start_paused = true)]
    async fn readonly_429_retry_schedule_respects_virtual_clock_and_attempt_ceiling() {
        use std::sync::{Arc, Mutex};
        use tokio::time::{advance, Instant};

        // The same policy used by send_readonly_as, but with a virtual Tokio
        // clock. No real sleeps, Google account, external request, or secret.
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::RETRY_AFTER, "2".parse().unwrap());
        let status = reqwest::StatusCode::TOO_MANY_REQUESTS;
        let delays = [
            readonly_retry_delay(status, &headers, 0).unwrap(),
            readonly_retry_delay(status, &headers, 1).unwrap(),
        ];
        for delay in delays {
            assert!(delay >= Duration::from_secs(2));
            assert!(delay <= Duration::from_millis(2100));
        }
        let events = Arc::new(Mutex::new(Vec::new()));
        let recorded = events.clone();
        let began = Instant::now();
        let worker = tokio::spawn(async move {
            for (attempt, delay) in delays.into_iter().enumerate() {
                recorded.lock().unwrap().push((attempt, Instant::now()));
                tokio::time::sleep(delay).await;
            }
            // Production caps read-only sends at 3 total attempts, even
            // when Google keeps returning a retryable response.
            recorded.lock().unwrap().push((2, Instant::now()));
        });

        tokio::task::yield_now().await;
        assert_eq!(events.lock().unwrap().len(), 1);
        advance(delays[0] - Duration::from_millis(1)).await;
        assert_eq!(events.lock().unwrap().len(), 1);
        advance(Duration::from_millis(1)).await;
        tokio::task::yield_now().await;
        assert_eq!(events.lock().unwrap().len(), 2);
        advance(delays[1] - Duration::from_millis(1)).await;
        assert_eq!(events.lock().unwrap().len(), 2);
        advance(Duration::from_millis(1)).await;
        worker.await.unwrap();
        let events = events.lock().unwrap();
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].0, 0);
        assert_eq!(events[1].0, 1);
        assert_eq!(events[2].0, 2);
        assert_eq!(events[1].1.duration_since(began), delays[0]);
        assert_eq!(events[2].1.duration_since(began), delays[0] + delays[1]);
        assert!(readonly_retry_delay(reqwest::StatusCode::FORBIDDEN, &headers, 0).is_none());
    }

    #[tokio::test]
    async fn readonly_report_retry_is_bounded_and_never_replays_writes() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let responder = tokio::spawn(async move {
            for index in 0..2 {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut buffer = [0_u8; 4096];
                let n = socket.read(&mut buffer).await.unwrap();
                let request = String::from_utf8_lossy(&buffer[..n]);
                assert!(request.contains("properties/42:runReport"));
                let (status, body) = if index == 0 {
                    (
                        "429 Too Many Requests",
                        r#"{"error":{"message":"secret-fixture"}}"#,
                    )
                } else {
                    ("200 OK", r#"{"rowCount":0}"#)
                };
                let reply = format!(
                    "HTTP/1.1 {status}\r\nRetry-After: 0\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                socket.write_all(reply.as_bytes()).await.unwrap();
            }
        });
        let client = GoogleClient::with_endpoints(Some("private-test-token".into()), &url, &url);
        let body = report_request(
            Some(vec![json!({"startDate":"yesterday","endDate":"today"})]),
            vec![],
            vec!["activeUsers".to_owned()],
            vec![],
        )
        .unwrap();
        assert_eq!(
            client.report("42", body, false).await.unwrap()["rowCount"],
            0
        );
        responder.await.unwrap();
    }

    #[tokio::test]
    async fn readonly_429_exhausts_three_attempts_with_redacted_error() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            for _ in 0..3 {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut buffer = [0_u8; 4096];
                let n = socket.read(&mut buffer).await.unwrap();
                assert!(String::from_utf8_lossy(&buffer[..n])
                    .contains("POST /v1beta/properties/42:runReport HTTP/1.1"));
                let body = r#"{"error":{"message":"synthetic-private-upstream-canary"}}"#;
                let response = format!(
                    "HTTP/1.1 429 Too Many Requests\r\nRetry-After: 0\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                socket.write_all(response.as_bytes()).await.unwrap();
            }
        });
        let client =
            GoogleClient::with_endpoints(Some("synthetic-token".into()), &endpoint, &endpoint);
        let request = report_request(
            Some(vec![json!({"startDate":"yesterday","endDate":"today"})]),
            vec![],
            vec!["activeUsers".to_owned()],
            vec![],
        )
        .unwrap();
        assert_eq!(
            client.report("42", request, false).await,
            Err(ApiError::Http(429))
        );
        server.await.unwrap();
        assert!(!format!("{:?}", ApiError::Http(429)).contains("synthetic-private-upstream-canary"));
    }

    #[tokio::test]
    async fn readonly_socket_drop_fails_without_unsafe_transport_retries() {
        use tokio::io::AsyncReadExt;
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = [0_u8; 4096];
            let n = socket.read(&mut buffer).await.unwrap();
            assert!(String::from_utf8_lossy(&buffer[..n])
                .contains("POST /v1beta/properties/42:runReport HTTP/1.1"));
            drop(socket);
            assert!(
                tokio::time::timeout(Duration::from_millis(750), listener.accept())
                    .await
                    .is_err(),
                "A failed transport must not trigger an automatic retry"
            );
        });
        let client =
            GoogleClient::with_endpoints(Some("synthetic-token".into()), &endpoint, &endpoint);
        let request = report_request(
            Some(vec![json!({"startDate":"yesterday","endDate":"today"})]),
            vec![],
            vec!["activeUsers".to_owned()],
            vec![],
        )
        .unwrap();
        assert_eq!(
            client.report("42", request, false).await,
            Err(ApiError::RequestFailed)
        );
        server.await.unwrap();
    }

    #[tokio::test]
    async fn annotation_writes_do_not_retry_429_500_or_503_responses() {
        for (status, code) in [
            ("429 Too Many Requests", 429),
            ("500 Internal Server Error", 500),
            ("503 Service Unavailable", 503),
        ] {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let endpoint = format!("http://{}", listener.local_addr().unwrap());
            let responder = tokio::spawn(async move {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut buffer = [0_u8; 4096];
                let length = socket.read(&mut buffer).await.unwrap();
                let request = String::from_utf8_lossy(&buffer[..length]).into_owned();
                assert!(request
                    .starts_with("POST /v1alpha/properties/42/reportingDataAnnotations HTTP/1.1"));
                let body = r#"{"error":{"message":"private upstream detail"}}"#;
                let response = format!(
                    "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nRetry-After: 1\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                socket.write_all(response.as_bytes()).await.unwrap();
                request
            });
            let client = GoogleClient::with_endpoints(
                Some("private-annotation-token".into()),
                &endpoint,
                &endpoint,
            );
            let body = annotation_payload("Release", "2026-10-25", None, None, "BLUE").unwrap();
            assert_eq!(
                client.create_annotation("42", body).await,
                Err(ApiError::Http(code))
            );
            let request = responder.await.unwrap();
            assert!(!format!("{:?}", ApiError::Http(code)).contains("private"));
            assert!(
                request.contains("authorization: Bearer private-annotation-token")
                    || request
                        .to_ascii_lowercase()
                        .contains("authorization: bearer private-annotation-token")
            );
        }
    }

    #[tokio::test]
    async fn production_google_hosts_are_pinned_and_http_redirects_never_followed() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let production = GoogleClient::new(Some("fixture-token".into()));
        assert_eq!(
            production.admin_base,
            "https://analyticsadmin.googleapis.com"
        );
        assert_eq!(production.data_base, "https://analyticsdata.googleapis.com");
        for injected in [
            "https://attacker.example/properties/42",
            "42@attacker.example",
            "42%2f..%2f",
            "properties/42?next=https://attacker.example",
            "//attacker.example/42",
        ] {
            assert_eq!(property_name(injected), Err(ApiError::InvalidIdentifier));
            assert_eq!(account_name(injected), Err(ApiError::InvalidIdentifier));
        }

        // Test-only origin substitution exercises redirect handling without
        // contacting Google. Redirect policy must not forward OAuth credentials.
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let responder = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = [0_u8; 4096];
            let count = socket.read(&mut buffer).await.unwrap();
            assert!(String::from_utf8_lossy(&buffer[..count]).contains("properties/42:runReport"));
            let response = "HTTP/1.1 302 Found\r\nLocation: https://attacker.example/collect\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
            socket.write_all(response.as_bytes()).await.unwrap();
        });
        let client =
            GoogleClient::with_endpoints(Some("private-token".into()), &endpoint, &endpoint);
        let body = report_request(
            Some(vec![json!({"startDate":"yesterday","endDate":"today"})]),
            vec![],
            vec!["activeUsers".into()],
            vec![],
        )
        .unwrap();
        assert_eq!(
            client.report("42", body, false).await,
            Err(ApiError::Http(302))
        );
        responder.await.unwrap();
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
    fn funnel_steps_preserve_sequencing_durations_and_bounded_breakdown_limits() {
        let body = funnel_request(
            vec![
                json!({"name":"Viewed","event":"page_view"}),
                json!({
                    "name":"Purchased",
                    "event":"purchase",
                    "is_directly_followed_by":true,
                    "within_duration_from_prior_step":"3600.5s"
                }),
            ],
            None,
            Some(json!({"breakdown_dimension":"deviceCategory","limit":15})),
            Some(json!({"next_action_dimension":"eventName","limit":5})),
            None,
            false,
        )
        .unwrap();
        assert_eq!(body["funnel"]["steps"][1]["isDirectlyFollowedBy"], true);
        assert_eq!(
            body["funnel"]["steps"][1]["withinDurationFromPriorStep"],
            "3600.5s"
        );
        assert_eq!(body["funnelBreakdown"]["limit"], "15");
        assert_eq!(body["funnelNextAction"]["limit"], "5");

        for invalid in ["-2s", "3600", "1.1234567890s", "not-a-duration"] {
            assert!(funnel_request(
                vec![
                    json!({"event":"page_view"}),
                    json!({"event":"purchase","within_duration_from_prior_step":invalid}),
                ],
                None,
                None,
                None,
                None,
                false
            )
            .is_err());
        }
        for invalid in [0, 6, 1000] {
            assert!(funnel_request(
                vec![json!({"event":"purchase"})],
                None,
                None,
                Some(json!({"next_action_dimension":"eventName","limit":invalid})),
                None,
                false
            )
            .is_err());
        }
        for invalid in [0, 16] {
            assert!(funnel_request(
                vec![json!({"event":"purchase"})],
                None,
                Some(json!({"breakdown_dimension":"country","limit":invalid})),
                None,
                None,
                false
            )
            .is_err());
        }
    }

    #[test]
    fn funnel_options_map_open_trended_and_dimension_filters_with_bounds() {
        let base = funnel_request(
            vec![json!({"name":"Viewed","event":"page_view"})],
            None,
            None,
            None,
            None,
            false,
        )
        .unwrap();
        let request = funnel_request_options(
            base.clone(),
            true,
            Some("TRENDED_FUNNEL"),
            Some(150),
            Some(json!("country==US")),
        )
        .unwrap();
        assert_eq!(request["funnel"]["isOpenFunnel"], true);
        assert_eq!(request["funnelVisualizationType"], "TRENDED_FUNNEL");
        assert_eq!(request["limit"], "150");
        assert_eq!(request["dimensionFilter"]["filter"]["fieldName"], "country");
        for bad in [0, 250_001, u64::MAX] {
            assert_eq!(
                funnel_request_options(base.clone(), false, None, Some(bad), None),
                Err(ApiError::InvalidRequest)
            );
        }
        assert_eq!(
            funnel_request_options(base, false, Some("UNKNOWN"), None, None),
            Err(ApiError::InvalidRequest)
        );
    }

    #[test]
    fn compatibility_verdict_fails_closed_without_losing_provider_field_names() {
        let dimensions = vec!["country".to_owned()];
        let metrics = vec!["activeUsers".to_owned()];
        let incompatible = json!({
            "dimensionCompatibilities":[{
                "dimensionMetadata":{"apiName":"country"},
                "compatibility":"COMPATIBLE"
            }],
            "metricCompatibilities":[{
                "metricMetadata":{"apiName":"activeUsers"},
                "compatibility":"INCOMPATIBLE"
            }]
        });
        let verdict = compatibility_verdict(&dimensions, &metrics, &incompatible).unwrap();
        assert_eq!(verdict["status"], "incompatible");
        assert_eq!(verdict["safe_to_run"], false);
        assert_eq!(verdict["incompatible_fields"][0]["name"], "activeUsers");
        assert_eq!(verdict["incompatible_fields"][0]["kind"], "metric");

        let mut compatible = incompatible.clone();
        compatible["metricCompatibilities"][0]["compatibility"] = json!("COMPATIBLE");
        let verdict = compatibility_verdict(&dimensions, &metrics, &compatible).unwrap();
        assert_eq!(verdict["status"], "compatible");
        assert_eq!(verdict["safe_to_run"], true);
        assert_eq!(verdict["complete"], true);

        compatible["metricCompatibilities"] = json!([]);
        let verdict = compatibility_verdict(&dimensions, &metrics, &compatible).unwrap();
        assert_eq!(verdict["status"], "incomplete");
        assert_eq!(verdict["safe_to_run"], false);
        assert_eq!(verdict["unchecked_fields"][0]["name"], "activeUsers");

        let inventory = compatibility_verdict(
            &[],
            &[],
            &json!({"dimensionCompatibilities":[],"metricCompatibilities":[]}),
        )
        .unwrap();
        assert_eq!(inventory["status"], "inventory");
        assert_eq!(inventory["safe_to_run"], false);

        let duplicated = json!({
            "dimensionCompatibilities":[
                {"dimensionMetadata":{"apiName":"country"},"compatibility":"COMPATIBLE"},
                {"dimensionMetadata":{"apiName":"country"},"compatibility":"INCOMPATIBLE"}
            ]
        });
        assert_eq!(
            compatibility_verdict(&dimensions, &[], &duplicated),
            Err(ApiError::InvalidResponse)
        );
        assert_eq!(
            compatibility_verdict(&dimensions, &metrics, &json!({})),
            Err(ApiError::InvalidResponse)
        );
        assert_eq!(
            compatibility_request(vec!["country".into(), "country".into()], vec![]),
            Err(ApiError::InvalidRequest)
        );
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
    fn adversarial_report_filter_corpus_is_bounded_before_google_network_access() {
        // Deterministic corpus checks the exact depth, node-count, breadth,
        // byte-size, key-size and cell-size preflight boundaries.
        let mut deep = json!({"filter":{"fieldName":"country"}});
        for _ in 0..15 {
            deep = json!({"notExpression":deep});
        }
        let mut large_object = Map::new();
        large_object.insert("x".repeat(129), json!(1));
        let high_node_count = json!({
            "andGroup": {
                "expressions": (0..60).map(|index| {
                    json!({"filter":{
                        "fieldName":format!("country{index}"),
                        "stringFilter":{"value":"US"}
                    }})
                }).collect::<Vec<_>>()
            }
        });
        let corpus = vec![
            json!(null),
            json!(true),
            json!([]),
            json!("not a filter"),
            json!({}),
            json!({"filter":{}}),
            json!({"andGroup":{"expressions":vec![false;65]}}),
            json!({"longValue":"x".repeat(1025)}),
            json!({"bigJSON":"x".repeat(17_000)}),
            Value::Object(large_object),
            high_node_count,
            deep,
        ];
        for (index, payload) in corpus.into_iter().enumerate() {
            assert_eq!(
                validate_bounded_filter(&payload),
                Err(ApiError::InvalidRequest),
                "filter corpus item {index} was accepted"
            );
            let request = report_request(
                Some(vec![json!({"startDate":"yesterday","endDate":"today"})]),
                vec!["country".into()],
                vec!["activeUsers".into()],
                vec![("dimensionFilter", Some(payload))],
            );
            // A syntax-invalid object may still be Google-validatable; this
            // particular corpus must always hit a local resource boundary.
            assert_eq!(
                request,
                Err(ApiError::InvalidRequest),
                "network-boundary corpus item {index} was accepted"
            );
        }
    }

    #[test]
    fn seeded_json_and_filter_mutations_are_bounded_and_panic_free() {
        // Fixed seeds plus bounded input sizes permit exact CI reproduction.
        // Every filter exceeds the documented cell-length boundary; every
        // raw JSON specimen has trailing NUL and must fail before networking.
        fn next(state: &mut u64) -> u64 {
            *state ^= *state << 13;
            *state ^= *state >> 7;
            *state ^= *state << 17;
            *state
        }
        let alphabet = b"abcXYZ0123456789{}[]\"\\/:,%?<>\n";
        for seed in [
            0xC0DE_42F0_19A5_88E1u64,
            0x0000_0000_0001_CAFE,
            0x9A3D_75F1_B2C4_0E87,
        ] {
            let mut state = seed;
            for index in 0..128 {
                let count = (next(&mut state) % 129) as usize;
                let mut bytes = vec![b'{'];
                for _ in 0..count {
                    bytes.push(alphabet[(next(&mut state) as usize) % alphabet.len()]);
                }
                bytes.push(0);
                let tail = (0..16)
                    .map(|_| alphabet[(next(&mut state) as usize) % alphabet.len()] as char)
                    .collect::<String>();
                let filter = json!({
                    "filter": {
                        "fieldName": "country",
                        "stringFilter": {"value":format!("{}{tail}", "X".repeat(1025))}
                    }
                });
                let result = std::panic::catch_unwind(|| {
                    let parsed = serde_json::from_slice::<Value>(&bytes);
                    let bounded = validate_bounded_filter(&filter);
                    let prepared = report_request(
                        Some(vec![json!({"startDate":"yesterday","endDate":"today"})]),
                        vec!["country".into()],
                        vec!["activeUsers".into()],
                        vec![("dimensionFilter", Some(filter))],
                    );
                    (parsed, bounded, prepared)
                });
                assert!(
                    result.is_ok(),
                    "input caused panic: seed {seed:016x}, iteration {index}"
                );
                let (parsed, bounded, prepared) = result.unwrap();
                assert!(
                    parsed.is_err() && bounded.is_err() && prepared.is_err(),
                    "input escaped local preflight: seed {seed:016x}, iteration {index}"
                );
            }
        }
    }

    #[test]
    fn historical_report_rejects_ambiguous_or_malformed_field_names_before_google() {
        for (dimensions, metrics) in [
            (
                vec!["country".to_string(); 2],
                vec!["activeUsers".to_string()],
            ),
            (vec!["country".into()], vec!["activeUsers".to_string(); 2]),
            (vec!["country".into()], vec!["bad\nmetric".into()]),
            (vec!["country".into()], vec![" activeUsers".into()]),
            (vec!["country".into()], vec!["x".repeat(129)]),
        ] {
            assert_eq!(
                report_request(
                    Some(vec![json!({"startDate":"yesterday","endDate":"today"})]),
                    dimensions,
                    metrics,
                    vec![],
                ),
                Err(ApiError::InvalidRequest)
            );
        }
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

    #[test]
    fn google_http_errors_have_actionable_redacted_status_specific_guidance() {
        for (status, phrase) in [
            (401, "synmetra auth status/doctor"),
            (403, "account/property role"),
            (429, "rate or quota"),
            (500, "annotation mutations are never automatically replayed"),
            (503, "Read-only retries are bounded"),
            (400, "check request validation"),
        ] {
            let text = ApiError::Http(status).to_string();
            assert!(text.contains(&format!("HTTP {status}")));
            assert!(text.contains(phrase), "{status}: {text}");
            for canary in [
                "synthetic-private-token",
                "synthetic-private-property",
                "synthetic-personal-report",
            ] {
                assert!(!text.contains(canary));
            }
        }
        assert_ne!(
            ApiError::Http(401).to_string(),
            ApiError::Http(403).to_string()
        );
        assert_ne!(
            ApiError::Http(403).to_string(),
            ApiError::Http(429).to_string()
        );
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
    async fn compatibility_cache_is_isolated_by_property_and_credential_identity() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let responder = tokio::spawn(async move {
            for (index, expected_property, expected_token) in [
                (1, "42", "token-for-caller-a"),
                (2, "43", "token-for-caller-a"),
                (3, "42", "token-for-caller-b"),
            ] {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut buffer = [0_u8; 4096];
                let read = stream.read(&mut buffer).await.unwrap();
                let request = String::from_utf8_lossy(&buffer[..read]);
                assert!(request.contains(&format!(
                    "/v1beta/properties/{expected_property}:checkCompatibility"
                )));
                assert!(request.contains(&format!("Bearer {expected_token}")));
                let body = json!({
                    "metricCompatibilities":[{
                        "metricMetadata":{"apiName":format!("metric{index}")},
                        "compatibility":"COMPATIBLE"
                    }]
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
        let first =
            GoogleClient::with_endpoints(Some("token-for-caller-a".into()), &endpoint, &endpoint);
        let mut second =
            GoogleClient::with_endpoints(Some("token-for-caller-b".into()), &endpoint, &endpoint);
        // Simulate an intentionally shared cache across client instances; the
        // bearer fingerprint must prevent cross-caller result reuse.
        second.compatibility_cache = first.compatibility_cache.clone();
        let request = json!({
            "metrics":[{"name":"activeUsers"}],
            "dimensions":[{"name":"country"}]
        });
        let one = first
            .data_operation("42", request.clone(), "checkCompatibility")
            .await
            .unwrap();
        let cached = first
            .data_operation("42", request.clone(), "checkCompatibility")
            .await
            .unwrap();
        assert_eq!(cached, one);
        let other_property = first
            .data_operation("43", request.clone(), "checkCompatibility")
            .await
            .unwrap();
        let other_caller = second
            .data_operation("42", request, "checkCompatibility")
            .await
            .unwrap();
        responder.await.unwrap();
        assert_eq!(
            one["metricCompatibilities"][0]["metricMetadata"]["apiName"],
            "metric1"
        );
        assert_eq!(
            other_property["metricCompatibilities"][0]["metricMetadata"]["apiName"],
            "metric2"
        );
        assert_eq!(
            other_caller["metricCompatibilities"][0]["metricMetadata"]["apiName"],
            "metric3"
        );
        assert_eq!(first.compatibility_cache.lock().await.len(), 3);
    }

    #[tokio::test]
    async fn compatibility_cache_expires_and_does_not_cache_403_denials() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let responder = tokio::spawn(async move {
            for (status, label) in [
                ("200 OK", "initial"),
                ("200 OK", "refreshed"),
                ("403 Forbidden", "private-error-body"),
                ("200 OK", "recovered"),
            ] {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut buffer = [0_u8; 4096];
                let read = stream.read(&mut buffer).await.unwrap();
                assert!(String::from_utf8_lossy(&buffer[..read])
                    .contains("properties/42:checkCompatibility"));
                let body = json!({
                    "metricCompatibilities":[{
                        "metricMetadata":{"apiName":label},
                        "compatibility":"COMPATIBLE"
                    }]
                })
                .to_string();
                let reply = format!(
                    "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(reply.as_bytes()).await.unwrap();
            }
        });
        let client = GoogleClient::with_endpoints(
            Some("private-cache-fixture-token".into()),
            &endpoint,
            &endpoint,
        );
        let request = json!({"metrics":[{"name":"activeUsers"}],"dimensions":[]});
        let first = client
            .data_operation("42", request.clone(), "checkCompatibility")
            .await
            .unwrap();
        let cached = client
            .data_operation("42", request.clone(), "checkCompatibility")
            .await
            .unwrap();
        assert_eq!(first, cached);
        {
            let mut cache = client.compatibility_cache.lock().await;
            for (created, _) in cache.values_mut() {
                *created = Instant::now() - Duration::from_secs(31);
            }
        }
        let refreshed = client
            .data_operation("42", request.clone(), "checkCompatibility")
            .await
            .unwrap();
        assert_eq!(
            refreshed["metricCompatibilities"][0]["metricMetadata"]["apiName"],
            "refreshed"
        );
        {
            let mut cache = client.compatibility_cache.lock().await;
            for (created, _) in cache.values_mut() {
                *created = Instant::now() - Duration::from_secs(31);
            }
        }
        assert_eq!(
            client
                .data_operation("42", request.clone(), "checkCompatibility")
                .await,
            Err(ApiError::Http(403))
        );
        let recovered = client
            .data_operation("42", request, "checkCompatibility")
            .await
            .unwrap();
        responder.await.unwrap();
        assert_eq!(
            recovered["metricCompatibilities"][0]["metricMetadata"]["apiName"],
            "recovered"
        );
        assert_eq!(client.compatibility_cache.lock().await.len(), 1);
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
    async fn admin_beta_and_alpha_routes_retain_versioned_payloads() {
        // Admin calls must use the Admin host even when the Data host differs.
        for (kind, path, payload, key) in [
            (
                "property",
                "/v1beta/properties/42",
                r#"{"name":"properties/42","timeZone":"America/Mexico_City"}"#,
                "name",
            ),
            (
                "retention",
                "/v1beta/properties/42/dataRetentionSettings",
                r#"{"eventDataRetention":"FOURTEEN_MONTHS","resetUserDataOnNewActivity":false}"#,
                "eventDataRetention",
            ),
            (
                "account_sharing",
                "/v1beta/accounts/77/dataSharingSettings",
                r#"{"name":"accounts/77/dataSharingSettings","sharingWithGoogleSupportEnabled":false}"#,
                "name",
            ),
            (
                "ads_links",
                "/v1beta/properties/42/googleAdsLinks",
                r#"{"googleAdsLinks":[{"name":"properties/42/googleAdsLinks/7"}]}"#,
                "googleAdsLinks",
            ),
            (
                "annotations",
                "/v1alpha/properties/42/reportingDataAnnotations",
                r#"{"reportingDataAnnotations":[{"name":"properties/42/reportingDataAnnotations/8","color":"BLUE","annotationDate":{"year":2026,"month":10,"day":25}}]}"#,
                "reportingDataAnnotations",
            ),
        ] {
            let (admin_host, handle) = once(payload).await;
            let client = GoogleClient::with_endpoints(
                Some("private-fixture-token".into()),
                &admin_host,
                "http://127.0.0.1:9",
            );
            let data = match kind {
                "property" => client.property("42").await,
                "retention" => client.retention_settings("42").await,
                "account_sharing" => client.account_sharing("77").await,
                "ads_links" => client.ads_links_with_options("42", None, None).await,
                "annotations" => client.annotations_with_options("42", None, None).await,
                _ => unreachable!(),
            }
            .unwrap();
            assert!(data.get(key).is_some(), "Missing {key} for {kind}");
            let request = handle.await.unwrap();
            assert!(
                request.starts_with(&format!("GET {path} HTTP/1.1")),
                "Wrong API version or path for {kind}"
            );
        }
    }

    #[tokio::test]
    async fn annotation_pages_preserve_all_color_enums_and_inclusive_date_targets() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let admin_host = format!("http://{}", listener.local_addr().unwrap());
        let colors = [
            "COLOR_UNSPECIFIED",
            "PURPLE",
            "BROWN",
            "BLUE",
            "GREEN",
            "RED",
            "CYAN",
            "ORANGE",
        ];
        let responder = tokio::spawn(async move {
            for page in 0..2 {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut bytes = [0_u8; 4096];
                let count = socket.read(&mut bytes).await.unwrap();
                let request = String::from_utf8_lossy(&bytes[..count]);
                assert!(request.starts_with("GET /v1alpha/properties/42/reportingDataAnnotations?"));
                assert!(request.contains("pageSize=4"));
                if page == 0 {
                    assert!(!request.contains("pageToken="));
                } else {
                    assert!(request.contains("pageToken=second"));
                }
                let annotations = colors[page * 4..page * 4 + 4]
                    .iter()
                    .enumerate()
                    .map(|(n, color)| {
                        let mut entry = json!({
                            "name":format!(
                                "properties/42/reportingDataAnnotations/{}",
                                page * 4 + n + 1
                            ),
                            "title":"fixture",
                            "color":color,
                            "systemGenerated":*color == "ORANGE",
                        });
                        if n % 2 == 0 {
                            entry["annotationDate"] = json!({
                                "year":2026,"month":10,"day":25
                            });
                        } else {
                            entry["annotationDateRange"] = json!({
                                "startDate":{"year":2026,"month":10,"day":24},
                                "endDate":{"year":2026,"month":10,"day":25}
                            });
                        }
                        entry
                    })
                    .collect::<Vec<_>>();
                let mut body = json!({"reportingDataAnnotations":annotations});
                if page == 0 {
                    body["nextPageToken"] = json!("second");
                }
                let body = body.to_string();
                let reply = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                socket.write_all(reply.as_bytes()).await.unwrap();
            }
        });
        let client = GoogleClient::with_endpoints(
            Some("private-fixture-token".into()),
            &admin_host,
            "http://127.0.0.1:9",
        );
        let data = client
            .annotations_with_options("42", Some(4), Some(2))
            .await
            .unwrap();
        responder.await.unwrap();
        let annotations = data["reportingDataAnnotations"].as_array().unwrap();
        assert_eq!(annotations.len(), 8);
        for (index, color) in colors.iter().enumerate() {
            assert_eq!(annotations[index]["color"], *color);
            if index % 2 == 0 {
                assert_eq!(annotations[index]["annotationDate"]["day"], 25);
            } else {
                assert_eq!(
                    annotations[index]["annotationDateRange"]["startDate"]["day"],
                    24
                );
                assert_eq!(
                    annotations[index]["annotationDateRange"]["endDate"]["day"],
                    25
                );
            }
        }
        assert_eq!(annotations[7]["systemGenerated"], true);
        assert!(data.get("nextPageToken").is_none());
    }

    #[tokio::test]
    async fn retention_defaults_and_sparse_optional_fields_remain_provider_owned() {
        for payload in [
            r#"{"name":"properties/42/dataRetentionSettings"}"#,
            r#"{"name":"properties/42/dataRetentionSettings","eventDataRetention":"TWO_MONTHS"}"#,
            r#"{"name":"properties/42/dataRetentionSettings","eventDataRetention":"FOURTEEN_MONTHS","resetUserDataOnNewActivity":false}"#,
        ] {
            let (url, handle) = once(payload).await;
            let client = GoogleClient::with_endpoints(
                Some("test-only-token".into()),
                &url,
                "http://127.0.0.1:9",
            );
            let actual = client.retention_settings("properties/42").await.unwrap();
            let expected: Value = serde_json::from_str(payload).unwrap();
            assert_eq!(actual, expected);
            assert_eq!(
                actual.get("resetUserDataOnNewActivity"),
                expected.get("resetUserDataOnNewActivity")
            );
            let request = handle.await.unwrap();
            assert!(request.starts_with("GET /v1beta/properties/42/dataRetentionSettings HTTP/1.1"));
        }
    }

    #[tokio::test]
    async fn custom_catalogs_paginate_scopes_and_preserve_reserved_names() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let admin_host = format!("http://{}", listener.local_addr().unwrap());
        let responder = tokio::spawn(async move {
            for (index, expected_path, payload) in [
                (
                    0,
                    "/v1beta/properties/42/customDimensions",
                    json!({
                        "customDimensions":[
                            {"name":"properties/42/customDimensions/1","parameterName":"customer_tier","scope":"USER"},
                            {"name":"properties/42/customDimensions/2","parameterName":"item_group","scope":"ITEM"}
                        ],
                        "nextPageToken":"dim-next"
                    }),
                ),
                (
                    1,
                    "/v1beta/properties/42/customDimensions",
                    json!({
                        "customDimensions":[
                            {"name":"properties/42/customDimensions/3","parameterName":"session_id","scope":"EVENT"}
                        ]
                    }),
                ),
                (
                    2,
                    "/v1beta/properties/42/customMetrics",
                    json!({
                        "customMetrics":[
                            {"name":"properties/42/customMetrics/4","parameterName":"order_margin","scope":"EVENT","measurementUnit":"CURRENCY"}
                        ],
                        "nextPageToken":"metric-next"
                    }),
                ),
                (
                    3,
                    "/v1beta/properties/42/customMetrics",
                    json!({
                        "customMetrics":[
                            {"name":"properties/42/customMetrics/5","parameterName":"reserved_revenue","scope":"EVENT","measurementUnit":"STANDARD"}
                        ]
                    }),
                ),
            ] {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut buffer = [0_u8; 4096];
                let count = stream.read(&mut buffer).await.unwrap();
                let request = String::from_utf8_lossy(&buffer[..count]);
                assert!(request.starts_with(&format!("GET {expected_path}")));
                if index == 1 {
                    assert!(request.contains("pageToken=dim-next"));
                }
                if index == 3 {
                    assert!(request.contains("pageToken=metric-next"));
                }
                let body = payload.to_string();
                let reply = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(reply.as_bytes()).await.unwrap();
            }
        });
        let client = GoogleClient::with_endpoints(
            Some("test-only-token".into()),
            &admin_host,
            "http://127.0.0.1:9",
        );
        let catalog = client.custom_definitions("42").await.unwrap();
        responder.await.unwrap();
        let dimensions = catalog["custom_dimensions"].as_array().unwrap();
        let metrics = catalog["custom_metrics"].as_array().unwrap();
        assert_eq!(dimensions.len(), 3);
        assert_eq!(metrics.len(), 2);
        assert_eq!(dimensions[0]["scope"], "USER");
        assert_eq!(dimensions[1]["scope"], "ITEM");
        assert_eq!(dimensions[2]["scope"], "EVENT");
        // A catalog listing preserves even a name that overlaps a built-in field:
        // it must not silently rename, discard, or claim the field is query-safe.
        assert_eq!(dimensions[2]["parameterName"], "session_id");
        assert_eq!(metrics[0]["measurementUnit"], "CURRENCY");
        assert_eq!(metrics[1]["parameterName"], "reserved_revenue");
        assert_eq!(dimensions[2]["name"], "properties/42/customDimensions/3");
        assert!(catalog.get("nextPageToken").is_none());
    }

    #[tokio::test]
    async fn account_hierarchy_preserves_property_parent_timezone_and_permission_scope() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let admin_host = format!("http://{}", listener.local_addr().unwrap());
        let responder = tokio::spawn(async move {
            for (path, status, body) in [
                (
                    "/v1beta/accountSummaries",
                    "200 OK",
                    json!({
                        "accountSummaries":[{
                            "name":"accounts/77",
                            "displayName":"Fixture Organization",
                            "propertySummaries":[
                                {"property":"properties/42","displayName":"Mexico Property"},
                                {"property":"properties/43","displayName":"Sibling Property"}
                            ]
                        }]
                    }),
                ),
                (
                    "/v1beta/properties/42",
                    "200 OK",
                    json!({
                        "name":"properties/42",
                        "parent":"accounts/77",
                        "displayName":"Mexico Property",
                        "timeZone":"America/Mexico_City",
                        "currencyCode":"MXN"
                    }),
                ),
                (
                    "/v1beta/properties/99",
                    "403 Forbidden",
                    json!({"error":{"message":"private account denial details"}}),
                ),
            ] {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut bytes = [0_u8; 4096];
                let size = stream.read(&mut bytes).await.unwrap();
                let request = String::from_utf8_lossy(&bytes[..size]);
                assert!(request.starts_with(&format!("GET {path} HTTP/1.1")));
                let body = body.to_string();
                let response = format!(
                    "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(response.as_bytes()).await.unwrap();
            }
        });
        let client = GoogleClient::with_endpoints(
            Some("private-fixture-token".into()),
            &admin_host,
            "http://127.0.0.1:9",
        );
        let summaries = client
            .account_summaries_with_options(None, None)
            .await
            .unwrap();
        let properties = &summaries["accountSummaries"][0]["propertySummaries"];
        assert_eq!(properties[0]["property"], "properties/42");
        assert_eq!(properties[1]["property"], "properties/43");
        let selected = client.property("properties/42").await.unwrap();
        assert_eq!(selected["parent"], "accounts/77");
        assert_eq!(selected["timeZone"], "America/Mexico_City");
        assert_eq!(selected["currencyCode"], "MXN");
        assert_eq!(client.property("99").await, Err(ApiError::Http(403)));
        responder.await.unwrap();
    }

    #[tokio::test]
    async fn account_metadata_is_refetched_and_never_shared_across_principals() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let admin_host = format!("http://{}", listener.local_addr().unwrap());
        let responder = tokio::spawn(async move {
            for (token, returned_account) in [
                ("principal-a", "accounts/10"),
                ("principal-a", "accounts/11"),
                ("principal-b", "accounts/99"),
            ] {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut buffer = [0_u8; 4096];
                let count = stream.read(&mut buffer).await.unwrap();
                let request = String::from_utf8_lossy(&buffer[..count]);
                assert!(request.starts_with("GET /v1beta/accountSummaries HTTP/1.1"));
                assert!(request.contains(&format!("Bearer {token}")));
                let body = json!({
                    "accountSummaries":[{"name":returned_account}]
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
        let first = GoogleClient::with_endpoints(
            Some("principal-a".into()),
            &admin_host,
            "http://127.0.0.1:9",
        );
        let second = GoogleClient::with_endpoints(
            Some("principal-b".into()),
            &admin_host,
            "http://127.0.0.1:9",
        );
        let a1 = first
            .account_summaries_with_options(None, None)
            .await
            .unwrap();
        let a2 = first
            .account_summaries_with_options(None, None)
            .await
            .unwrap();
        let b1 = second
            .account_summaries_with_options(None, None)
            .await
            .unwrap();
        responder.await.unwrap();
        assert_eq!(a1["accountSummaries"][0]["name"], "accounts/10");
        assert_eq!(a2["accountSummaries"][0]["name"], "accounts/11");
        assert_eq!(b1["accountSummaries"][0]["name"], "accounts/99");
        for response in [&a1, &a2, &b1] {
            let public = response.to_string();
            assert!(!public.contains("principal-a"));
            assert!(!public.contains("principal-b"));
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
