use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `OwnOrders`.
pub struct IOwnOrdersController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/OwnOrders`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output = Vec<crate::web_api::interface::own_orders::view_models::OwnOrderListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<Vec<crate::web_api::interface::own_orders::view_models::OwnOrderListElement>>(HttpMethod::Get, "/api/OwnOrders", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders", query)
                .await
        }
    }

    /// `GET` `/api/OwnOrders`
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
                HttpMethod::Get,
                "/api/OwnOrders",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders", query)
                .await
        }
    }

    /// `GET` `/api/OwnOrders`
    ///
    /// Parameter `number`: Numer dokumentu zamówienia własnego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrane zamówienie własne o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByNumberBuffer {
        /// Numer dokumentu zamówienia własnego.
        pub number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie pobrane zamówienie własne o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByNumberBuffer {
        type Output = crate::web_api::interface::own_orders::view_models::OwnOrder;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<crate::web_api::interface::own_orders::view_models::OwnOrder>(
                HttpMethod::Get,
                "/api/OwnOrders",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders", query)
                .await
        }
    }
}

pub mod get_list_by_deliverer {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/OwnOrders/Filter`
    ///
    /// Parameter `deliverer_id`: Id kontrahenta (dostawcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByDelivererIdDateFromDateTo {
        /// Id kontrahenta (dostawcy).
        pub deliverer_id: i32,
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
    impl Overload for ByDelivererIdDateFromDateTo {
        type Output = Vec<crate::web_api::interface::own_orders::view_models::OwnOrderListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "delivererId", &self.deliverer_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::own_orders::view_models::OwnOrderListElement>>(HttpMethod::Get, "/api/OwnOrders/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "delivererId", &self.deliverer_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/Filter", query)
                .await
        }
    }

    /// `GET` `/api/OwnOrders/Filter`
    ///
    /// Parameter `deliverer_code`: Kod kontrahenta (dostawcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByDelivererCodeDateFromDateTo {
        /// Kod kontrahenta (dostawcy).
        pub deliverer_code: String,
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
    impl Overload for ByDelivererCodeDateFromDateTo {
        type Output = Vec<crate::web_api::interface::own_orders::view_models::OwnOrderListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "delivererCode", &self.deliverer_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::own_orders::view_models::OwnOrderListElement>>(HttpMethod::Get, "/api/OwnOrders/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "delivererCode", &self.deliverer_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/Filter", query)
                .await
        }
    }
}

pub mod get_list_by_seller {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/OwnOrders/Filter`
    ///
    /// Parameter `seller_id`: Id kontrahenta (sprzedawcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct BySellerIdDateFromDateTo {
        /// Id kontrahenta (sprzedawcy).
        pub seller_id: i32,
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
    impl Overload for BySellerIdDateFromDateTo {
        type Output = Vec<crate::web_api::interface::own_orders::view_models::OwnOrderListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "sellerId", &self.seller_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::own_orders::view_models::OwnOrderListElement>>(HttpMethod::Get, "/api/OwnOrders/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "sellerId", &self.seller_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/Filter", query)
                .await
        }
    }

    /// `GET` `/api/OwnOrders/Filter`
    ///
    /// Parameter `seller_code`: Kod kontrahenta (sprzedawcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct BySellerCodeDateFromDateTo {
        /// Kod kontrahenta (sprzedawcy).
        pub seller_code: String,
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
    impl Overload for BySellerCodeDateFromDateTo {
        type Output = Vec<crate::web_api::interface::own_orders::view_models::OwnOrderListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "sellerCode", &self.seller_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::own_orders::view_models::OwnOrderListElement>>(HttpMethod::Get, "/api/OwnOrders/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "sellerCode", &self.seller_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/Filter", query)
                .await
        }
    }
}

pub mod get_pz {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/OwnOrders/PZ`
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
            api.request_no_body::<Vec<crate::web_api::interface::own_orders::view_models::OwnOrderPZ>>(HttpMethod::Get, "/api/OwnOrders/PZ", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/PZ", query)
                .await
        }
    }

    /// `GET` `/api/OwnOrders/PZ`
    ///
    /// Parameter `order_number`: Numer zamówienia własnego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie własne o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByOrderNumberBuffer {
        /// Numer zamówienia własnego.
        pub order_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie własne o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderNumberBuffer {
        type Output = Vec<crate::web_api::interface::own_orders::view_models::OwnOrderPZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<Vec<crate::web_api::interface::own_orders::view_models::OwnOrderPZ>>(HttpMethod::Get, "/api/OwnOrders/PZ", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/PZ", query)
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

    /// `GET` `/api/OwnOrders/FV`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub struct ByOrderId {
        /// Id zamówienia własnego.
        pub order_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderId {
        type Output = Vec<crate::web_api::interface::own_orders::view_models::OwnOrderFV>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::own_orders::view_models::OwnOrderFV>>(HttpMethod::Get, "/api/OwnOrders/FV", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/FV", query)
                .await
        }
    }

    /// `GET` `/api/OwnOrders/FV`
    ///
    /// Parameter `order_number`: Numer zamówienia własnego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie własne o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByOrderNumberBuffer {
        /// Numer zamówienia własnego.
        pub order_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie własne o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderNumberBuffer {
        type Output = Vec<crate::web_api::interface::own_orders::view_models::OwnOrderFV>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<Vec<crate::web_api::interface::own_orders::view_models::OwnOrderFV>>(HttpMethod::Get, "/api/OwnOrders/FV", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/FV", query)
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

    /// `GET` `/api/OwnOrders/PDF`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    ///
    /// Parameter `print_note`: Czy umieścić notatkę na wydruku PDF. W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
    pub struct ByOrderIdPrintNote {
        /// Id zamówienia własnego.
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
                "/api/OwnOrders/PDF",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            push_query_param(&mut query, "printNote", &self.print_note)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/PDF", query)
                .await
        }
    }

    /// `GET` `/api/OwnOrders/PDF`
    ///
    /// Parameter `order_number`: Numer zamówienia własnego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie własne o podanym numerze dokumentu, które nie znajduje się w buforze.
    ///
    /// Parameter `print_note`: Czy umieścić notatkę na wydruku PDF. W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
    pub struct ByOrderNumberBufferPrintNote {
        /// Numer zamówienia własnego.
        pub order_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie własne o podanym numerze dokumentu, które nie znajduje się w buforze.
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
                "/api/OwnOrders/PDF",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            push_query_param(&mut query, "printNote", &self.print_note)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/PDF", query)
                .await
        }
    }

    /// `PATCH` `/api/OwnOrders/PDF`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    ///
    /// Body `settings`: Ustawienia wydruku.
    pub struct ByOrderId<'a> {
        /// Id zamówienia własnego.
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
                "/api/OwnOrders/PDF",
                query,
                self.settings,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_with_body_raw(
                HttpMethod::Patch,
                "/api/OwnOrders/PDF",
                query,
                self.settings,
            )
            .await
        }
    }

    /// `PATCH` `/api/OwnOrders/PDF`
    ///
    /// Parameter `order_number`: Numer zamówienia własnego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
    ///
    /// Body `settings`: Ustawienia wydruku.
    pub struct ByOrderNumberBuffer<'a> {
        /// Numer zamówienia własnego.
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
                "/api/OwnOrders/PDF",
                query,
                self.settings,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_with_body_raw(
                HttpMethod::Patch,
                "/api/OwnOrders/PDF",
                query,
                self.settings,
            )
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

    /// `GET` `/api/OwnOrders/Status`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub struct ByOrderId {
        /// Id zamówienia własnego.
        pub order_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderId {
        type Output = crate::web_api::interface::own_orders::view_models::OwnOrderStatus;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body::<crate::web_api::interface::own_orders::view_models::OwnOrderStatus>(HttpMethod::Get, "/api/OwnOrders/Status", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/Status", query)
                .await
        }
    }

    /// `GET` `/api/OwnOrders/Status`
    ///
    /// Parameter `order_number`: Numer zamówienia własnego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie własne o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByOrderNumberBuffer {
        /// Numer zamówienia własnego.
        pub order_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie własne o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderNumberBuffer {
        type Output = crate::web_api::interface::own_orders::view_models::OwnOrderStatus;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<crate::web_api::interface::own_orders::view_models::OwnOrderStatus>(HttpMethod::Get, "/api/OwnOrders/Status", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/Status", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IOwnOrdersController {
    /// `GET` `/api/OwnOrders`
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/OwnOrders`
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/OwnOrders/Filter`
    ///
    /// Parameter `deliverer_id`: Id kontrahenta (dostawcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByDeliverer<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_list_by_deliverer::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/OwnOrders/Filter`
    ///
    /// Parameter `deliverer_id`: Id kontrahenta (dostawcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByDelivererRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_list_by_deliverer::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/OwnOrders/Filter`
    ///
    /// Parameter `seller_id`: Id kontrahenta (sprzedawcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListBySeller<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_list_by_seller::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/OwnOrders/Filter`
    ///
    /// Parameter `seller_id`: Id kontrahenta (sprzedawcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListBySellerRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_list_by_seller::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/OwnOrders/PZ`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub async fn GetPZ<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_pz::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/OwnOrders/PZ`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub async fn GetPZRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_pz::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/OwnOrders/FV`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub async fn GetFV<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_fv::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/OwnOrders/FV`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub async fn GetFVRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_fv::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/OwnOrders/PDF`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
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

    /// `GET` `/api/OwnOrders/PDF`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
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

    /// `GET` `/api/OwnOrders/Status`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub async fn GetStatus<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_status::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/OwnOrders/Status`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub async fn GetStatusRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_status::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/OwnOrders/Filter`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie zamówienia do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie zamówienia od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetList(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::own_orders::view_models::OwnOrderListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::own_orders::view_models::OwnOrderListElement>>(HttpMethod::Get, "/api/OwnOrders/Filter", query).await
    }

    /// `GET` `/api/OwnOrders/Filter`
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
        api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/Filter", query)
            .await
    }

    /// `GET` `/api/OwnOrders/Filter`
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
        ResponseEnvelope<
            Vec<crate::web_api::interface::own_orders::view_models::OwnOrderListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dimensionCode", &dimension_code)?;
        push_query_param(&mut query, "dictionaryValue", &dictionary_value)?;
        push_query_param(&mut query, "value", &value)?;
        api.request_no_body::<Vec<crate::web_api::interface::own_orders::view_models::OwnOrderListElement>>(HttpMethod::Get, "/api/OwnOrders/Filter", query).await
    }

    /// `GET` `/api/OwnOrders/Filter`
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
        api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/Filter", query)
            .await
    }

    /// `GET` `/api/OwnOrders/ZMODocumentTypes`
    pub async fn GetZMWDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/OwnOrders/ZMODocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/OwnOrders/ZMODocumentTypes`
    pub async fn GetZMWDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/ZMODocumentTypes", query)
            .await
    }

    /// `GET` `/api/OwnOrders/ZWODocumentTypes`
    pub async fn GetZWWDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/OwnOrders/ZWODocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/OwnOrders/ZWODocumentTypes`
    pub async fn GetZWWDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/ZWODocumentTypes", query)
            .await
    }

    /// `GET` `/api/OwnOrders/DocumentSeries`
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
            "/api/OwnOrders/DocumentSeries",
            query,
        )
        .await
    }

    /// `GET` `/api/OwnOrders/DocumentSeries`
    ///
    /// Parameter `document_type_id`: Id typu dokumentu.
    pub async fn GetDocumentSeriesRaw(
        api: &ApiClient,
        document_type_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentTypeId", &document_type_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/DocumentSeries", query)
            .await
    }

    /// `GET` `/api/OwnOrders/Markers`
    pub async fn GetMarkers(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Marker>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Marker>>(
            HttpMethod::Get,
            "/api/OwnOrders/Markers",
            query,
        )
        .await
    }

    /// `GET` `/api/OwnOrders/Markers`
    pub async fn GetMarkersRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/Markers", query)
            .await
    }

    /// `GET` `/api/OwnOrders/Kinds`
    pub async fn GetKinds(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Kind>>, ApiError>
    {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Kind>>(
            HttpMethod::Get,
            "/api/OwnOrders/Kinds",
            query,
        )
        .await
    }

    /// `GET` `/api/OwnOrders/Kinds`
    pub async fn GetKindsRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/Kinds", query)
            .await
    }

    /// `GET` `/api/OwnOrders/Catalogs`
    pub async fn GetCatalogs(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Catalog>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Catalog>>(
            HttpMethod::Get,
            "/api/OwnOrders/Catalogs",
            query,
        )
        .await
    }

    /// `GET` `/api/OwnOrders/Catalogs`
    pub async fn GetCatalogsRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/Catalogs", query)
            .await
    }

    /// `GET` `/api/OwnOrders/PositionRelations`
    ///
    /// Parameter `position_id`: Id pozycji zamówienia własnego.
    pub async fn GetPositionRelations(
        api: &ApiClient,
        position_id: i32,
    ) -> Result<
        ResponseEnvelope<
            crate::web_api::interface::own_orders::view_models::OwnOrderPositionRelation,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "positionId", &position_id)?;
        api.request_no_body::<crate::web_api::interface::own_orders::view_models::OwnOrderPositionRelation>(HttpMethod::Get, "/api/OwnOrders/PositionRelations", query).await
    }

    /// `GET` `/api/OwnOrders/PositionRelations`
    ///
    /// Parameter `position_id`: Id pozycji zamówienia własnego.
    pub async fn GetPositionRelationsRaw(
        api: &ApiClient,
        position_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "positionId", &position_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/PositionRelations", query)
            .await
    }

    /// `GET` `/api/OwnOrders/Page`
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
                crate::web_api::interface::own_orders::view_models::OwnOrderListElement,
            >,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body::<crate::web_api::interface::view_models::Page<
            crate::web_api::interface::own_orders::view_models::OwnOrderListElement,
        >>(HttpMethod::Get, "/api/OwnOrders/Page", query)
            .await
    }

    /// `GET` `/api/OwnOrders/Page`
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
        api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrders/Page", query)
            .await
    }

    /// `GET` `/api/OwnOrders/Filter/ByDocumentTypes`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie zamówienia do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie zamówienia od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDocumentTypesWithRange(
        api: &ApiClient,
        date_from: crate::web_api::interface::common::view_models::FilterDocumentType,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::own_orders::view_models::OwnOrderListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::own_orders::view_models::OwnOrderListElement>>(HttpMethod::Get, "/api/OwnOrders/Filter/ByDocumentTypes", query).await
    }

    /// `GET` `/api/OwnOrders/Filter/ByDocumentTypes`
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
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/OwnOrders/Filter/ByDocumentTypes",
            query,
        )
        .await
    }
}
