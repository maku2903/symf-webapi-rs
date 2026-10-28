use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `Payments`.
pub struct IPaymentsController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Payments`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output = Vec<crate::web_api::interface::payments::view_models::PaymentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<Vec<crate::web_api::interface::payments::view_models::PaymentListElement>>(HttpMethod::Get, "/api/Payments", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(HttpMethod::Get, "/api/Payments", query)
                .await
        }
    }

    /// `GET` `/api/Payments`
    ///
    /// Parameter `id`: Id dokumentu płatności.
    pub struct ById {
        /// Id dokumentu płatności.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = crate::web_api::interface::payments::view_models::Payment;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<crate::web_api::interface::payments::view_models::Payment>(
                HttpMethod::Get,
                "/api/Payments",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Payments", query)
                .await
        }
    }

    /// `GET` `/api/Payments`
    ///
    /// Parameter `number`: Numer dokumentu płatności.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
    pub struct ByNumberBuffer {
        /// Numer dokumentu płatności.
        pub number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie pobrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByNumberBuffer {
        type Output = crate::web_api::interface::payments::view_models::Payment;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<crate::web_api::interface::payments::view_models::Payment>(
                HttpMethod::Get,
                "/api/Payments",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Payments", query)
                .await
        }
    }
}

pub mod get_list_by_contractor {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Payments/Filter`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByContractorIdDateFromDateTo {
        /// Id kontrahenta.
        pub contractor_id: i32,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorIdDateFromDateTo {
        type Output = Vec<crate::web_api::interface::payments::view_models::PaymentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::payments::view_models::PaymentListElement>>(HttpMethod::Get, "/api/Payments/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Payments/Filter", query)
                .await
        }
    }

    /// `GET` `/api/Payments/Filter`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByContractorCodeDateFromDateTo {
        /// Kod kontrahenta.
        pub contractor_code: String,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorCodeDateFromDateTo {
        type Output = Vec<crate::web_api::interface::payments::view_models::PaymentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::payments::view_models::PaymentListElement>>(HttpMethod::Get, "/api/Payments/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Payments/Filter", query)
                .await
        }
    }
}

pub mod get_list_by_payment_registry {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Payments/Filter`
    ///
    /// Parameter `payment_registry_id`: Id rejestru płatności.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByPaymentRegistryIdDateFromDateTo {
        /// Id rejestru płatności.
        pub payment_registry_id: i32,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByPaymentRegistryIdDateFromDateTo {
        type Output = Vec<crate::web_api::interface::payments::view_models::PaymentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "paymentRegistryId", &self.payment_registry_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::payments::view_models::PaymentListElement>>(HttpMethod::Get, "/api/Payments/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "paymentRegistryId", &self.payment_registry_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Payments/Filter", query)
                .await
        }
    }

    /// `GET` `/api/Payments/Filter`
    ///
    /// Parameter `payment_registry_code`: Kod rejestru płatności.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByPaymentRegistryCodeDateFromDateTo {
        /// Kod rejestru płatności.
        pub payment_registry_code: String,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByPaymentRegistryCodeDateFromDateTo {
        type Output = Vec<crate::web_api::interface::payments::view_models::PaymentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(
                &mut query,
                "paymentRegistryCode",
                &self.payment_registry_code,
            )?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::payments::view_models::PaymentListElement>>(HttpMethod::Get, "/api/Payments/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(
                &mut query,
                "paymentRegistryCode",
                &self.payment_registry_code,
            )?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Payments/Filter", query)
                .await
        }
    }
}

pub mod get_list_by_receiving_payment_registry {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Payments/Filter`
    ///
    /// Parameter `receiving_payment_registry_id`: Id rejestru płatności.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByReceivingPaymentRegistryIdDateFromDateTo {
        /// Id rejestru płatności.
        pub receiving_payment_registry_id: i32,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByReceivingPaymentRegistryIdDateFromDateTo {
        type Output = Vec<crate::web_api::interface::payments::view_models::PaymentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(
                &mut query,
                "receivingPaymentRegistryId",
                &self.receiving_payment_registry_id,
            )?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::payments::view_models::PaymentListElement>>(HttpMethod::Get, "/api/Payments/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(
                &mut query,
                "receivingPaymentRegistryId",
                &self.receiving_payment_registry_id,
            )?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Payments/Filter", query)
                .await
        }
    }

    /// `GET` `/api/Payments/Filter`
    ///
    /// Parameter `receiving_payment_registry_code`: Kod kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByReceivingPaymentRegistryCodeDateFromDateTo {
        /// Kod kontrahenta.
        pub receiving_payment_registry_code: String,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByReceivingPaymentRegistryCodeDateFromDateTo {
        type Output = Vec<crate::web_api::interface::payments::view_models::PaymentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(
                &mut query,
                "receivingPaymentRegistryCode",
                &self.receiving_payment_registry_code,
            )?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::payments::view_models::PaymentListElement>>(HttpMethod::Get, "/api/Payments/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(
                &mut query,
                "receivingPaymentRegistryCode",
                &self.receiving_payment_registry_code,
            )?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Payments/Filter", query)
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

    /// `GET` `/api/Payments/PDF`
    ///
    /// Parameter `document_id`: Id dokumentu płatności.
    ///
    /// Parameter `print_note`: Czy umieścić notatkę na wydruku PDF. W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
    pub struct ByDocumentIdPrintNote {
        /// Id dokumentu płatności.
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
                "/api/Payments/PDF",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            push_query_param(&mut query, "printNote", &self.print_note)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Payments/PDF", query)
                .await
        }
    }

    /// `GET` `/api/Payments/PDF`
    ///
    /// Parameter `document_number`: Numer dokumentu płatności.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
    ///
    /// Parameter `print_note`: Czy umieścić notatkę na wydruku PDF. W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
    pub struct ByDocumentNumberBufferPrintNote {
        /// Numer dokumentu płatności.
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
                "/api/Payments/PDF",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            push_query_param(&mut query, "printNote", &self.print_note)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Payments/PDF", query)
                .await
        }
    }
}

pub mod issue_transfer_plus {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/Payments/TransferPlus`
    ///
    /// Parameter `payment_id`: Id dokumentu TR-.
    pub struct ByPaymentId {
        /// Id dokumentu TR-.
        pub payment_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByPaymentId {
        type Output = crate::web_api::interface::payments::view_models::Payment;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "paymentId", &self.payment_id)?;
            api.request_no_body::<crate::web_api::interface::payments::view_models::Payment>(
                HttpMethod::Put,
                "/api/Payments/TransferPlus",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "paymentId", &self.payment_id)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/Payments/TransferPlus", query)
                .await
        }
    }

    /// `PUT` `/api/Payments/TransferPlus`
    ///
    /// Parameter `payment_number`: Numer dokumentu TR-.
    pub struct ByPaymentNumber {
        /// Numer dokumentu TR-.
        pub payment_number: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByPaymentNumber {
        type Output = crate::web_api::interface::payments::view_models::Payment;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "paymentNumber", &self.payment_number)?;
            api.request_no_body::<crate::web_api::interface::payments::view_models::Payment>(
                HttpMethod::Put,
                "/api/Payments/TransferPlus",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "paymentNumber", &self.payment_number)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/Payments/TransferPlus", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IPaymentsController {
    /// `GET` `/api/Payments`
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Payments`
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Payments/Filter`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByContractor<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_list_by_contractor::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Payments/Filter`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByContractorRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_list_by_contractor::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Payments/Filter`
    ///
    /// Parameter `payment_registry_id`: Id rejestru płatności.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByPaymentRegistry<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_list_by_payment_registry::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Payments/Filter`
    ///
    /// Parameter `payment_registry_id`: Id rejestru płatności.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByPaymentRegistryRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_list_by_payment_registry::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Payments/Filter`
    ///
    /// Parameter `receiving_payment_registry_id`: Id rejestru płatności.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByReceivingPaymentRegistry<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_list_by_receiving_payment_registry::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Payments/Filter`
    ///
    /// Parameter `receiving_payment_registry_id`: Id rejestru płatności.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByReceivingPaymentRegistryRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_list_by_receiving_payment_registry::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Payments/PDF`
    ///
    /// Parameter `document_id`: Id dokumentu płatności.
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

    /// `GET` `/api/Payments/PDF`
    ///
    /// Parameter `document_id`: Id dokumentu płatności.
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

    /// `PUT` `/api/Payments/TransferPlus`
    ///
    /// Parameter `payment_id`: Id dokumentu TR-.
    pub async fn IssueTransferPlus<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: issue_transfer_plus::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/Payments/TransferPlus`
    ///
    /// Parameter `payment_id`: Id dokumentu TR-.
    pub async fn IssueTransferPlusRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: issue_transfer_plus::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Payments/Filter`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetList(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::payments::view_models::PaymentListElement>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::payments::view_models::PaymentListElement>>(HttpMethod::Get, "/api/Payments/Filter", query).await
    }

    /// `GET` `/api/Payments/Filter`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty płatności od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListRaw(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Payments/Filter", query)
            .await
    }

    /// `GET` `/api/Payments/NALDocumentTypes`
    pub async fn GetNALDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Payments/NALDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Payments/NALDocumentTypes`
    pub async fn GetNALDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Payments/NALDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Payments/ZOBDocumentTypes`
    pub async fn GetZOBDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Payments/ZOBDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Payments/ZOBDocumentTypes`
    pub async fn GetZOBDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Payments/ZOBDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Payments/KPDocumentTypes`
    pub async fn GetKPDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Payments/KPDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Payments/KPDocumentTypes`
    pub async fn GetKPDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Payments/KPDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Payments/KWDocumentTypes`
    pub async fn GetKWDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Payments/KWDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Payments/KWDocumentTypes`
    pub async fn GetKWDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Payments/KWDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Payments/BPDocumentTypes`
    pub async fn GetBPDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Payments/BPDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Payments/BPDocumentTypes`
    pub async fn GetBPDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Payments/BPDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Payments/BWDocumentTypes`
    pub async fn GetBWDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Payments/BWDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Payments/BWDocumentTypes`
    pub async fn GetBWDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Payments/BWDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Payments/IPDocumentTypes`
    pub async fn GetIPDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Payments/IPDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Payments/IPDocumentTypes`
    pub async fn GetIPDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Payments/IPDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Payments/IWDocumentTypes`
    pub async fn GetIWDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Payments/IWDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Payments/IWDocumentTypes`
    pub async fn GetIWDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Payments/IWDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Payments/TRPlusDocumentTypes`
    pub async fn GetTRPlusDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Payments/TRPlusDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Payments/TRPlusDocumentTypes`
    pub async fn GetTRPlusDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Payments/TRPlusDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Payments/TRMinusDocumentTypes`
    pub async fn GetTRMinusDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Payments/TRMinusDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Payments/TRMinusDocumentTypes`
    pub async fn GetTRMinusDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Payments/TRMinusDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Payments/DocumentSeries`
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
            "/api/Payments/DocumentSeries",
            query,
        )
        .await
    }

    /// `GET` `/api/Payments/DocumentSeries`
    ///
    /// Parameter `document_type_id`: Id typu dokumentu.
    pub async fn GetDocumentSeriesRaw(
        api: &ApiClient,
        document_type_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentTypeId", &document_type_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Payments/DocumentSeries", query)
            .await
    }

    /// `POST` `/api/Payments/Payment`
    ///
    /// Body `payment`: Obiekt dokumentu płatności do wystawienia.
    pub async fn IssuePayment(
        api: &ApiClient,
        payment: &crate::web_api::interface::payments::view_models::issue::PaymentIssue,
    ) -> Result<ResponseEnvelope<crate::web_api::interface::payments::view_models::Payment>, ApiError>
    {
        let query = vec![];
        api.request_with_body::<crate::web_api::interface::payments::view_models::Payment, _>(
            HttpMethod::Post,
            "/api/Payments/Payment",
            query,
            payment,
        )
        .await
    }

    /// `POST` `/api/Payments/Payment`
    ///
    /// Body `payment`: Obiekt dokumentu płatności do wystawienia.
    pub async fn IssuePaymentRaw(
        api: &ApiClient,
        payment: &crate::web_api::interface::payments::view_models::issue::PaymentIssue,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(HttpMethod::Post, "/api/Payments/Payment", query, payment)
            .await
    }

    /// `POST` `/api/Payments/TransferMinus`
    ///
    /// Body `payment`: Obiekt dokumentu płatności do wystawienia.
    pub async fn IssueTransferMinus(
        api: &ApiClient,
        payment: &crate::web_api::interface::payments::view_models::issue::PaymentIssue,
    ) -> Result<ResponseEnvelope<crate::web_api::interface::payments::view_models::Payment>, ApiError>
    {
        let query = vec![];
        api.request_with_body::<crate::web_api::interface::payments::view_models::Payment, _>(
            HttpMethod::Post,
            "/api/Payments/TransferMinus",
            query,
            payment,
        )
        .await
    }

    /// `POST` `/api/Payments/TransferMinus`
    ///
    /// Body `payment`: Obiekt dokumentu płatności do wystawienia.
    pub async fn IssueTransferMinusRaw(
        api: &ApiClient,
        payment: &crate::web_api::interface::payments::view_models::issue::PaymentIssue,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Post,
            "/api/Payments/TransferMinus",
            query,
            payment,
        )
        .await
    }

    /// `POST` `/api/Payments/IssueKP`
    ///
    /// Body `payment`: Obiekt dokumentu płatności do utworzenia.
    pub async fn IssueKP(
        api: &ApiClient,
        payment: &crate::web_api::interface::payments::view_models::issue_kp::PaymentKPIssue,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let query = vec![];
        api.request_with_body::<(), _>(HttpMethod::Post, "/api/Payments/IssueKP", query, payment)
            .await
    }

    /// `POST` `/api/Payments/IssueKP`
    ///
    /// Body `payment`: Obiekt dokumentu płatności do utworzenia.
    pub async fn IssueKPRaw(
        api: &ApiClient,
        payment: &crate::web_api::interface::payments::view_models::issue_kp::PaymentKPIssue,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(HttpMethod::Post, "/api/Payments/IssueKP", query, payment)
            .await
    }

    /// `GET` `/api/Payments/Page`
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
                crate::web_api::interface::payments::view_models::PaymentListElement,
            >,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body::<crate::web_api::interface::view_models::Page<
            crate::web_api::interface::payments::view_models::PaymentListElement,
        >>(HttpMethod::Get, "/api/Payments/Page", query)
            .await
    }

    /// `GET` `/api/Payments/Page`
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
        api.request_no_body_raw(HttpMethod::Get, "/api/Payments/Page", query)
            .await
    }
}
