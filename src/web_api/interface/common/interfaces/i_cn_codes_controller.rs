use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `CNCodes`.
pub struct ICNCodesController;

#[allow(non_snake_case)]
impl ICNCodesController {
    /// `GET` `/api/CNCodes`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::CNCode>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::CNCode>>(
            HttpMethod::Get,
            "/api/CNCodes",
            query,
        )
        .await
    }

    /// `GET` `/api/CNCodes`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/CNCodes", query)
            .await
    }
}
