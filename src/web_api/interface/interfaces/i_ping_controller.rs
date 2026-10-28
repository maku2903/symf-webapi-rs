use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `Ping`.
pub struct IPingController;

#[allow(non_snake_case)]
impl IPingController {
    /// `GET` `/api/Ping`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<crate::web_api::interface::view_models::Ping>, ApiError> {
        let query = vec![];
        api.request_no_body::<crate::web_api::interface::view_models::Ping>(
            HttpMethod::Get,
            "/api/Ping",
            query,
        )
        .await
    }

    /// `GET` `/api/Ping`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Ping", query)
            .await
    }
}
