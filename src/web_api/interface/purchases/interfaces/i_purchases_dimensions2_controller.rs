use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `PurchasesDimensions2`.
pub struct IPurchasesDimensions2Controller;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/v2/PurchasesDimensions`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub struct ByDocumentId {
        /// Id dokumentu zakupu.
        pub document_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentId {
        type Output = Vec<crate::web_api::interface::common::view_models::Dimension>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Dimension>>(
                HttpMethod::Get,
                "/api/v2/PurchasesDimensions",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/v2/PurchasesDimensions", query)
                .await
        }
    }

    /// `GET` `/api/v2/PurchasesDimensions`
    ///
    /// Parameter `document_number`: Numer dokumentu zakupu.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych dokumentu zakupu o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByDocumentNumberBuffer {
        /// Numer dokumentu zakupu.
        pub document_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych dokumentu zakupu o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumberBuffer {
        type Output = Vec<crate::web_api::interface::common::view_models::Dimension>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Dimension>>(
                HttpMethod::Get,
                "/api/v2/PurchasesDimensions",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/v2/PurchasesDimensions", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IPurchasesDimensions2Controller {
    /// `GET` `/api/v2/PurchasesDimensions`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/v2/PurchasesDimensions`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/v2/PurchasesDimensions/Positions`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub async fn GetPositionsByDocumentId(
        api: &ApiClient,
        document_id: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::PositionDimension>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::PositionDimension>>(HttpMethod::Get, "/api/v2/PurchasesDimensions/Positions", query).await
    }

    /// `GET` `/api/v2/PurchasesDimensions/Positions`
    ///
    /// Parameter `document_id`: Id dokumentu zakupu.
    pub async fn GetPositionsByDocumentIdRaw(
        api: &ApiClient,
        document_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/v2/PurchasesDimensions/Positions",
            query,
        )
        .await
    }

    /// `GET` `/api/v2/PurchasesDimensions/Positions`
    ///
    /// Parameter `document_number`: Numer dokumentu zakupu.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych pozycji dokumentu zakupu o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub async fn GetPositionsByDocumentNumber(
        api: &ApiClient,
        document_number: String,
        buffer: bool,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::PositionDimension>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentNumber", &document_number)?;
        push_query_param(&mut query, "buffer", &buffer)?;
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::PositionDimension>>(HttpMethod::Get, "/api/v2/PurchasesDimensions/Positions", query).await
    }

    /// `GET` `/api/v2/PurchasesDimensions/Positions`
    ///
    /// Parameter `document_number`: Numer dokumentu zakupu.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych pozycji dokumentu zakupu o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub async fn GetPositionsByDocumentNumberRaw(
        api: &ApiClient,
        document_number: String,
        buffer: bool,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentNumber", &document_number)?;
        push_query_param(&mut query, "buffer", &buffer)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/v2/PurchasesDimensions/Positions",
            query,
        )
        .await
    }

    /// `GET` `/api/v2/PurchasesDimensions/Positions`
    ///
    /// Parameter `position_id`: Id pozycji dokumentu zakupu.
    pub async fn GetPosition(
        api: &ApiClient,
        position_id: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Dimension>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "positionId", &position_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Dimension>>(
            HttpMethod::Get,
            "/api/v2/PurchasesDimensions/Positions",
            query,
        )
        .await
    }

    /// `GET` `/api/v2/PurchasesDimensions/Positions`
    ///
    /// Parameter `position_id`: Id pozycji dokumentu zakupu.
    pub async fn GetPositionRaw(
        api: &ApiClient,
        position_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "positionId", &position_id)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/v2/PurchasesDimensions/Positions",
            query,
        )
        .await
    }
}
