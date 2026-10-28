use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `OwnOrdersIssue`.
pub struct IOwnOrdersIssueController;

pub mod delete {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `DELETE` `/api/OwnOrdersIssue/Delete`
    ///
    /// Parameter `id`: Id zamówienia własnego.
    pub struct ById {
        /// Id zamówienia własnego.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<()>(HttpMethod::Delete, "/api/OwnOrdersIssue/Delete", query)
                .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Delete, "/api/OwnOrdersIssue/Delete", query)
                .await
        }
    }

    /// `DELETE` `/api/OwnOrdersIssue/Delete`
    ///
    /// Parameter `number`: Numer dokumentu zamówienia własnego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByNumberBuffer {
        /// Numer dokumentu zamówienia własnego.
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
            api.request_no_body::<()>(HttpMethod::Delete, "/api/OwnOrdersIssue/Delete", query)
                .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Delete, "/api/OwnOrdersIssue/Delete", query)
                .await
        }
    }
}

pub mod issue {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/OwnOrdersIssue/InBuffer`
    ///
    /// Parameter `id`: Id zamówienia własnego.
    pub struct ById {
        /// Id zamówienia własnego.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = crate::web_api::interface::own_orders::view_models::OwnOrder;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<crate::web_api::interface::own_orders::view_models::OwnOrder>(
                HttpMethod::Put,
                "/api/OwnOrdersIssue/InBuffer",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/OwnOrdersIssue/InBuffer", query)
                .await
        }
    }

    /// `PUT` `/api/OwnOrdersIssue/InBuffer`
    ///
    /// Parameter `number`: Numer zamówienia własnego.
    pub struct ByNumber {
        /// Numer zamówienia własnego.
        pub number: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByNumber {
        type Output = crate::web_api::interface::own_orders::view_models::OwnOrder;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            api.request_no_body::<crate::web_api::interface::own_orders::view_models::OwnOrder>(
                HttpMethod::Put,
                "/api/OwnOrdersIssue/InBuffer",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/OwnOrdersIssue/InBuffer", query)
                .await
        }
    }
}

pub mod issue_pz {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/OwnOrdersIssue/PZ`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub struct ByOrderId {
        /// Id zamówienia własnego.
        pub order_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderId {
        type Output = Vec<crate::web_api::interface::own_orders::view_models::OwnOrderPZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::own_orders::view_models::OwnOrderPZ>>(HttpMethod::Put, "/api/OwnOrdersIssue/PZ", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/OwnOrdersIssue/PZ", query)
                .await
        }
    }

    /// `PUT` `/api/OwnOrdersIssue/PZ`
    ///
    /// Parameter `order_number`: Numer zamówienia własnego.
    pub struct ByOrderNumber {
        /// Numer zamówienia własnego.
        pub order_number: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderNumber {
        type Output = Vec<crate::web_api::interface::own_orders::view_models::OwnOrderPZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            api.request_no_body::<Vec<crate::web_api::interface::own_orders::view_models::OwnOrderPZ>>(HttpMethod::Put, "/api/OwnOrdersIssue/PZ", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/OwnOrdersIssue/PZ", query)
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

    /// `PUT` `/api/OwnOrdersIssue/FV`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub struct ByOrderId {
        /// Id zamówienia własnego.
        pub order_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderId {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body::<()>(HttpMethod::Put, "/api/OwnOrdersIssue/FV", query)
                .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/OwnOrdersIssue/FV", query)
                .await
        }
    }

    /// `PUT` `/api/OwnOrdersIssue/FV`
    ///
    /// Parameter `order_number`: Numer zamówienia własnego.
    pub struct ByOrderNumber {
        /// Numer zamówienia własnego.
        pub order_number: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderNumber {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            api.request_no_body::<()>(HttpMethod::Put, "/api/OwnOrdersIssue/FV", query)
                .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/OwnOrdersIssue/FV", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IOwnOrdersIssueController {
    /// `DELETE` `/api/OwnOrdersIssue/Delete`
    ///
    /// Parameter `id`: Id zamówienia własnego.
    pub async fn Delete<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: delete::Overload,
    {
        overload.call(api).await
    }

    /// `DELETE` `/api/OwnOrdersIssue/Delete`
    ///
    /// Parameter `id`: Id zamówienia własnego.
    pub async fn DeleteRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: delete::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/OwnOrdersIssue/InBuffer`
    ///
    /// Parameter `id`: Id zamówienia własnego.
    pub async fn Issue<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: issue::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/OwnOrdersIssue/InBuffer`
    ///
    /// Parameter `id`: Id zamówienia własnego.
    pub async fn IssueRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: issue::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/OwnOrdersIssue/PZ`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub async fn IssuePZ<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: issue_pz::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/OwnOrdersIssue/PZ`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub async fn IssuePZRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: issue_pz::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/OwnOrdersIssue/FV`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub async fn IssueFV<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: issue_fv::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/OwnOrdersIssue/FV`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub async fn IssueFVRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: issue_fv::Overload,
    {
        overload.call_raw(api).await
    }

    /// `POST` `/api/OwnOrdersIssue/New`
    ///
    /// Parameter `issue`: Czy wystawić dokument sprzedaży. W przypadku, gdy parametr nie zostanie podany dokument sprzedaży zostanie dodany do bufora.
    ///
    /// Body `order`: Obiekt zamówienia własnego do dodania do bufora lub wystawienia.
    pub async fn AddNew(
        api: &ApiClient,
        issue: bool,
        order: &crate::web_api::interface::own_orders::view_models::OwnOrderIssue,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::own_orders::view_models::OwnOrder>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "issue", &issue)?;
        api.request_with_body::<crate::web_api::interface::own_orders::view_models::OwnOrder, _>(
            HttpMethod::Post,
            "/api/OwnOrdersIssue/New",
            query,
            order,
        )
        .await
    }

    /// `POST` `/api/OwnOrdersIssue/New`
    ///
    /// Parameter `issue`: Czy wystawić dokument sprzedaży. W przypadku, gdy parametr nie zostanie podany dokument sprzedaży zostanie dodany do bufora.
    ///
    /// Body `order`: Obiekt zamówienia własnego do dodania do bufora lub wystawienia.
    pub async fn AddNewRaw(
        api: &ApiClient,
        issue: bool,
        order: &crate::web_api::interface::own_orders::view_models::OwnOrderIssue,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "issue", &issue)?;
        api.request_with_body_raw(HttpMethod::Post, "/api/OwnOrdersIssue/New", query, order)
            .await
    }

    /// `PATCH` `/api/OwnOrdersIssue/DocumentNumber`
    ///
    /// Parameter `id`: Id zamówienia własnego.
    ///
    /// Parameter `number`: Nowy numer zamówienia własnego.
    pub async fn ChangeDocumentNumber(
        api: &ApiClient,
        id: i32,
        number: String,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        push_query_param(&mut query, "number", &number)?;
        api.request_no_body::<()>(
            HttpMethod::Patch,
            "/api/OwnOrdersIssue/DocumentNumber",
            query,
        )
        .await
    }

    /// `PATCH` `/api/OwnOrdersIssue/DocumentNumber`
    ///
    /// Parameter `id`: Id zamówienia własnego.
    ///
    /// Parameter `number`: Nowy numer zamówienia własnego.
    pub async fn ChangeDocumentNumberRaw(
        api: &ApiClient,
        id: i32,
        number: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        push_query_param(&mut query, "number", &number)?;
        api.request_no_body_raw(
            HttpMethod::Patch,
            "/api/OwnOrdersIssue/DocumentNumber",
            query,
        )
        .await
    }
}
