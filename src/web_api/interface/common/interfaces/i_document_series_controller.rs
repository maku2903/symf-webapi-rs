use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `DocumentSeries`.
pub struct IDocumentSeriesController;

#[allow(non_snake_case)]
impl IDocumentSeriesController {
    /// `GET` `/api/DocumentSeries`
    ///
    /// Parameter `document_type_id`: Id typu dokumentu.
    pub async fn Get(
        api: &ApiClient,
        document_type_id: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentSeries>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentTypeId", &document_type_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentSeries>>(
            HttpMethod::Get,
            "/api/DocumentSeries",
            query,
        )
        .await
    }

    /// `GET` `/api/DocumentSeries`
    ///
    /// Parameter `document_type_id`: Id typu dokumentu.
    pub async fn GetRaw(
        api: &ApiClient,
        document_type_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentTypeId", &document_type_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/DocumentSeries", query)
            .await
    }
}
