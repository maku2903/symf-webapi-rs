use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `FKVatRegisters`.
pub struct IFKVatRegistersController;

#[allow(non_snake_case)]
impl IFKVatRegistersController {
    /// `GET` `/api/FKVatRegisters`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::common::view_models::VatRegister>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::VatRegister>>(HttpMethod::Get, "/api/FKVatRegisters", query).await
    }

    /// `GET` `/api/FKVatRegisters`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/FKVatRegisters", query)
            .await
    }
}
