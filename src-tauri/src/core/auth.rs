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
    #[serde(default)]
    pub accounts: Vec<AuthAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAccount {
    pub account_key: String,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthSnapshot {
    pub account_key: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub account_name: Option<String>,
    #[serde(default)]
    pub workspace_name: Option<String>,
    #[serde(default)]
    pub profile_name: Option<String>,
    #[serde(default)]
    pub plan: String,
    #[serde(default)]
    pub auth_mode: String,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub captured_at: Option<i64>,
    #[serde(default)]
    pub accounts: Vec<AuthAccount>,
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

pub fn make_auth_snapshot(
    auth: &AuthFile,
    _auth_path: &std::path::Path,
) -> Result<AuthSnapshot, String> {
    let ts = current_timestamp();
    match auth.accounts.first() {
        Some(acc) => Ok(AuthSnapshot {
            account_key: acc.account_key.clone(),
            email: String::new(),
            account_name: None,
            workspace_name: None,
            profile_name: None,
            plan: String::new(),
            auth_mode: String::new(),
            created_at: Some(ts),
            captured_at: Some(ts),
            accounts: auth.accounts.clone(),
        }),
        None => Ok(AuthSnapshot {
            account_key: String::new(),
            email: String::new(),
            account_name: None,
            workspace_name: None,
            profile_name: None,
            plan: String::new(),
            auth_mode: String::new(),
            created_at: None,
            captured_at: Some(ts),
            accounts: vec![],
        }),
    }
}
