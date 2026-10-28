use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `CompanyInformation`.
pub struct ICompanyInformationController;

#[allow(non_snake_case)]
impl ICompanyInformationController {
    /// `GET` `/api/CompanyInformation`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<crate::web_api::interface::view_models::CompanyInfo>, ApiError>
    {
        let query = vec![];
        api.request_no_body::<crate::web_api::interface::view_models::CompanyInfo>(
            HttpMethod::Get,
            "/api/CompanyInformation",
            query,
        )
        .await
    }

    /// `GET` `/api/CompanyInformation`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/CompanyInformation", query)
            .await
    }
}
