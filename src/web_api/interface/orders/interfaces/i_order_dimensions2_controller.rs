use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `OrderDimensions2`.
pub struct IOrderDimensions2Controller;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/v2/OrderDimensions`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub struct ByOrderId {
        /// Id zamówienia obcego.
        pub order_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderId {
        type Output = Vec<crate::web_api::interface::common::view_models::Dimension>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Dimension>>(
                HttpMethod::Get,
                "/api/v2/OrderDimensions",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/v2/OrderDimensions", query)
                .await
        }
    }

    /// `GET` `/api/v2/OrderDimensions`
    ///
    /// Parameter `order_number`: Numer dokumentu zamówienia obcego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych zamówienia obcego o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByOrderNumberBuffer {
        /// Numer dokumentu zamówienia obcego.
        pub order_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych zamówienia obcego o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderNumberBuffer {
        type Output = Vec<crate::web_api::interface::common::view_models::Dimension>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Dimension>>(
                HttpMethod::Get,
                "/api/v2/OrderDimensions",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/v2/OrderDimensions", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IOrderDimensions2Controller {
    /// `GET` `/api/v2/OrderDimensions`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/v2/OrderDimensions`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/v2/OrderDimensions/Positions`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn GetPositionsByOrderId(
        api: &ApiClient,
        order_id: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::PositionDimension>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "orderId", &order_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::PositionDimension>>(HttpMethod::Get, "/api/v2/OrderDimensions/Positions", query).await
    }

    /// `GET` `/api/v2/OrderDimensions/Positions`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn GetPositionsByOrderIdRaw(
        api: &ApiClient,
        order_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "orderId", &order_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/v2/OrderDimensions/Positions", query)
            .await
    }

    /// `GET` `/api/v2/OrderDimensions/Positions`
    ///
    /// Parameter `order_number`: Numer dokumentu zamówienia obcego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych pozycji zamówienia obcego o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub async fn GetPositionsByOrderNumber(
        api: &ApiClient,
        order_number: String,
        buffer: bool,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::PositionDimension>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "orderNumber", &order_number)?;
        push_query_param(&mut query, "buffer", &buffer)?;
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::PositionDimension>>(HttpMethod::Get, "/api/v2/OrderDimensions/Positions", query).await
    }

    /// `GET` `/api/v2/OrderDimensions/Positions`
    ///
    /// Parameter `order_number`: Numer dokumentu zamówienia obcego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych pozycji zamówienia obcego o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub async fn GetPositionsByOrderNumberRaw(
        api: &ApiClient,
        order_number: String,
        buffer: bool,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "orderNumber", &order_number)?;
        push_query_param(&mut query, "buffer", &buffer)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/v2/OrderDimensions/Positions", query)
            .await
    }

    /// `GET` `/api/v2/OrderDimensions/Positions`
    ///
    /// Parameter `position_id`: Id pozycji zamówienia obcego.
    pub async fn GetPosition(
        api: &ApiClient,
        position_id: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Dimension>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "positionId", &position_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Dimension>>(
            HttpMethod::Get,
            "/api/v2/OrderDimensions/Positions",
            query,
        )
        .await
    }

    /// `GET` `/api/v2/OrderDimensions/Positions`
    ///
    /// Parameter `position_id`: Id pozycji zamówienia obcego.
    pub async fn GetPositionRaw(
        api: &ApiClient,
        position_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "positionId", &position_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/v2/OrderDimensions/Positions", query)
            .await
    }
}
