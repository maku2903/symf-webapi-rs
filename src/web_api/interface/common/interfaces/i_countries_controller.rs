use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `Countries`.
pub struct ICountriesController;

#[allow(non_snake_case)]
impl ICountriesController {
    /// `GET` `/api/Countries`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Country>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Country>>(
            HttpMethod::Get,
            "/api/Countries",
            query,
        )
        .await
    }

    /// `GET` `/api/Countries`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Countries", query)
            .await
    }
}
