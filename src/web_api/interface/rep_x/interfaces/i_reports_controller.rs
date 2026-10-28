use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `Reports`.
pub struct IReportsController;

#[allow(non_snake_case)]
impl IReportsController {
    /// `PATCH` `/api/Reports`
    ///
    /// Parameter `report_id`: Id wydruku.
    ///
    /// Parameter `object_id`: Id obiektu.
    ///
    /// Body `data`: Ustawienia wydruku.
    pub async fn Get(
        api: &ApiClient,
        report_id: i32,
        object_id: i32,
        data: &crate::web_api::interface::rep_x::view_models::PrintData,
    ) -> Result<ResponseEnvelope<crate::web_api::interface::common::view_models::PDF>, ApiError>
    {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "reportId", &report_id)?;
        push_query_param(&mut query, "objectId", &object_id)?;
        api.request_with_body::<crate::web_api::interface::common::view_models::PDF, _>(
            HttpMethod::Patch,
            "/api/Reports",
            query,
            data,
        )
        .await
    }

    /// `PATCH` `/api/Reports`
    ///
    /// Parameter `report_id`: Id wydruku.
    ///
    /// Parameter `object_id`: Id obiektu.
    ///
    /// Body `data`: Ustawienia wydruku.
    pub async fn GetRaw(
        api: &ApiClient,
        report_id: i32,
        object_id: i32,
        data: &crate::web_api::interface::rep_x::view_models::PrintData,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "reportId", &report_id)?;
        push_query_param(&mut query, "objectId", &object_id)?;
        api.request_with_body_raw(HttpMethod::Patch, "/api/Reports", query, data)
            .await
    }

    /// `GET` `/api/Reports/Definitions`
    pub async fn GetDefinitions(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::rep_x::view_models::ReportDefinition>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::rep_x::view_models::ReportDefinition>>(
            HttpMethod::Get,
            "/api/Reports/Definitions",
            query,
        )
        .await
    }

    /// `GET` `/api/Reports/Definitions`
    pub async fn GetDefinitionsRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Reports/Definitions", query)
            .await
    }
}
