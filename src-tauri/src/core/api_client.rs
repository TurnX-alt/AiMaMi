use super::auth::ApiRequestContext;
use super::models::{ApiProxyConfigPayload, ApiProxyDetectPayload, ApiProxyTestPayload};

pub fn test_api_connectivity(
    _config: &ApiProxyConfigPayload,
    _context: Option<&ApiRequestContext>,
) -> ApiProxyTestPayload {
    ApiProxyTestPayload {
        code: "stub".into(),
        reachable: false,
        status_code: None,
        message: "api_client stub: not implemented in open-source build".into(),
    }
}

pub fn detect_api_proxy_config(
    _context: Option<&ApiRequestContext>,
) -> ApiProxyDetectPayload {
    ApiProxyDetectPayload {
        found: false,
        mode: None,
        url: None,
        probe: ApiProxyTestPayload {
            code: "stub".into(),
            reachable: false,
            status_code: None,
            message: "api_client stub: not implemented".into(),
        },
    }
}

pub fn sanitize_proxy_config(
    config: &ApiProxyConfigPayload,
) -> Result<ApiProxyConfigPayload, super::models::CoreError> {
    Ok(config.clone())
}
