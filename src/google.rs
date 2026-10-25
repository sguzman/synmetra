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
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCredentials => write!(f, "No Google access token configured. Set SYNMETRA_GOOGLE_ACCESS_TOKEN to a scoped, short-lived token."),
            Self::InvalidIdentifier => write!(f, "Invalid Google Analytics property identifier: expected positive decimal digits or properties/<digits>."),
            Self::InvalidRequest => write!(f, "Invalid Google Analytics request: report fields must be a JSON object."),
            Self::RequestFailed => write!(f, "Google Analytics request failed (network or timeout)."),
            Self::Http(status) => write!(f, "Google Analytics API returned HTTP {status}. Check property access, scopes, quotas, and the request."),
            Self::ResponseTooLarge => write!(f, "Google Analytics response exceeded the 4 MiB safety limit."),
            Self::InvalidResponse => write!(f, "Google Analytics response was not valid JSON or had an invalid pagination contract."),
            Self::TooManyPages => write!(f, "Google Analytics pagination exceeded the 100-page safety limit."),
        }
    }
}

pub fn property_name(input: &str) -> Result<String, ApiError> {
    let digits = input.strip_prefix("properties/").unwrap_or(input);
    if digits.is_empty()
        || digits.starts_with('0')
        || !digits.bytes().all(|c| c.is_ascii_digit())
        || digits.parse::<u64>().is_err()
    {
        return Err(ApiError::InvalidIdentifier);
    }
    Ok(format!("properties/{digits}"))
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

    fn token(&self) -> Result<&str, ApiError> {
        self.bearer.as_deref().ok_or(ApiError::MissingCredentials)
    }

    async fn send(&self, req: reqwest::RequestBuilder) -> Result<Value, ApiError> {
        let bearer = self.token()?;
        let response = req
            .bearer_auth(bearer)
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

    pub async fn report(&self, property: &str, request: Value, realtime: bool) -> Result<Value, ApiError> {
        let property = property_name(property)?;
        if !request.is_object() {
            return Err(ApiError::InvalidRequest);
        }
        let method = if realtime { "runRealtimeReport" } else { "runReport" };
        let url = format!("{}/v1beta/{property}:{method}", self.data_base);
        self.send(self.http.post(url).json(&request)).await
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
}
