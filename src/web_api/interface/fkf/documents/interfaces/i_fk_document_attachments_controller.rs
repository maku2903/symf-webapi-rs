use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `FKDocumentAttachments`.
pub struct IFKDocumentAttachmentsController;

#[allow(non_snake_case)]
impl IFKDocumentAttachmentsController {
    /// `POST` `/api/FKDocumentAttachments/AddNew`
    ///
    /// Parameter `document_id`: Id dokumentu.
    ///
    /// Parameter `year_id`: Id roku.
    ///
    /// Body `new_document_attachment`: Obiekt załącznika.
    pub async fn AddNew(
        api: &ApiClient,
        document_id: i32,
        year_id: i32,
        new_document_attachment: &crate::web_api::interface::fkf::documents::view_models::rdf::RdfDocumentAttachment,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        push_query_param(&mut query, "yearId", &year_id)?;
        api.request_with_body::<(), _>(
            HttpMethod::Post,
            "/api/FKDocumentAttachments/AddNew",
            query,
            new_document_attachment,
        )
        .await
    }

    /// `POST` `/api/FKDocumentAttachments/AddNew`
    ///
    /// Parameter `document_id`: Id dokumentu.
    ///
    /// Parameter `year_id`: Id roku.
    ///
    /// Body `new_document_attachment`: Obiekt załącznika.
    pub async fn AddNewRaw(
        api: &ApiClient,
        document_id: i32,
        year_id: i32,
        new_document_attachment: &crate::web_api::interface::fkf::documents::view_models::rdf::RdfDocumentAttachment,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        push_query_param(&mut query, "yearId", &year_id)?;
        api.request_with_body_raw(
            HttpMethod::Post,
            "/api/FKDocumentAttachments/AddNew",
            query,
            new_document_attachment,
        )
        .await
    }

    /// `PUT` `/api/FKDocumentAttachments/Update`
    ///
    /// Parameter `document_id`: Id dokumentu.
    ///
    /// Parameter `year_id`: Id roku.
    ///
    /// Body `new_document_attachment`: Obiekt załącznika.
    pub async fn Update(
        api: &ApiClient,
        document_id: i32,
        year_id: i32,
        new_document_attachment: &crate::web_api::interface::fkf::documents::view_models::rdf::RdfDocumentAttachment,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        push_query_param(&mut query, "yearId", &year_id)?;
        api.request_with_body::<(), _>(
            HttpMethod::Put,
            "/api/FKDocumentAttachments/Update",
            query,
            new_document_attachment,
        )
        .await
    }

    /// `PUT` `/api/FKDocumentAttachments/Update`
    ///
    /// Parameter `document_id`: Id dokumentu.
    ///
    /// Parameter `year_id`: Id roku.
    ///
    /// Body `new_document_attachment`: Obiekt załącznika.
    pub async fn UpdateRaw(
        api: &ApiClient,
        document_id: i32,
        year_id: i32,
        new_document_attachment: &crate::web_api::interface::fkf::documents::view_models::rdf::RdfDocumentAttachment,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        push_query_param(&mut query, "yearId", &year_id)?;
        api.request_with_body_raw(
            HttpMethod::Put,
            "/api/FKDocumentAttachments/Update",
            query,
            new_document_attachment,
        )
        .await
    }
}
