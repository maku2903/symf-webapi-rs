use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `OwnOrderDimensions`.
pub struct IOwnOrderDimensionsController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/OwnOrderDimensions`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub struct ByOrderId {
        /// Id zamówienia własnego.
        pub order_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderId {
        type Output = Vec<crate::web_api::interface::common::view_models::Dimension>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Dimension>>(
                HttpMethod::Get,
                "/api/OwnOrderDimensions",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrderDimensions", query)
                .await
        }
    }

    /// `GET` `/api/OwnOrderDimensions`
    ///
    /// Parameter `order_number`: Numer dokumentu zamówienia własnego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych zamówienia własnego o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByOrderNumberBuffer {
        /// Numer dokumentu zamówienia własnego.
        pub order_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych zamówienia własnego o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderNumberBuffer {
        type Output = Vec<crate::web_api::interface::common::view_models::Dimension>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Dimension>>(
                HttpMethod::Get,
                "/api/OwnOrderDimensions",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrderDimensions", query)
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

    /// `PUT` `/api/OwnOrderDimensions/Update`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    ///
    /// Body `order_dimension`: Obiekt pola własnego zamówienia własnego do aktualizacji.
    pub struct ByOrderId<'a> {
        /// Id zamówienia własnego.
        pub order_id: i32,
        /// Obiekt pola własnego zamówienia własnego do aktualizacji.
        pub order_dimension: &'a crate::web_api::interface::common::view_models::Dimension,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByOrderId<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/OwnOrderDimensions/Update",
                query,
                self.order_dimension,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/OwnOrderDimensions/Update",
                query,
                self.order_dimension,
            )
            .await
        }
    }

    /// `PUT` `/api/OwnOrderDimensions/Update`
    ///
    /// Parameter `order_number`: Numer zamówienia własnego.
    ///
    /// Parameter `buffer`: Obiekt pola własnego zamówienia własnego do aktualizacji.
    ///
    /// Body `order_dimension`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie zaktualizowane pole własne zamówienia własnego o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByOrderNumberBuffer<'a> {
        /// Numer zamówienia własnego.
        pub order_number: String,
        /// Obiekt pola własnego zamówienia własnego do aktualizacji.
        pub buffer: bool,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie zaktualizowane pole własne zamówienia własnego o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub order_dimension: &'a crate::web_api::interface::common::view_models::Dimension,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByOrderNumberBuffer<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/OwnOrderDimensions/Update",
                query,
                self.order_dimension,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/OwnOrderDimensions/Update",
                query,
                self.order_dimension,
            )
            .await
        }
    }

    /// `PUT` `/api/OwnOrderDimensions/UpdateList`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    ///
    /// Body `order_dimensions`: Lista obiektów pól własnych zamówienia własnego do aktualizacji.
    pub struct ByOrderIdUpdateList<'a> {
        /// Id zamówienia własnego.
        pub order_id: i32,
        /// Lista obiektów pól własnych zamówienia własnego do aktualizacji.
        pub order_dimensions: &'a Vec<crate::web_api::interface::common::view_models::Dimension>,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByOrderIdUpdateList<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/OwnOrderDimensions/UpdateList",
                query,
                self.order_dimensions,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/OwnOrderDimensions/UpdateList",
                query,
                self.order_dimensions,
            )
            .await
        }
    }

    /// `PUT` `/api/OwnOrderDimensions/UpdateList`
    ///
    /// Parameter `order_number`: Numer zamówienia własnego.
    ///
    /// Parameter `buffer`: Lista obiektów pól własnych zamówienia własnego do aktualizacji.
    ///
    /// Body `order_dimensions`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostaną zaktualizowane pola własne zamówienia własnego o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByOrderNumberBufferUpdateList<'a> {
        /// Numer zamówienia własnego.
        pub order_number: String,
        /// Lista obiektów pól własnych zamówienia własnego do aktualizacji.
        pub buffer: bool,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostaną zaktualizowane pola własne zamówienia własnego o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub order_dimensions: &'a Vec<crate::web_api::interface::common::view_models::Dimension>,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByOrderNumberBufferUpdateList<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/OwnOrderDimensions/UpdateList",
                query,
                self.order_dimensions,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/OwnOrderDimensions/UpdateList",
                query,
                self.order_dimensions,
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

    /// `PUT` `/api/OwnOrderDimensions/UpdatePosition`
    ///
    /// Parameter `position_id`: Id pozycji zamówienia własnego.
    ///
    /// Body `position_dimension`: Obiekt pola własnego pozycji zamówienia własnego do aktualizacji.
    pub struct ByPositionId<'a> {
        /// Id pozycji zamówienia własnego.
        pub position_id: i32,
        /// Obiekt pola własnego pozycji zamówienia własnego do aktualizacji.
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
                "/api/OwnOrderDimensions/UpdatePosition",
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
                "/api/OwnOrderDimensions/UpdatePosition",
                query,
                self.position_dimension,
            )
            .await
        }
    }

    /// `PUT` `/api/OwnOrderDimensions/UpdatePositionList`
    ///
    /// Parameter `position_id`: Id pozycji zamówienia własnego.
    ///
    /// Body `position_dimensions`: Lista obiektów pól własnych pozycji zamówienia własnego do aktualizacji.
    pub struct ByPositionIdUpdatePositionList<'a> {
        /// Id pozycji zamówienia własnego.
        pub position_id: i32,
        /// Lista obiektów pól własnych pozycji zamówienia własnego do aktualizacji.
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
                "/api/OwnOrderDimensions/UpdatePositionList",
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
                "/api/OwnOrderDimensions/UpdatePositionList",
                query,
                self.position_dimensions,
            )
            .await
        }
    }

    /// `PUT` `/api/OwnOrderDimensions/UpdateMultiPositionsList`
    ///
    /// Body `position_dimension`: Lista obiektów pól własnych wielu pozycji zamówienia własnego do aktualizacji.
    pub struct PositionDimension<'a> {
        /// Lista obiektów pól własnych wielu pozycji zamówienia własnego do aktualizacji.
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
                "/api/OwnOrderDimensions/UpdateMultiPositionsList",
                query,
                self.position_dimension,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/OwnOrderDimensions/UpdateMultiPositionsList",
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

    /// `GET` `/api/OwnOrderDimensions/Aspects`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub struct ByOrderId {
        /// Id zamówienia własnego.
        pub order_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderId {
        type Output = crate::web_api::interface::common::view_models::aspects::AspectDocument;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body::<crate::web_api::interface::common::view_models::aspects::AspectDocument>(HttpMethod::Get, "/api/OwnOrderDimensions/Aspects", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrderDimensions/Aspects", query)
                .await
        }
    }

    /// `GET` `/api/OwnOrderDimensions/Aspects`
    ///
    /// Parameter `order_number`: Id zamówienia własnego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostaną pobrane wymiary pozycji zamówienia własnego o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByOrderNumberBuffer {
        /// Id zamówienia własnego.
        pub order_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostaną pobrane wymiary pozycji zamówienia własnego o podanym numerze dokumentu, które nie znajduje się w buforze.
        pub buffer: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByOrderNumberBuffer {
        type Output = crate::web_api::interface::common::view_models::aspects::AspectDocument;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body::<crate::web_api::interface::common::view_models::aspects::AspectDocument>(HttpMethod::Get, "/api/OwnOrderDimensions/Aspects", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrderDimensions/Aspects", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IOwnOrderDimensionsController {
    /// `GET` `/api/OwnOrderDimensions`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/OwnOrderDimensions`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/OwnOrderDimensions/Update`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    ///
    /// Body `order_dimension`: Obiekt pola własnego zamówienia własnego do aktualizacji.
    pub async fn Update<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: update::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/OwnOrderDimensions/Update`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    ///
    /// Body `order_dimension`: Obiekt pola własnego zamówienia własnego do aktualizacji.
    pub async fn UpdateRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: update::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/OwnOrderDimensions/UpdatePosition`
    ///
    /// Parameter `position_id`: Id pozycji zamówienia własnego.
    ///
    /// Body `position_dimension`: Obiekt pola własnego pozycji zamówienia własnego do aktualizacji.
    pub async fn UpdatePosition<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: update_position::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/OwnOrderDimensions/UpdatePosition`
    ///
    /// Parameter `position_id`: Id pozycji zamówienia własnego.
    ///
    /// Body `position_dimension`: Obiekt pola własnego pozycji zamówienia własnego do aktualizacji.
    pub async fn UpdatePositionRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: update_position::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/OwnOrderDimensions/Aspects`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub async fn GetAspects<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_aspects::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/OwnOrderDimensions/Aspects`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub async fn GetAspectsRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_aspects::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/OwnOrderDimensions/Positions`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub async fn GetPositionsByOrderId(
        api: &ApiClient,
        order_id: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::PositionDimension>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "orderId", &order_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::PositionDimension>>(HttpMethod::Get, "/api/OwnOrderDimensions/Positions", query).await
    }

    /// `GET` `/api/OwnOrderDimensions/Positions`
    ///
    /// Parameter `order_id`: Id zamówienia własnego.
    pub async fn GetPositionsByOrderIdRaw(
        api: &ApiClient,
        order_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "orderId", &order_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrderDimensions/Positions", query)
            .await
    }

    /// `GET` `/api/OwnOrderDimensions/Positions`
    ///
    /// Parameter `order_number`: Numer dokumentu zamówienia własnego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych pozycji zamówienia własnego o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub async fn GetPositionsByOrderNumber(
        api: &ApiClient,
        order_number: String,
        buffer: bool,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::PositionDimension>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "orderNumber", &order_number)?;
        push_query_param(&mut query, "buffer", &buffer)?;
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::PositionDimension>>(HttpMethod::Get, "/api/OwnOrderDimensions/Positions", query).await
    }

    /// `GET` `/api/OwnOrderDimensions/Positions`
    ///
    /// Parameter `order_number`: Numer dokumentu zamówienia własnego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych pozycji zamówienia własnego o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub async fn GetPositionsByOrderNumberRaw(
        api: &ApiClient,
        order_number: String,
        buffer: bool,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "orderNumber", &order_number)?;
        push_query_param(&mut query, "buffer", &buffer)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrderDimensions/Positions", query)
            .await
    }

    /// `GET` `/api/OwnOrderDimensions/Positions`
    ///
    /// Parameter `position_id`: Id pozycji zamówienia własnego.
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
            "/api/OwnOrderDimensions/Positions",
            query,
        )
        .await
    }

    /// `GET` `/api/OwnOrderDimensions/Positions`
    ///
    /// Parameter `position_id`: Id pozycji zamówienia własnego.
    pub async fn GetPositionRaw(
        api: &ApiClient,
        position_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "positionId", &position_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/OwnOrderDimensions/Positions", query)
            .await
    }

    /// `GET` `/api/OwnOrderDimensions/Classification`
    pub async fn GetClassification(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::common::view_models::DimensionClassification>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DimensionClassification>>(HttpMethod::Get, "/api/OwnOrderDimensions/Classification", query).await
    }

    /// `GET` `/api/OwnOrderDimensions/Classification`
    pub async fn GetClassificationRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/OwnOrderDimensions/Classification",
            query,
        )
        .await
    }

    /// `GET` `/api/OwnOrderDimensions/PositionClassification`
    pub async fn GetPositionClassification(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::common::view_models::DimensionClassification>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DimensionClassification>>(HttpMethod::Get, "/api/OwnOrderDimensions/PositionClassification", query).await
    }

    /// `GET` `/api/OwnOrderDimensions/PositionClassification`
    pub async fn GetPositionClassificationRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/OwnOrderDimensions/PositionClassification",
            query,
        )
        .await
    }

    /// `PUT` `/api/OwnOrderDimensions/Aspects`
    ///
    /// Body `aspect_position`: Obiekt reprezentujący informacje o wymiarach pozycji zamówienia własnego.
    pub async fn UpdateAspect(
        api: &ApiClient,
        aspect_position: &crate::web_api::interface::common::view_models::aspects::AspectPositionEdit,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let query = vec![];
        api.request_with_body::<(), _>(
            HttpMethod::Put,
            "/api/OwnOrderDimensions/Aspects",
            query,
            aspect_position,
        )
        .await
    }

    /// `PUT` `/api/OwnOrderDimensions/Aspects`
    ///
    /// Body `aspect_position`: Obiekt reprezentujący informacje o wymiarach pozycji zamówienia własnego.
    pub async fn UpdateAspectRaw(
        api: &ApiClient,
        aspect_position: &crate::web_api::interface::common::view_models::aspects::AspectPositionEdit,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Put,
            "/api/OwnOrderDimensions/Aspects",
            query,
            aspect_position,
        )
        .await
    }
}
