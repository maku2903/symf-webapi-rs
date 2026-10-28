use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `Regions`.
pub struct IRegionsController;

#[allow(non_snake_case)]
impl IRegionsController {
    /// `GET` `/api/Regions`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Region>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Region>>(
            HttpMethod::Get,
            "/api/Regions",
            query,
        )
        .await
    }

    /// `GET` `/api/Regions`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Regions", query)
            .await
    }
}
