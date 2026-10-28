use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `AgriculturalPromotionFunds`.
pub struct IAgriculturalPromotionFundsController;

#[allow(non_snake_case)]
impl IAgriculturalPromotionFundsController {
    /// `GET` `/api/AgriculturalPromotionFunds`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::common::view_models::AgriculturalPromotionFund>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::AgriculturalPromotionFund>>(HttpMethod::Get, "/api/AgriculturalPromotionFunds", query).await
    }

    /// `GET` `/api/AgriculturalPromotionFunds`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/AgriculturalPromotionFunds", query)
            .await
    }
}
