use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `DevelogicSales`.
pub struct IDevelogicSalesController;

#[allow(non_snake_case)]
impl IDevelogicSalesController {
    /// `POST` `/api/DevelogicSales/Invoice`
    ///
    /// Parameter `issue`: Czy wystawić dokument sprzedaży. W przypadku, gdy parametr nie zostanie podany dokument sprzedaży zostanie dodany do bufora.
    ///
    /// Parameter `invoice_kind`: Parametr dotyczące rodzaju wystawianej faktury w kontekście KSeF.
    ///
    /// Body `document`: Obiekt dokumentu sprzedaży do dodania do bufora lub wystawienia.
    pub async fn AddNew(
        api: &ApiClient,
        issue: bool,
        invoice_kind: serde_json::Value,
        document: &crate::web_api::interface::hmf::develogic::view_models::DevelogicDocumentIssue,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::sales::view_models::SaleDocument>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "issue", &issue)?;
        push_query_param(&mut query, "invoiceKind", &invoice_kind)?;
        api.request_with_body::<crate::web_api::interface::sales::view_models::SaleDocument, _>(
            HttpMethod::Post,
            "/api/DevelogicSales/Invoice",
            query,
            document,
        )
        .await
    }

    /// `POST` `/api/DevelogicSales/Invoice`
    ///
    /// Parameter `issue`: Czy wystawić dokument sprzedaży. W przypadku, gdy parametr nie zostanie podany dokument sprzedaży zostanie dodany do bufora.
    ///
    /// Parameter `invoice_kind`: Parametr dotyczące rodzaju wystawianej faktury w kontekście KSeF.
    ///
    /// Body `document`: Obiekt dokumentu sprzedaży do dodania do bufora lub wystawienia.
    pub async fn AddNewRaw(
        api: &ApiClient,
        issue: bool,
        invoice_kind: serde_json::Value,
        document: &crate::web_api::interface::hmf::develogic::view_models::DevelogicDocumentIssue,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "issue", &issue)?;
        push_query_param(&mut query, "invoiceKind", &invoice_kind)?;
        api.request_with_body_raw(
            HttpMethod::Post,
            "/api/DevelogicSales/Invoice",
            query,
            document,
        )
        .await
    }

    /// `POST` `/api/DevelogicSales/AdvancePayment`
    ///
    /// Parameter `contract_id`: Identyfikator kontraktu.
    ///
    /// Parameter `invoice_kind`: Parametr dotyczące rodzaju wystawianej faktury w kontekście KSeF.
    pub async fn IssueAdvancePayment(
        api: &ApiClient,
        contract_id: crate::web_api::interface::hmf::develogic::view_models::DevelogicAdvancePayment,
        invoice_kind: i32,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::sales::view_models::SaleDocument>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractId", &contract_id)?;
        push_query_param(&mut query, "invoiceKind", &invoice_kind)?;
        api.request_no_body::<crate::web_api::interface::sales::view_models::SaleDocument>(
            HttpMethod::Post,
            "/api/DevelogicSales/AdvancePayment",
            query,
        )
        .await
    }

    /// `POST` `/api/DevelogicSales/AdvancePayment`
    ///
    /// Parameter `contract_id`: Identyfikator kontraktu.
    ///
    /// Parameter `invoice_kind`: Parametr dotyczące rodzaju wystawianej faktury w kontekście KSeF.
    pub async fn IssueAdvancePaymentRaw(
        api: &ApiClient,
        contract_id: crate::web_api::interface::hmf::develogic::view_models::DevelogicAdvancePayment,
        invoice_kind: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractId", &contract_id)?;
        push_query_param(&mut query, "invoiceKind", &invoice_kind)?;
        api.request_no_body_raw(
            HttpMethod::Post,
            "/api/DevelogicSales/AdvancePayment",
            query,
        )
        .await
    }

    /// `POST` `/api/DevelogicSales/ContractInvoice`
    ///
    /// Parameter `contract_id`: Identyfikator kontraktu.
    ///
    /// Parameter `invoice_kind`: Parametr dotyczące rodzaju wystawianej faktury w kontekście KSeF.
    pub async fn IssueContractInvoice(
        api: &ApiClient,
        contract_id: crate::web_api::interface::hmf::develogic::view_models::DevelogicContractInvoiceIssue,
        invoice_kind: i32,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::sales::view_models::SaleDocument>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractId", &contract_id)?;
        push_query_param(&mut query, "invoiceKind", &invoice_kind)?;
        api.request_no_body::<crate::web_api::interface::sales::view_models::SaleDocument>(
            HttpMethod::Post,
            "/api/DevelogicSales/ContractInvoice",
            query,
        )
        .await
    }

    /// `POST` `/api/DevelogicSales/ContractInvoice`
    ///
    /// Parameter `contract_id`: Identyfikator kontraktu.
    ///
    /// Parameter `invoice_kind`: Parametr dotyczące rodzaju wystawianej faktury w kontekście KSeF.
    pub async fn IssueContractInvoiceRaw(
        api: &ApiClient,
        contract_id: crate::web_api::interface::hmf::develogic::view_models::DevelogicContractInvoiceIssue,
        invoice_kind: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractId", &contract_id)?;
        push_query_param(&mut query, "invoiceKind", &invoice_kind)?;
        api.request_no_body_raw(
            HttpMethod::Post,
            "/api/DevelogicSales/ContractInvoice",
            query,
        )
        .await
    }
}
