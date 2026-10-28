use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `FKTaxOffices`.
pub struct IFKTaxOfficesController;

#[allow(non_snake_case)]
impl IFKTaxOfficesController {
    /// `GET` `/api/FKTaxOffices`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::common::view_models::TaxOffice>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::TaxOffice>>(
            HttpMethod::Get,
            "/api/FKTaxOffices",
            query,
        )
        .await
    }

    /// `GET` `/api/FKTaxOffices`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/FKTaxOffices", query)
            .await
    }
}
