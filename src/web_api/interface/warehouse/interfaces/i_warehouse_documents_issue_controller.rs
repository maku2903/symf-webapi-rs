use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `WarehouseDocumentsIssue`.
pub struct IWarehouseDocumentsIssueController;

pub mod issue {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/WarehouseDocumentsIssue/InBuffer`
    ///
    /// Parameter `id`: Id dokumentu magazynowego.
    pub struct ById {
        /// Id dokumentu magazynowego.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = crate::web_api::interface::warehouse::view_models::WarehouseDocument;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<crate::web_api::interface::warehouse::view_models::WarehouseDocument>(HttpMethod::Put, "/api/WarehouseDocumentsIssue/InBuffer", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(
                HttpMethod::Put,
                "/api/WarehouseDocumentsIssue/InBuffer",
                query,
            )
            .await
        }
    }

    /// `PUT` `/api/WarehouseDocumentsIssue/InBuffer`
    ///
    /// Parameter `number`: Numer dokumentu magazynowego.
    pub struct ByNumber {
        /// Numer dokumentu magazynowego.
        pub number: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByNumber {
        type Output = crate::web_api::interface::warehouse::view_models::WarehouseDocument;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            api.request_no_body::<crate::web_api::interface::warehouse::view_models::WarehouseDocument>(HttpMethod::Put, "/api/WarehouseDocumentsIssue/InBuffer", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "number", &self.number)?;
            api.request_no_body_raw(
                HttpMethod::Put,
                "/api/WarehouseDocumentsIssue/InBuffer",
                query,
            )
            .await
        }
    }
}

pub mod issue_mm_plus {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/WarehouseDocumentsIssue/MMPlus`
    ///
    /// Parameter `document_id`: Id dokumentu MM-.
    pub struct ByDocumentId {
        /// Id dokumentu MM-.
        pub document_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentId {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body::<()>(
                HttpMethod::Put,
                "/api/WarehouseDocumentsIssue/MMPlus",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body_raw(
                HttpMethod::Put,
                "/api/WarehouseDocumentsIssue/MMPlus",
                query,
            )
            .await
        }
    }

    /// `PUT` `/api/WarehouseDocumentsIssue/MMPlus`
    ///
    /// Parameter `document_number`: Numer dokumentu MM-.
    pub struct ByDocumentNumber {
        /// Numer dokumentu MM-.
        pub document_number: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumber {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            api.request_no_body::<()>(
                HttpMethod::Put,
                "/api/WarehouseDocumentsIssue/MMPlus",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            api.request_no_body_raw(
                HttpMethod::Put,
                "/api/WarehouseDocumentsIssue/MMPlus",
                query,
            )
            .await
        }
    }
}

#[allow(non_snake_case)]
impl IWarehouseDocumentsIssueController {
    /// `PUT` `/api/WarehouseDocumentsIssue/InBuffer`
    ///
    /// Parameter `id`: Id dokumentu magazynowego.
    pub async fn Issue<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: issue::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/WarehouseDocumentsIssue/InBuffer`
    ///
    /// Parameter `id`: Id dokumentu magazynowego.
    pub async fn IssueRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: issue::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/WarehouseDocumentsIssue/MMPlus`
    ///
    /// Parameter `document_id`: Id dokumentu MM-.
    pub async fn IssueMMPlus<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: issue_mm_plus::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/WarehouseDocumentsIssue/MMPlus`
    ///
    /// Parameter `document_id`: Id dokumentu MM-.
    pub async fn IssueMMPlusRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: issue_mm_plus::Overload,
    {
        overload.call_raw(api).await
    }

    /// `POST` `/api/WarehouseDocumentsIssue/New`
    ///
    /// Parameter `issue`: Czy wystawić dokument magazynowego. W przypadku, gdy parametr nie zostanie podany dokument magazynowy zostanie dodany do bufora.
    ///
    /// Body `document`: Obiekt dokumentu magazynowego do dodania do bufora lub wystawienia.
    pub async fn AddNew(
        api: &ApiClient,
        issue: bool,
        document: &crate::web_api::interface::warehouse::view_models::WarehouseDocumentIssue,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::warehouse::view_models::WarehouseDocument>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "issue", &issue)?;
        api.request_with_body::<crate::web_api::interface::warehouse::view_models::WarehouseDocument, _>(HttpMethod::Post, "/api/WarehouseDocumentsIssue/New", query, document).await
    }

    /// `POST` `/api/WarehouseDocumentsIssue/New`
    ///
    /// Parameter `issue`: Czy wystawić dokument magazynowego. W przypadku, gdy parametr nie zostanie podany dokument magazynowy zostanie dodany do bufora.
    ///
    /// Body `document`: Obiekt dokumentu magazynowego do dodania do bufora lub wystawienia.
    pub async fn AddNewRaw(
        api: &ApiClient,
        issue: bool,
        document: &crate::web_api::interface::warehouse::view_models::WarehouseDocumentIssue,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "issue", &issue)?;
        api.request_with_body_raw(
            HttpMethod::Post,
            "/api/WarehouseDocumentsIssue/New",
            query,
            document,
        )
        .await
    }

    /// `PUT` `/api/WarehouseDocumentsIssue/Edit`
    ///
    /// Body `document`: Obiekt dokumentu magazynowego do edycji.
    pub async fn Edit(
        api: &ApiClient,
        document: &crate::web_api::interface::warehouse::view_models::WarehouseDocumentEdit,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::warehouse::view_models::WarehouseDocument>,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<crate::web_api::interface::warehouse::view_models::WarehouseDocument, _>(HttpMethod::Put, "/api/WarehouseDocumentsIssue/Edit", query, document).await
    }

    /// `PUT` `/api/WarehouseDocumentsIssue/Edit`
    ///
    /// Body `document`: Obiekt dokumentu magazynowego do edycji.
    pub async fn EditRaw(
        api: &ApiClient,
        document: &crate::web_api::interface::warehouse::view_models::WarehouseDocumentEdit,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Put,
            "/api/WarehouseDocumentsIssue/Edit",
            query,
            document,
        )
        .await
    }

    /// `PATCH` `/api/WarehouseDocumentsIssue/DocumentNumber`
    ///
    /// Parameter `id`: Id dokumentu magazynowego.
    ///
    /// Parameter `number`: Nowy numer dokumentu magazynowego.
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
            "/api/WarehouseDocumentsIssue/DocumentNumber",
            query,
        )
        .await
    }

    /// `PATCH` `/api/WarehouseDocumentsIssue/DocumentNumber`
    ///
    /// Parameter `id`: Id dokumentu magazynowego.
    ///
    /// Parameter `number`: Nowy numer dokumentu magazynowego.
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
            "/api/WarehouseDocumentsIssue/DocumentNumber",
            query,
        )
        .await
    }
}
