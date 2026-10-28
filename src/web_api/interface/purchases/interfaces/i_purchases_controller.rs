use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `Purchases`.
pub struct IPurchasesController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Purchases`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output =
            Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentListElement>>(HttpMethod::Get, "/api/Purchases", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases", query)
                .await
        }
    }

    /// `GET` `/api/Purchases`
    ///
    /// Parameter `id`: Id dokumentu zakupu.
    pub struct ById {
        /// Id dokumentu zakupu.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = crate::web_api::interface::purchases::view_models::PurchaseDocument;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<crate::web_api::interface::purchases::view_models::PurchaseDocument>(HttpMethod::Get, "/api/Purchases", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases", query)
                .await
        }
    }

    /// `GET` `/api/Purchases`
    ///
    /// Parameter `number`: Numer dokumentu zakupu.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
    pub struct ByNumberBuffer {
        /// Numer dokumentu zakupu.
        pub number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie pobrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByNumberBuffer {
        type Output = crate::web_api::interface::purchases::view_models::PurchaseDocument;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<crate::web_api::interface::purchases::view_models::PurchaseDocument>(HttpMethod::Get, "/api/Purchases", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases", query)
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

    /// `GET` `/api/Purchases/Correction`
    ///
    /// Parameter `id`: Id korekty dokumentu zakupu.
    pub struct ById {
        /// Id korekty dokumentu zakupu.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = crate::web_api::interface::purchases::view_models::PurchaseCorrection;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<crate::web_api::interface::purchases::view_models::PurchaseCorrection>(HttpMethod::Get, "/api/Purchases/Correction", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/Correction", query)
                .await
        }
    }

    /// `GET` `/api/Purchases/Correction`
    ///
    /// Parameter `number`: Numer korekty dokumentu zakupu.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
    pub struct ByNumberBuffer {
        /// Numer korekty dokumentu zakupu.
        pub number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie pobrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByNumberBuffer {
        type Output = crate::web_api::interface::purchases::view_models::PurchaseCorrection;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<crate::web_api::interface::purchases::view_models::PurchaseCorrection>(HttpMethod::Get, "/api/Purchases/Correction", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/Correction", query)
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

    /// `GET` `/api/Purchases/Filter`
    ///
    /// Parameter `seller_id`: Id kontrahenta (sprzedawcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct BySellerIdDateFromDateTo {
        /// Id kontrahenta (sprzedawcy).
        pub seller_id: i32,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for BySellerIdDateFromDateTo {
        type Output =
            Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "sellerId", &self.seller_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentListElement>>(HttpMethod::Get, "/api/Purchases/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "sellerId", &self.seller_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/Filter", query)
                .await
        }
    }

    /// `GET` `/api/Purchases/Filter`
    ///
    /// Parameter `seller_code`: Kod kontrahenta (sprzedawcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct BySellerCodeDateFromDateTo {
        /// Kod kontrahenta (sprzedawcy).
        pub seller_code: String,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for BySellerCodeDateFromDateTo {
        type Output =
            Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "sellerCode", &self.seller_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentListElement>>(HttpMethod::Get, "/api/Purchases/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "sellerCode", &self.seller_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/Filter", query)
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

    /// `GET` `/api/Purchases/Filter`
    ///
    /// Parameter `deliverer_id`: Id kontrahenta (dostawcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByDelivererIdDateFromDateTo {
        /// Id kontrahenta (dostawcy).
        pub deliverer_id: i32,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByDelivererIdDateFromDateTo {
        type Output =
            Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "delivererId", &self.deliverer_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentListElement>>(HttpMethod::Get, "/api/Purchases/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "delivererId", &self.deliverer_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/Filter", query)
                .await
        }
    }

    /// `GET` `/api/Purchases/Filter`
    ///
    /// Parameter `deliverer_code`: Kod kontrahenta (dostawcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByDelivererCodeDateFromDateTo {
        /// Kod kontrahenta (dostawcy).
        pub deliverer_code: String,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByDelivererCodeDateFromDateTo {
        type Output =
            Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "delivererCode", &self.deliverer_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentListElement>>(HttpMethod::Get, "/api/Purchases/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "delivererCode", &self.deliverer_code)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/Filter", query)
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

    /// `GET` `/api/Purchases/PZ`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub struct ByDocumentId {
        /// Id dokumentu zakupu.
        pub document_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentId {
        type Output = Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentPZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentPZ>>(HttpMethod::Get, "/api/Purchases/PZ", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/PZ", query)
                .await
        }
    }

    /// `GET` `/api/Purchases/PZ`
    ///
    /// Parameter `document_number`: Numer dokumentu zakupu.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
    pub struct ByDocumentNumberBuffer {
        /// Numer dokumentu zakupu.
        pub document_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumberBuffer {
        type Output = Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentPZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentPZ>>(HttpMethod::Get, "/api/Purchases/PZ", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/PZ", query)
                .await
        }
    }
}

pub mod get_zmw {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Purchases/ZMW`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub struct ByDocumentId {
        /// Id dokumentu zakupu.
        pub document_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentId {
        type Output = Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentPZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentPZ>>(HttpMethod::Get, "/api/Purchases/ZMW", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/ZMW", query)
                .await
        }
    }

    /// `GET` `/api/Purchases/ZMW`
    ///
    /// Parameter `document_number`: Numer dokumentu zakupu.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
    pub struct ByDocumentNumberBuffer {
        /// Numer dokumentu zakupu.
        pub document_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumberBuffer {
        type Output = Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentPZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentPZ>>(HttpMethod::Get, "/api/Purchases/ZMW", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/ZMW", query)
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

    /// `GET` `/api/Purchases/CorrectionSequence`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub struct ByDocumentId {
        /// Id dokumentu zakupu.
        pub document_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentId {
        type Output =
            Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentCorrection>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentCorrection>>(HttpMethod::Get, "/api/Purchases/CorrectionSequence", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/CorrectionSequence", query)
                .await
        }
    }

    /// `GET` `/api/Purchases/CorrectionSequence`
    ///
    /// Parameter `document_number`: Numer dokumentu zakupu.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
    pub struct ByDocumentNumberBuffer {
        /// Numer dokumentu zakupu.
        pub document_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie wybrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumberBuffer {
        type Output =
            Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentCorrection>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentCorrection>>(HttpMethod::Get, "/api/Purchases/CorrectionSequence", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/CorrectionSequence", query)
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

    /// `GET` `/api/Purchases/Status`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub struct ByDocumentId {
        /// Id dokumentu zakupu.
        pub document_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentId {
        type Output = crate::web_api::interface::purchases::view_models::PurchaseDocumentStatus;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body::<crate::web_api::interface::purchases::view_models::PurchaseDocumentStatus>(HttpMethod::Get, "/api/Purchases/Status", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/Status", query)
                .await
        }
    }

    /// `GET` `/api/Purchases/Status`
    ///
    /// Parameter `document_number`: Numer dokumentu zakupu.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrany status dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
    pub struct ByDocumentNumberBuffer {
        /// Numer dokumentu zakupu.
        pub document_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie pobrany status dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumberBuffer {
        type Output = crate::web_api::interface::purchases::view_models::PurchaseDocumentStatus;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<crate::web_api::interface::purchases::view_models::PurchaseDocumentStatus>(HttpMethod::Get, "/api/Purchases/Status", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/Status", query)
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

    /// `GET` `/api/Purchases/PDF`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    ///
    /// Parameter `print_note`: Czy umieścić notatkę na wydruku PDF. W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
    pub struct ByDocumentIdPrintNote {
        /// Id dokumentu zakupu.
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
                "/api/Purchases/PDF",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            push_query_param(&mut query, "printNote", &self.print_note)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/PDF", query)
                .await
        }
    }

    /// `GET` `/api/Purchases/PDF`
    ///
    /// Parameter `document_number`: Numer dokumentu zakupu.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrany dokument o podanym numerze dokumentu, który nie znajduje się w buforze.
    ///
    /// Parameter `print_note`: Czy umieścić notatkę na wydruku PDF. W przypadku, gdy parametr nie zostanie podany notatka nie zostanie umieszczona na wydruku PDF.
    pub struct ByDocumentNumberBufferPrintNote {
        /// Numer dokumentu zakupu.
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
                "/api/Purchases/PDF",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            push_query_param(&mut query, "printNote", &self.print_note)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/PDF", query)
                .await
        }
    }

    /// `PATCH` `/api/Purchases/PDF`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    ///
    /// Body `settings`: Ustawienia wydruku.
    pub struct ByDocumentId<'a> {
        /// Id dokumentu zakupu.
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
                "/api/Purchases/PDF",
                query,
                self.settings,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_with_body_raw(
                HttpMethod::Patch,
                "/api/Purchases/PDF",
                query,
                self.settings,
            )
            .await
        }
    }

    /// `PATCH` `/api/Purchases/PDF`
    ///
    /// Parameter `document_number`: Numer dokumentu zakupu.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie wybrane zamówienie obce o podanym numerze dokumentu, które nie znajduje się w buforze.
    ///
    /// Body `settings`: Ustawienia wydruku.
    pub struct ByDocumentNumberBuffer<'a> {
        /// Numer dokumentu zakupu.
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
                "/api/Purchases/PDF",
                query,
                self.settings,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_with_body_raw(
                HttpMethod::Patch,
                "/api/Purchases/PDF",
                query,
                self.settings,
            )
            .await
        }
    }
}

#[allow(non_snake_case)]
impl IPurchasesController {
    /// `GET` `/api/Purchases`
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Purchases`
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Purchases/Correction`
    ///
    /// Parameter `id`: Id korekty dokumentu zakupu.
    pub async fn GetCorrection<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_correction::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Purchases/Correction`
    ///
    /// Parameter `id`: Id korekty dokumentu zakupu.
    pub async fn GetCorrectionRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_correction::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Purchases/Filter`
    ///
    /// Parameter `seller_id`: Id kontrahenta (sprzedawcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListBySeller<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_list_by_seller::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Purchases/Filter`
    ///
    /// Parameter `seller_id`: Id kontrahenta (sprzedawcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListBySellerRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_list_by_seller::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Purchases/Filter`
    ///
    /// Parameter `deliverer_id`: Id kontrahenta (dostawcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByDeliverer<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_list_by_deliverer::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Purchases/Filter`
    ///
    /// Parameter `deliverer_id`: Id kontrahenta (dostawcy).
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByDelivererRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_list_by_deliverer::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Purchases/PZ`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub async fn GetPZ<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_pz::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Purchases/PZ`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub async fn GetPZRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_pz::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Purchases/ZMW`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub async fn GetZMW<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_zmw::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Purchases/ZMW`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub async fn GetZMWRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_zmw::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Purchases/CorrectionSequence`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub async fn GetCorrectionSequence<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_correction_sequence::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Purchases/CorrectionSequence`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub async fn GetCorrectionSequenceRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_correction_sequence::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Purchases/Status`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub async fn GetStatus<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_status::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Purchases/Status`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub async fn GetStatusRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_status::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Purchases/PDF`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
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

    /// `GET` `/api/Purchases/PDF`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
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

    /// `GET` `/api/Purchases/Filter`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetList(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentListElement>>(HttpMethod::Get, "/api/Purchases/Filter", query).await
    }

    /// `GET` `/api/Purchases/Filter`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty zakupu od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListRaw(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/Filter", query)
            .await
    }

    /// `GET` `/api/Purchases/Filter`
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
            Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dimensionCode", &dimension_code)?;
        push_query_param(&mut query, "dictionaryValue", &dictionary_value)?;
        push_query_param(&mut query, "value", &value)?;
        api.request_no_body::<Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentListElement>>(HttpMethod::Get, "/api/Purchases/Filter", query).await
    }

    /// `GET` `/api/Purchases/Filter`
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
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/Filter", query)
            .await
    }

    /// `GET` `/api/Purchases/Markers`
    pub async fn GetMarkers(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Marker>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Marker>>(
            HttpMethod::Get,
            "/api/Purchases/Markers",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/Markers`
    pub async fn GetMarkersRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/Markers", query)
            .await
    }

    /// `GET` `/api/Purchases/Kinds`
    pub async fn GetKinds(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Kind>>, ApiError>
    {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Kind>>(
            HttpMethod::Get,
            "/api/Purchases/Kinds",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/Kinds`
    pub async fn GetKindsRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/Kinds", query)
            .await
    }

    /// `GET` `/api/Purchases/Catalogs`
    pub async fn GetCatalogs(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Catalog>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Catalog>>(
            HttpMethod::Get,
            "/api/Purchases/Catalogs",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/Catalogs`
    pub async fn GetCatalogsRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/Catalogs", query)
            .await
    }

    /// `GET` `/api/Purchases/DIMDocumentTypes`
    pub async fn GetDIMDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Purchases/DIMDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/DIMDocumentTypes`
    pub async fn GetDIMDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/DIMDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Purchases/FVMDocumentTypes`
    pub async fn GetFVMDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Purchases/FVMDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/FVMDocumentTypes`
    pub async fn GetFVMDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/FVMDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Purchases/FVRDocumentTypes`
    pub async fn GetFVRDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Purchases/FVRDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/FVRDocumentTypes`
    pub async fn GetFVRDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/FVRDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Purchases/FVZDocumentTypes`
    pub async fn GetFVZDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Purchases/FVZDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/FVZDocumentTypes`
    pub async fn GetFVZDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/FVZDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Purchases/FWZDocumentTypes`
    pub async fn GetFWZDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Purchases/FWZDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/FWZDocumentTypes`
    pub async fn GetFWZDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/FWZDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Purchases/RUZDocumentTypes`
    pub async fn GetRUZDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Purchases/RUZDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/RUZDocumentTypes`
    pub async fn GetRUZDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/RUZDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Purchases/WNTDocumentTypes`
    pub async fn GetWNTDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Purchases/WNTDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/WNTDocumentTypes`
    pub async fn GetWNTDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/WNTDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Purchases/ZRZDocumentTypes`
    pub async fn GetZRZDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Purchases/ZRZDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/ZRZDocumentTypes`
    pub async fn GetZRZDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/ZRZDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Purchases/DMKDocumentTypes`
    pub async fn GetDMKDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Purchases/DMKDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/DMKDocumentTypes`
    pub async fn GetDMKDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/DMKDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Purchases/FMKDocumentTypes`
    pub async fn GetFMKDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Purchases/FMKDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/FMKDocumentTypes`
    pub async fn GetFMKDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/FMKDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Purchases/FRKDocumentTypes`
    pub async fn GetFRKDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Purchases/FRKDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/FRKDocumentTypes`
    pub async fn GetFRKDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/FRKDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Purchases/FZKDocumentTypes`
    pub async fn GetFZKDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Purchases/FZKDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/FZKDocumentTypes`
    pub async fn GetFZKDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/FZKDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Purchases/FWZKDocumentTypes`
    pub async fn GetFWZKDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Purchases/FWZKDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/FWZKDocumentTypes`
    pub async fn GetFWZKDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/FWZKDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Purchases/RKZDocumentTypes`
    pub async fn GetRKZDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Purchases/RKZDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/RKZDocumentTypes`
    pub async fn GetRKZDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/RKZDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Purchases/ZRKDocumentTypes`
    pub async fn GetZRKDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Purchases/ZRKDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/ZRKDocumentTypes`
    pub async fn GetZRKDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/ZRKDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Purchases/WNKDocumentTypes`
    pub async fn GetWNKDocumentTypes(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/Purchases/WNKDocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/WNKDocumentTypes`
    pub async fn GetWNKDocumentTypesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/WNKDocumentTypes", query)
            .await
    }

    /// `GET` `/api/Purchases/DocumentSeries`
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
            "/api/Purchases/DocumentSeries",
            query,
        )
        .await
    }

    /// `GET` `/api/Purchases/DocumentSeries`
    ///
    /// Parameter `document_type_id`: Id typu dokumentu.
    pub async fn GetDocumentSeriesRaw(
        api: &ApiClient,
        document_type_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentTypeId", &document_type_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/DocumentSeries", query)
            .await
    }

    /// `GET` `/api/Purchases/Page`
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
                crate::web_api::interface::purchases::view_models::PurchaseDocumentListElement,
            >,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body::<crate::web_api::interface::view_models::Page<
            crate::web_api::interface::purchases::view_models::PurchaseDocumentListElement,
        >>(HttpMethod::Get, "/api/Purchases/Page", query)
            .await
    }

    /// `GET` `/api/Purchases/Page`
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
        api.request_no_body_raw(HttpMethod::Get, "/api/Purchases/Page", query)
            .await
    }

    /// `GET` `/api/Purchases/Filter/ByDocumentTypes`
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
            Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentListElement>>(HttpMethod::Get, "/api/Purchases/Filter/ByDocumentTypes", query).await
    }

    /// `GET` `/api/Purchases/Filter/ByDocumentTypes`
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
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/Purchases/Filter/ByDocumentTypes",
            query,
        )
        .await
    }
}
