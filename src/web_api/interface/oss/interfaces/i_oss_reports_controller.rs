use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `OssReports`.
pub struct IOssReportsController;

#[allow(non_snake_case)]
impl IOssReportsController {
    /// `GET` `/api/OssReports/Order`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn GetOrderPDF(
        api: &ApiClient,
        order_id: i32,
    ) -> Result<ResponseEnvelope<crate::web_api::interface::common::view_models::PDF>, ApiError>
    {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "orderId", &order_id)?;
        api.request_no_body::<crate::web_api::interface::common::view_models::PDF>(
            HttpMethod::Get,
            "/api/OssReports/Order",
            query,
        )
        .await
    }

    /// `GET` `/api/OssReports/Order`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn GetOrderPDFRaw(
        api: &ApiClient,
        order_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "orderId", &order_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/OssReports/Order", query)
            .await
    }

    /// `GET` `/api/OssReports/SaleDocument`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub async fn GetSaleDocumentPDF(
        api: &ApiClient,
        document_id: i32,
    ) -> Result<ResponseEnvelope<crate::web_api::interface::common::view_models::PDF>, ApiError>
    {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        api.request_no_body::<crate::web_api::interface::common::view_models::PDF>(
            HttpMethod::Get,
            "/api/OssReports/SaleDocument",
            query,
        )
        .await
    }

    /// `GET` `/api/OssReports/SaleDocument`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub async fn GetSaleDocumentPDFRaw(
        api: &ApiClient,
        document_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/OssReports/SaleDocument", query)
            .await
    }

    /// `GET` `/api/OssReports/WarehouseDocument`
    ///
    /// Parameter `document_id`: Id dokumentu magazynowego.
    pub async fn GetWarehouseDocumentPDF(
        api: &ApiClient,
        document_id: i32,
    ) -> Result<ResponseEnvelope<crate::web_api::interface::common::view_models::PDF>, ApiError>
    {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        api.request_no_body::<crate::web_api::interface::common::view_models::PDF>(
            HttpMethod::Get,
            "/api/OssReports/WarehouseDocument",
            query,
        )
        .await
    }

    /// `GET` `/api/OssReports/WarehouseDocument`
    ///
    /// Parameter `document_id`: Id dokumentu magazynowego.
    pub async fn GetWarehouseDocumentPDFRaw(
        api: &ApiClient,
        document_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/OssReports/WarehouseDocument", query)
            .await
    }
}
