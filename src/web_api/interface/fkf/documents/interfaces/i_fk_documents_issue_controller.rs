use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `FKDocumentsIssue`.
pub struct IFKDocumentsIssueController;

#[allow(non_snake_case)]
impl IFKDocumentsIssueController {
    /// `POST` `/api/FKDocumentsIssue/NewCustom`
    ///
    /// Body `document_issue`: Obiekt dokumentu do utworzenia.
    pub async fn AddNew(
        api: &ApiClient,
        document_issue: &crate::web_api::interface::fkf::documents::view_models::issue::custom::DocumentIssue,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::fkf::documents::view_models::Document>,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<crate::web_api::interface::fkf::documents::view_models::Document, _>(HttpMethod::Post, "/api/FKDocumentsIssue/NewCustom", query, document_issue).await
    }

    /// `POST` `/api/FKDocumentsIssue/NewCustom`
    ///
    /// Body `document_issue`: Obiekt dokumentu do utworzenia.
    pub async fn AddNewRaw(
        api: &ApiClient,
        document_issue: &crate::web_api::interface::fkf::documents::view_models::issue::custom::DocumentIssue,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Post,
            "/api/FKDocumentsIssue/NewCustom",
            query,
            document_issue,
        )
        .await
    }

    /// `PATCH` `/api/FKDocumentsIssue/NewCustom`
    ///
    /// Body `document_issue`: Obiekt dokumentu do walidacji.
    pub async fn Validate(
        api: &ApiClient,
        document_issue: &crate::web_api::interface::fkf::documents::view_models::issue::purchase_invoice::DocumentIssue,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::documents::view_models::DocumentMessage>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<Vec<crate::web_api::interface::fkf::documents::view_models::DocumentMessage>, _>(HttpMethod::Patch, "/api/FKDocumentsIssue/NewCustom", query, document_issue).await
    }

    /// `PATCH` `/api/FKDocumentsIssue/NewCustom`
    ///
    /// Body `document_issue`: Obiekt dokumentu do walidacji.
    pub async fn ValidateRaw(
        api: &ApiClient,
        document_issue: &crate::web_api::interface::fkf::documents::view_models::issue::purchase_invoice::DocumentIssue,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Patch,
            "/api/FKDocumentsIssue/NewCustom",
            query,
            document_issue,
        )
        .await
    }
}
