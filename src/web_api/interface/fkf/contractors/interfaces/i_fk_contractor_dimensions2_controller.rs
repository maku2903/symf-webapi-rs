use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `FKContractorDimensions2`.
pub struct IFKContractorDimensions2Controller;

#[allow(non_snake_case)]
impl IFKContractorDimensions2Controller {
    /// `GET` `/api/v2/FKContractorDimensions`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    pub async fn GetById(
        api: &ApiClient,
        contractor_id: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::common::view_models::Dimension>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorId", &contractor_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::Dimension>>(
            HttpMethod::Get,
            "/api/v2/FKContractorDimensions",
            query,
        )
        .await
    }

    /// `GET` `/api/v2/FKContractorDimensions`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    pub async fn GetByIdRaw(
        api: &ApiClient,
        contractor_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorId", &contractor_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/v2/FKContractorDimensions", query)
            .await
    }

    /// `GET` `/api/v2/FKContractorDimensions`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    pub async fn GetByPosition(
        api: &ApiClient,
        contractor_position: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::common::view_models::Dimension>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::Dimension>>(
            HttpMethod::Get,
            "/api/v2/FKContractorDimensions",
            query,
        )
        .await
    }

    /// `GET` `/api/v2/FKContractorDimensions`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    pub async fn GetByPositionRaw(
        api: &ApiClient,
        contractor_position: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/v2/FKContractorDimensions", query)
            .await
    }

    /// `GET` `/api/v2/FKContractorDimensions`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    pub async fn GetByCode(
        api: &ApiClient,
        contractor_code: String,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::common::view_models::Dimension>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorCode", &contractor_code)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::Dimension>>(
            HttpMethod::Get,
            "/api/v2/FKContractorDimensions",
            query,
        )
        .await
    }

    /// `GET` `/api/v2/FKContractorDimensions`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    pub async fn GetByCodeRaw(
        api: &ApiClient,
        contractor_code: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorCode", &contractor_code)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/v2/FKContractorDimensions", query)
            .await
    }
}
