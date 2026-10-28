use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `PaymentRegistries`.
pub struct IPaymentRegistriesController;

#[allow(non_snake_case)]
impl IPaymentRegistriesController {
    /// `GET` `/api/PaymentRegistries`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::PaymentRegistry>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::PaymentRegistry>>(
            HttpMethod::Get,
            "/api/PaymentRegistries",
            query,
        )
        .await
    }

    /// `GET` `/api/PaymentRegistries`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/PaymentRegistries", query)
            .await
    }
}
