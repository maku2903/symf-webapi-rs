use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `Orders`.
pub struct IOrdersController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Orders`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output = Vec<crate::web_api::interface::orders::view_models::OrderListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<Vec<crate::web_api::interface::orders::view_models::OrderListElement>>(HttpMethod::Get, "/api/Orders", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(HttpMethod::Get, "/api/Orders", query)
                .await
        }
    }

    /// `GET` `/api/Orders`
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
                HttpMethod::Get,
                "/api/Orders",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Orders", query)
                .await
        }
    }

    /// `GET` `/api/Orders`
    ///
    /// Parameter `number`: Numer dokumentu zamówienia obcego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByNumberBuffer {
        /// Numer dokumentu zamówienia obcego.
        pub number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie pobrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByNumberBuffer {
        type Output = crate::web_api::interface::orders::view_models::Order;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<crate::web_api::interface::orders::view_models::Order>(
                HttpMethod::Get,
                "/api/Orders",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Orders", query)
                .await
        }
    }
}

pub mod get_list_by_recipient {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Orders/Filter`
    ///
    /// Parameter `recipient_id`: Id kontrahenta (odbiorcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByRecipientIdDateFromDateTo {
        /// Id kontrahenta (odbiorcy).
        pub recipient_id: i32,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByRecipientIdDateFromDateTo {
        type Output = Vec<crate::web_api::interface::orders::view_models::OrderListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "recipientId", &self.recipient_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::orders::view_models::OrderListElement>>(HttpMethod::Get, "/api/Orders/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "recipientId", &self.recipient_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Orders/Filter", query)
                .await
        }
    }

    /// `GET` `/api/Orders/Filter`
    ///
    /// Parameter `recipient_code`: Kod kontrahenta (odbiorcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByRecipientCodeDateFromDateTo {
        /// Kod kontrahenta (odbiorcy).
        pub recipient_code: String,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByRecipientCodeDateFromDateTo {
        type Output = Vec<crate::web_api::interface::orders::view_models::OrderListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "recipientCode", &self.recipient_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::orders::view_models::OrderListElement>>(HttpMethod::Get, "/api/Orders/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "recipientCode", &self.recipient_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Orders/Filter", query)
                .await
        }
    }
}

pub mod get_list_by_buyer {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Orders/Filter`
    ///
    /// Parameter `buyer_id`: Id kontrahenta (nabywcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByBuyerIdDateFromDateTo {
        /// Id kontrahenta (nabywcy).
        pub buyer_id: i32,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByBuyerIdDateFromDateTo {
        type Output = Vec<crate::web_api::interface::orders::view_models::OrderListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "buyerId", &self.buyer_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::orders::view_models::OrderListElement>>(HttpMethod::Get, "/api/Orders/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "buyerId", &self.buyer_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Orders/Filter", query)
                .await
        }
    }

    /// `GET` `/api/Orders/Filter`
    ///
    /// Parameter `buyer_code`: Kod kontrahenta (nabywcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByBuyerCodeDateFromDateTo {
        /// Kod kontrahenta (nabywcy).
        pub buyer_code: String,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByBuyerCodeDateFromDateTo {
        type Output = Vec<crate::web_api::interface::orders::view_models::OrderListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "buyerCode", &self.buyer_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::orders::view_models::OrderListElement>>(HttpMethod::Get, "/api/Orders/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "buyerCode", &self.buyer_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Orders/Filter", query)
                .await
        }
    }
}

pub mod get_wz {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Orders/WZ`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub struct ByOrderId {
        /// Id zamówienia obcego.
        pub order_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderId {
        type Output = Vec<crate::web_api::interface::orders::view_models::OrderWZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::orders::view_models::OrderWZ>>(
                HttpMethod::Get,
                "/api/Orders/WZ",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Orders/WZ", query)
                .await
        }
    }

    /// `GET` `/api/Orders/WZ`
    ///
    /// Parameter `order_number`: Numer zamówienia obcego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByOrderNumberBuffer {
        /// Numer zamówienia obcego.
        pub order_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderNumberBuffer {
        type Output = Vec<crate::web_api::interface::orders::view_models::OrderWZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<Vec<crate::web_api::interface::orders::view_models::OrderWZ>>(
                HttpMethod::Get,
                "/api/Orders/WZ",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Orders/WZ", query)
                .await
        }
    }
}

pub mod get_fv {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Orders/FV`
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
                HttpMethod::Get,
                "/api/Orders/FV",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Orders/FV", query)
                .await
        }
    }

    /// `GET` `/api/Orders/FV`
    ///
    /// Parameter `order_number`: Numer zamówienia obcego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByOrderNumberBuffer {
        /// Numer zamówienia obcego.
        pub order_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderNumberBuffer {
        type Output = Vec<crate::web_api::interface::orders::view_models::OrderFV>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<Vec<crate::web_api::interface::orders::view_models::OrderFV>>(
                HttpMethod::Get,
                "/api/Orders/FV",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Orders/FV", query)
                .await
        }
    }
}

pub mod get_pdf {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Orders/PDF`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    ///
    /// Parameter `print_note`: Czy umieścić notatkę na wydruku PDF. W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
    pub struct ByOrderIdPrintNote {
        /// Id zamówienia obcego.
        pub order_id: i32,
        /// Czy umieścić notatkę na wydruku PDF.
        /// W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
        pub print_note: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderIdPrintNote {
        type Output = crate::web_api::interface::common::view_models::PDF;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            push_query_param(&mut query, "printNote", &self.print_note)?;
            api.request_no_body::<crate::web_api::interface::common::view_models::PDF>(
                HttpMethod::Get,
                "/api/Orders/PDF",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            push_query_param(&mut query, "printNote", &self.print_note)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Orders/PDF", query)
                .await
        }
    }

    /// `GET` `/api/Orders/PDF`
    ///
    /// Parameter `order_number`: Numer zamówienia obcego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
    ///
    /// Parameter `print_note`: Czy umieścić notatkę na wydruku PDF. W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
    pub struct ByOrderNumberBufferPrintNote {
        /// Numer zamówienia obcego.
        pub order_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub buffer: bool,
        /// Czy umieścić notatkę na wydruku PDF.
        /// W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
        pub print_note: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderNumberBufferPrintNote {
        type Output = crate::web_api::interface::common::view_models::PDF;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            push_query_param(&mut query, "printNote", &self.print_note)?;
            api.request_no_body::<crate::web_api::interface::common::view_models::PDF>(
                HttpMethod::Get,
                "/api/Orders/PDF",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            push_query_param(&mut query, "printNote", &self.print_note)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Orders/PDF", query)
                .await
        }
    }

    /// `PATCH` `/api/Orders/PDF`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    ///
    /// Body `settings`: Ustawienia wydruku.
    pub struct ByOrderId<'a> {
        /// Id zamówienia obcego.
        pub order_id: i32,
        /// Ustawienia wydruku.
        pub settings: &'a crate::web_api::interface::common::view_models::PDFSettings,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByOrderId<'a> {
        type Output = crate::web_api::interface::common::view_models::PDF;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_with_body::<crate::web_api::interface::common::view_models::PDF, _>(
                HttpMethod::Patch,
                "/api/Orders/PDF",
                query,
                self.settings,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_with_body_raw(HttpMethod::Patch, "/api/Orders/PDF", query, self.settings)
                .await
        }
    }

    /// `PATCH` `/api/Orders/PDF`
    ///
    /// Parameter `order_number`: Numer zamówienia obcego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
    ///
    /// Body `settings`: Ustawienia wydruku.
    pub struct ByOrderNumberBuffer<'a> {
        /// Numer zamówienia obcego.
        pub order_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub buffer: bool,
        /// Ustawienia wydruku.
        pub settings: &'a crate::web_api::interface::common::view_models::PDFSettings,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByOrderNumberBuffer<'a> {
        type Output = crate::web_api::interface::common::view_models::PDF;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_with_body::<crate::web_api::interface::common::view_models::PDF, _>(
                HttpMethod::Patch,
                "/api/Orders/PDF",
                query,
                self.settings,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_with_body_raw(HttpMethod::Patch, "/api/Orders/PDF", query, self.settings)
                .await
        }
    }
}

pub mod get_invoices_pdf {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Orders/InvoicesPDF`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    ///
    /// Parameter `print_note`: Czy umieścić notatkę na wydruku PDF. W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
    pub struct ByOrderIdPrintNote {
        /// Id zamówienia obcego.
        pub order_id: i32,
        /// Czy umieścić notatkę na wydruku PDF.
        /// W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
        pub print_note: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderIdPrintNote {
        type Output = Vec<crate::web_api::interface::common::view_models::PDF>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            push_query_param(&mut query, "printNote", &self.print_note)?;
            api.request_no_body::<Vec<crate::web_api::interface::common::view_models::PDF>>(
                HttpMethod::Get,
                "/api/Orders/InvoicesPDF",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            push_query_param(&mut query, "printNote", &self.print_note)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Orders/InvoicesPDF", query)
                .await
        }
    }

    /// `GET` `/api/Orders/InvoicesPDF`
    ///
    /// Parameter `order_number`: Numer zamówienia obcego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
    ///
    /// Parameter `print_note`: Czy umieścić notatkę na wydruku PDF. W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
    pub struct ByOrderNumberBufferPrintNote {
        /// Numer zamówienia obcego.
        pub order_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub buffer: bool,
        /// Czy umieścić notatkę na wydruku PDF.
        /// W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
        pub print_note: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderNumberBufferPrintNote {
        type Output = Vec<crate::web_api::interface::common::view_models::PDF>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            push_query_param(&mut query, "printNote", &self.print_note)?;
            api.request_no_body::<Vec<crate::web_api::interface::common::view_models::PDF>>(
                HttpMethod::Get,
                "/api/Orders/InvoicesPDF",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            push_query_param(&mut query, "printNote", &self.print_note)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Orders/InvoicesPDF", query)
                .await
        }
    }
}

pub mod get_status {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Orders/Status`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub struct ByOrderId {
        /// Id zamówienia obcego.
        pub order_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderId {
        type Output = crate::web_api::interface::orders::view_models::OrderStatus;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body::<crate::web_api::interface::orders::view_models::OrderStatus>(
                HttpMethod::Get,
                "/api/Orders/Status",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Orders/Status", query)
                .await
        }
    }

    /// `GET` `/api/Orders/Status`
    ///
    /// Parameter `order_number`: Numer zamówienia obcego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByOrderNumberBuffer {
        /// Numer zamówienia obcego.
        pub order_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderNumberBuffer {
        type Output = crate::web_api::interface::orders::view_models::OrderStatus;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<crate::web_api::interface::orders::view_models::OrderStatus>(
                HttpMethod::Get,
                "/api/Orders/Status",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Orders/Status", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IOrdersController {
    /// `GET` `/api/Orders`
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Orders`
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Orders/Filter`
    ///
    /// Parameter `recipient_id`: Id kontrahenta (odbiorcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByRecipient<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_list_by_recipient::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Orders/Filter`
    ///
    /// Parameter `recipient_id`: Id kontrahenta (odbiorcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByRecipientRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_list_by_recipient::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Orders/Filter`
    ///
    /// Parameter `buyer_id`: Id kontrahenta (nabywcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByBuyer<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_list_by_buyer::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Orders/Filter`
    ///
    /// Parameter `buyer_id`: Id kontrahenta (nabywcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByBuyerRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_list_by_buyer::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Orders/WZ`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn GetWZ<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_wz::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Orders/WZ`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn GetWZRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_wz::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Orders/FV`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn GetFV<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_fv::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Orders/FV`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn GetFVRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_fv::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Orders/PDF`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    ///
    /// Parameter `print_note`: Czy umieścić notatkę na wydruku PDF. W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
    pub async fn GetPDF<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_pdf::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Orders/PDF`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    ///
    /// Parameter `print_note`: Czy umieścić notatkę na wydruku PDF. W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
    pub async fn GetPDFRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_pdf::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Orders/InvoicesPDF`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    ///
    /// Parameter `print_note`: Czy umieścić notatkę na wydruku PDF. W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
    pub async fn GetInvoicesPDF<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_invoices_pdf::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Orders/InvoicesPDF`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    ///
    /// Parameter `print_note`: Czy umieścić notatkę na wydruku PDF. W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
    pub async fn GetInvoicesPDFRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_invoices_pdf::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Orders/Status`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn GetStatus<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_status::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Orders/Status`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn GetStatusRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_status::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Orders/Filter`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie zamówienia do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie zamówienia od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetList(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::orders::view_models::OrderListElement>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::orders::view_models::OrderListElement>>(HttpMethod::Get, "/api/Orders/Filter", query).await
    }

    /// `GET` `/api/Orders/Filter`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie zamówienia do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie zamówienia od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListRaw(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Orders/Filter", query)
            .await
    }

    /// `GET` `/api/Orders/Filter`
    ///
    /// Parameter `dimension_code`: Kod pola własnego
    ///
    /// Parameter `dictionary_value`: Kod elementu słownika.
    ///
    /// Parameter `value`: Inferred from route query template.
    pub async fn GetListByDimension(
        api: &ApiClient,
        dimension_code: String,
        dictionary_value: String,
        value: String,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::orders::view_models::OrderListElement>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dimensionCode", &dimension_code)?;
        push_query_param(&mut query, "dictionaryValue", &dictionary_value)?;
        push_query_param(&mut query, "value", &value)?;
        api.request_no_body::<Vec<crate::web_api::interface::orders::view_models::OrderListElement>>(HttpMethod::Get, "/api/Orders/Filter", query).await
    }

    /// `GET` `/api/Orders/Filter`
    ///
    /// Parameter `dimension_code`: Kod pola własnego
    ///
    /// Parameter `dictionary_value`: Kod elementu słownika.
    ///
    /// Parameter `value`: Inferred from route query template.
    pub async fn GetListByDimensionRaw(
        api: &ApiClient,
        dimension_code: String,
        dictionary_value: String,
        value: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dimensionCode", &dimension_code)?;
        push_query_param(&mut query, "dictionaryValue", &dictionary_value)?;
        push_query_param(&mut query, "value", &value)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Orders/Filter", query)
            .await
    }

    /// `GET` `/api/Orders/ZMODocumentTypes`
    pub async fn GetZMODocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Orders/ZMODocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Orders/ZMODocumentTypes`
    pub async fn GetZMODocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Orders/ZMODocumentTypes", query)
            .await
    }

    /// `GET` `/api/Orders/ZWODocumentTypes`
    pub async fn GetZWODocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Orders/ZWODocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Orders/ZWODocumentTypes`
    pub async fn GetZWODocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Orders/ZWODocumentTypes", query)
            .await
    }

    /// `GET` `/api/Orders/DocumentSeries`
    ///
    /// Parameter `document_type_id`: Id typu dokumentu.
    pub async fn GetDocumentSeries(
        api: &ApiClient,
        document_type_id: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentSeries>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentTypeId", &document_type_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentSeries>>(
            HttpMethod::Get,
            "/api/Orders/DocumentSeries",
            query,
        )
        .await
    }

    /// `GET` `/api/Orders/DocumentSeries`
    ///
    /// Parameter `document_type_id`: Id typu dokumentu.
    pub async fn GetDocumentSeriesRaw(
        api: &ApiClient,
        document_type_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentTypeId", &document_type_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Orders/DocumentSeries", query)
            .await
    }

    /// `GET` `/api/Orders/Markers`
    pub async fn GetMarkers(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Marker>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Marker>>(
            HttpMethod::Get,
            "/api/Orders/Markers",
            query,
        )
        .await
    }

    /// `GET` `/api/Orders/Markers`
    pub async fn GetMarkersRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Orders/Markers", query)
            .await
    }

    /// `GET` `/api/Orders/Kinds`
    pub async fn GetKinds(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Kind>>, ApiError>
    {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Kind>>(
            HttpMethod::Get,
            "/api/Orders/Kinds",
            query,
        )
        .await
    }

    /// `GET` `/api/Orders/Kinds`
    pub async fn GetKindsRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Orders/Kinds", query)
            .await
    }

    /// `GET` `/api/Orders/Catalogs`
    pub async fn GetCatalogs(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Catalog>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Catalog>>(
            HttpMethod::Get,
            "/api/Orders/Catalogs",
            query,
        )
        .await
    }

    /// `GET` `/api/Orders/Catalogs`
    pub async fn GetCatalogsRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Orders/Catalogs", query)
            .await
    }

    /// `GET` `/api/Orders/PositionRelations`
    ///
    /// Parameter `position_id`: Id pozycji zamówienia obcego.
    pub async fn GetPositionRelations(
        api: &ApiClient,
        position_id: i32,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::orders::view_models::OrderPositionRelation>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "positionId", &position_id)?;
        api.request_no_body::<crate::web_api::interface::orders::view_models::OrderPositionRelation>(HttpMethod::Get, "/api/Orders/PositionRelations", query).await
    }

    /// `GET` `/api/Orders/PositionRelations`
    ///
    /// Parameter `position_id`: Id pozycji zamówienia obcego.
    pub async fn GetPositionRelationsRaw(
        api: &ApiClient,
        position_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "positionId", &position_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Orders/PositionRelations", query)
            .await
    }

    /// `GET` `/api/Orders/IncrementalSync`
    ///
    /// Parameter `date_from`: Inferred from route query template.
    pub async fn IncrementalSync(
        api: &ApiClient,
        date_from: chrono::NaiveDateTime,
    ) -> Result<
        ResponseEnvelope<
            Vec<
                crate::web_api::interface::common::view_models::IncrementalSyncListElement<
                    serde_json::Value,
                >,
            >,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        api.request_no_body::<Vec<
            crate::web_api::interface::common::view_models::IncrementalSyncListElement<
                serde_json::Value,
            >,
        >>(HttpMethod::Get, "/api/Orders/IncrementalSync", query)
            .await
    }

    /// `GET` `/api/Orders/IncrementalSync`
    ///
    /// Parameter `date_from`: Inferred from route query template.
    pub async fn IncrementalSyncRaw(
        api: &ApiClient,
        date_from: chrono::NaiveDateTime,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Orders/IncrementalSync", query)
            .await
    }

    /// `GET` `/api/Orders/Page`
    ///
    /// Parameter `page`: Numer strony.
    ///
    /// Parameter `size`: Ilość wyników na stronie.
    ///
    /// Parameter `order_by`: Sortowanie wyników na stronie.
    pub async fn GetPagedDocument(
        api: &ApiClient,
        page: i32,
        size: i32,
        order_by: crate::web_api::interface::enums::enumOrderByType,
    ) -> Result<
        ResponseEnvelope<
            crate::web_api::interface::view_models::Page<
                crate::web_api::interface::orders::view_models::OrderListElement,
            >,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body::<crate::web_api::interface::view_models::Page<
            crate::web_api::interface::orders::view_models::OrderListElement,
        >>(HttpMethod::Get, "/api/Orders/Page", query)
            .await
    }

    /// `GET` `/api/Orders/Page`
    ///
    /// Parameter `page`: Numer strony.
    ///
    /// Parameter `size`: Ilość wyników na stronie.
    ///
    /// Parameter `order_by`: Sortowanie wyników na stronie.
    pub async fn GetPagedDocumentRaw(
        api: &ApiClient,
        page: i32,
        size: i32,
        order_by: crate::web_api::interface::enums::enumOrderByType,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Orders/Page", query)
            .await
    }

    /// `GET` `/api/Orders/Filter/ByDocumentTypes`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie zamówienia do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie zamówienia od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDocumentTypesWithRange(
        api: &ApiClient,
        date_from: crate::web_api::interface::common::view_models::FilterDocumentType,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::orders::view_models::OrderListElement>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::orders::view_models::OrderListElement>>(HttpMethod::Get, "/api/Orders/Filter/ByDocumentTypes", query).await
    }

    /// `GET` `/api/Orders/Filter/ByDocumentTypes`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie zamówienia do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie zamówienia od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDocumentTypesWithRangeRaw(
        api: &ApiClient,
        date_from: crate::web_api::interface::common::view_models::FilterDocumentType,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Orders/Filter/ByDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Orders/ExternalMetadata`
    ///
    /// Parameter `page`: Numer strony.
    ///
    /// Parameter `size`: Rozmiar strony.
    ///
    /// Parameter `order_by`: Kierunek sortowania zgodny z .
    pub async fn GetDocumentExternalMetadata(
        api: &ApiClient,
        page: Option<i32>,
        size: Option<i32>,
        order_by: Option<crate::web_api::interface::enums::enumOrderByType>,
    ) -> Result<ResponseEnvelope<Vec<serde_json::Value>>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body::<Vec<serde_json::Value>>(
            HttpMethod::Get,
            "/api/Orders/ExternalMetadata",
            query,
        )
        .await
    }

    /// `GET` `/api/Orders/ExternalMetadata`
    ///
    /// Parameter `page`: Numer strony.
    ///
    /// Parameter `size`: Rozmiar strony.
    ///
    /// Parameter `order_by`: Kierunek sortowania zgodny z .
    pub async fn GetDocumentExternalMetadataRaw(
        api: &ApiClient,
        page: Option<i32>,
        size: Option<i32>,
        order_by: Option<crate::web_api::interface::enums::enumOrderByType>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Orders/ExternalMetadata", query)
            .await
    }
}
