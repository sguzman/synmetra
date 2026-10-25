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
    session::local::LocalSessionManager,
    StreamableHttpServerConfig, StreamableHttpService,
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
    let host_valid = headers.get(header::HOST)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v == access.host);
    let origin_valid = headers.get(header::ORIGIN)
        .map(|v| v.to_str().ok() == Some(access.origin.as_str()))
        .unwrap_or(true);
    let token = headers.get(header::AUTHORIZATION)
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
    let config = StreamableHttpServerConfig::default()
        .with_legacy_session_mode(false)
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
    if token.len() < 32 || token.bytes().any(|v| v.is_ascii_whitespace() || !v.is_ascii_graphic()) {
        return Err("SYNMETRA_MCP_BEARER_TOKEN must contain at least 32 printable nonspace ASCII characters.".into());
    }
    let port = match std::env::var("SYNMETRA_MCP_PORT") {
        Ok(raw) => raw.parse::<u16>().map_err(|_| "Invalid SYNMETRA_MCP_PORT value")?,
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
        let server = Synmetra { api: GoogleClient::new(None) };
        let token = "test-only-token-that-is-long-enough-123".to_owned();
        let task = tokio::spawn(async move {
            axum::serve(listener, app(server, token, port)).await.unwrap();
        });
        let client = reqwest::Client::new();
        let endpoint = format!("http://127.0.0.1:{port}/mcp");
        let unauthorized = client.post(&endpoint).body("{}").send().await.unwrap();
        assert_eq!(unauthorized.status(), StatusCode::UNAUTHORIZED);

        let forbidden_host = client.post(&endpoint)
            .header(header::AUTHORIZATION, "Bearer test-only-token-that-is-long-enough-123")
            .header(header::HOST, "malicious.example")
            .body("{}").send().await.unwrap();
        assert_eq!(forbidden_host.status(), StatusCode::UNAUTHORIZED);

        let forbidden_origin = client.post(&endpoint)
            .header(header::AUTHORIZATION, "Bearer test-only-token-that-is-long-enough-123")
            .header(header::ORIGIN, "https://malicious.example")
            .body("{}").send().await.unwrap();
        assert_eq!(forbidden_origin.status(), StatusCode::UNAUTHORIZED);
        task.abort();
    }
}
