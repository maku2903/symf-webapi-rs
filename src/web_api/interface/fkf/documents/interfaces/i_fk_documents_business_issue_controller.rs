use crate::runtime::{ApiClient, ApiError, HttpMethod, ResponseEnvelope};
use serde_json::Value;

/// WebAPI controller `FKDocumentsBusinessIssue`.
pub struct IFKDocumentsBusinessIssueController;

#[allow(non_snake_case)]
impl IFKDocumentsBusinessIssueController {
    /// `POST` `/api/FKDocumentsBusinessIssue/NewPurchaseInvoice`
    ///
    /// Body `document_issue`: Obiekt dokumentu do utworzenia.
    pub async fn NewPurchaseInvoice(
        api: &ApiClient,
        document_issue: &crate::web_api::interface::fkf::documents::view_models::issue::business::BusinessDocumentIssue,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::fkf::documents::view_models::Document>,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<crate::web_api::interface::fkf::documents::view_models::Document, _>(HttpMethod::Post, "/api/FKDocumentsBusinessIssue/NewPurchaseInvoice", query, document_issue).await
    }

    /// `POST` `/api/FKDocumentsBusinessIssue/NewPurchaseInvoice`
    ///
    /// Body `document_issue`: Obiekt dokumentu do utworzenia.
    pub async fn NewPurchaseInvoiceRaw(
        api: &ApiClient,
        document_issue: &crate::web_api::interface::fkf::documents::view_models::issue::business::BusinessDocumentIssue,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Post,
            "/api/FKDocumentsBusinessIssue/NewPurchaseInvoice",
            query,
            document_issue,
        )
        .await
    }

    /// `POST` `/api/FKDocumentsBusinessIssue/NewPurchaseInvoiceCorrection`
    ///
    /// Body `document_issue`: Obiekt dokumentu do utworzenia.
    pub async fn NewPurchaseInvoiceCorrection(
        api: &ApiClient,
        document_issue: &crate::web_api::interface::fkf::documents::view_models::issue::business::BusinessDocumentIssue,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::fkf::documents::view_models::Document>,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<crate::web_api::interface::fkf::documents::view_models::Document, _>(HttpMethod::Post, "/api/FKDocumentsBusinessIssue/NewPurchaseInvoiceCorrection", query, document_issue).await
    }

    /// `POST` `/api/FKDocumentsBusinessIssue/NewPurchaseInvoiceCorrection`
    ///
    /// Body `document_issue`: Obiekt dokumentu do utworzenia.
    pub async fn NewPurchaseInvoiceCorrectionRaw(
        api: &ApiClient,
        document_issue: &crate::web_api::interface::fkf::documents::view_models::issue::business::BusinessDocumentIssue,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Post,
            "/api/FKDocumentsBusinessIssue/NewPurchaseInvoiceCorrection",
            query,
            document_issue,
        )
        .await
    }

    /// `POST` `/api/FKDocumentsBusinessIssue/NewSaleInvoice`
    ///
    /// Body `document_issue`: Obiekt dokumentu do utworzenia.
    pub async fn NewSaleInvoice(
        api: &ApiClient,
        document_issue: &crate::web_api::interface::fkf::documents::view_models::issue::business::BusinessDocumentIssue,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::fkf::documents::view_models::Document>,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<crate::web_api::interface::fkf::documents::view_models::Document, _>(HttpMethod::Post, "/api/FKDocumentsBusinessIssue/NewSaleInvoice", query, document_issue).await
    }

    /// `POST` `/api/FKDocumentsBusinessIssue/NewSaleInvoice`
    ///
    /// Body `document_issue`: Obiekt dokumentu do utworzenia.
    pub async fn NewSaleInvoiceRaw(
        api: &ApiClient,
        document_issue: &crate::web_api::interface::fkf::documents::view_models::issue::business::BusinessDocumentIssue,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Post,
            "/api/FKDocumentsBusinessIssue/NewSaleInvoice",
            query,
            document_issue,
        )
        .await
    }

    /// `POST` `/api/FKDocumentsBusinessIssue/NewSaleInvoiceCorrection`
    ///
    /// Body `document_issue`: Obiekt dokumentu do utworzenia.
    pub async fn NewSaleInvoiceCorrection(
        api: &ApiClient,
        document_issue: &crate::web_api::interface::fkf::documents::view_models::issue::business::BusinessDocumentIssue,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::fkf::documents::view_models::Document>,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<crate::web_api::interface::fkf::documents::view_models::Document, _>(HttpMethod::Post, "/api/FKDocumentsBusinessIssue/NewSaleInvoiceCorrection", query, document_issue).await
    }

    /// `POST` `/api/FKDocumentsBusinessIssue/NewSaleInvoiceCorrection`
    ///
    /// Body `document_issue`: Obiekt dokumentu do utworzenia.
    pub async fn NewSaleInvoiceCorrectionRaw(
        api: &ApiClient,
        document_issue: &crate::web_api::interface::fkf::documents::view_models::issue::business::BusinessDocumentIssue,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Post,
            "/api/FKDocumentsBusinessIssue/NewSaleInvoiceCorrection",
            query,
            document_issue,
        )
        .await
    }
}
