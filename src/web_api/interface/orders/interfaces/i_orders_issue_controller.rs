use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `OrdersIssue`.
pub struct IOrdersIssueController;

pub mod issue {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/OrdersIssue/InBuffer`
    ///
    /// Parameter `id`: Id zamówienia obcego.
    pub struct ById {
        /// Id zamówienia obcego.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = crate::web_api::interface::orders::view_models::Order;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<crate::web_api::interface::orders::view_models::Order>(
                HttpMethod::Put,
                "/api/OrdersIssue/InBuffer",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/OrdersIssue/InBuffer", query)
                .await
        }
    }

    /// `PUT` `/api/OrdersIssue/InBuffer`
    ///
    /// Parameter `number`: Numer zamówienia obcego.
    pub struct ByNumber {
        /// Numer zamówienia obcego.
        pub number: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByNumber {
        type Output = crate::web_api::interface::orders::view_models::Order;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            api.request_no_body::<crate::web_api::interface::orders::view_models::Order>(
                HttpMethod::Put,
                "/api/OrdersIssue/InBuffer",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/OrdersIssue/InBuffer", query)
                .await
        }
    }
}

pub mod issue_wz {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/OrdersIssue/WZ`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    ///
    /// Parameter `in_buffer`: Czy WZ ma pozostać w buforze, czy też ma zostać automatycznie wystawiona.
    pub struct ByOrderIdInBuffer {
        /// Id zamówienia obcego.
        pub order_id: i32,
        /// Czy WZ ma pozostać w buforze, czy też ma zostać automatycznie wystawiona.
        pub in_buffer: Option<i32>,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderIdInBuffer {
        type Output = Vec<crate::web_api::interface::orders::view_models::OrderWZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            push_query_param(&mut query, "inBuffer", &self.in_buffer)?;
            api.request_no_body::<Vec<crate::web_api::interface::orders::view_models::OrderWZ>>(
                HttpMethod::Put,
                "/api/OrdersIssue/WZ",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            push_query_param(&mut query, "inBuffer", &self.in_buffer)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/OrdersIssue/WZ", query)
                .await
        }
    }

    /// `PUT` `/api/OrdersIssue/WZ`
    ///
    /// Parameter `order_number`: Numer zamówienia obcego.
    ///
    /// Parameter `in_buffer`: Czy WZ ma pozostać w buforze, czy też ma zostać automatycznie wystawiona.
    pub struct ByOrderNumberInBuffer {
        /// Numer zamówienia obcego.
        pub order_number: String,
        /// Czy WZ ma pozostać w buforze, czy też ma zostać automatycznie wystawiona.
        pub in_buffer: Option<i32>,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderNumberInBuffer {
        type Output = Vec<crate::web_api::interface::orders::view_models::OrderWZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "inBuffer", &self.in_buffer)?;
            api.request_no_body::<Vec<crate::web_api::interface::orders::view_models::OrderWZ>>(
                HttpMethod::Put,
                "/api/OrdersIssue/WZ",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "inBuffer", &self.in_buffer)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/OrdersIssue/WZ", query)
                .await
        }
    }
}

pub mod issue_fv {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/OrdersIssue/FV`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub struct ByOrderId {
        /// Id zamówienia obcego.
        pub order_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderId {
        type Output = Vec<crate::web_api::interface::orders::view_models::OrderFV>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::orders::view_models::OrderFV>>(
                HttpMethod::Put,
                "/api/OrdersIssue/FV",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/OrdersIssue/FV", query)
                .await
        }
    }

    /// `PUT` `/api/OrdersIssue/FV`
    ///
    /// Parameter `order_number`: Numer zamówienia obcego.
    pub struct ByOrderNumber {
        /// Numer zamówienia obcego.
        pub order_number: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderNumber {
        type Output = Vec<crate::web_api::interface::orders::view_models::OrderFV>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            api.request_no_body::<Vec<crate::web_api::interface::orders::view_models::OrderFV>>(
                HttpMethod::Put,
                "/api/OrdersIssue/FV",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/OrdersIssue/FV", query)
                .await
        }
    }
}

pub mod delete {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `DELETE` `/api/OrdersIssue/Delete`
    ///
    /// Parameter `id`: Id zamówienia obcego.
    pub struct ById {
        /// Id zamówienia obcego.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<()>(HttpMethod::Delete, "/api/OrdersIssue/Delete", query)
                .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Delete, "/api/OrdersIssue/Delete", query)
                .await
        }
    }

    /// `DELETE` `/api/OrdersIssue/Delete`
    ///
    /// Parameter `number`: Numer dokumentu zamówienia obcego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByNumberBuffer {
        /// Numer dokumentu zamówienia obcego.
        pub number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByNumberBuffer {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<()>(HttpMethod::Delete, "/api/OrdersIssue/Delete", query)
                .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Delete, "/api/OrdersIssue/Delete", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IOrdersIssueController {
    /// `PUT` `/api/OrdersIssue/InBuffer`
    ///
    /// Parameter `id`: Id zamówienia obcego.
    pub async fn Issue<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: issue::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/OrdersIssue/InBuffer`
    ///
    /// Parameter `id`: Id zamówienia obcego.
    pub async fn IssueRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: issue::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/OrdersIssue/WZ`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    ///
    /// Parameter `in_buffer`: Czy WZ ma pozostać w buforze, czy też ma zostać automatycznie wystawiona.
    pub async fn IssueWZ<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: issue_wz::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/OrdersIssue/WZ`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    ///
    /// Parameter `in_buffer`: Czy WZ ma pozostać w buforze, czy też ma zostać automatycznie wystawiona.
    pub async fn IssueWZRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: issue_wz::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/OrdersIssue/FV`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn IssueFV<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: issue_fv::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/OrdersIssue/FV`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn IssueFVRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: issue_fv::Overload,
    {
        overload.call_raw(api).await
    }

    /// `DELETE` `/api/OrdersIssue/Delete`
    ///
    /// Parameter `id`: Id zamówienia obcego.
    pub async fn Delete<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: delete::Overload,
    {
        overload.call(api).await
    }

    /// `DELETE` `/api/OrdersIssue/Delete`
    ///
    /// Parameter `id`: Id zamówienia obcego.
    pub async fn DeleteRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: delete::Overload,
    {
        overload.call_raw(api).await
    }

    /// `POST` `/api/OrdersIssue/New`
    ///
    /// Parameter `issue`: Czy wystawić zamówienie obce. W przypadku, gdy parametr nie zostanie podany zamówienie obce zostanie wystawione.
    ///
    /// Body `order`: Obiekt zamówienia obcego do wystawienia.
    pub async fn AddNew(
        api: &ApiClient,
        issue: bool,
        order: &crate::web_api::interface::orders::view_models::OrderIssue,
    ) -> Result<ResponseEnvelope<crate::web_api::interface::orders::view_models::Order>, ApiError>
    {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "issue", &issue)?;
        api.request_with_body::<crate::web_api::interface::orders::view_models::Order, _>(
            HttpMethod::Post,
            "/api/OrdersIssue/New",
            query,
            order,
        )
        .await
    }

    /// `POST` `/api/OrdersIssue/New`
    ///
    /// Parameter `issue`: Czy wystawić zamówienie obce. W przypadku, gdy parametr nie zostanie podany zamówienie obce zostanie wystawione.
    ///
    /// Body `order`: Obiekt zamówienia obcego do wystawienia.
    pub async fn AddNewRaw(
        api: &ApiClient,
        issue: bool,
        order: &crate::web_api::interface::orders::view_models::OrderIssue,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "issue", &issue)?;
        api.request_with_body_raw(HttpMethod::Post, "/api/OrdersIssue/New", query, order)
            .await
    }

    /// `PUT` `/api/OrdersIssue/Edit`
    ///
    /// Body `order`: Obiekt zamówienia obcego do edycji.
    pub async fn Edit(
        api: &ApiClient,
        order: &crate::web_api::interface::orders::view_models::OrderEdit,
    ) -> Result<ResponseEnvelope<crate::web_api::interface::orders::view_models::Order>, ApiError>
    {
        let query = vec![];
        api.request_with_body::<crate::web_api::interface::orders::view_models::Order, _>(
            HttpMethod::Put,
            "/api/OrdersIssue/Edit",
            query,
            order,
        )
        .await
    }

    /// `PUT` `/api/OrdersIssue/Edit`
    ///
    /// Body `order`: Obiekt zamówienia obcego do edycji.
    pub async fn EditRaw(
        api: &ApiClient,
        order: &crate::web_api::interface::orders::view_models::OrderEdit,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(HttpMethod::Put, "/api/OrdersIssue/Edit", query, order)
            .await
    }

    /// `PATCH` `/api/OrdersIssue/DocumentNumber`
    ///
    /// Parameter `id`: Id zamówienia obcego.
    ///
    /// Parameter `number`: Nowy numer zamówienia obcego.
    pub async fn ChangeDocumentNumber(
        api: &ApiClient,
        id: i32,
        number: String,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        push_query_param(&mut query, "number", &number)?;
        api.request_no_body::<()>(HttpMethod::Patch, "/api/OrdersIssue/DocumentNumber", query)
            .await
    }

    /// `PATCH` `/api/OrdersIssue/DocumentNumber`
    ///
    /// Parameter `id`: Id zamówienia obcego.
    ///
    /// Parameter `number`: Nowy numer zamówienia obcego.
    pub async fn ChangeDocumentNumberRaw(
        api: &ApiClient,
        id: i32,
        number: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        push_query_param(&mut query, "number", &number)?;
        api.request_no_body_raw(HttpMethod::Patch, "/api/OrdersIssue/DocumentNumber", query)
            .await
    }
}
