use serde::{Deserialize, Serialize};

pub fn current_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequestContext {
    pub auth_token: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthFile {
    pub accounts: Vec<AuthAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAccount {
    pub account_key: String,
    pub token: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub key: String,
}

pub fn load_auth_file(path: &std::path::Path) -> Result<AuthFile, String> {
    if path.exists() {
        let content =
            std::fs::read_to_string(path).map_err(|e| format!("Failed to read auth: {}", e))?;
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse auth: {}", e))
    } else {
        Ok(AuthFile::default())
    }
}

pub fn make_api_request_context(auth: &AuthFile) -> Option<ApiRequestContext> {
    auth.accounts.first().map(|acc| ApiRequestContext {
        auth_token: acc.token.clone(),
        session_id: acc.session_id.clone(),
    })
}
