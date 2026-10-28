use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `VatRates`.
pub struct IVatRatesController;

#[allow(non_snake_case)]
impl IVatRatesController {
    /// `GET` `/api/VatRates`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::VatRate>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::VatRate>>(
            HttpMethod::Get,
            "/api/VatRates",
            query,
        )
        .await
    }

    /// `GET` `/api/VatRates`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/VatRates", query)
            .await
    }
}
