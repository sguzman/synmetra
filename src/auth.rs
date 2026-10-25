//! Native, private Google Desktop OAuth credentials for a Linux single-user MCP.
//! Never print tokens, credentials JSON, callback codes or HTTP response bodies.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::{rngs::OsRng, RngCore};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    env, fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    time::timeout,
};

const SCOPE: &str = "https://www.googleapis.com/auth/analytics.readonly";
const SCOPE_EDIT: &str = "https://www.googleapis.com/auth/analytics.edit";

fn default_scope() -> String {
    SCOPE.to_owned()
}
const AUTHORIZE: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";

#[derive(Debug, Deserialize)]
struct OAuthClientFile {
    installed: InstalledClient,
}

#[derive(Debug, Deserialize)]
struct InstalledClient {
    client_id: String,
    #[serde(default)]
    client_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Stored {
    client_id: String,
    client_secret: String,
    refresh_token: String,
    access_token: String,
    expires_at: u64,
    #[serde(default = "default_scope")]
    granted_scope: String,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    #[serde(default)]
    refresh_token: Option<String>,
    expires_in: u64,
    #[serde(default)]
    scope: Option<String>,
}

fn epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn credentials_path() -> Result<PathBuf, String> {
    let root = match env::var_os("XDG_CONFIG_HOME") {
        Some(path) if !path.is_empty() => PathBuf::from(path),
        _ => PathBuf::from(env::var_os("HOME").ok_or("HOME is not set")?).join(".config"),
    };
    if !root.is_absolute() {
        return Err("XDG_CONFIG_HOME must be an absolute path".to_owned());
    }
    Ok(root.join("synmetra").join("credentials.json"))
}

#[cfg(unix)]
fn private_file(path: &Path) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    let meta = fs::symlink_metadata(path).map_err(|_| "Cannot read credential file metadata")?;
    if !meta.is_file() || meta.file_type().is_symlink() || meta.permissions().mode() & 0o077 != 0 {
        return Err("Credentials are not a regular private (0600) file".into());
    }
    Ok(())
}

#[cfg(not(unix))]
fn private_file(path: &Path) -> Result<(), String> {
    if !path.is_file() || path.is_symlink() {
        return Err("Credentials path is not a regular file".to_owned());
    }
    Ok(())
}

fn read_credentials_at(path: &Path) -> Result<Option<Stored>, String> {
    if let Some(dir) = path.parent() {
        if let Ok(meta) = fs::symlink_metadata(dir) {
            if !meta.is_dir() || meta.file_type().is_symlink() {
                return Err("Refusing credentials in a symlinked auth directory".into());
            }
        }
    }
    if !path.exists() {
        // A dangling symlink is still unsafe: don't follow/create at this path.
        if fs::symlink_metadata(path).is_ok() {
            return Err("Credentials path is an invalid symlink".into());
        }
        return Ok(None);
    }
    private_file(path)?;
    let file = fs::File::open(path).map_err(|_| "Cannot open credentials")?;
    let mut data = String::new();
    file.take(32 * 1024)
        .read_to_string(&mut data)
        .map_err(|_| "Cannot read credentials")?;
    let stored: Stored =
        serde_json::from_str(&data).map_err(|_| "Credentials file has invalid format")?;
    if stored.client_id.is_empty() || stored.refresh_token.is_empty() {
        return Err("Credentials file lacks a client ID or refresh token".into());
    }
    Ok(Some(stored))
}

fn read_credentials() -> Result<Option<Stored>, String> {
    read_credentials_at(&credentials_path()?)
}

fn clear_credentials_at(path: &Path) -> Result<bool, String> {
    let dir = path.parent().ok_or("No credential directory")?;
    match fs::symlink_metadata(dir) {
        Ok(meta) => {
            if !meta.is_dir() || meta.file_type().is_symlink() {
                return Err("Refusing logout through a symlinked auth directory".into());
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if meta.permissions().mode() & 0o077 != 0 {
                    return Err("Refusing logout through an insecure auth directory".into());
                }
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(_) => return Err("Cannot inspect credential directory".into()),
    }

    match fs::symlink_metadata(path) {
        Ok(_) => {
            private_file(path)?;
            fs::remove_file(path).map_err(|_| "Could not remove Synmetra credentials")?;
            Ok(true)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(_) => Err("Could not inspect Synmetra credential file".into()),
    }
}

fn persist_credentials_at(path: &Path, stored: &Stored) -> Result<(), String> {
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
    let dir = path.parent().ok_or("No credential directory")?;
    fs::create_dir_all(dir).map_err(|_| "Cannot create private auth directory")?;
    let meta = fs::symlink_metadata(dir).map_err(|_| "Cannot stat auth directory")?;
    if !meta.is_dir() || meta.file_type().is_symlink() {
        return Err("Refusing credentials in a symlinked auth directory".into());
    }
    fs::set_permissions(dir, fs::Permissions::from_mode(0o700))
        .map_err(|_| "Cannot secure auth directory")?;
    // Refuse modifying or replacing a symlink at the destination.
    if fs::symlink_metadata(path).is_ok() {
        private_file(path)?;
    }
    let temp = dir.join(format!(".credentials.{}.tmp", std::process::id()));
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(&temp)
        .map_err(|_| "Cannot create private temporary credential file")?;
    let data = serde_json::to_vec(stored).map_err(|_| "Cannot serialize credentials")?;
    let result = (|| -> Result<(), String> {
        file.write_all(&data)
            .map_err(|_| "Cannot write credentials")?;
        file.sync_all().map_err(|_| "Cannot sync credentials")?;
        fs::rename(&temp, path).map_err(|_| "Cannot install credentials")?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}

fn persist_credentials(stored: &Stored) -> Result<(), String> {
    persist_credentials_at(&credentials_path()?, stored)
}

fn nonce() -> String {
    let mut data = [0u8; 32];
    OsRng.fill_bytes(&mut data);
    URL_SAFE_NO_PAD.encode(data)
}

fn authorization_url(
    client_id: &str,
    redirect: &str,
    state: &str,
    verifier: &str,
    scopes: &str,
) -> Url {
    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
    let mut url = Url::parse(AUTHORIZE).expect("fixed Google URL");
    url.query_pairs_mut()
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", redirect)
        .append_pair("response_type", "code")
        .append_pair("scope", scopes)
        .append_pair("access_type", "offline")
        .append_pair("prompt", "consent")
        .append_pair("state", state)
        .append_pair("code_challenge", &challenge)
        .append_pair("code_challenge_method", "S256");
    url
}

fn callback_code(request: &str, expected_state: &str) -> Result<String, String> {
    let line = request
        .lines()
        .next()
        .ok_or("Missing callback request line")?;
    let mut parts = line.split_whitespace();
    if parts.next() != Some("GET") {
        return Err("OAuth callback must use GET".into());
    }
    let path = parts.next().ok_or("Missing callback path")?;
    let parsed =
        Url::parse(&format!("http://localhost{path}")).map_err(|_| "Invalid callback URL")?;
    if parsed.path() != "/callback" {
        return Err("Unexpected OAuth callback path".into());
    }
    let mut state = None;
    let mut code = None;
    for (key, value) in parsed.query_pairs() {
        if key == "state" {
            state = Some(value.to_string());
        }
        if key == "code" {
            code = Some(value.to_string());
        }
        if key == "error" {
            return Err("Google rejected the OAuth login".into());
        }
    }
    if state.as_deref() != Some(expected_state) {
        return Err("OAuth state mismatch".into());
    }
    code.filter(|v| !v.is_empty() && v.len() < 4096)
        .ok_or("Missing authorization code".into())
}

async fn token_exchange(fields: &[(&str, &str)]) -> Result<TokenResponse, String> {
    let http = reqwest::Client::builder()
        .timeout(Duration::from_secs(20))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| "Failed to build token client")?;
    let response = http
        .post(TOKEN_ENDPOINT)
        .form(fields)
        .send()
        .await
        .map_err(|_| "Google token endpoint unavailable")?;
    if !response.status().is_success() {
        return Err(format!(
            "Google token endpoint returned HTTP {}",
            response.status().as_u16()
        ));
    }
    response
        .json::<TokenResponse>()
        .await
        .map_err(|_| "Malformed Google token response".into())
}

pub async fn access_token() -> Result<Option<String>, String> {
    static REFRESH_LOCK: tokio::sync::OnceCell<tokio::sync::Mutex<()>> =
        tokio::sync::OnceCell::const_new();
    let lock = REFRESH_LOCK
        .get_or_init(|| async { tokio::sync::Mutex::new(()) })
        .await;
    let _held = lock.lock().await;
    let Some(mut stored) = read_credentials()? else {
        return Ok(None);
    };
    if stored.expires_at > epoch_seconds().saturating_add(120) && !stored.access_token.is_empty() {
        return Ok(Some(stored.access_token));
    }
    let refresh = token_exchange(&[
        ("client_id", &stored.client_id),
        ("client_secret", &stored.client_secret),
        ("refresh_token", &stored.refresh_token),
        ("grant_type", "refresh_token"),
    ])
    .await?;
    if refresh.access_token.is_empty() || refresh.expires_in == 0 {
        return Err("Google returned invalid refreshed credentials".into());
    }
    if let Some(rotated) = refresh.refresh_token.filter(|s| !s.is_empty()) {
        stored.refresh_token = rotated;
    }
    stored.access_token = refresh.access_token;
    stored.expires_at = epoch_seconds().saturating_add(refresh.expires_in);
    persist_credentials(&stored)?;
    Ok(Some(stored.access_token))
}

#[derive(Debug, Default)]
struct LoginOptions {
    client_id_file: String,
    allow_edit: bool,
    headless: bool,
    account: Option<String>,
    callback_port: Option<u16>,
}

fn parse_login_options(args: &[String]) -> Result<LoginOptions, String> {
    let mut options = LoginOptions::default();
    let mut seen = std::collections::HashSet::new();
    let mut index = 0;
    while index < args.len() {
        let flag = args[index].as_str();
        if !seen.insert(flag) {
            return Err("Duplicate OAuth login option".into());
        }
        match flag {
            "--client-id-file" | "--account" | "--callback-port" => {
                let value = args
                    .get(index + 1)
                    .ok_or("Missing OAuth login option value")?;
                if value.is_empty()
                    || value.starts_with("--")
                    || value.chars().any(char::is_control)
                {
                    return Err("Invalid OAuth login option value".into());
                }
                match flag {
                    "--client-id-file" => {
                        if value.len() > 4096 {
                            return Err("OAuth client path too long".into());
                        }
                        options.client_id_file = value.clone();
                    }
                    "--account" => {
                        if value.len() > 254 {
                            return Err("Google account hint too long".into());
                        }
                        options.account = Some(value.clone());
                    }
                    "--callback-port" => {
                        if !value.bytes().all(|byte| byte.is_ascii_digit()) {
                            return Err("Invalid OAuth callback port".into());
                        }
                        options.callback_port = Some(
                            value
                                .parse::<u16>()
                                .map_err(|_| "Invalid OAuth callback port")?,
                        );
                    }
                    _ => unreachable!(),
                }
                index += 2;
            }
            "--headless" => {
                options.headless = true;
                index += 1;
            }
            "--allow-edit" => {
                options.allow_edit = true;
                index += 1;
            }
            _ => return Err("Unsupported OAuth login option".into()),
        }
    }
    if options.client_id_file.is_empty() {
        return Err("OAuth login requires --client-id-file".into());
    }
    Ok(options)
}

async fn login(options: &LoginOptions) -> Result<(), String> {
    let path = &options.client_id_file;
    let allow_edit = options.allow_edit;
    let file = fs::read(path).map_err(|_| "Cannot read Google OAuth Desktop client JSON")?;
    if file.len() > 128 * 1024 {
        return Err("OAuth client JSON too large".into());
    }
    let client: OAuthClientFile = serde_json::from_slice(&file)
        .map_err(|_| "Expected Google OAuth Desktop client JSON under installed")?;
    if client.installed.client_id.is_empty()
        || !client
            .installed
            .client_id
            .ends_with(".apps.googleusercontent.com")
    {
        return Err("Invalid Google Desktop OAuth client ID".into());
    }
    let listener = TcpListener::bind(format!("127.0.0.1:{}", options.callback_port.unwrap_or(0)))
        .await
        .map_err(|_| "Could not bind local OAuth callback")?;
    let redirect = format!(
        "http://127.0.0.1:{}/callback",
        listener
            .local_addr()
            .map_err(|_| "Could not read callback port")?
            .port()
    );
    let state = nonce();
    let verifier = nonce();
    let requested_scope = if allow_edit {
        format!("{SCOPE} {SCOPE_EDIT}")
    } else {
        SCOPE.to_owned()
    };
    let mut url = authorization_url(
        &client.installed.client_id,
        &redirect,
        &state,
        &verifier,
        &requested_scope,
    );
    if let Some(account) = options.account.as_deref() {
        url.query_pairs_mut().append_pair("login_hint", account);
    }
    eprintln!("Open this Google login URL in your browser:\n{url}\n");
    if !options.headless {
        let _ = std::process::Command::new("xdg-open")
            .arg(url.as_str())
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
    }
    let code = loop {
        let (mut socket, _) = timeout(Duration::from_secs(300), listener.accept())
            .await
            .map_err(|_| "Google login timed out")?
            .map_err(|_| "OAuth callback listener failed")?;
        let mut req = vec![0u8; 8192];
        let n = timeout(Duration::from_secs(10), socket.read(&mut req))
            .await
            .map_err(|_| "OAuth callback timed out")?
            .map_err(|_| "Cannot read callback")?;
        let input = std::str::from_utf8(&req[..n]).map_err(|_| "Callback encoding invalid")?;
        let code = callback_code(input, &state);
        let (status, body) = if code.is_ok() {
            (
                "200 OK",
                "Synmetra authorization received. You may close this page.",
            )
        } else {
            ("400 Bad Request", "Invalid authorization callback.")
        };
        let reply = format!("HTTP/1.1 {status}\r\nContent-Type: text/plain; charset=utf-8\r\nCache-Control: no-store\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{body}", body.len());
        let _ = socket.write_all(reply.as_bytes()).await;
        if let Ok(code) = code {
            break code;
        }
    };
    let token = token_exchange(&[
        ("client_id", &client.installed.client_id),
        ("client_secret", &client.installed.client_secret),
        ("code", &code),
        ("redirect_uri", &redirect),
        ("code_verifier", &verifier),
        ("grant_type", "authorization_code"),
    ])
    .await?;
    if token.access_token.is_empty() || token.expires_in == 0 {
        return Err("Google returned invalid credentials".into());
    }
    let granted_scope = token
        .scope
        .as_deref()
        .ok_or("Google did not report granted OAuth scopes")?;
    if !requested_scope.split_whitespace().all(|requested| {
        granted_scope
            .split_whitespace()
            .any(|scope| scope == requested)
    }) {
        return Err("Google did not grant every explicitly requested Analytics scope".into());
    }
    let refresh_token = token
        .refresh_token
        .filter(|t| !t.is_empty())
        .ok_or("Google did not return an offline refresh token; check Desktop OAuth setup")?;
    let stored = Stored {
        client_id: client.installed.client_id,
        client_secret: client.installed.client_secret,
        access_token: token.access_token,
        refresh_token,
        expires_at: epoch_seconds().saturating_add(token.expires_in),
        granted_scope: granted_scope.to_owned(),
    };
    persist_credentials(&stored)?;
    if allow_edit {
        println!("Google Analytics read+edit credentials saved privately. The annotation write MCP tool still requires --allow-write.");
    } else {
        println!("Google Analytics read-only credentials saved privately for Synmetra.");
    }
    Ok(())
}

pub fn public_status() -> serde_json::Value {
    let override_token =
        std::env::var("SYNMETRA_GOOGLE_ACCESS_TOKEN").is_ok_and(|value| !value.trim().is_empty());
    match read_credentials() {
        Ok(Some(config)) => serde_json::json!({
            "local_oauth_configured": true, "override_token_present": override_token,
            "cached_token_fresh": config.expires_at > epoch_seconds().saturating_add(120),
            "scope": config.granted_scope
        }),
        Ok(None) => serde_json::json!({
            "local_oauth_configured": false, "override_token_present": override_token,
            "scope": "none"
        }),
        Err(_) => serde_json::json!({
            "local_oauth_status": "unreadable_private_credentials",
            "override_token_present": override_token,
            "scope": "unknown"
        }),
    }
}

pub async fn command(args: &[String]) -> Result<(), String> {
    match args {
        [mode, rest @ ..] if mode == "login" => {
            let options = parse_login_options(rest)?;
            login(&options).await
        }
        [mode] if mode == "status" => {
            match read_credentials()? {
                Some(c) => {
                    println!(
                        "Synmetra OAuth credentials are configured ({}).",
                        if c.granted_scope.split_whitespace().any(|v| v == SCOPE_EDIT) {
                            "explicit read+edit"
                        } else {
                            "read-only"
                        }
                    );
                    println!(
                        "Current access token valid: {}",
                        c.expires_at > epoch_seconds().saturating_add(120)
                    );
                }
                None => println!("Synmetra has no saved OAuth credentials."),
            }
            Ok(())
        }
        [mode] if mode == "doctor" => {
            let status = public_status();
            println!("Synmetra local authentication diagnostic (no Google network call):");
            println!("{}", serde_json::to_string_pretty(&status).map_err(|_| "Cannot render auth status")?);
            if status["local_oauth_configured"] != true
                && status["override_token_present"] != true
            {
                println!("To configure native OAuth: synmetra auth login --client-id-file /path/to/desktop-client.json");
            } else if status["cached_token_fresh"] == false {
                println!("Cached access token may be stale; the next MCP request attempts a refresh.");
            }
            println!("Google Analytics Admin and Data APIs must be enabled in your Google Cloud project.");
            Ok(())
        }
        [mode] if mode == "command" => {
            println!("synmetra auth login --client-id-file /path/to/desktop-client.json");
            Ok(())
        }
        [mode] if mode == "logout" => {
            clear_credentials_at(&credentials_path()?)?;
            println!("Synmetra local credentials cleared. Revoke access in your Google account if desired.");
            Ok(())
        }
        _ => Err(
            "Usage: synmetra auth login --client-id-file PATH [--account EMAIL] [--callback-port PORT] [--headless] [--allow-edit] | status | doctor | command | logout"
                .into(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_flags_are_opt_in_and_reject_malformed_arguments() {
        let parse = |input: &[&str]| {
            parse_login_options(&input.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>())
        };
        let default = parse(&["--client-id-file", "/home/user/oauth.json"]).unwrap();
        assert!(!default.allow_edit);
        assert!(!default.headless);
        assert_eq!(default.callback_port, None);
        assert_eq!(default.account, None);

        let opts = parse(&[
            "--headless",
            "--account",
            "user@example.com",
            "--callback-port",
            "39721",
            "--client-id-file",
            "/home/user/My OAuth.json",
            "--allow-edit",
        ])
        .unwrap();
        assert!(opts.headless);
        assert!(opts.allow_edit);
        assert_eq!(opts.callback_port, Some(39721));
        assert_eq!(opts.account.as_deref(), Some("user@example.com"));

        for bad in [
            vec!["--client-id-file", "file", "--headless", "--headless"],
            vec!["--client-id-file", "file", "--callback-port", "-1"],
            vec!["--client-id-file", "file", "--callback-port", "65536"],
            vec!["--client-id-file", "file", "--account", "bad\nhint"],
            vec!["--client-id-file", "file", "--quota-project", "abc"],
            vec!["--allow-edit"],
        ] {
            assert!(parse(&bad).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn status_has_no_token_or_client_secret_fields() {
        let status = public_status();
        let object = status.as_object().unwrap();
        assert!(!object.contains_key("access_token"));
        assert!(!object.contains_key("refresh_token"));
        assert!(!object.contains_key("client_secret"));
        assert!(!object.contains_key("client_id"));
    }

    #[test]
    fn state_callback_rejects_errors_and_wrong_paths() {
        assert_eq!(
            callback_code("GET /callback?state=s&code=a HTTP/1.1\r\n", "s").unwrap(),
            "a"
        );
        for bad in [
            "GET /callback?state=other&code=secret HTTP/1.1",
            "GET /evil?state=s&code=a HTTP/1.1",
            "GET /callback?state=s HTTP/1.1",
            "POST /callback?state=s&code=a HTTP/1.1",
        ] {
            assert!(callback_code(bad, "s").is_err());
        }
    }

    #[cfg(unix)]
    #[test]
    fn credential_files_are_private_and_symlinks_are_rejected() {
        use std::os::unix::fs::{symlink, PermissionsExt};
        let base = std::env::temp_dir().join(format!(
            "synmetra-cred-test-{}-{}",
            std::process::id(),
            nonce()
        ));
        let dir = base.join("synmetra");
        let path = dir.join("credentials.json");
        let stored = Stored {
            client_id: "example.apps.googleusercontent.com".into(),
            client_secret: "synthetic-client".into(),
            refresh_token: "synthetic-refresh".into(),
            access_token: "synthetic-access".into(),
            expires_at: 42,
            granted_scope: SCOPE.to_owned(),
        };
        persist_credentials_at(&path, &stored).unwrap();
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o600
        );
        assert_eq!(
            fs::metadata(&dir).unwrap().permissions().mode() & 0o777,
            0o700
        );
        assert_eq!(
            read_credentials_at(&path).unwrap().unwrap().refresh_token,
            "synthetic-refresh"
        );

        let alias = dir.join("bad.json");
        symlink(&path, &alias).unwrap();
        assert!(read_credentials_at(&alias).is_err());
        assert!(persist_credentials_at(&alias, &stored).is_err());

        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
        assert!(read_credentials_at(&path).is_err());
        assert!(persist_credentials_at(&path, &stored).is_err());
        fs::remove_file(alias).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();

        let symlink_dir = base.join("linked");
        symlink(&dir, &symlink_dir).unwrap();
        assert!(read_credentials_at(&symlink_dir.join("credentials.json")).is_err());
        assert!(persist_credentials_at(&symlink_dir.join("credentials.json"), &stored).is_err());
        fs::remove_dir_all(base).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn logout_removes_only_private_synmetra_credentials_without_touching_neighbors() {
        use std::os::unix::fs::{symlink, PermissionsExt};

        let base = std::env::temp_dir().join(format!(
            "synmetra-logout-test-{}-{}",
            std::process::id(),
            nonce()
        ));
        let path = base.join("synmetra").join("credentials.json");
        let neighbor = base.join("other-google-app.json");
        fs::create_dir_all(&base).unwrap();
        fs::write(&neighbor, "unrelated application data").unwrap();
        let stored = Stored {
            client_id: "sample.apps.googleusercontent.com".into(),
            client_secret: "synthetic-client".into(),
            refresh_token: "synthetic-refresh".into(),
            access_token: "synthetic-access".into(),
            expires_at: 42,
            granted_scope: SCOPE.into(),
        };
        persist_credentials_at(&path, &stored).unwrap();
        assert_eq!(clear_credentials_at(&path), Ok(true));
        assert_eq!(clear_credentials_at(&path), Ok(false));
        assert!(!path.exists());
        assert_eq!(
            fs::read_to_string(&neighbor).unwrap(),
            "unrelated application data"
        );

        symlink(&neighbor, &path).unwrap();
        assert!(clear_credentials_at(&path).is_err());
        assert_eq!(
            fs::read_to_string(&neighbor).unwrap(),
            "unrelated application data"
        );
        fs::remove_file(&path).unwrap();

        persist_credentials_at(&path, &stored).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
        assert!(clear_credentials_at(&path).is_err());
        assert!(path.exists());
        fs::remove_file(&path).unwrap();

        let alias_dir = base.join("linked-synmetra");
        symlink(path.parent().unwrap(), &alias_dir).unwrap();
        assert!(clear_credentials_at(&alias_dir.join("credentials.json")).is_err());
        assert!(neighbor.exists());
        fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn edit_scopes_require_explicit_consent_in_authorization_url() {
        let verifier = nonce();
        let state = nonce();
        let scope = format!("{SCOPE} {SCOPE_EDIT}");
        let url = authorization_url(
            "sample.apps.googleusercontent.com",
            "http://127.0.0.1:5000/callback",
            &state,
            &verifier,
            &scope,
        );
        let pairs: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
        assert_eq!(pairs["scope"], scope);
        assert!(pairs["scope"].split_whitespace().any(|v| v == SCOPE_EDIT));
        assert_eq!(pairs["code_challenge_method"], "S256");
        assert!(!url.as_str().contains(&verifier));
    }

    #[test]
    fn pkce_authorization_is_readonly_and_state_bound() {
        let verifier = nonce();
        let state = nonce();
        assert!(verifier.len() >= 43 && state.len() >= 43);
        let url = authorization_url(
            "sample.apps.googleusercontent.com",
            "http://127.0.0.1:5000/callback",
            &state,
            &verifier,
            SCOPE,
        );
        let pairs: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
        assert_eq!(pairs["scope"], SCOPE);
        assert_eq!(pairs["state"], state);
        assert_eq!(pairs["code_challenge_method"], "S256");
        assert!(!url.as_str().contains(&verifier));
        assert!(!url.as_str().contains("analytics.edit"));
    }
}
