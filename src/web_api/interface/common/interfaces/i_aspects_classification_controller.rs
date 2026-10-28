use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `AspectsClassification`.
pub struct IAspectsClassificationController;

#[allow(non_snake_case)]
impl IAspectsClassificationController {
    /// `GET` `/api/AspectsClassification`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::common::view_models::DimensionClassification>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DimensionClassification>>(HttpMethod::Get, "/api/AspectsClassification", query).await
    }

    /// `GET` `/api/AspectsClassification`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/AspectsClassification", query)
            .await
    }
}
