use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `FKVatRegisterMarkers`.
pub struct IFKVatRegisterMarkersController;

#[allow(non_snake_case)]
impl IFKVatRegisterMarkersController {
    /// `GET` `/api/FKVatRegisterMarkers`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::common::view_models::VatRegisterMarker>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::VatRegisterMarker>>(HttpMethod::Get, "/api/FKVatRegisterMarkers", query).await
    }

    /// `GET` `/api/FKVatRegisterMarkers`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/FKVatRegisterMarkers", query)
            .await
    }
}
