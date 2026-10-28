use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `FKDimensions`.
pub struct IFKDimensionsController;

#[allow(non_snake_case)]
impl IFKDimensionsController {
    /// `GET` `/api/FKDimensions/AccountChart`
    ///
    /// Parameter `id`: Id konta.
    ///
    /// Parameter `year_id`: Id roku.
    pub async fn GetAccountChartByIdAndYear(
        api: &ApiClient,
        id: i32,
        year_id: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::common::view_models::Dimension>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        push_query_param(&mut query, "yearId", &year_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::Dimension>>(
            HttpMethod::Get,
            "/api/FKDimensions/AccountChart",
            query,
        )
        .await
    }

    /// `GET` `/api/FKDimensions/AccountChart`
    ///
    /// Parameter `id`: Id konta.
    ///
    /// Parameter `year_id`: Id roku.
    pub async fn GetAccountChartByIdAndYearRaw(
        api: &ApiClient,
        id: i32,
        year_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        push_query_param(&mut query, "yearId", &year_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKDimensions/AccountChart", query)
            .await
    }

    /// `GET` `/api/FKDimensions/Document`
    ///
    /// Parameter `id`: Id dokumentu.
    ///
    /// Parameter `year_id`: Id roku.
    pub async fn GetDocumentByIdAndYear(
        api: &ApiClient,
        id: i32,
        year_id: i32,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::fkf::dimensions::view_models::Document>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        push_query_param(&mut query, "yearId", &year_id)?;
        api.request_no_body::<crate::web_api::interface::fkf::dimensions::view_models::Document>(
            HttpMethod::Get,
            "/api/FKDimensions/Document",
            query,
        )
        .await
    }

    /// `GET` `/api/FKDimensions/Document`
    ///
    /// Parameter `id`: Id dokumentu.
    ///
    /// Parameter `year_id`: Id roku.
    pub async fn GetDocumentByIdAndYearRaw(
        api: &ApiClient,
        id: i32,
        year_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        push_query_param(&mut query, "yearId", &year_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKDimensions/Document", query)
            .await
    }

    /// `GET` `/api/FKDimensions/Classification`
    pub async fn GetClassification(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::common::view_models::DimensionClassification>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::DimensionClassification>>(HttpMethod::Get, "/api/FKDimensions/Classification", query).await
    }

    /// `GET` `/api/FKDimensions/Classification`
    pub async fn GetClassificationRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/FKDimensions/Classification", query)
            .await
    }
}
