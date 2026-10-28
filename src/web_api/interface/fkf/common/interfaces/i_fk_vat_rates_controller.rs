use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `FKVatRates`.
pub struct IFKVatRatesController;

#[allow(non_snake_case)]
impl IFKVatRatesController {
    /// `GET` `/api/FKVatRates`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::common::view_models::VatRate>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::VatRate>>(
            HttpMethod::Get,
            "/api/FKVatRates",
            query,
        )
        .await
    }

    /// `GET` `/api/FKVatRates`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/FKVatRates", query)
            .await
    }
}
