use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `Sales`.
pub struct ISalesController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Sales`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output = Vec<crate::web_api::interface::sales::view_models::SaleDocumentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentListElement>>(HttpMethod::Get, "/api/Sales", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales", query)
                .await
        }
    }

    /// `GET` `/api/Sales`
    ///
    /// Parameter `id`: Id dokumentu sprzedaży.
    pub struct ById {
        /// Id dokumentu sprzedaży.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = crate::web_api::interface::sales::view_models::SaleDocument;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<crate::web_api::interface::sales::view_models::SaleDocument>(
                HttpMethod::Get,
                "/api/Sales",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales", query)
                .await
        }
    }

    /// `GET` `/api/Sales`
    ///
    /// Parameter `number`: Numer dokumentu sprzedaży.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
    pub struct ByNumberBuffer {
        /// Numer dokumentu sprzedaży.
        pub number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie pobrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByNumberBuffer {
        type Output = crate::web_api::interface::sales::view_models::SaleDocument;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<crate::web_api::interface::sales::view_models::SaleDocument>(
                HttpMethod::Get,
                "/api/Sales",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales", query)
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

    /// `GET` `/api/Sales/Filter`
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
        type Output = Vec<crate::web_api::interface::sales::view_models::SaleDocumentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "buyerId", &self.buyer_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentListElement>>(HttpMethod::Get, "/api/Sales/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "buyerId", &self.buyer_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales/Filter", query)
                .await
        }
    }

    /// `GET` `/api/Sales/Filter`
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
        type Output = Vec<crate::web_api::interface::sales::view_models::SaleDocumentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "buyerCode", &self.buyer_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentListElement>>(HttpMethod::Get, "/api/Sales/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "buyerCode", &self.buyer_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales/Filter", query)
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

    /// `GET` `/api/Sales/Filter`
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
        type Output = Vec<crate::web_api::interface::sales::view_models::SaleDocumentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "recipientId", &self.recipient_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentListElement>>(HttpMethod::Get, "/api/Sales/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "recipientId", &self.recipient_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales/Filter", query)
                .await
        }
    }

    /// `GET` `/api/Sales/Filter`
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
        type Output = Vec<crate::web_api::interface::sales::view_models::SaleDocumentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "recipientCode", &self.recipient_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentListElement>>(HttpMethod::Get, "/api/Sales/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "recipientCode", &self.recipient_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales/Filter", query)
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

    /// `GET` `/api/Sales/WZ`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub struct ByDocumentId {
        /// Id dokumentu sprzedaży.
        pub document_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentId {
        type Output = Vec<crate::web_api::interface::sales::view_models::SaleDocumentWZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentWZ>>(HttpMethod::Get, "/api/Sales/WZ", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales/WZ", query)
                .await
        }
    }

    /// `GET` `/api/Sales/WZ`
    ///
    /// Parameter `document_number`: Numer dokumentu sprzedaży.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
    pub struct ByDocumentNumberBuffer {
        /// Numer dokumentu sprzedaży.
        pub document_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumberBuffer {
        type Output = Vec<crate::web_api::interface::sales::view_models::SaleDocumentWZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentWZ>>(HttpMethod::Get, "/api/Sales/WZ", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales/WZ", query)
                .await
        }
    }
}

pub mod get_zmo {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Sales/ZMO`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub struct ByDocumentId {
        /// Id dokumentu sprzedaży.
        pub document_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentId {
        type Output = Vec<crate::web_api::interface::sales::view_models::SaleDocumentZMO>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentZMO>>(HttpMethod::Get, "/api/Sales/ZMO", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales/ZMO", query)
                .await
        }
    }

    /// `GET` `/api/Sales/ZMO`
    ///
    /// Parameter `document_number`: Numer dokumentu sprzedaży.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
    pub struct ByDocumentNumberBuffer {
        /// Numer dokumentu sprzedaży.
        pub document_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumberBuffer {
        type Output = Vec<crate::web_api::interface::sales::view_models::SaleDocumentZMO>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentZMO>>(HttpMethod::Get, "/api/Sales/ZMO", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales/ZMO", query)
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

    /// `GET` `/api/Sales/Status`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub struct ByDocumentId {
        /// Id dokumentu sprzedaży.
        pub document_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentId {
        type Output = crate::web_api::interface::sales::view_models::SaleDocumentStatus;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body::<crate::web_api::interface::sales::view_models::SaleDocumentStatus>(HttpMethod::Get, "/api/Sales/Status", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales/Status", query)
                .await
        }
    }

    /// `GET` `/api/Sales/Status`
    ///
    /// Parameter `document_number`: Numer dokumentu sprzedaży.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrany status dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
    pub struct ByDocumentNumberBuffer {
        /// Numer dokumentu sprzedaży.
        pub document_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie pobrany status dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumberBuffer {
        type Output = crate::web_api::interface::sales::view_models::SaleDocumentStatus;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<crate::web_api::interface::sales::view_models::SaleDocumentStatus>(HttpMethod::Get, "/api/Sales/Status", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales/Status", query)
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

    /// `GET` `/api/Sales/PDF`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    ///
    /// Parameter `print_note`: Czy umieścić notatkę na wydruku PDF. W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
    pub struct ByDocumentIdPrintNote {
        /// Id dokumentu sprzedaży.
        pub document_id: i32,
        /// Czy umieścić notatkę na wydruku PDF.
        /// W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
        pub print_note: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentIdPrintNote {
        type Output = crate::web_api::interface::common::view_models::PDF;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            push_query_param(&mut query, "printNote", &self.print_note)?;
            api.request_no_body::<crate::web_api::interface::common::view_models::PDF>(
                HttpMethod::Get,
                "/api/Sales/PDF",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            push_query_param(&mut query, "printNote", &self.print_note)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales/PDF", query)
                .await
        }
    }

    /// `GET` `/api/Sales/PDF`
    ///
    /// Parameter `document_number`: Numer dokumentu sprzedaży.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
    ///
    /// Parameter `print_note`: Czy umieścić notatkę na wydruku PDF. W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
    pub struct ByDocumentNumberBufferPrintNote {
        /// Numer dokumentu sprzedaży.
        pub document_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
        pub buffer: bool,
        /// Czy umieścić notatkę na wydruku PDF.
        /// W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
        pub print_note: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumberBufferPrintNote {
        type Output = crate::web_api::interface::common::view_models::PDF;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            push_query_param(&mut query, "printNote", &self.print_note)?;
            api.request_no_body::<crate::web_api::interface::common::view_models::PDF>(
                HttpMethod::Get,
                "/api/Sales/PDF",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            push_query_param(&mut query, "printNote", &self.print_note)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales/PDF", query)
                .await
        }
    }

    /// `PATCH` `/api/Sales/PDF`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    ///
    /// Body `settings`: Ustawienia wydruku.
    pub struct ByDocumentId<'a> {
        /// Id dokumentu sprzedaży.
        pub document_id: i32,
        /// Ustawienia wydruku.
        pub settings: &'a crate::web_api::interface::common::view_models::PDFSettings,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByDocumentId<'a> {
        type Output = crate::web_api::interface::common::view_models::PDF;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_with_body::<crate::web_api::interface::common::view_models::PDF, _>(
                HttpMethod::Patch,
                "/api/Sales/PDF",
                query,
                self.settings,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_with_body_raw(HttpMethod::Patch, "/api/Sales/PDF", query, self.settings)
                .await
        }
    }

    /// `PATCH` `/api/Sales/PDF`
    ///
    /// Parameter `document_number`: Numer dokumentu sprzedaży.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
    ///
    /// Body `settings`: Ustawienia wydruku.
    pub struct ByDocumentNumberBuffer<'a> {
        /// Numer dokumentu sprzedaży.
        pub document_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub buffer: bool,
        /// Ustawienia wydruku.
        pub settings: &'a crate::web_api::interface::common::view_models::PDFSettings,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByDocumentNumberBuffer<'a> {
        type Output = crate::web_api::interface::common::view_models::PDF;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_with_body::<crate::web_api::interface::common::view_models::PDF, _>(
                HttpMethod::Patch,
                "/api/Sales/PDF",
                query,
                self.settings,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_with_body_raw(HttpMethod::Patch, "/api/Sales/PDF", query, self.settings)
                .await
        }
    }
}

pub mod get_correction {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Sales/Correction`
    ///
    /// Parameter `id`: Id korekty dokumentu sprzedaży.
    pub struct ById {
        /// Id korekty dokumentu sprzedaży.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = crate::web_api::interface::sales::view_models::SaleCorrection;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<crate::web_api::interface::sales::view_models::SaleCorrection>(
                HttpMethod::Get,
                "/api/Sales/Correction",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales/Correction", query)
                .await
        }
    }

    /// `GET` `/api/Sales/Correction`
    ///
    /// Parameter `number`: Numer korekty dokumentu sprzedaży.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
    pub struct ByNumberBuffer {
        /// Numer korekty dokumentu sprzedaży.
        pub number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie pobrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByNumberBuffer {
        type Output = crate::web_api::interface::sales::view_models::SaleCorrection;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<crate::web_api::interface::sales::view_models::SaleCorrection>(
                HttpMethod::Get,
                "/api/Sales/Correction",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales/Correction", query)
                .await
        }
    }
}

pub mod get_correction_sequence {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Sales/CorrectionSequence`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub struct ByDocumentId {
        /// Id dokumentu sprzedaży.
        pub document_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentId {
        type Output = Vec<crate::web_api::interface::sales::view_models::SaleDocumentCorrection>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentCorrection>>(HttpMethod::Get, "/api/Sales/CorrectionSequence", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales/CorrectionSequence", query)
                .await
        }
    }

    /// `GET` `/api/Sales/CorrectionSequence`
    ///
    /// Parameter `document_number`: Numer dokumentu sprzedaży.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
    pub struct ByDocumentNumberBuffer {
        /// Numer dokumentu sprzedaży.
        pub document_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumberBuffer {
        type Output = Vec<crate::web_api::interface::sales::view_models::SaleDocumentCorrection>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentCorrection>>(HttpMethod::Get, "/api/Sales/CorrectionSequence", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Sales/CorrectionSequence", query)
                .await
        }
    }
}

pub mod refresh_ksef_status_async {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/v2026.2/Sales/Status/Refresh/{id:int}`
    ///
    /// Parameter `id`: Identyfikator dokumentu.
    pub struct ById {
        /// Identyfikator dokumentu.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<()>(
                HttpMethod::Get,
                "/api/v2026.2/Sales/Status/Refresh/{id:int}",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/v2026.2/Sales/Status/Refresh/{id:int}",
                query,
            )
            .await
        }
    }

    /// `GET` `/api/v2026.2/Sales/Status/Refresh/Batch`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<()>(
                HttpMethod::Get,
                "/api/v2026.2/Sales/Status/Refresh/Batch",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/v2026.2/Sales/Status/Refresh/Batch",
                query,
            )
            .await
        }
    }
}

#[allow(non_snake_case)]
impl ISalesController {
    /// `GET` `/api/Sales`
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Sales`
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Sales/Filter`
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

    /// `GET` `/api/Sales/Filter`
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

    /// `GET` `/api/Sales/Filter`
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

    /// `GET` `/api/Sales/Filter`
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

    /// `GET` `/api/Sales/WZ`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub async fn GetWZ<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_wz::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Sales/WZ`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub async fn GetWZRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_wz::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Sales/ZMO`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub async fn GetZMO<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_zmo::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Sales/ZMO`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub async fn GetZMORaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_zmo::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Sales/Status`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub async fn GetStatus<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_status::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Sales/Status`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub async fn GetStatusRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_status::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Sales/PDF`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
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

    /// `GET` `/api/Sales/PDF`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
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

    /// `GET` `/api/Sales/Correction`
    ///
    /// Parameter `id`: Id korekty dokumentu sprzedaży.
    pub async fn GetCorrection<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_correction::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Sales/Correction`
    ///
    /// Parameter `id`: Id korekty dokumentu sprzedaży.
    pub async fn GetCorrectionRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_correction::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Sales/CorrectionSequence`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub async fn GetCorrectionSequence<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_correction_sequence::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Sales/CorrectionSequence`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub async fn GetCorrectionSequenceRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_correction_sequence::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/v2026.2/Sales/Status/Refresh/{id:int}`
    ///
    /// Parameter `id`: Identyfikator dokumentu.
    pub async fn RefreshKsefStatusAsync<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: refresh_ksef_status_async::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/v2026.2/Sales/Status/Refresh/{id:int}`
    ///
    /// Parameter `id`: Identyfikator dokumentu.
    pub async fn RefreshKsefStatusAsyncRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: refresh_ksef_status_async::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Sales/Filter`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetList(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::sales::view_models::SaleDocumentListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentListElement>>(HttpMethod::Get, "/api/Sales/Filter", query).await
    }

    /// `GET` `/api/Sales/Filter`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty sprzedaży od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListRaw(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/Filter", query)
            .await
    }

    /// `GET` `/api/Sales/Filter`
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
            Vec<crate::web_api::interface::sales::view_models::SaleDocumentListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dimensionCode", &dimension_code)?;
        push_query_param(&mut query, "dictionaryValue", &dictionary_value)?;
        push_query_param(&mut query, "value", &value)?;
        api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentListElement>>(HttpMethod::Get, "/api/Sales/Filter", query).await
    }

    /// `GET` `/api/Sales/Filter`
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
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/Filter", query)
            .await
    }

    /// `GET` `/api/Sales/Markers`
    pub async fn GetMarkers(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Marker>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Marker>>(
            HttpMethod::Get,
            "/api/Sales/Markers",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/Markers`
    pub async fn GetMarkersRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/Markers", query)
            .await
    }

    /// `GET` `/api/Sales/Kinds`
    pub async fn GetKinds(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Kind>>, ApiError>
    {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Kind>>(
            HttpMethod::Get,
            "/api/Sales/Kinds",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/Kinds`
    pub async fn GetKindsRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/Kinds", query)
            .await
    }

    /// `GET` `/api/Sales/Catalogs`
    pub async fn GetCatalogs(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Catalog>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Catalog>>(
            HttpMethod::Get,
            "/api/Sales/Catalogs",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/Catalogs`
    pub async fn GetCatalogsRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/Catalogs", query)
            .await
    }

    /// `GET` `/api/Sales/DEXDocumentTypes`
    pub async fn GetDEXDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/DEXDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/DEXDocumentTypes`
    pub async fn GetDEXDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/DEXDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/FVSMDocumentTypes`
    pub async fn GetFVSMDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/FVSMDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/FVSMDocumentTypes`
    pub async fn GetFVSMDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/FVSMDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/FVSDocumentTypes`
    pub async fn GetFVSDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/FVSDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/FVSDocumentTypes`
    pub async fn GetFVSDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/FVSDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/FVWDocumentTypes`
    pub async fn GetFVWDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/FVWDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/FVWDocumentTypes`
    pub async fn GetFVWDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/FVWDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/SKODocumentTypes`
    pub async fn GetSKODocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/SKODocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/SKODocumentTypes`
    pub async fn GetSKODocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/SKODocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/SKWDocumentTypes`
    pub async fn GetSKWDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/SKWDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/SKWDocumentTypes`
    pub async fn GetSKWDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/SKWDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/PARDocumentTypes`
    pub async fn GetPARDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/PARDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/PARDocumentTypes`
    pub async fn GetPARDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/PARDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/SRSDocumentTypes`
    pub async fn GetSRSDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/SRSDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/SRSDocumentTypes`
    pub async fn GetSRSDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/SRSDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/REXDocumentTypes`
    pub async fn GetREXDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/REXDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/REXDocumentTypes`
    pub async fn GetREXDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/REXDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/RUSDocumentTypes`
    pub async fn GetRUSDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/RUSDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/RUSDocumentTypes`
    pub async fn GetRUSDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/RUSDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/WDTDocumentTypes`
    pub async fn GetWDTDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/WDTDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/WDTDocumentTypes`
    pub async fn GetWDTDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/WDTDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/DXKDocumentTypes`
    pub async fn GetDXKDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/DXKDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/DXKDocumentTypes`
    pub async fn GetDXKDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/DXKDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/PRKDocumentTypes`
    pub async fn GetPRKDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/PRKDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/PRKDocumentTypes`
    pub async fn GetPRKDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/PRKDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/SRKDocumentTypes`
    pub async fn GetSRKDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/SRKDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/SRKDocumentTypes`
    pub async fn GetSRKDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/SRKDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/RKSDocumentTypes`
    pub async fn GetRKSDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/RKSDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/RKSDocumentTypes`
    pub async fn GetRKSDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/RKSDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/RXKDocumentTypes`
    pub async fn GetRXKDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/RXKDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/RXKDocumentTypes`
    pub async fn GetRXKDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/RXKDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/FVMKDocumentTypes`
    pub async fn GetFVMKDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/FVMKDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/FVMKDocumentTypes`
    pub async fn GetFVMKDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/FVMKDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/FKSDocumentTypes`
    pub async fn GetFKSDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/FKSDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/FKSDocumentTypes`
    pub async fn GetFKSDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/FKSDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/FKWDocumentTypes`
    pub async fn GetFKWDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/FKWDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/FKWDocumentTypes`
    pub async fn GetFKWDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/FKWDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/SKKDocumentTypes`
    pub async fn GetSKKDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/SKKDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/SKKDocumentTypes`
    pub async fn GetSKKDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/SKKDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/SKWKDocumentTypes`
    pub async fn GetSKWKDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/SKWKDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/SKWKDocumentTypes`
    pub async fn GetSKWKDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/SKWKDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/WKSDocumentTypes`
    pub async fn GetWKSDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/WKSDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/WKSDocumentTypes`
    pub async fn GetWKSDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/WKSDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/WDTKDocumentTypes`
    pub async fn GetWDTKDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Sales/WDTKDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/WDTKDocumentTypes`
    pub async fn GetWDTKDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/WDTKDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/DocumentSeries`
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
            "/api/Sales/DocumentSeries",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/DocumentSeries`
    ///
    /// Parameter `document_type_id`: Id typu dokumentu.
    pub async fn GetDocumentSeriesRaw(
        api: &ApiClient,
        document_type_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentTypeId", &document_type_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/DocumentSeries", query)
            .await
    }

    /// `GET` `/api/Sales/IncrementalSync`
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
        >>(HttpMethod::Get, "/api/Sales/IncrementalSync", query)
            .await
    }

    /// `GET` `/api/Sales/IncrementalSync`
    ///
    /// Parameter `date_from`: Inferred from route query template.
    pub async fn IncrementalSyncRaw(
        api: &ApiClient,
        date_from: chrono::NaiveDateTime,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/IncrementalSync", query)
            .await
    }

    /// `GET` `/api/Sales/Page`
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
                crate::web_api::interface::sales::view_models::SaleDocumentListElement,
            >,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body::<crate::web_api::interface::view_models::Page<
            crate::web_api::interface::sales::view_models::SaleDocumentListElement,
        >>(HttpMethod::Get, "/api/Sales/Page", query)
            .await
    }

    /// `GET` `/api/Sales/Page`
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
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/Page", query)
            .await
    }

    /// `GET` `/api/Sales/Filter/ByDocumentTypes`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDocumentTypesWithRange(
        api: &ApiClient,
        date_from: crate::web_api::interface::common::view_models::FilterDocumentType,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::sales::view_models::SaleDocumentListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentListElement>>(HttpMethod::Get, "/api/Sales/Filter/ByDocumentTypes", query).await
    }

    /// `GET` `/api/Sales/Filter/ByDocumentTypes`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDocumentTypesWithRangeRaw(
        api: &ApiClient,
        date_from: crate::web_api::interface::common::view_models::FilterDocumentType,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/Filter/ByDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Sales/ExternalMetadata`
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
            "/api/Sales/ExternalMetadata",
            query,
        )
        .await
    }

    /// `GET` `/api/Sales/ExternalMetadata`
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
        api.request_no_body_raw(HttpMethod::Get, "/api/Sales/ExternalMetadata", query)
            .await
    }
}
