use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `Licence`.
pub struct ILicenceController;

#[allow(non_snake_case)]
impl ILicenceController {
    /// `GET` `/api/Licence`
    pub async fn Get(api: &ApiClient) -> Result<ResponseEnvelope<()>, ApiError> {
        let query = vec![];
        api.request_no_body::<()>(HttpMethod::Get, "/api/Licence", query)
            .await
    }

    /// `GET` `/api/Licence`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Licence", query)
            .await
    }
}
