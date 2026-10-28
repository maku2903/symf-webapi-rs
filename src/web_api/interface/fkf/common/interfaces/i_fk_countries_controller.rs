use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `FKCountries`.
pub struct IFKCountriesController;

#[allow(non_snake_case)]
impl IFKCountriesController {
    /// `GET` `/api/FKCountries`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::common::view_models::Country>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::Country>>(
            HttpMethod::Get,
            "/api/FKCountries",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCountries`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/FKCountries", query)
            .await
    }
}
