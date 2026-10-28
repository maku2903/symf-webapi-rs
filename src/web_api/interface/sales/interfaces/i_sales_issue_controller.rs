use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `SalesIssue`.
pub struct ISalesIssueController;

pub mod add_new {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `POST` `/api/SalesIssue/New`
    ///
    /// Parameter `issue`: Czy wystawić dokument sprzedaży. W przypadku, gdy parametr nie zostanie podany dokument sprzedaży zostanie dodany do bufora.
    ///
    /// Body `document`: Obiekt dokumentu sprzedaży do dodania do bufora lub wystawienia.
    pub struct ByIssue<'a> {
        /// Czy wystawić dokument sprzedaży.
        /// W przypadku, gdy parametr nie zostanie podany dokument sprzedaży zostanie dodany do bufora.
        pub issue: bool,
        /// Obiekt dokumentu sprzedaży do dodania do bufora lub wystawienia.
        pub document: &'a crate::web_api::interface::sales::view_models::SaleDocumentIssue,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByIssue<'a> {
        type Output = crate::web_api::interface::sales::view_models::SaleDocument;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "issue", &self.issue)?;
            api.request_with_body::<crate::web_api::interface::sales::view_models::SaleDocument, _>(
                HttpMethod::Post,
                "/api/SalesIssue/New",
                query,
                self.document,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "issue", &self.issue)?;
            api.request_with_body_raw(
                HttpMethod::Post,
                "/api/SalesIssue/New",
                query,
                self.document,
            )
            .await
        }
    }

    /// `POST` `/api/SalesIssue/NewCorrection`
    ///
    /// Parameter `issue`: Czy wystawić dokument sprzedaży. W przypadku, gdy parametr nie zostanie podany dokument sprzedaży zostanie dodany do bufora.
    ///
    /// Body `correction`: Obiekt korekty dokumentu sprzedaży do dodania do bufora lub wystawienia.
    pub struct ByIssueNewCorrection<'a> {
        /// Czy wystawić dokument sprzedaży.
        /// W przypadku, gdy parametr nie zostanie podany dokument sprzedaży zostanie dodany do bufora.
        pub issue: bool,
        /// Obiekt korekty dokumentu sprzedaży do dodania do bufora lub wystawienia.
        pub correction: &'a crate::web_api::interface::sales::view_models::SaleCorrectionIssue,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByIssueNewCorrection<'a> {
        type Output = crate::web_api::interface::sales::view_models::SaleCorrection;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "issue", &self.issue)?;
            api.request_with_body::<crate::web_api::interface::sales::view_models::SaleCorrection, _>(HttpMethod::Post, "/api/SalesIssue/NewCorrection", query, self.correction).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "issue", &self.issue)?;
            api.request_with_body_raw(
                HttpMethod::Post,
                "/api/SalesIssue/NewCorrection",
                query,
                self.correction,
            )
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

    /// `PUT` `/api/SalesIssue/InBuffer`
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
                HttpMethod::Put,
                "/api/SalesIssue/InBuffer",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/SalesIssue/InBuffer", query)
                .await
        }
    }

    /// `PUT` `/api/SalesIssue/InBuffer`
    ///
    /// Parameter `number`: Numer dokumentu sprzedaży.
    pub struct ByNumber {
        /// Numer dokumentu sprzedaży.
        pub number: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByNumber {
        type Output = crate::web_api::interface::sales::view_models::SaleDocument;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            api.request_no_body::<crate::web_api::interface::sales::view_models::SaleDocument>(
                HttpMethod::Put,
                "/api/SalesIssue/InBuffer",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/SalesIssue/InBuffer", query)
                .await
        }
    }
}

pub mod issue_advance_payment {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/SalesIssue/AdvancePayment`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    ///
    /// Body `options`: Obiekt zawierający opcje utworzenia dokumentu.
    pub struct ByDocumentId<'a> {
        /// Id dokumentu sprzedaży.
        pub document_id: i32,
        /// Obiekt zawierający opcje utworzenia dokumentu.
        pub options: &'a crate::web_api::interface::sales::view_models::AdvancePaymentOptions,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByDocumentId<'a> {
        type Output = crate::web_api::interface::sales::view_models::SaleDocument;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_with_body::<crate::web_api::interface::sales::view_models::SaleDocument, _>(
                HttpMethod::Put,
                "/api/SalesIssue/AdvancePayment",
                query,
                self.options,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/SalesIssue/AdvancePayment",
                query,
                self.options,
            )
            .await
        }
    }

    /// `PUT` `/api/SalesIssue/AdvancePayment`
    ///
    /// Parameter `document_number`: Numer dokumentu sprzedaży.
    ///
    /// Body `options`: Obiekt zawierający opcje utworzenia dokumentu.
    pub struct ByDocumentNumber<'a> {
        /// Numer dokumentu sprzedaży.
        pub document_number: String,
        /// Obiekt zawierający opcje utworzenia dokumentu.
        pub options: &'a crate::web_api::interface::sales::view_models::AdvancePaymentOptions,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByDocumentNumber<'a> {
        type Output = crate::web_api::interface::sales::view_models::SaleDocument;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            api.request_with_body::<crate::web_api::interface::sales::view_models::SaleDocument, _>(
                HttpMethod::Put,
                "/api/SalesIssue/AdvancePayment",
                query,
                self.options,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/SalesIssue/AdvancePayment",
                query,
                self.options,
            )
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

    /// `PUT` `/api/SalesIssue/WZ`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    ///
    /// Parameter `in_buffer`: Czy WZ ma pozostać w buforze, czy też ma zostać automatycznie wystawiona.
    pub struct ByDocumentIdInBuffer {
        /// Id dokumentu sprzedaży.
        pub document_id: i32,
        /// Czy WZ ma pozostać w buforze, czy też ma zostać automatycznie wystawiona.
        pub in_buffer: Option<i32>,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentIdInBuffer {
        type Output = Vec<crate::web_api::interface::sales::view_models::SaleDocumentWZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            push_query_param(&mut query, "inBuffer", &self.in_buffer)?;
            api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentWZ>>(HttpMethod::Put, "/api/SalesIssue/WZ", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            push_query_param(&mut query, "inBuffer", &self.in_buffer)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/SalesIssue/WZ", query)
                .await
        }
    }

    /// `PUT` `/api/SalesIssue/WZ`
    ///
    /// Parameter `document_number`: Numer dokumentu sprzedaży.
    ///
    /// Parameter `in_buffer`: Czy WZ ma pozostać w buforze, czy też ma zostać automatycznie wystawiona.
    pub struct ByDocumentNumberInBuffer {
        /// Numer dokumentu sprzedaży.
        pub document_number: String,
        /// Czy WZ ma pozostać w buforze, czy też ma zostać automatycznie wystawiona.
        pub in_buffer: Option<i32>,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumberInBuffer {
        type Output = Vec<crate::web_api::interface::sales::view_models::SaleDocumentWZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "inBuffer", &self.in_buffer)?;
            api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentWZ>>(HttpMethod::Put, "/api/SalesIssue/WZ", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "inBuffer", &self.in_buffer)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/SalesIssue/WZ", query)
                .await
        }
    }
}

pub mod issue_wz_correction {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/SalesIssue/IssueWZCorrection`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    ///
    /// Parameter `issue`: Jeżeli to możliwe, czy wystawić korekty dokumentów magazynowych. W przypadku, gdy parametr nie zostanie podany korekty dokumentów magazynowych zostaną dodane do bufora.
    pub struct ByDocumentIdIssue {
        /// Id dokumentu sprzedaży.
        pub document_id: i32,
        /// Jeżeli to możliwe, czy wystawić korekty dokumentów magazynowych.
        /// W przypadku, gdy parametr nie zostanie podany korekty dokumentów magazynowych zostaną dodane do bufora.
        pub issue: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentIdIssue {
        type Output = Vec<crate::web_api::interface::sales::view_models::SaleDocumentWZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            push_query_param(&mut query, "issue", &self.issue)?;
            api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentWZ>>(HttpMethod::Put, "/api/SalesIssue/IssueWZCorrection", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            push_query_param(&mut query, "issue", &self.issue)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/SalesIssue/IssueWZCorrection", query)
                .await
        }
    }

    /// `PUT` `/api/SalesIssue/IssueWZCorrection`
    ///
    /// Parameter `document_number`: Numer dokumentu sprzedaży.
    ///
    /// Parameter `issue`: Jeżeli to możliwe, czy wystawić korekty dokumentów magazynowych. W przypadku, gdy parametr nie zostanie podany korekty dokumentów magazynowych zostaną dodane do bufora.
    pub struct ByDocumentNumberIssue {
        /// Numer dokumentu sprzedaży.
        pub document_number: String,
        /// Jeżeli to możliwe, czy wystawić korekty dokumentów magazynowych.
        /// W przypadku, gdy parametr nie zostanie podany korekty dokumentów magazynowych zostaną dodane do bufora.
        pub issue: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumberIssue {
        type Output = Vec<crate::web_api::interface::sales::view_models::SaleDocumentWZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "issue", &self.issue)?;
            api.request_no_body::<Vec<crate::web_api::interface::sales::view_models::SaleDocumentWZ>>(HttpMethod::Put, "/api/SalesIssue/IssueWZCorrection", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "issue", &self.issue)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/SalesIssue/IssueWZCorrection", query)
                .await
        }
    }
}

pub mod issue_pn {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/SalesIssue/IssuePayment`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub struct ByDocumentId {
        /// Id dokumentu sprzedaży.
        pub document_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentId {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body::<()>(HttpMethod::Put, "/api/SalesIssue/IssuePayment", query)
                .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/SalesIssue/IssuePayment", query)
                .await
        }
    }

    /// `PUT` `/api/SalesIssue/IssuePayment`
    ///
    /// Parameter `document_number`: Numer dokumentu sprzedaży.
    pub struct ByDocumentNumber {
        /// Numer dokumentu sprzedaży.
        pub document_number: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumber {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            api.request_no_body::<()>(HttpMethod::Put, "/api/SalesIssue/IssuePayment", query)
                .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/SalesIssue/IssuePayment", query)
                .await
        }
    }
}

pub mod change_fiscal_status {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PATCH` `/api/SalesIssue/FiscalStatus`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży
    pub struct ByDocumentId {
        /// Id dokumentu sprzedaży
        pub document_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentId {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body::<()>(HttpMethod::Patch, "/api/SalesIssue/FiscalStatus", query)
                .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body_raw(HttpMethod::Patch, "/api/SalesIssue/FiscalStatus", query)
                .await
        }
    }

    /// `PATCH` `/api/SalesIssue/FiscalStatus`
    ///
    /// Parameter `document_number`: Numer dokumentu sprzedaży.
    pub struct ByDocumentNumber {
        /// Numer dokumentu sprzedaży.
        pub document_number: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumber {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            api.request_no_body::<()>(HttpMethod::Patch, "/api/SalesIssue/FiscalStatus", query)
                .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            api.request_no_body_raw(HttpMethod::Patch, "/api/SalesIssue/FiscalStatus", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl ISalesIssueController {
    /// `POST` `/api/SalesIssue/New`
    ///
    /// Parameter `issue`: Czy wystawić dokument sprzedaży. W przypadku, gdy parametr nie zostanie podany dokument sprzedaży zostanie dodany do bufora.
    ///
    /// Body `document`: Obiekt dokumentu sprzedaży do dodania do bufora lub wystawienia.
    pub async fn AddNew<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: add_new::Overload,
    {
        overload.call(api).await
    }

    /// `POST` `/api/SalesIssue/New`
    ///
    /// Parameter `issue`: Czy wystawić dokument sprzedaży. W przypadku, gdy parametr nie zostanie podany dokument sprzedaży zostanie dodany do bufora.
    ///
    /// Body `document`: Obiekt dokumentu sprzedaży do dodania do bufora lub wystawienia.
    pub async fn AddNewRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: add_new::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/SalesIssue/InBuffer`
    ///
    /// Parameter `id`: Id dokumentu sprzedaży.
    pub async fn Issue<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: issue::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/SalesIssue/InBuffer`
    ///
    /// Parameter `id`: Id dokumentu sprzedaży.
    pub async fn IssueRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: issue::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/SalesIssue/AdvancePayment`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    ///
    /// Body `options`: Obiekt zawierający opcje utworzenia dokumentu.
    pub async fn IssueAdvancePayment<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: issue_advance_payment::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/SalesIssue/AdvancePayment`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    ///
    /// Body `options`: Obiekt zawierający opcje utworzenia dokumentu.
    pub async fn IssueAdvancePaymentRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: issue_advance_payment::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/SalesIssue/WZ`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
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

    /// `PUT` `/api/SalesIssue/WZ`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
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

    /// `PUT` `/api/SalesIssue/IssueWZCorrection`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    ///
    /// Parameter `issue`: Jeżeli to możliwe, czy wystawić korekty dokumentów magazynowych. W przypadku, gdy parametr nie zostanie podany korekty dokumentów magazynowych zostaną dodane do bufora.
    pub async fn IssueWZCorrection<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: issue_wz_correction::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/SalesIssue/IssueWZCorrection`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    ///
    /// Parameter `issue`: Jeżeli to możliwe, czy wystawić korekty dokumentów magazynowych. W przypadku, gdy parametr nie zostanie podany korekty dokumentów magazynowych zostaną dodane do bufora.
    pub async fn IssueWZCorrectionRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: issue_wz_correction::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/SalesIssue/IssuePayment`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub async fn IssuePN<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: issue_pn::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/SalesIssue/IssuePayment`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub async fn IssuePNRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: issue_pn::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PATCH` `/api/SalesIssue/FiscalStatus`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży
    pub async fn ChangeFiscalStatus<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: change_fiscal_status::Overload,
    {
        overload.call(api).await
    }

    /// `PATCH` `/api/SalesIssue/FiscalStatus`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży
    pub async fn ChangeFiscalStatusRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: change_fiscal_status::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PATCH` `/api/SalesIssue/DocumentNumber`
    ///
    /// Parameter `id`: Id dokumentu sprzedaży.
    ///
    /// Parameter `number`: Nowy numer dokumentu sprzedaży.
    pub async fn ChangeDocumentNumber(
        api: &ApiClient,
        id: i32,
        number: String,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        push_query_param(&mut query, "number", &number)?;
        api.request_no_body::<()>(HttpMethod::Patch, "/api/SalesIssue/DocumentNumber", query)
            .await
    }

    /// `PATCH` `/api/SalesIssue/DocumentNumber`
    ///
    /// Parameter `id`: Id dokumentu sprzedaży.
    ///
    /// Parameter `number`: Nowy numer dokumentu sprzedaży.
    pub async fn ChangeDocumentNumberRaw(
        api: &ApiClient,
        id: i32,
        number: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        push_query_param(&mut query, "number", &number)?;
        api.request_no_body_raw(HttpMethod::Patch, "/api/SalesIssue/DocumentNumber", query)
            .await
    }
}
