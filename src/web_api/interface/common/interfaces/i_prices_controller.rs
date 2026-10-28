use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `Prices`.
pub struct IPricesController;

#[allow(non_snake_case)]
impl IPricesController {
    /// `GET` `/api/Prices/SalePriceTypes`
    pub async fn GetSalePriceTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::PriceType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::PriceType>>(
            HttpMethod::Get,
            "/api/Prices/SalePriceTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Prices/SalePriceTypes`
    pub async fn GetSalePriceTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Prices/SalePriceTypes", query)
            .await
    }
}
