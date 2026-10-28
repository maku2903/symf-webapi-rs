use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `OssOrder`.
pub struct IOssOrderController;

#[allow(non_snake_case)]
impl IOssOrderController {
    /// `POST` `/api/OssOrder/New`
    ///
    /// Parameter `issue`: Czy wystawić zamówienie obce. W przypadku, gdy parametr nie zostanie podany zamówienie obce zostanie wystawione.
    ///
    /// Body `order`: Obiekt zamówienia obcego do wystawienia.
    pub async fn AddNew(
        api: &ApiClient,
        issue: bool,
        order: &crate::web_api::interface::oss::view_models::orders::OssOrder,
    ) -> Result<ResponseEnvelope<crate::web_api::interface::orders::view_models::Order>, ApiError>
    {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "issue", &issue)?;
        api.request_with_body::<crate::web_api::interface::orders::view_models::Order, _>(
            HttpMethod::Post,
            "/api/OssOrder/New",
            query,
            order,
        )
        .await
    }

    /// `POST` `/api/OssOrder/New`
    ///
    /// Parameter `issue`: Czy wystawić zamówienie obce. W przypadku, gdy parametr nie zostanie podany zamówienie obce zostanie wystawione.
    ///
    /// Body `order`: Obiekt zamówienia obcego do wystawienia.
    pub async fn AddNewRaw(
        api: &ApiClient,
        issue: bool,
        order: &crate::web_api::interface::oss::view_models::orders::OssOrder,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "issue", &issue)?;
        api.request_with_body_raw(HttpMethod::Post, "/api/OssOrder/New", query, order)
            .await
    }

    /// `PUT` `/api/OssOrder/FV`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn IssueFV(
        api: &ApiClient,
        order_id: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::orders::view_models::OrderFV>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "orderId", &order_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::orders::view_models::OrderFV>>(
            HttpMethod::Put,
            "/api/OssOrder/FV",
            query,
        )
        .await
    }

    /// `PUT` `/api/OssOrder/FV`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn IssueFVRaw(
        api: &ApiClient,
        order_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "orderId", &order_id)?;
        api.request_no_body_raw(HttpMethod::Put, "/api/OssOrder/FV", query)
            .await
    }

    /// `PUT` `/api/OssOrder/WZ`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    ///
    /// Parameter `in_buffer`: Czy WZ ma pozostać w buforze, czy też ma zostać automatycznie wystawiona.
    pub async fn IssueWZ(
        api: &ApiClient,
        order_id: i32,
        in_buffer: bool,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::orders::view_models::OrderWZ>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "orderId", &order_id)?;
        push_query_param(&mut query, "inBuffer", &in_buffer)?;
        api.request_no_body::<Vec<crate::web_api::interface::orders::view_models::OrderWZ>>(
            HttpMethod::Put,
            "/api/OssOrder/WZ",
            query,
        )
        .await
    }

    /// `PUT` `/api/OssOrder/WZ`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    ///
    /// Parameter `in_buffer`: Czy WZ ma pozostać w buforze, czy też ma zostać automatycznie wystawiona.
    pub async fn IssueWZRaw(
        api: &ApiClient,
        order_id: i32,
        in_buffer: bool,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "orderId", &order_id)?;
        push_query_param(&mut query, "inBuffer", &in_buffer)?;
        api.request_no_body_raw(HttpMethod::Put, "/api/OssOrder/WZ", query)
            .await
    }
}
