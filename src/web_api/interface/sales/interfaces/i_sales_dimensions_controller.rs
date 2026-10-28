use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `SalesDimensions`.
pub struct ISalesDimensionsController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/SalesDimensions`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub struct ByDocumentId {
        /// Id dokumentu sprzedaży.
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
                "/api/SalesDimensions",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/SalesDimensions", query)
                .await
        }
    }

    /// `GET` `/api/SalesDimensions`
    ///
    /// Parameter `document_number`: Numer dokumentu sprzedaży.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych dokumentu sprzedaży o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByDocumentNumberBuffer {
        /// Numer dokumentu sprzedaży.
        pub document_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych dokumentu sprzedaży o podanym numerze dokumentu, które nie znajduje się w buforze.
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
                "/api/SalesDimensions",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/SalesDimensions", query)
                .await
        }
    }
}

pub mod update {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/SalesDimensions/Update`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    ///
    /// Body `document_dimension`: Obiekt pola własnego dokumentu sprzedaży do aktualizacji.
    pub struct ByDocumentId<'a> {
        /// Id dokumentu sprzedaży.
        pub document_id: i32,
        /// Obiekt pola własnego dokumentu sprzedaży do aktualizacji.
        pub document_dimension: &'a crate::web_api::interface::common::view_models::Dimension,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByDocumentId<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/SalesDimensions/Update",
                query,
                self.document_dimension,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/SalesDimensions/Update",
                query,
                self.document_dimension,
            )
            .await
        }
    }

    /// `PUT` `/api/SalesDimensions/Update`
    ///
    /// Parameter `document_number`: Numer dokumentu sprzedaży.
    ///
    /// Parameter `buffer`: Obiekt pola własnego dokumentu sprzedaży do aktualizacji.
    ///
    /// Body `document_dimension`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie zaktualizowane pole własne dokumentu sprzedaży o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByDocumentNumberBuffer<'a> {
        /// Numer dokumentu sprzedaży.
        pub document_number: String,
        /// Obiekt pola własnego dokumentu sprzedaży do aktualizacji.
        pub buffer: bool,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie zaktualizowane pole własne dokumentu sprzedaży o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub document_dimension: &'a crate::web_api::interface::common::view_models::Dimension,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByDocumentNumberBuffer<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/SalesDimensions/Update",
                query,
                self.document_dimension,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/SalesDimensions/Update",
                query,
                self.document_dimension,
            )
            .await
        }
    }

    /// `PUT` `/api/SalesDimensions/UpdateList`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    ///
    /// Body `document_dimensions`: Lista obiektów pól własnych dokumentu sprzedaży do aktualizacji.
    pub struct ByDocumentIdUpdateList<'a> {
        /// Id dokumentu sprzedaży.
        pub document_id: i32,
        /// Lista obiektów pól własnych dokumentu sprzedaży do aktualizacji.
        pub document_dimensions: &'a Vec<crate::web_api::interface::common::view_models::Dimension>,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByDocumentIdUpdateList<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/SalesDimensions/UpdateList",
                query,
                self.document_dimensions,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/SalesDimensions/UpdateList",
                query,
                self.document_dimensions,
            )
            .await
        }
    }

    /// `PUT` `/api/SalesDimensions/UpdateList`
    ///
    /// Parameter `document_number`: Numer dokumentu sprzedaży.
    ///
    /// Parameter `buffer`: Lista obiektów pól własnych dokumentu sprzedaży do aktualizacji.
    ///
    /// Body `document_dimensions`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostaną zaktualizowane pola własne dokumentu sprzedaży o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByDocumentNumberBufferUpdateList<'a> {
        /// Numer dokumentu sprzedaży.
        pub document_number: String,
        /// Lista obiektów pól własnych dokumentu sprzedaży do aktualizacji.
        pub buffer: bool,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostaną zaktualizowane pola własne dokumentu sprzedaży o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub document_dimensions: &'a Vec<crate::web_api::interface::common::view_models::Dimension>,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByDocumentNumberBufferUpdateList<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/SalesDimensions/UpdateList",
                query,
                self.document_dimensions,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/SalesDimensions/UpdateList",
                query,
                self.document_dimensions,
            )
            .await
        }
    }
}

pub mod update_position {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/SalesDimensions/UpdatePosition`
    ///
    /// Parameter `position_id`: Id pozycji dokumentu sprzedaży.
    ///
    /// Body `position_dimension`: Obiekt pola własnego pozycji dokumentu sprzedaży do aktualizacji.
    pub struct ByPositionId<'a> {
        /// Id pozycji dokumentu sprzedaży.
        pub position_id: i32,
        /// Obiekt pola własnego pozycji dokumentu sprzedaży do aktualizacji.
        pub position_dimension: &'a crate::web_api::interface::common::view_models::Dimension,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByPositionId<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "positionId", &self.position_id)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/SalesDimensions/UpdatePosition",
                query,
                self.position_dimension,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "positionId", &self.position_id)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/SalesDimensions/UpdatePosition",
                query,
                self.position_dimension,
            )
            .await
        }
    }

    /// `PUT` `/api/SalesDimensions/UpdatePositionList`
    ///
    /// Parameter `position_id`: Id pozycji dokumentu sprzedaży.
    ///
    /// Body `position_dimensions`: Lista obiektów pól własnych pozycji dokumentu sprzedaży do aktualizacji.
    pub struct ByPositionIdUpdatePositionList<'a> {
        /// Id pozycji dokumentu sprzedaży.
        pub position_id: i32,
        /// Lista obiektów pól własnych pozycji dokumentu sprzedaży do aktualizacji.
        pub position_dimensions: &'a Vec<crate::web_api::interface::common::view_models::Dimension>,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByPositionIdUpdatePositionList<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "positionId", &self.position_id)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/SalesDimensions/UpdatePositionList",
                query,
                self.position_dimensions,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "positionId", &self.position_id)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/SalesDimensions/UpdatePositionList",
                query,
                self.position_dimensions,
            )
            .await
        }
    }

    /// `PUT` `/api/SalesDimensions/UpdateMultiPositionsList`
    ///
    /// Body `position_dimension`: Lista obiektów pól własnych wielu pozycji dokumentu sprzedaży do aktualizacji.
    pub struct PositionDimension<'a> {
        /// Lista obiektów pól własnych wielu pozycji dokumentu sprzedaży do aktualizacji.
        pub position_dimension:
            &'a Vec<crate::web_api::interface::common::view_models::PositionDimension>,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for PositionDimension<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/SalesDimensions/UpdateMultiPositionsList",
                query,
                self.position_dimension,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/SalesDimensions/UpdateMultiPositionsList",
                query,
                self.position_dimension,
            )
            .await
        }
    }
}

pub mod get_aspects {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/SalesDimensions/Aspects`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub struct ByDocumentId {
        /// Id dokumentu sprzedaży.
        pub document_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentId {
        type Output = crate::web_api::interface::common::view_models::aspects::AspectDocument;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body::<crate::web_api::interface::common::view_models::aspects::AspectDocument>(HttpMethod::Get, "/api/SalesDimensions/Aspects", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentId", &self.document_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/SalesDimensions/Aspects", query)
                .await
        }
    }

    /// `GET` `/api/SalesDimensions/Aspects`
    ///
    /// Parameter `document_number`: Id dokumentu sprzedaży.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostaną pobrane wymiary pozycji dokumentu sprzedaży o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByDocumentNumberBuffer {
        /// Id dokumentu sprzedaży.
        pub document_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostaną pobrane wymiary pozycji dokumentu sprzedaży o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByDocumentNumberBuffer {
        type Output = crate::web_api::interface::common::view_models::aspects::AspectDocument;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<crate::web_api::interface::common::view_models::aspects::AspectDocument>(HttpMethod::Get, "/api/SalesDimensions/Aspects", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "documentNumber", &self.document_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/SalesDimensions/Aspects", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl ISalesDimensionsController {
    /// `GET` `/api/SalesDimensions`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/SalesDimensions`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/SalesDimensions/Update`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    ///
    /// Body `document_dimension`: Obiekt pola własnego dokumentu sprzedaży do aktualizacji.
    pub async fn Update<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: update::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/SalesDimensions/Update`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    ///
    /// Body `document_dimension`: Obiekt pola własnego dokumentu sprzedaży do aktualizacji.
    pub async fn UpdateRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: update::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/SalesDimensions/UpdatePosition`
    ///
    /// Parameter `position_id`: Id pozycji dokumentu sprzedaży.
    ///
    /// Body `position_dimension`: Obiekt pola własnego pozycji dokumentu sprzedaży do aktualizacji.
    pub async fn UpdatePosition<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: update_position::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/SalesDimensions/UpdatePosition`
    ///
    /// Parameter `position_id`: Id pozycji dokumentu sprzedaży.
    ///
    /// Body `position_dimension`: Obiekt pola własnego pozycji dokumentu sprzedaży do aktualizacji.
    pub async fn UpdatePositionRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: update_position::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/SalesDimensions/Aspects`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub async fn GetAspects<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_aspects::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/SalesDimensions/Aspects`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub async fn GetAspectsRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_aspects::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/SalesDimensions/Positions`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub async fn GetPositionsByDocumentId(
        api: &ApiClient,
        document_id: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::PositionDimension>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::PositionDimension>>(HttpMethod::Get, "/api/SalesDimensions/Positions", query).await
    }

    /// `GET` `/api/SalesDimensions/Positions`
    ///
    /// Parameter `document_id`: Id dokumentu sprzedaży.
    pub async fn GetPositionsByDocumentIdRaw(
        api: &ApiClient,
        document_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentId", &document_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/SalesDimensions/Positions", query)
            .await
    }

    /// `GET` `/api/SalesDimensions/Positions`
    ///
    /// Parameter `document_number`: Numer dokumentu sprzedaży.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych pozycji dokumentu sprzedaży o podanym numerze dokumentu, które nie znajduje się w buforze.
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
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::PositionDimension>>(HttpMethod::Get, "/api/SalesDimensions/Positions", query).await
    }

    /// `GET` `/api/SalesDimensions/Positions`
    ///
    /// Parameter `document_number`: Numer dokumentu sprzedaży.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych pozycji dokumentu sprzedaży o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub async fn GetPositionsByDocumentNumberRaw(
        api: &ApiClient,
        document_number: String,
        buffer: bool,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentNumber", &document_number)?;
        push_query_param(&mut query, "buffer", &buffer)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/SalesDimensions/Positions", query)
            .await
    }

    /// `GET` `/api/SalesDimensions/Positions`
    ///
    /// Parameter `position_id`: Id pozycji dokumentu sprzedaży.
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
            "/api/SalesDimensions/Positions",
            query,
        )
        .await
    }

    /// `GET` `/api/SalesDimensions/Positions`
    ///
    /// Parameter `position_id`: Id pozycji dokumentu sprzedaży.
    pub async fn GetPositionRaw(
        api: &ApiClient,
        position_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "positionId", &position_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/SalesDimensions/Positions", query)
            .await
    }

    /// `GET` `/api/SalesDimensions/Classification`
    pub async fn GetClassification(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::common::view_models::DimensionClassification>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DimensionClassification>>(HttpMethod::Get, "/api/SalesDimensions/Classification", query).await
    }

    /// `GET` `/api/SalesDimensions/Classification`
    pub async fn GetClassificationRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/SalesDimensions/Classification",
            query,
        )
        .await
    }

    /// `GET` `/api/SalesDimensions/PositionClassification`
    pub async fn GetPositionClassification(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::common::view_models::DimensionClassification>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DimensionClassification>>(HttpMethod::Get, "/api/SalesDimensions/PositionClassification", query).await
    }

    /// `GET` `/api/SalesDimensions/PositionClassification`
    pub async fn GetPositionClassificationRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/SalesDimensions/PositionClassification",
            query,
        )
        .await
    }

    /// `PUT` `/api/SalesDimensions/Aspects`
    ///
    /// Body `aspect_position`: Obiekt reprezentujący informacje o wymiarach pozycji dokumentu sprzedaży.
    pub async fn UpdateAspect(
        api: &ApiClient,
        aspect_position: &crate::web_api::interface::common::view_models::aspects::AspectPositionEdit,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let query = vec![];
        api.request_with_body::<(), _>(
            HttpMethod::Put,
            "/api/SalesDimensions/Aspects",
            query,
            aspect_position,
        )
        .await
    }

    /// `PUT` `/api/SalesDimensions/Aspects`
    ///
    /// Body `aspect_position`: Obiekt reprezentujący informacje o wymiarach pozycji dokumentu sprzedaży.
    pub async fn UpdateAspectRaw(
        api: &ApiClient,
        aspect_position: &crate::web_api::interface::common::view_models::aspects::AspectPositionEdit,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Put,
            "/api/SalesDimensions/Aspects",
            query,
            aspect_position,
        )
        .await
    }
}
