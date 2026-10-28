use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `Warehouses`.
pub struct IWarehousesController;

#[allow(non_snake_case)]
impl IWarehousesController {
    /// `GET` `/api/Warehouses`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Warehouse>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Warehouse>>(
            HttpMethod::Get,
            "/api/Warehouses",
            query,
        )
        .await
    }

    /// `GET` `/api/Warehouses`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Warehouses", query)
            .await
    }
}
