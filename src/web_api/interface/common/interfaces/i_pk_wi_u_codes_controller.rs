use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `PKWiUCodes`.
pub struct IPKWiUCodesController;

#[allow(non_snake_case)]
impl IPKWiUCodesController {
    /// `GET` `/api/PKWiUCodes`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::PKWiUCode>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::PKWiUCode>>(
            HttpMethod::Get,
            "/api/PKWiUCodes",
            query,
        )
        .await
    }

    /// `GET` `/api/PKWiUCodes`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/PKWiUCodes", query)
            .await
    }
}
