use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `Departments`.
pub struct IDepartmentsController;

#[allow(non_snake_case)]
impl IDepartmentsController {
    /// `GET` `/api/Departments`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Department>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Department>>(
            HttpMethod::Get,
            "/api/Departments",
            query,
        )
        .await
    }

    /// `GET` `/api/Departments`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Departments", query)
            .await
    }
}
