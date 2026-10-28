use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `PurchasesIssue`.
pub struct IPurchasesIssueController;

pub mod add_new {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `POST` `/api/PurchasesIssue/New`
    ///
    /// Parameter `issue`: Czy wystawić dokument zakupu. W przypadku, gdy parametr nie zostanie podany dokument zakupu zostanie dodany do bufora.
    ///
    /// Body `document`: Obiekt dokumentu zakupu do dodania do bufora lub wystawienia.
    pub struct ByIssue<'a> {
        /// Czy wystawić dokument zakupu.
        /// W przypadku, gdy parametr nie zostanie podany dokument zakupu zostanie dodany do bufora.
        pub issue: bool,
        /// Obiekt dokumentu zakupu do dodania do bufora lub wystawienia.
        pub document: &'a crate::web_api::interface::purchases::view_models::PurchaseDocumentIssue,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByIssue<'a> {
        type Output = crate::web_api::interface::purchases::view_models::PurchaseDocument;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "issue", &self.issue)?;
            api.request_with_body::<crate::web_api::interface::purchases::view_models::PurchaseDocument, _>(HttpMethod::Post, "/api/PurchasesIssue/New", query, self.document).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "issue", &self.issue)?;
            api.request_with_body_raw(
                HttpMethod::Post,
                "/api/PurchasesIssue/New",
                query,
                self.document,
            )
            .await
        }
    }

    /// `POST` `/api/PurchasesIssue/NewCorrection`
    ///
    /// Parameter `issue`: Czy wystawić dokument zakupu. W przypadku, gdy parametr nie zostanie podany dokument zakupu zostanie dodany do bufora.
    ///
    /// Body `correction`: Obiekt korekty dokumentu zakupu do dodania do bufora lub wystawienia.
    pub struct ByIssueNewCorrection<'a> {
        /// Czy wystawić dokument zakupu.
        /// W przypadku, gdy parametr nie zostanie podany dokument zakupu zostanie dodany do bufora.
        pub issue: bool,
        /// Obiekt korekty dokumentu zakupu do dodania do bufora lub wystawienia.
        pub correction:
            &'a crate::web_api::interface::purchases::view_models::PurchaseCorrectionIssue,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByIssueNewCorrection<'a> {
        type Output = crate::web_api::interface::purchases::view_models::PurchaseCorrection;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "issue", &self.issue)?;
            api.request_with_body::<crate::web_api::interface::purchases::view_models::PurchaseCorrection, _>(HttpMethod::Post, "/api/PurchasesIssue/NewCorrection", query, self.correction).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "issue", &self.issue)?;
            api.request_with_body_raw(
                HttpMethod::Post,
                "/api/PurchasesIssue/NewCorrection",
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

    /// `PUT` `/api/PurchasesIssue/InBuffer`
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
            api.request_no_body::<crate::web_api::interface::purchases::view_models::PurchaseDocument>(HttpMethod::Put, "/api/PurchasesIssue/InBuffer", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/PurchasesIssue/InBuffer", query)
                .await
        }
    }

    /// `PUT` `/api/PurchasesIssue/InBuffer`
    ///
    /// Parameter `number`: Numer dokumentu zakupu.
    pub struct ByNumber {
        /// Numer dokumentu zakupu.
        pub number: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByNumber {
        type Output = crate::web_api::interface::purchases::view_models::PurchaseDocument;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            api.request_no_body::<crate::web_api::interface::purchases::view_models::PurchaseDocument>(HttpMethod::Put, "/api/PurchasesIssue/InBuffer", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/PurchasesIssue/InBuffer", query)
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

    /// `PUT` `/api/PurchasesIssue/PZ`
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
            api.request_no_body::<Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentPZ>>(HttpMethod::Put, "/api/PurchasesIssue/PZ", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/PurchasesIssue/PZ", query)
                .await
        }
    }

    /// `PUT` `/api/PurchasesIssue/PZ`
    ///
    /// Parameter `document_number`: Numer dokumentu zakupu.
    pub struct ByDocumentNumber {
        /// Numer dokumentu zakupu.
        pub document_number: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumber {
        type Output = Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentPZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            api.request_no_body::<Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentPZ>>(HttpMethod::Put, "/api/PurchasesIssue/PZ", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/PurchasesIssue/PZ", query)
                .await
        }
    }
}

pub mod issue_pz_correction {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/PurchasesIssue/PZCorrection`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    ///
    /// Parameter `issue`: Jeżeli to możliwe, czy wystawić korekty dokumentów magazynowych. W przypadku, gdy parametr nie zostanie podany korekty dokumentów magazynowych zostaną dodane do bufora.
    pub struct ByDocumentIdIssue {
        /// Id dokumentu zakupu.
        pub document_id: i32,
        /// Jeżeli to możliwe, czy wystawić korekty dokumentów magazynowych.
        /// W przypadku, gdy parametr nie zostanie podany korekty dokumentów magazynowych zostaną dodane do bufora.
        pub issue: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentIdIssue {
        type Output = Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentPZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            push_query_param(&mut query, "issue", &self.issue)?;
            api.request_no_body::<Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentPZ>>(HttpMethod::Put, "/api/PurchasesIssue/PZCorrection", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            push_query_param(&mut query, "issue", &self.issue)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/PurchasesIssue/PZCorrection", query)
                .await
        }
    }

    /// `PUT` `/api/PurchasesIssue/PZCorrection`
    ///
    /// Parameter `document_number`: Numer dokumentu zakupu.
    ///
    /// Parameter `issue`: Jeżeli to możliwe, czy wystawić korekty dokumentów magazynowych. W przypadku, gdy parametr nie zostanie podany korekty dokumentów magazynowych zostaną dodane do bufora.
    pub struct ByDocumentNumberIssue {
        /// Numer dokumentu zakupu.
        pub document_number: String,
        /// Jeżeli to możliwe, czy wystawić korekty dokumentów magazynowych.
        /// W przypadku, gdy parametr nie zostanie podany korekty dokumentów magazynowych zostaną dodane do bufora.
        pub issue: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumberIssue {
        type Output = Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentPZ>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "issue", &self.issue)?;
            api.request_no_body::<Vec<crate::web_api::interface::purchases::view_models::PurchaseDocumentPZ>>(HttpMethod::Put, "/api/PurchasesIssue/PZCorrection", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "issue", &self.issue)?;
            api.request_no_body_raw(HttpMethod::Put, "/api/PurchasesIssue/PZCorrection", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IPurchasesIssueController {
    /// `POST` `/api/PurchasesIssue/New`
    ///
    /// Parameter `issue`: Czy wystawić dokument zakupu. W przypadku, gdy parametr nie zostanie podany dokument zakupu zostanie dodany do bufora.
    ///
    /// Body `document`: Obiekt dokumentu zakupu do dodania do bufora lub wystawienia.
    pub async fn AddNew<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: add_new::Overload,
    {
        overload.call(api).await
    }

    /// `POST` `/api/PurchasesIssue/New`
    ///
    /// Parameter `issue`: Czy wystawić dokument zakupu. W przypadku, gdy parametr nie zostanie podany dokument zakupu zostanie dodany do bufora.
    ///
    /// Body `document`: Obiekt dokumentu zakupu do dodania do bufora lub wystawienia.
    pub async fn AddNewRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: add_new::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/PurchasesIssue/InBuffer`
    ///
    /// Parameter `id`: Id dokumentu zakupu.
    pub async fn Issue<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: issue::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/PurchasesIssue/InBuffer`
    ///
    /// Parameter `id`: Id dokumentu zakupu.
    pub async fn IssueRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: issue::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/PurchasesIssue/PZ`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub async fn IssuePZ<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: issue_pz::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/PurchasesIssue/PZ`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub async fn IssuePZRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: issue_pz::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/PurchasesIssue/PZCorrection`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    ///
    /// Parameter `issue`: Jeżeli to możliwe, czy wystawić korekty dokumentów magazynowych. W przypadku, gdy parametr nie zostanie podany korekty dokumentów magazynowych zostaną dodane do bufora.
    pub async fn IssuePZCorrection<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: issue_pz_correction::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/PurchasesIssue/PZCorrection`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    ///
    /// Parameter `issue`: Jeżeli to możliwe, czy wystawić korekty dokumentów magazynowych. W przypadku, gdy parametr nie zostanie podany korekty dokumentów magazynowych zostaną dodane do bufora.
    pub async fn IssuePZCorrectionRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: issue_pz_correction::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PATCH` `/api/PurchasesIssue/DocumentNumber`
    ///
    /// Parameter `id`: Id dokumentu zakupu.
    ///
    /// Parameter `number`: Nowy numer dokumentu zakupu.
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
            "/api/PurchasesIssue/DocumentNumber",
            query,
        )
        .await
    }

    /// `PATCH` `/api/PurchasesIssue/DocumentNumber`
    ///
    /// Parameter `id`: Id dokumentu zakupu.
    ///
    /// Parameter `number`: Nowy numer dokumentu zakupu.
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
            "/api/PurchasesIssue/DocumentNumber",
            query,
        )
        .await
    }
}
