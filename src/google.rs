//! Bounded and least-privilege Google Analytics HTTP client.
//! Credentials are never included in logs or response error strings.

use futures_util::StreamExt;
use serde_json::{json, Map, Value};
use std::{fmt, time::Duration};

const ADMIN: &str = "https://analyticsadmin.googleapis.com";
const DATA: &str = "https://analyticsdata.googleapis.com";
const MAX_RESPONSE_BYTES: usize = 4 * 1024 * 1024;
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
    RequestFailed,
    Http(u16),
    ResponseTooLarge,
    InvalidResponse,
    TooManyPages,
    AuthenticationFailed,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCredentials => write!(f, "No Google access token configured. Run synmetra auth login --client-id-file PATH or set SYNMETRA_GOOGLE_ACCESS_TOKEN to a scoped, short-lived token."),
            Self::InvalidIdentifier => write!(f, "Invalid Google Analytics property identifier: expected positive decimal digits or properties/<digits>."),
            Self::InvalidRequest => write!(f, "Invalid Google Analytics request: report fields must be a JSON object."),
            Self::RequestFailed => write!(f, "Google Analytics request failed (network or timeout)."),
            Self::Http(status) => write!(f, "Google Analytics API returned HTTP {status}. Check property access, scopes, quotas, and the request."),
            Self::ResponseTooLarge => write!(f, "Google Analytics response exceeded the 4 MiB safety limit."),
            Self::InvalidResponse => write!(f, "Google Analytics response was not valid JSON or had an invalid pagination contract."),
            Self::TooManyPages => write!(f, "Google Analytics pagination exceeded the 100-page safety limit."),
            Self::AuthenticationFailed => write!(f, "Synmetra OAuth credentials could not be loaded or refreshed. Run synmetra auth status, then login again if necessary."),
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

impl GoogleClient {
    pub fn new(bearer: Option<String>) -> Self {
        Self::with_endpoints(bearer, ADMIN, DATA)
    }

    fn with_endpoints(bearer: Option<String>, admin: &str, data: &str) -> Self {
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
        if response.content_length().is_some_and(|n| n > MAX_RESPONSE_BYTES as u64) {
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

    async fn get(&self, base: &str, path: &str, page_token: Option<&str>) -> Result<Value, ApiError> {
        let req = self.http.get(format!("{base}{path}"));
        let req = if let Some(token) = page_token { req.query(&[("pageToken", token)]) } else { req };
        self.send(req).await
    }

    pub async fn collection(&self, path: &str, field: &str, alpha: bool) -> Result<Value, ApiError> {
        let base = &self.admin_base;
        let path = format!("/{}{path}", if alpha { "v1alpha" } else { "v1beta" });
        let mut output = Vec::new();
        let mut next: Option<String> = None;
        for _ in 0..MAX_PAGES {
            let page = self.get(base, &path, next.as_deref()).await?;
            if let Some(items) = page.get(field) {
                let Some(items) = items.as_array() else { return Err(ApiError::InvalidResponse) };
                output.extend_from_slice(items);
            }
            next = page.get("nextPageToken").and_then(Value::as_str).filter(|s| !s.is_empty()).map(ToOwned::to_owned);
            if next.is_none() {
                return Ok(json!({field: output}));
            }
        }
        Err(ApiError::TooManyPages)
    }

    pub async fn account_summaries(&self) -> Result<Value, ApiError> {
        self.collection("/accountSummaries", "accountSummaries", false).await
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

    pub async fn ads_links(&self, property: &str) -> Result<Value, ApiError> {
        let path = format!("/{}/googleAdsLinks", property_name(property)?);
        self.collection(&path, "googleAdsLinks", false).await
    }

    pub async fn annotations(&self, property: &str) -> Result<Value, ApiError> {
        let path = format!("/{}/reportingDataAnnotations", property_name(property)?);
        self.collection(&path, "reportingDataAnnotations", true).await
    }

    pub async fn custom_definitions(&self, property: &str) -> Result<Value, ApiError> {
        let path = property_name(property)?;
        let dimensions = self
            .collection(&format!("/{path}/customDimensions"), "customDimensions", false)
            .await?;
        let metrics = self
            .collection(&format!("/{path}/customMetrics"), "customMetrics", false)
            .await?;
        Ok(json!({
            "custom_dimensions": dimensions["customDimensions"],
            "custom_metrics": metrics["customMetrics"]
        }))
    }

    fn validate_body(request: &Value) -> Result<(), ApiError> {
        if !request.is_object()
            || serde_json::to_vec(request).map_or(true, |bytes| bytes.len() > 64 * 1024)
        {
            return Err(ApiError::InvalidRequest);
        }
        Ok(())
    }

    pub async fn data_operation(&self, property: &str, request: Value, method: &str) -> Result<Value, ApiError> {
        let property = property_name(property)?;
        Self::validate_body(&request)?;
        if !["runReport", "runRealtimeReport", "runPivotReport", "batchRunReports", "checkCompatibility"]
            .contains(&method)
        {
            return Err(ApiError::InvalidRequest);
        }
        if method == "batchRunReports"
            && request["requests"].as_array().is_none_or(|list| list.is_empty() || list.len() > 5)
        {
            return Err(ApiError::InvalidRequest);
        }
        let url = format!("{}/v1beta/{property}:{method}", self.data_base);
        self.send(self.http.post(url).json(&request)).await
    }

    pub async fn access_report(&self, entity: &str, request: Value, account: bool) -> Result<Value, ApiError> {
        let entity = if account { account_name(entity)? } else { property_name(entity)? };
        Self::validate_body(&request)?;
        // This Admin API endpoint requires Administrator privileges on the selected entity.
        let url = format!("{}/v1beta/{entity}:runAccessReport", self.admin_base);
        self.send(self.http.post(url).json(&request)).await
    }

    pub async fn report(&self, property: &str, request: Value, realtime: bool) -> Result<Value, ApiError> {
        self.data_operation(property, request, if realtime { "runRealtimeReport" } else { "runReport" }).await
    }
}

/// Google REST APIs accept array entries as objects, not raw field names.
pub fn report_request(
    dates: Option<Vec<Value>>,
    dimensions: Vec<String>,
    metrics: Vec<String>,
    extras: Vec<(&str, Option<Value>)>,
) -> Result<Value, ApiError> {
    if metrics.is_empty() || metrics.len() > 10 || dimensions.len() > 9 || metrics.iter().chain(dimensions.iter()).any(|s| s.trim().is_empty()) {
        return Err(ApiError::InvalidRequest);
    }
    if dates.as_ref().is_some_and(|d| d.is_empty() || d.len() > 4) {
        return Err(ApiError::InvalidRequest);
    }
    let mut request = Map::new();
    if let Some(dates) = dates {
        request.insert("dateRanges".to_owned(), Value::Array(dates));
    }
    request.insert("dimensions".to_owned(), Value::Array(dimensions.into_iter().map(|name| json!({"name": name})).collect()));
    request.insert("metrics".to_owned(), Value::Array(metrics.into_iter().map(|name| json!({"name": name})).collect()));
    for (name, val) in extras {
        if let Some(val) = val {
            request.insert(name.to_owned(), val);
        }
    }
    Ok(Value::Object(request))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};

    #[test]
    fn rejects_property_path_and_zero() {
        for bad in ["", "0", "001", "-1", "../x", "properties/2/ads", "https://example.org", "123?x=1", " 21", "18446744073709551616"] {
            assert!(property_name(bad).is_err(), "{bad}");
        }
        assert_eq!(property_name("321").unwrap(), "properties/321");
        assert_eq!(property_name("properties/321").unwrap(), "properties/321");
        assert_eq!(account_name("accounts/123").unwrap(), "accounts/123");
        assert!(account_name("properties/5").is_err());
    }

    #[test]
    fn report_inputs_are_typed_and_bounded() {
        let req = report_request(
            Some(vec![json!({"startDate":"7daysAgo","endDate":"today"})]),
            vec!["country".into()],
            vec!["activeUsers".into()],
            vec![("limit", Some(json!("10")))],
        ).unwrap();
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
    async fn account_summaries_use_scoped_bearer_and_safe_host() {
        let (url, handle) = once(r#"{"accountSummaries":[{"name":"accounts/1"}]}"#).await;
        let client = GoogleClient::with_endpoints(Some("test-token".into()), &url, &url);
        let response = client.account_summaries().await.unwrap();
        assert_eq!(response["accountSummaries"][0]["name"], "accounts/1");
        let request = handle.await.unwrap();
        assert!(request.starts_with("GET /v1beta/accountSummaries HTTP/1.1"));
        assert!(request.to_lowercase().contains("authorization: bearer test-token"));
        assert!(!format!("{:?}", ApiError::Http(403)).contains("test-token"));
    }

    #[tokio::test]
    async fn report_posts_to_correct_data_api_endpoint() {
        let (url, handle) = once(r#"{"rowCount":1}"#).await;
        let client = GoogleClient::with_endpoints(Some("token".into()), &url, &url);
        let response = client.report("properties/42", json!({"metrics":[{"name":"activeUsers"}]}), false).await.unwrap();
        assert_eq!(response["rowCount"], 1);
        let request = handle.await.unwrap();
        assert!(request.starts_with("POST /v1beta/properties/42:runReport HTTP/1.1"));
    }

    #[tokio::test]
    async fn missing_credentials_reject_before_http() {
        let client = GoogleClient::new(None);
        assert_eq!(client.account_summaries().await.unwrap_err(), ApiError::MissingCredentials);
    }

    #[tokio::test]
    async fn account_sharing_and_retention_use_correct_paths() {
        let (url, handle) = once(r#"{"sharingWithOthersEnabled":false}"#).await;
        let c = GoogleClient::with_endpoints(Some("token".into()), &url, &url);
        let value = c.account_sharing("accounts/87").await.unwrap();
        assert_eq!(value["sharingWithOthersEnabled"], false);
        assert!(handle.await.unwrap().starts_with("GET /v1beta/accounts/87/dataSharingSettings HTTP/1.1"));

        let (url, handle) = once(r#"{"eventDataRetention":"FOURTEEN_MONTHS"}"#).await;
        let c = GoogleClient::with_endpoints(Some("token".into()), &url, &url);
        let value = c.retention_settings("42").await.unwrap();
        assert_eq!(value["eventDataRetention"], "FOURTEEN_MONTHS");
        assert!(handle.await.unwrap().starts_with("GET /v1beta/properties/42/dataRetentionSettings HTTP/1.1"));
    }

    #[tokio::test]
    async fn advanced_reports_use_read_only_api_paths() {
        for method in ["runPivotReport", "checkCompatibility", "batchRunReports"] {
            let (url, handle) = once(r#"{"ok":true}"#).await;
            let c = GoogleClient::with_endpoints(Some("token".into()), &url, &url);
            let body = if method == "batchRunReports" {
                json!({"requests":[{"metrics":[{"name":"activeUsers"}]}]})
            } else { json!({"metrics":[{"name":"activeUsers"}]}) };
            assert_eq!(c.data_operation("properties/77", body, method).await.unwrap()["ok"], true);
            assert!(handle.await.unwrap().starts_with(&format!("POST /v1beta/properties/77:{method} HTTP/1.1")));
        }
    }

    #[tokio::test]
    async fn access_reports_are_scoped_to_account_or_property() {
        for (id, account) in [("accounts/16", true), ("properties/82", false)] {
            let (url, handle) = once(r#"{"rowCount":0}"#).await;
            let c = GoogleClient::with_endpoints(Some("token".into()), &url, &url);
            assert_eq!(c.access_report(id, json!({"dimensions":[]}), account).await.unwrap()["rowCount"], 0);
            assert!(handle.await.unwrap().starts_with(&format!("POST /v1beta/{id}:runAccessReport HTTP/1.1")));
        }
    }

    #[tokio::test]
    async fn rejects_invalid_batch_without_http_request() {
        let c = GoogleClient::new(Some("token".into()));
        for bad in [json!({}), json!({"requests":[]}), json!({"requests":[{},{},{},{},{},{}]})] {
            assert_eq!(c.data_operation("99", bad, "batchRunReports").await.unwrap_err(), ApiError::InvalidRequest);
        }
        assert_eq!(c.data_operation("99", json!({}), "notARealMethod").await.unwrap_err(), ApiError::InvalidRequest);
    }
}
