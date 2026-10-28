use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `UnitsOfMeasurement`.
pub struct IUnitsOfMeasurementController;

#[allow(non_snake_case)]
impl IUnitsOfMeasurementController {
    /// `GET` `/api/UnitsOfMeasurement`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::UnitOfMeasurement>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::UnitOfMeasurement>>(HttpMethod::Get, "/api/UnitsOfMeasurement", query).await
    }

    /// `GET` `/api/UnitsOfMeasurement`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/UnitsOfMeasurement", query)
            .await
    }

    /// `GET` `/api/UnitsOfMeasurement/Definition`
    pub async fn GetDefinition(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::common::view_models::UnitOfMeasurementDefinition>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::UnitOfMeasurementDefinition>>(HttpMethod::Get, "/api/UnitsOfMeasurement/Definition", query).await
    }

    /// `GET` `/api/UnitsOfMeasurement/Definition`
    pub async fn GetDefinitionRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/UnitsOfMeasurement/Definition", query)
            .await
    }
}
