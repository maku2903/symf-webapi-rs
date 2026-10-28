use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `FKCurrencies`.
pub struct IFKCurrenciesController;

#[allow(non_snake_case)]
impl IFKCurrenciesController {
    /// `GET` `/api/FKCurrencies`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::common::view_models::Currency>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::Currency>>(
            HttpMethod::Get,
            "/api/FKCurrencies",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCurrencies`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/FKCurrencies", query)
            .await
    }

    /// `GET` `/api/FKCurrencies/InTable`
    pub async fn GetCurrencyInTable(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::common::view_models::Currency>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::Currency>>(
            HttpMethod::Get,
            "/api/FKCurrencies/InTable",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCurrencies/InTable`
    pub async fn GetCurrencyInTableRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/FKCurrencies/InTable", query)
            .await
    }
}
