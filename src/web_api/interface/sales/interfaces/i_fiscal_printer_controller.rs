use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `FiscalPrinter`.
pub struct IFiscalPrinterController;

#[allow(non_snake_case)]
impl IFiscalPrinterController {
    /// `GET` `/api/FiscalPrinter/Status`
    pub async fn GetStatus(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::sales::view_models::FiscalPrinterStatus>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<crate::web_api::interface::sales::view_models::FiscalPrinterStatus>(
            HttpMethod::Get,
            "/api/FiscalPrinter/Status",
            query,
        )
        .await
    }

    /// `GET` `/api/FiscalPrinter/Status`
    pub async fn GetStatusRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/FiscalPrinter/Status", query)
            .await
    }

    /// `PATCH` `/api/FiscalPrinter/Fiscalize`
    ///
    /// Parameter `document_id`: Id dokumentu do zafiskalizowania.
    pub async fn Fiscalize(
        api: &ApiClient,
        document_id: i32,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::sales::view_models::FiscalizationResult>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        api.request_no_body::<crate::web_api::interface::sales::view_models::FiscalizationResult>(
            HttpMethod::Patch,
            "/api/FiscalPrinter/Fiscalize",
            query,
        )
        .await
    }

    /// `PATCH` `/api/FiscalPrinter/Fiscalize`
    ///
    /// Parameter `document_id`: Id dokumentu do zafiskalizowania.
    pub async fn FiscalizeRaw(
        api: &ApiClient,
        document_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        api.request_no_body_raw(HttpMethod::Patch, "/api/FiscalPrinter/Fiscalize", query)
            .await
    }
}
