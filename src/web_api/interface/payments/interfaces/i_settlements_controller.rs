use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `Settlements`.
pub struct ISettlementsController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Settlements`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output = Vec<crate::web_api::interface::payments::view_models::SettlementListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<Vec<crate::web_api::interface::payments::view_models::SettlementListElement>>(HttpMethod::Get, "/api/Settlements", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(HttpMethod::Get, "/api/Settlements", query)
                .await
        }
    }

    /// `GET` `/api/Settlements`
    ///
    /// Parameter `id`: Id dokumentu rozrachunku.
    pub struct ById {
        /// Id dokumentu rozrachunku.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = crate::web_api::interface::payments::view_models::Settlement;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<crate::web_api::interface::payments::view_models::Settlement>(
                HttpMethod::Get,
                "/api/Settlements",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Settlements", query)
                .await
        }
    }

    /// `GET` `/api/Settlements`
    ///
    /// Parameter `number`: Numer dokumentu rozrachunku.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
    pub struct ByNumberBuffer {
        /// Numer dokumentu rozrachunku.
        pub number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie pobrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByNumberBuffer {
        type Output = crate::web_api::interface::payments::view_models::Settlement;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<crate::web_api::interface::payments::view_models::Settlement>(
                HttpMethod::Get,
                "/api/Settlements",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Settlements", query)
                .await
        }
    }
}

pub mod get_by_document {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Settlements`
    ///
    /// Parameter `document_id`: Id dokumentu.
    pub struct ByDocumentId {
        /// Id dokumentu.
        pub document_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentId {
        type Output = crate::web_api::interface::payments::view_models::Settlement;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body::<crate::web_api::interface::payments::view_models::Settlement>(
                HttpMethod::Get,
                "/api/Settlements",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Settlements", query)
                .await
        }
    }

    /// `GET` `/api/Settlements`
    ///
    /// Parameter `document_number`: Numer dokumentu.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
    pub struct ByDocumentNumberBuffer {
        /// Numer dokumentu.
        pub document_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie pobrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumberBuffer {
        type Output = crate::web_api::interface::payments::view_models::Settlement;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<crate::web_api::interface::payments::view_models::Settlement>(
                HttpMethod::Get,
                "/api/Settlements",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Settlements", query)
                .await
        }
    }
}

pub mod get_list_by_issue_date {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Settlements/Filter/ByIssueDate`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByDateFromDateTo {
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByDateFromDateTo {
        type Output = Vec<crate::web_api::interface::payments::view_models::SettlementListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::payments::view_models::SettlementListElement>>(HttpMethod::Get, "/api/Settlements/Filter/ByIssueDate", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/Settlements/Filter/ByIssueDate",
                query,
            )
            .await
        }
    }

    /// `GET` `/api/Settlements/Filter/ByIssueDate`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByContractorIdDateFromDateTo {
        /// Id kontrahenta.
        pub contractor_id: i32,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorIdDateFromDateTo {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<()>(
                HttpMethod::Get,
                "/api/Settlements/Filter/ByIssueDate",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/Settlements/Filter/ByIssueDate",
                query,
            )
            .await
        }
    }

    /// `GET` `/api/Settlements/Filter/ByIssueDate`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByContractorCodeDateFromDateTo {
        /// Kod kontrahenta.
        pub contractor_code: String,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorCodeDateFromDateTo {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<()>(
                HttpMethod::Get,
                "/api/Settlements/Filter/ByIssueDate",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/Settlements/Filter/ByIssueDate",
                query,
            )
            .await
        }
    }
}

pub mod get_list_by_maturity_date {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Settlements/Filter/ByMaturityDate`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByDateFromDateTo {
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByDateFromDateTo {
        type Output = Vec<crate::web_api::interface::payments::view_models::SettlementListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::payments::view_models::SettlementListElement>>(HttpMethod::Get, "/api/Settlements/Filter/ByMaturityDate", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/Settlements/Filter/ByMaturityDate",
                query,
            )
            .await
        }
    }

    /// `GET` `/api/Settlements/Filter/ByMaturityDate`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByContractorIdDateFromDateTo {
        /// Id kontrahenta.
        pub contractor_id: i32,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorIdDateFromDateTo {
        type Output = Vec<crate::web_api::interface::payments::view_models::SettlementListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::payments::view_models::SettlementListElement>>(HttpMethod::Get, "/api/Settlements/Filter/ByMaturityDate", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/Settlements/Filter/ByMaturityDate",
                query,
            )
            .await
        }
    }

    /// `GET` `/api/Settlements/Filter/ByMaturityDate`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByContractorCodeDateFromDateTo {
        /// Kod kontrahenta.
        pub contractor_code: String,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorCodeDateFromDateTo {
        type Output = Vec<crate::web_api::interface::payments::view_models::SettlementListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::payments::view_models::SettlementListElement>>(HttpMethod::Get, "/api/Settlements/Filter/ByMaturityDate", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/Settlements/Filter/ByMaturityDate",
                query,
            )
            .await
        }
    }
}

pub mod get_not_settled_by_issue_date {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Settlements/Filter/NotSettled/ByIssueDate`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByDateFromDateTo {
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByDateFromDateTo {
        type Output = Vec<crate::web_api::interface::payments::view_models::SettlementListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::payments::view_models::SettlementListElement>>(HttpMethod::Get, "/api/Settlements/Filter/NotSettled/ByIssueDate", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/Settlements/Filter/NotSettled/ByIssueDate",
                query,
            )
            .await
        }
    }

    /// `GET` `/api/Settlements/Filter/NotSettled/ByIssueDate`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByContractorIdDateFromDateTo {
        /// Id kontrahenta.
        pub contractor_id: i32,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorIdDateFromDateTo {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<()>(
                HttpMethod::Get,
                "/api/Settlements/Filter/NotSettled/ByIssueDate",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/Settlements/Filter/NotSettled/ByIssueDate",
                query,
            )
            .await
        }
    }

    /// `GET` `/api/Settlements/Filter/NotSettled/ByIssueDate`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByContractorCodeDateFromDateTo {
        /// Kod kontrahenta.
        pub contractor_code: String,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorCodeDateFromDateTo {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<()>(
                HttpMethod::Get,
                "/api/Settlements/Filter/NotSettled/ByIssueDate",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/Settlements/Filter/NotSettled/ByIssueDate",
                query,
            )
            .await
        }
    }
}

pub mod get_not_settled_by_maturity_date {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Settlements/Filter/NotSettled/ByMaturityDate`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByDateFromDateTo {
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByDateFromDateTo {
        type Output = Vec<crate::web_api::interface::payments::view_models::SettlementListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::payments::view_models::SettlementListElement>>(HttpMethod::Get, "/api/Settlements/Filter/NotSettled/ByMaturityDate", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/Settlements/Filter/NotSettled/ByMaturityDate",
                query,
            )
            .await
        }
    }

    /// `GET` `/api/Settlements/Filter/NotSettled/ByMaturityDate`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByContractorIdDateFromDateTo {
        /// Id kontrahenta.
        pub contractor_id: i32,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorIdDateFromDateTo {
        type Output = Vec<crate::web_api::interface::payments::view_models::SettlementListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::payments::view_models::SettlementListElement>>(HttpMethod::Get, "/api/Settlements/Filter/NotSettled/ByMaturityDate", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/Settlements/Filter/NotSettled/ByMaturityDate",
                query,
            )
            .await
        }
    }

    /// `GET` `/api/Settlements/Filter/NotSettled/ByMaturityDate`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByContractorCodeDateFromDateTo {
        /// Kod kontrahenta.
        pub contractor_code: String,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorCodeDateFromDateTo {
        type Output = Vec<crate::web_api::interface::payments::view_models::SettlementListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::payments::view_models::SettlementListElement>>(HttpMethod::Get, "/api/Settlements/Filter/NotSettled/ByMaturityDate", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/Settlements/Filter/NotSettled/ByMaturityDate",
                query,
            )
            .await
        }
    }
}

#[allow(non_snake_case)]
impl ISettlementsController {
    /// `GET` `/api/Settlements`
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Settlements`
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Settlements`
    ///
    /// Parameter `document_id`: Id dokumentu.
    pub async fn GetByDocument<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_by_document::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Settlements`
    ///
    /// Parameter `document_id`: Id dokumentu.
    pub async fn GetByDocumentRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_by_document::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Settlements/Filter/ByIssueDate`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByIssueDate<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_list_by_issue_date::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Settlements/Filter/ByIssueDate`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByIssueDateRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_list_by_issue_date::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Settlements/Filter/ByMaturityDate`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByMaturityDate<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_list_by_maturity_date::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Settlements/Filter/ByMaturityDate`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByMaturityDateRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_list_by_maturity_date::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Settlements/Filter/NotSettled/ByIssueDate`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetNotSettledByIssueDate<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_not_settled_by_issue_date::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Settlements/Filter/NotSettled/ByIssueDate`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetNotSettledByIssueDateRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_not_settled_by_issue_date::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Settlements/Filter/NotSettled/ByMaturityDate`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetNotSettledByMaturityDate<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_not_settled_by_maturity_date::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Settlements/Filter/NotSettled/ByMaturityDate`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetNotSettledByMaturityDateRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_not_settled_by_maturity_date::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Settlements/Filter/NotSettled`
    pub async fn GetNotSettled(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::payments::view_models::SettlementListElement>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::payments::view_models::SettlementListElement>>(HttpMethod::Get, "/api/Settlements/Filter/NotSettled", query).await
    }

    /// `GET` `/api/Settlements/Filter/NotSettled`
    pub async fn GetNotSettledRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Settlements/Filter/NotSettled", query)
            .await
    }

    /// `POST` `/api/Settlements/Issue`
    ///
    /// Body `settlement`: Obiekt dokumentu rozrachunku do wystawienia.
    pub async fn Issue(
        api: &ApiClient,
        settlement: &crate::web_api::interface::payments::view_models::issue::SettlementIssue,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::payments::view_models::Settlement>,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<crate::web_api::interface::payments::view_models::Settlement, _>(
            HttpMethod::Post,
            "/api/Settlements/Issue",
            query,
            settlement,
        )
        .await
    }

    /// `POST` `/api/Settlements/Issue`
    ///
    /// Body `settlement`: Obiekt dokumentu rozrachunku do wystawienia.
    pub async fn IssueRaw(
        api: &ApiClient,
        settlement: &crate::web_api::interface::payments::view_models::issue::SettlementIssue,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Post,
            "/api/Settlements/Issue",
            query,
            settlement,
        )
        .await
    }
}
