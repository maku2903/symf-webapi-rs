use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `FKRegions`.
pub struct IFKRegionsController;

#[allow(non_snake_case)]
impl IFKRegionsController {
    /// `GET` `/api/FKRegions`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::common::view_models::Region>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::Region>>(
            HttpMethod::Get,
            "/api/FKRegions",
            query,
        )
        .await
    }

    /// `GET` `/api/FKRegions`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/FKRegions", query)
            .await
    }
}
