//! Optional local-only authenticated Streamable HTTP MCP transport.
//! Public and non-loopback deployment is intentionally unsupported.

use crate::Synmetra;
use axum::{
    body::{to_bytes, Body},
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use rmcp::transport::streamable_http_server::{
    session::local::LocalSessionManager, StreamableHttpServerConfig, StreamableHttpService,
};
use serde_json::json;
use std::{sync::Arc, time::Duration};
use subtle::ConstantTimeEq;
use tokio::sync::Semaphore;

const MAX_HTTP_REQUEST_BYTES: usize = 1_048_576;
const MAX_INFLIGHT_HTTP_REQUESTS: usize = 16;
const MAX_BODY_WAIT: Duration = Duration::from_secs(10);
const MAX_HANDLER_WAIT: Duration = Duration::from_secs(30);

#[derive(Clone)]
struct Access {
    secret: Arc<[u8]>,
    host: String,
    origin: String,
    in_flight: Arc<Semaphore>,
    body_timeout: Duration,
}

async fn require_local_auth(
    State(access): State<Access>,
    request: Request,
    next: Next,
) -> Response {
    let headers = request.headers();
    let host_valid = headers
        .get(header::HOST)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v == access.host);
    let origin_valid = headers
        .get(header::ORIGIN)
        .map(|v| v.to_str().ok() == Some(access.origin.as_str()))
        .unwrap_or(true);
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or("");
    let authorized = token.as_bytes().ct_eq(access.secret.as_ref()).unwrap_u8() == 1;
    if !host_valid || !origin_valid || !authorized {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    // Reject overload and oversized/slow bodies before invoking MCP parsing or
    // Google API calls. Chunked bodies are bounded by to_bytes, not only headers.
    let _permit = match access.in_flight.try_acquire() {
        Ok(permit) => permit,
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };
    if request
        .headers()
        .get(header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<usize>().ok())
        .is_some_and(|len| len > MAX_HTTP_REQUEST_BYTES)
    {
        return StatusCode::PAYLOAD_TOO_LARGE.into_response();
    }
    let (parts, body) = request.into_parts();
    let bytes =
        match tokio::time::timeout(access.body_timeout, to_bytes(body, MAX_HTTP_REQUEST_BYTES))
            .await
        {
            Ok(Ok(bytes)) => bytes,
            Ok(Err(_)) => return StatusCode::PAYLOAD_TOO_LARGE.into_response(),
            Err(_) => return StatusCode::REQUEST_TIMEOUT.into_response(),
        };
    let request = Request::from_parts(parts, Body::from(bytes));
    match tokio::time::timeout(MAX_HANDLER_WAIT, next.run(request)).await {
        Ok(response) => response,
        Err(_) => StatusCode::GATEWAY_TIMEOUT.into_response(),
    }
}

fn app(server: Synmetra, token: String, port: u16) -> Router {
    app_with_limits(
        server,
        token,
        port,
        Arc::new(Semaphore::new(MAX_INFLIGHT_HTTP_REQUESTS)),
        MAX_BODY_WAIT,
    )
}

fn app_with_limits(
    server: Synmetra,
    token: String,
    port: u16,
    in_flight: Arc<Semaphore>,
    body_timeout: Duration,
) -> Router {
    let access = Access {
        secret: Arc::from(token.into_bytes().into_boxed_slice()),
        host: format!("127.0.0.1:{port}"),
        origin: format!("http://127.0.0.1:{port}"),
        in_flight,
        body_timeout,
    };
    // Preserve legacy 2025 clients' MCP sessions. The current 2026 protocol
    // remains stateless by specification, independent of this legacy setting.
    let config = StreamableHttpServerConfig::default()
        .with_legacy_session_mode(true)
        .with_json_response(true);
    let service = StreamableHttpService::new(
        move || Ok(server.clone()),
        LocalSessionManager::default().into(),
        config,
    );
    Router::new()
        .route("/health", get(|| async { Json(json!({"status":"ok"})) }))
        .route("/metadata", get(|| async {
            Json(json!({"name":"synmetra","transport":"streamable-http","authentication":"bearer","network":"loopback-only"}))
        }))
        .nest_service("/mcp", service)
        .layer(middleware::from_fn_with_state(access, require_local_auth))
}

fn loopback_address(port: u16) -> std::net::SocketAddr {
    std::net::SocketAddr::from(([127, 0, 0, 1], port))
}

pub async fn serve_loopback(server: Synmetra) -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("SYNMETRA_MCP_BEARER_TOKEN").map_err(|_| {
        "Loopback HTTP requires SYNMETRA_MCP_BEARER_TOKEN (at least 32 characters)."
    })?;
    if token.len() < 32
        || token
            .bytes()
            .any(|v| v.is_ascii_whitespace() || !v.is_ascii_graphic())
    {
        return Err("SYNMETRA_MCP_BEARER_TOKEN must contain at least 32 printable nonspace ASCII characters.".into());
    }
    let port = match std::env::var("SYNMETRA_MCP_PORT") {
        Ok(raw) => raw
            .parse::<u16>()
            .map_err(|_| "Invalid SYNMETRA_MCP_PORT value")?,
        Err(_) => 44128,
    };
    if port == 0 {
        return Err("SYNMETRA_MCP_PORT must be nonzero.".into());
    }
    let listener = tokio::net::TcpListener::bind(loopback_address(port)).await?;
    eprintln!("Synmetra experimental authenticated MCP listening on http://127.0.0.1:{port}/mcp");
    axum::serve(listener, app(server, token, port)).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::google::GoogleClient;

    /// Legacy MCP HTTP sessions may return JSON-RPC over SSE rather than a
    /// single application/json body. Read one event, not an open SSE stream.
    async fn mcp_json_response(mut response: reqwest::Response) -> serde_json::Value {
        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("")
            .to_owned();
        if content_type.starts_with("application/json") {
            return response.json().await.unwrap();
        }
        assert!(
            content_type.starts_with("text/event-stream"),
            "Unexpected MCP response content type: {content_type}"
        );
        let mut stream = String::new();
        loop {
            let chunk = response
                .chunk()
                .await
                .unwrap()
                .expect("SSE stream ended before JSON-RPC result");
            stream.push_str(&String::from_utf8_lossy(&chunk));
            assert!(
                stream.len() <= 1_048_576,
                "Oversized synthetic SSE test response"
            );
            for line in stream.lines() {
                if let Some(text) = line.strip_prefix("data: ") {
                    if let Ok(payload) = serde_json::from_str::<serde_json::Value>(text) {
                        return payload;
                    }
                }
            }
        }
    }

    #[test]
    fn production_http_bind_is_always_ipv4_loopback() {
        for port in [1, 44128, 65535] {
            let address = loopback_address(port);
            assert_eq!(
                address.ip(),
                std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)
            );
            assert_eq!(address.port(), port);
        }
    }

    #[tokio::test]
    async fn loopback_http_rejects_unauthenticated_wrong_host_and_wrong_origin() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = Synmetra::new(GoogleClient::new(None), false);
        let token = "test-only-token-that-is-long-enough-123".to_owned();
        let task = tokio::spawn(async move {
            axum::serve(listener, app(server, token, port))
                .await
                .unwrap();
        });
        let client = reqwest::Client::new();
        let endpoint = format!("http://127.0.0.1:{port}/mcp");
        let unauthorized = client.post(&endpoint).body("{}").send().await.unwrap();
        assert_eq!(unauthorized.status(), StatusCode::UNAUTHORIZED);

        let forbidden_host = client
            .post(&endpoint)
            .header(
                header::AUTHORIZATION,
                "Bearer test-only-token-that-is-long-enough-123",
            )
            .header(header::HOST, "malicious.example")
            .body("{}")
            .send()
            .await
            .unwrap();
        assert_eq!(forbidden_host.status(), StatusCode::UNAUTHORIZED);

        let forbidden_origin = client
            .post(&endpoint)
            .header(
                header::AUTHORIZATION,
                "Bearer test-only-token-that-is-long-enough-123",
            )
            .header(header::ORIGIN, "https://malicious.example")
            .body("{}")
            .send()
            .await
            .unwrap();
        assert_eq!(forbidden_origin.status(), StatusCode::UNAUTHORIZED);
        task.abort();
    }

    #[tokio::test]
    async fn authenticated_http_session_lists_and_calls_tools_but_rejects_invalid_sessions() {
        use serde_json::json;

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let token = "http-session-synthetic-private-token-4736";
        let server = Synmetra::new(GoogleClient::new(None), false);
        let task = tokio::spawn(async move {
            axum::serve(listener, app(server, token.to_owned(), port))
                .await
                .unwrap();
        });
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .unwrap();
        let endpoint = format!("http://127.0.0.1:{port}/mcp");
        let origin = format!("http://127.0.0.1:{port}");
        let initialize = client
            .post(&endpoint)
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .header(header::ORIGIN, &origin)
            .header(header::ACCEPT, "application/json, text/event-stream")
            .json(&json!({
                "jsonrpc":"2.0",
                "id":1,
                "method":"initialize",
                "params":{
                    "protocolVersion":"2025-11-25",
                    "capabilities":{},
                    "clientInfo":{"name":"synthetic-native-http-client","version":"1"}
                }
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(initialize.status(), StatusCode::OK);
        let session = initialize
            .headers()
            .get("mcp-session-id")
            .expect("MCP HTTP initialization must return a session identifier")
            .to_str()
            .unwrap()
            .to_owned();
        let negotiated = mcp_json_response(initialize).await;
        assert_eq!(negotiated["result"]["serverInfo"]["name"], "synmetra");

        let request = |payload: serde_json::Value, session_id: &str| {
            client
                .post(&endpoint)
                .header(header::AUTHORIZATION, format!("Bearer {token}"))
                .header(header::ORIGIN, &origin)
                .header(header::ACCEPT, "application/json, text/event-stream")
                .header("mcp-protocol-version", "2025-11-25")
                .header("mcp-session-id", session_id)
                .json(&payload)
        };
        let notification = request(
            json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
            &session,
        )
        .send()
        .await
        .unwrap();
        assert!(notification.status().is_success());

        let discovered = request(
            json!({"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}),
            &session,
        )
        .send()
        .await
        .unwrap();
        assert_eq!(discovered.status(), StatusCode::OK);
        let listing = mcp_json_response(discovered).await;
        let tools = listing["result"]["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 20);
        assert!(tools.iter().any(|item| item["name"] == "ga4_get_started"));
        assert!(tools
            .iter()
            .all(|item| item["name"] != "create_property_annotation"));

        let invoked = request(
            json!({"jsonrpc":"2.0","id":3,"method":"tools/call","params":{
                "name":"ga4_get_started","arguments":{}
            }}),
            &session,
        )
        .send()
        .await
        .unwrap();
        assert_eq!(invoked.status(), StatusCode::OK);
        let result = mcp_json_response(invoked).await;
        assert_eq!(result["result"]["structuredContent"]["ok"], true);
        assert_eq!(
            result["result"]["structuredContent"]["data"]["scope"],
            "https://www.googleapis.com/auth/analytics.readonly"
        );
        assert!(!result.to_string().contains(token));

        let unknown = request(
            json!({"jsonrpc":"2.0","id":4,"method":"tools/list","params":{}}),
            "invalid-synmetra-session-000",
        )
        .send()
        .await
        .unwrap();
        assert!(!unknown.status().is_success());

        let bad_token = client
            .post(&endpoint)
            .header(header::AUTHORIZATION, "Bearer invalid-credential")
            .header(header::ORIGIN, &origin)
            .header("mcp-session-id", &session)
            .json(&json!({"jsonrpc":"2.0","id":5,"method":"tools/list"}))
            .send()
            .await
            .unwrap();
        assert_eq!(bad_token.status(), StatusCode::UNAUTHORIZED);

        let bad_origin = client
            .post(&endpoint)
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .header(header::ORIGIN, "https://example.invalid")
            .header("mcp-session-id", &session)
            .json(&json!({"jsonrpc":"2.0","id":6,"method":"tools/list"}))
            .send()
            .await
            .unwrap();
        assert_eq!(bad_origin.status(), StatusCode::UNAUTHORIZED);

        let terminated = client
            .delete(&endpoint)
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .header(header::ORIGIN, &origin)
            .header("mcp-protocol-version", "2025-11-25")
            .header("mcp-session-id", &session)
            .send()
            .await
            .unwrap();
        assert!(
            terminated.status().is_success(),
            "Authenticated legacy session DELETE must terminate its session"
        );
        let after_logout = request(
            json!({"jsonrpc":"2.0","id":7,"method":"tools/list","params":{}}),
            &session,
        )
        .send()
        .await
        .unwrap();
        assert_eq!(after_logout.status(), StatusCode::NOT_FOUND);
        task.abort();
    }

    #[tokio::test]
    async fn authenticated_http_rejects_oversize_slow_and_overloaded_requests() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let token = "synthetic-resource-limit-bearer-token-1234";
        let limiter = Arc::new(Semaphore::new(1));
        let server = Synmetra::new(GoogleClient::new(None), false);
        let task = tokio::spawn({
            let limiter = limiter.clone();
            async move {
                axum::serve(
                    listener,
                    app_with_limits(
                        server,
                        token.to_owned(),
                        port,
                        limiter,
                        Duration::from_millis(150),
                    ),
                )
                .await
                .unwrap();
            }
        });
        let client = reqwest::Client::new();
        let endpoint = format!("http://127.0.0.1:{port}/mcp");

        let oversized = client
            .post(&endpoint)
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .body(vec![b'x'; MAX_HTTP_REQUEST_BYTES + 1])
            .send()
            .await
            .unwrap();
        assert_eq!(oversized.status(), StatusCode::PAYLOAD_TOO_LARGE);

        let held = limiter.clone().acquire_owned().await.unwrap();
        let overloaded = client
            .post(&endpoint)
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .body("{}")
            .send()
            .await
            .unwrap();
        assert_eq!(overloaded.status(), StatusCode::SERVICE_UNAVAILABLE);
        drop(held);

        let mut slow = tokio::net::TcpStream::connect(("127.0.0.1", port))
            .await
            .unwrap();
        slow.write_all(
            format!(
                "POST /mcp HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nAuthorization: Bearer {token}\r\nContent-Length: 10\r\nConnection: close\r\n\r\nx"
            )
            .as_bytes(),
        )
        .await
        .unwrap();
        let mut response = [0u8; 2048];
        let len = tokio::time::timeout(Duration::from_secs(3), slow.read(&mut response))
            .await
            .unwrap()
            .unwrap();
        let result = String::from_utf8_lossy(&response[..len]);
        assert!(result.contains("408"), "{result}");
        task.abort();
    }

    #[tokio::test]
    async fn metadata_and_health_are_authenticated_and_never_contain_credentials() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let token = "synthetic-private-metadata-token-32-chars";
        let server = Synmetra::new(GoogleClient::new(None), false);
        let task = tokio::spawn(async move {
            axum::serve(listener, app(server, token.to_owned(), port))
                .await
                .unwrap();
        });
        let client = reqwest::Client::new();
        for path in ["health", "metadata"] {
            let endpoint = format!("http://127.0.0.1:{port}/{path}");
            let anonymous = client.get(&endpoint).send().await.unwrap();
            assert_eq!(anonymous.status(), StatusCode::UNAUTHORIZED);
            let bad = client
                .get(&endpoint)
                .header(header::AUTHORIZATION, "Bearer wrong-token")
                .send()
                .await
                .unwrap();
            assert_eq!(bad.status(), StatusCode::UNAUTHORIZED);
            let bad_origin = client
                .get(&endpoint)
                .header(header::AUTHORIZATION, format!("Bearer {token}"))
                .header(header::ORIGIN, "https://remote.example")
                .send()
                .await
                .unwrap();
            assert_eq!(bad_origin.status(), StatusCode::UNAUTHORIZED);

            let authorized = client
                .get(endpoint)
                .header(header::AUTHORIZATION, format!("Bearer {token}"))
                .send()
                .await
                .unwrap();
            assert_eq!(authorized.status(), StatusCode::OK);
            let data: serde_json::Value = authorized.json().await.unwrap();
            assert!(data.is_object());
            assert!(!data.to_string().contains(token));
            assert!(!data.to_string().contains("access_token"));
            assert!(!data.to_string().contains("refresh_token"));
            if path == "health" {
                assert_eq!(data["status"], "ok");
                assert!(data.get("tools").is_none());
            } else {
                assert_eq!(data["transport"], "streamable-http");
                assert_eq!(data["network"], "loopback-only");
            }
        }
        task.abort();
    }

    #[tokio::test]
    async fn authorized_loopback_http_initializes_real_mcp() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = Synmetra::new(GoogleClient::new(None), false);
        let task = tokio::spawn(async move {
            axum::serve(
                listener,
                app(
                    server,
                    "test-only-token-that-is-long-enough-123".to_owned(),
                    port,
                ),
            )
            .await
            .unwrap();
        });
        let endpoint = format!("http://127.0.0.1:{port}/mcp");
        let response = reqwest::Client::new()
            .post(endpoint)
            .header(
                header::AUTHORIZATION,
                "Bearer test-only-token-that-is-long-enough-123",
            )
            .header(header::ORIGIN, format!("http://127.0.0.1:{port}"))
            .header(header::ACCEPT, "application/json, text/event-stream")
            .header(header::CONTENT_TYPE, "application/json")
            .json(&serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {
                    "protocolVersion": "2025-11-25",
                    "capabilities": {},
                    "clientInfo": { "name": "synmetra-synthetic-http-test", "version": "0" }
                }
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let payload = mcp_json_response(response).await;
        assert_eq!(payload["result"]["serverInfo"]["name"], "synmetra");
        assert!(payload["result"]["capabilities"]["tools"].is_object());
        task.abort();
    }
}
