use serde::{Deserialize, Serialize};

use super::auth::current_timestamp;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuotaStoreFile {
    pub items: Vec<QuotaStoreItem>,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaStoreItem {
    pub account_key: String,
    pub captured_at: i64,
    pub last_updated_at: i64,
    pub primary_window_remaining: Option<i64>,
    pub secondary_window_remaining: Option<i64>,
    pub token_status: Option<String>,
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

pub fn find_item(store: &QuotaStoreFile, account_key: &str) -> Option<&QuotaStoreItem> {
    store
        .items
        .iter()
        .find(|item| item.account_key == account_key)
}

pub fn upsert_item(
    store: &mut QuotaStoreFile,
    item: QuotaStoreItem,
    ts: i64,
) -> bool {
    let existing = store
        .items
        .iter_mut()
        .find(|i| i.account_key == item.account_key);
    if let Some(existing) = existing {
        existing.clone_from(&item);
        existing.last_updated_at = ts;
    } else {
        let mut new_item = item;
        new_item.last_updated_at = ts;
        store.items.push(new_item);
    }
    true
}

pub fn save(path: &std::path::Path, store: &QuotaStoreFile) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("Failed to create dir: {}", e))?;
    }
    let content =
        serde_json::to_string_pretty(store).map_err(|e| format!("Failed to serialize: {}", e))?;
    std::fs::write(path, content).map_err(|e| format!("Failed to write: {}", e))?;
    Ok(())
}
