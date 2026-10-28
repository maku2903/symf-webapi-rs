use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `OssSale`.
pub struct IOssSaleController;

#[allow(non_snake_case)]
impl IOssSaleController {
    /// `POST` `/api/OssSale/New`
    ///
    /// Parameter `issue`: Czy wystawić fakturę. W przypadku, gdy parametr nie zostanie podany faktura zostanie wystawiona.
    ///
    /// Parameter `invoice_kind`: Parametr dotyczące rodzaju wystawianej faktury w kontekście KSeF.
    ///
    /// Body `sale_document`: Obiekt faktury do wystawienia.
    pub async fn AddNew(
        api: &ApiClient,
        issue: bool,
        invoice_kind: serde_json::Value,
        sale_document: &crate::web_api::interface::oss::view_models::sales::OssSaleDocument,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::sales::view_models::SaleDocument>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "issue", &issue)?;
        push_query_param(&mut query, "invoiceKind", &invoice_kind)?;
        api.request_with_body::<crate::web_api::interface::sales::view_models::SaleDocument, _>(
            HttpMethod::Post,
            "/api/OssSale/New",
            query,
            sale_document,
        )
        .await
    }

    /// `POST` `/api/OssSale/New`
    ///
    /// Parameter `issue`: Czy wystawić fakturę. W przypadku, gdy parametr nie zostanie podany faktura zostanie wystawiona.
    ///
    /// Parameter `invoice_kind`: Parametr dotyczące rodzaju wystawianej faktury w kontekście KSeF.
    ///
    /// Body `sale_document`: Obiekt faktury do wystawienia.
    pub async fn AddNewRaw(
        api: &ApiClient,
        issue: bool,
        invoice_kind: serde_json::Value,
        sale_document: &crate::web_api::interface::oss::view_models::sales::OssSaleDocument,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "issue", &issue)?;
        push_query_param(&mut query, "invoiceKind", &invoice_kind)?;
        api.request_with_body_raw(HttpMethod::Post, "/api/OssSale/New", query, sale_document)
            .await
    }

    /// `POST` `/api/OssSale/NewReceipt`
    ///
    /// Parameter `issue`: Czy wystawić paragon. W przypadku, gdy parametr nie zostanie podany paragon zostanie wystawiony.
    ///
    /// Parameter `invoice_kind`: Parametr dotyczące rodzaju wystawianej faktury w kontekście KSeF.
    ///
    /// Body `receipt`: Obiekt paragonu do wystawienia.
    pub async fn AddNewReceipt(
        api: &ApiClient,
        issue: bool,
        invoice_kind: serde_json::Value,
        receipt: &crate::web_api::interface::oss::view_models::sales::OssDomesticSaleDocument,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::sales::view_models::SaleDocument>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "issue", &issue)?;
        push_query_param(&mut query, "invoiceKind", &invoice_kind)?;
        api.request_with_body::<crate::web_api::interface::sales::view_models::SaleDocument, _>(
            HttpMethod::Post,
            "/api/OssSale/NewReceipt",
            query,
            receipt,
        )
        .await
    }

    /// `POST` `/api/OssSale/NewReceipt`
    ///
    /// Parameter `issue`: Czy wystawić paragon. W przypadku, gdy parametr nie zostanie podany paragon zostanie wystawiony.
    ///
    /// Parameter `invoice_kind`: Parametr dotyczące rodzaju wystawianej faktury w kontekście KSeF.
    ///
    /// Body `receipt`: Obiekt paragonu do wystawienia.
    pub async fn AddNewReceiptRaw(
        api: &ApiClient,
        issue: bool,
        invoice_kind: serde_json::Value,
        receipt: &crate::web_api::interface::oss::view_models::sales::OssDomesticSaleDocument,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "issue", &issue)?;
        push_query_param(&mut query, "invoiceKind", &invoice_kind)?;
        api.request_with_body_raw(HttpMethod::Post, "/api/OssSale/NewReceipt", query, receipt)
            .await
    }

    /// `PUT` `/api/OssSale/WZ`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    ///
    /// Parameter `in_buffer`: Czy WZ ma pozostać w buforze, czy też ma zostać automatycznie wystawiona.
    pub async fn IssueWZ(
        api: &ApiClient,
        document_id: i32,
        in_buffer: bool,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::sales::view_models::SaleDocumentWZ>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        push_query_param(&mut query, "inBuffer", &in_buffer)?;
        api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentWZ>>(
            HttpMethod::Put,
            "/api/OssSale/WZ",
            query,
        )
        .await
    }

    /// `PUT` `/api/OssSale/WZ`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    ///
    /// Parameter `in_buffer`: Czy WZ ma pozostać w buforze, czy też ma zostać automatycznie wystawiona.
    pub async fn IssueWZRaw(
        api: &ApiClient,
        document_id: i32,
        in_buffer: bool,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        push_query_param(&mut query, "inBuffer", &in_buffer)?;
        api.request_no_body_raw(HttpMethod::Put, "/api/OssSale/WZ", query)
            .await
    }
}
