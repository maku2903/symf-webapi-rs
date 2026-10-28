use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `Alive`.
pub struct IAliveController;

#[allow(non_snake_case)]
impl IAliveController {
    /// `GET` `/api/Alive`
    pub async fn Get(api: &ApiClient) -> Result<ResponseEnvelope<()>, ApiError> {
        let query = vec![];
        api.request_no_body::<()>(HttpMethod::Get, "/api/Alive", query)
            .await
    }

    /// `GET` `/api/Alive`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Alive", query)
            .await
    }
}
