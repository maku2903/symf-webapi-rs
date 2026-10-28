use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `Currencies`.
pub struct ICurrenciesController;

#[allow(non_snake_case)]
impl ICurrenciesController {
    /// `GET` `/api/Currencies`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Currency>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Currency>>(
            HttpMethod::Get,
            "/api/Currencies",
            query,
        )
        .await
    }

    /// `GET` `/api/Currencies`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Currencies", query)
            .await
    }
}
