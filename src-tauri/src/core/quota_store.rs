use serde::{Deserialize, Serialize};

use super::models::{CoreError, RateLimitWindow, UsageSource};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaStoreItem {
    pub account_key: String,
    pub captured_at: i64,
    #[serde(default)]
    pub last_updated_at: i64,
    pub usage_source: UsageSource,
    #[serde(default)]
    pub primary_window: Option<RateLimitWindow>,
    #[serde(default)]
    pub secondary_window: Option<RateLimitWindow>,
    #[serde(default)]
    pub primary_window_remaining: Option<i64>,
    #[serde(default)]
    pub secondary_window_remaining: Option<i64>,
    #[serde(default)]
    pub token_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuotaStoreFile {
    #[serde(default)]
    pub items: Vec<QuotaStoreItem>,
    #[serde(default)]
    pub updated_at: i64,
}

pub fn load_or_default(path: &std::path::Path) -> QuotaStoreFile {
    if path.exists() {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        QuotaStoreFile::default()
    }
}

pub fn find_item<'a>(store: &'a QuotaStoreFile, account_key: &str) -> Option<&'a QuotaStoreItem> {
    store
        .items
        .iter()
        .find(|item| item.account_key == account_key)
}

pub fn upsert_item(
    store: &mut QuotaStoreFile,
    item: QuotaStoreItem,
    _ts: i64,
) -> bool {
    store.items.push(item);
    true
}

pub fn save(path: &std::path::Path, store: &QuotaStoreFile) -> Result<(), CoreError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(store)?;
    std::fs::write(path, content)?;
    Ok(())
}
