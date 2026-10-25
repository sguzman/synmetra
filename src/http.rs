//! Optional local-only authenticated Streamable HTTP MCP transport.
//! Public and non-loopback deployment is intentionally unsupported.

use crate::Synmetra;
use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    Router,
};
use rmcp::transport::streamable_http_server::{
    session::local::LocalSessionManager, StreamableHttpServerConfig, StreamableHttpService,
};
use std::sync::Arc;
use subtle::ConstantTimeEq;

#[derive(Clone)]
struct Access {
    secret: Arc<[u8]>,
    host: String,
    origin: String,
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
    next.run(request).await
}

fn app(server: Synmetra, token: String, port: u16) -> Router {
    let access = Access {
        secret: Arc::from(token.into_bytes().into_boxed_slice()),
        host: format!("127.0.0.1:{port}"),
        origin: format!("http://127.0.0.1:{port}"),
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
        .nest_service("/mcp", service)
        .layer(middleware::from_fn_with_state(access, require_local_auth))
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
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port)).await?;
    eprintln!("Synmetra experimental authenticated MCP listening on http://127.0.0.1:{port}/mcp");
    axum::serve(listener, app(server, token, port)).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::google::GoogleClient;

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
        let negotiated: serde_json::Value = initialize.json().await.unwrap();
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
        let listing: serde_json::Value = discovered.json().await.unwrap();
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
        let result: serde_json::Value = invoked.json().await.unwrap();
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
        let payload: serde_json::Value = response.json().await.unwrap();
        assert_eq!(payload["result"]["serverInfo"]["name"], "synmetra");
        assert!(payload["result"]["capabilities"]["tools"].is_object());
        task.abort();
    }
}
