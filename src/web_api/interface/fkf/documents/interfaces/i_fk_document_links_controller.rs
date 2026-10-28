use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `FKDocumentLinks`.
pub struct IFKDocumentLinksController;

#[allow(non_snake_case)]
impl IFKDocumentLinksController {
    /// `POST` `/api/FKDocumentLinks/AddNew`
    ///
    /// Parameter `document_id`: Id dokumentu.
    ///
    /// Parameter `year_id`: Id roku.
    ///
    /// Body `new_document_link`: Obiekt linku.
    pub async fn AddNew(
        api: &ApiClient,
        document_id: i32,
        year_id: i32,
        new_document_link: &crate::web_api::interface::fkf::documents::view_models::rdf::RdfDocumentLink,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        push_query_param(&mut query, "yearId", &year_id)?;
        api.request_with_body::<(), _>(
            HttpMethod::Post,
            "/api/FKDocumentLinks/AddNew",
            query,
            new_document_link,
        )
        .await
    }

    /// `POST` `/api/FKDocumentLinks/AddNew`
    ///
    /// Parameter `document_id`: Id dokumentu.
    ///
    /// Parameter `year_id`: Id roku.
    ///
    /// Body `new_document_link`: Obiekt linku.
    pub async fn AddNewRaw(
        api: &ApiClient,
        document_id: i32,
        year_id: i32,
        new_document_link: &crate::web_api::interface::fkf::documents::view_models::rdf::RdfDocumentLink,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        push_query_param(&mut query, "yearId", &year_id)?;
        api.request_with_body_raw(
            HttpMethod::Post,
            "/api/FKDocumentLinks/AddNew",
            query,
            new_document_link,
        )
        .await
    }

    /// `PUT` `/api/FKDocumentLinks/Update`
    ///
    /// Parameter `document_id`: Id dokumentu.
    ///
    /// Parameter `year_id`: Id roku.
    ///
    /// Body `new_document_link`: Obiekt linku.
    pub async fn Update(
        api: &ApiClient,
        document_id: i32,
        year_id: i32,
        new_document_link: &crate::web_api::interface::fkf::documents::view_models::rdf::RdfDocumentLink,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        push_query_param(&mut query, "yearId", &year_id)?;
        api.request_with_body::<(), _>(
            HttpMethod::Put,
            "/api/FKDocumentLinks/Update",
            query,
            new_document_link,
        )
        .await
    }

    /// `PUT` `/api/FKDocumentLinks/Update`
    ///
    /// Parameter `document_id`: Id dokumentu.
    ///
    /// Parameter `year_id`: Id roku.
    ///
    /// Body `new_document_link`: Obiekt linku.
    pub async fn UpdateRaw(
        api: &ApiClient,
        document_id: i32,
        year_id: i32,
        new_document_link: &crate::web_api::interface::fkf::documents::view_models::rdf::RdfDocumentLink,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        push_query_param(&mut query, "yearId", &year_id)?;
        api.request_with_body_raw(
            HttpMethod::Put,
            "/api/FKDocumentLinks/Update",
            query,
            new_document_link,
        )
        .await
    }
}
