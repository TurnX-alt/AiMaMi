use serde::{Deserialize, Serialize};

use super::models::CoreSnapshotPayload;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BootstrapStatePayload {
    pub boot_count: u64,
    pub first_boot_at: Option<i64>,
    pub last_boot_at: Option<i64>,
    pub written_at: Option<i64>,
    pub snapshot_progressive: Option<CoreSnapshotPayload>,
    pub usage_analytics: Option<serde_json::Value>,
    pub mcp_servers: Option<serde_json::Value>,
    pub installed_skills: Option<serde_json::Value>,
}

pub fn load(path: &std::path::Path) -> BootstrapStatePayload {
    if path.exists() {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        BootstrapStatePayload::default()
    }
}

pub fn update<F>(path: &std::path::Path, apply: F) -> Result<(), super::models::CoreError>
where
    F: FnMut(&mut BootstrapStatePayload),
{
    let mut state = load(path);
    let mut apply = apply;
    apply(&mut state);
    let content = serde_json::to_string_pretty(&state)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content)?;
    Ok(())
}
