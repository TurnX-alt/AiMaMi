use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageAnalyticsPayload {
    #[serde(default)]
    pub usage_source: Option<String>,
    #[serde(default)]
    pub metrics: Option<serde_json::Value>,
}
