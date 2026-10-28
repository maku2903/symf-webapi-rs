use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `OrderDimensions`.
pub struct IOrderDimensionsController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/OrderDimensions`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub struct ByOrderId {
        /// Id zamówienia obcego.
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
                "/api/OrderDimensions",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderId", &self.order_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OrderDimensions", query)
                .await
        }
    }

    /// `GET` `/api/OrderDimensions`
    ///
    /// Parameter `order_number`: Numer dokumentu zamówienia obcego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych zamówienia obcego o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByOrderNumberBuffer {
        /// Numer dokumentu zamówienia obcego.
        pub order_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych zamówienia obcego o podanym numerze dokumentu, które nie znajduje się w buforze.
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
                "/api/OrderDimensions",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "orderNumber", &self.order_number)?;
            push_query_param(&mut query, "buffer", &self.buffer)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/OrderDimensions", query)
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

    /// `PUT` `/api/OrderDimensions/Update`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    ///
    /// Body `order_dimension`: Obiekt pola obcego zamówienia obcego do aktualizacji.
    pub struct ByOrderId<'a> {
        /// Id zamówienia obcego.
        pub order_id: i32,
        /// Obiekt pola obcego zamówienia obcego do aktualizacji.
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
                "/api/OrderDimensions/Update",
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
                "/api/OrderDimensions/Update",
                query,
                self.order_dimension,
            )
            .await
        }
    }

    /// `PUT` `/api/OrderDimensions/Update`
    ///
    /// Parameter `order_number`: Numer zamówienia obcego.
    ///
    /// Parameter `buffer`: Obiekt pola obcego zamówienia obcego do aktualizacji.
    ///
    /// Body `order_dimension`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie zaktualizowane pole własne zamówienia obcego o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByOrderNumberBuffer<'a> {
        /// Numer zamówienia obcego.
        pub order_number: String,
        /// Obiekt pola obcego zamówienia obcego do aktualizacji.
        pub buffer: bool,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostanie zaktualizowane pole własne zamówienia obcego o podanym numerze dokumentu, które nie znajduje się w buforze.
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
                "/api/OrderDimensions/Update",
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
                "/api/OrderDimensions/Update",
                query,
                self.order_dimension,
            )
            .await
        }
    }

    /// `PUT` `/api/OrderDimensions/UpdateList`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    ///
    /// Body `order_dimensions`: Lista obiektów pól własnych zamówienia obcego do aktualizacji.
    pub struct ByOrderIdUpdateList<'a> {
        /// Id zamówienia obcego.
        pub order_id: i32,
        /// Lista obiektów pól własnych zamówienia obcego do aktualizacji.
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
                "/api/OrderDimensions/UpdateList",
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
                "/api/OrderDimensions/UpdateList",
                query,
                self.order_dimensions,
            )
            .await
        }
    }

    /// `PUT` `/api/OrderDimensions/UpdateList`
    ///
    /// Parameter `order_number`: Numer zamówienia obcego.
    ///
    /// Parameter `buffer`: Lista obiektów pól własnych zamówienia obcego do aktualizacji.
    ///
    /// Body `order_dimensions`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostaną zaktualizowane pola własne zamówienia obcego o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByOrderNumberBufferUpdateList<'a> {
        /// Numer zamówienia obcego.
        pub order_number: String,
        /// Lista obiektów pól własnych zamówienia obcego do aktualizacji.
        pub buffer: bool,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostaną zaktualizowane pola własne zamówienia obcego o podanym numerze dokumentu, które nie znajduje się w buforze.
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
                "/api/OrderDimensions/UpdateList",
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
                "/api/OrderDimensions/UpdateList",
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

    /// `PUT` `/api/OrderDimensions/UpdatePosition`
    ///
    /// Parameter `position_id`: Id pozycji zamówienia obcego.
    ///
    /// Body `position_dimension`: Obiekt pola obcego pozycji zamówienia obcego do aktualizacji.
    pub struct ByPositionId<'a> {
        /// Id pozycji zamówienia obcego.
        pub position_id: i32,
        /// Obiekt pola obcego pozycji zamówienia obcego do aktualizacji.
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
                "/api/OrderDimensions/UpdatePosition",
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
                "/api/OrderDimensions/UpdatePosition",
                query,
                self.position_dimension,
            )
            .await
        }
    }

    /// `PUT` `/api/OrderDimensions/UpdatePositionList`
    ///
    /// Parameter `position_id`: Id pozycji zamówienia obcego.
    ///
    /// Body `position_dimensions`: Lista obiektów pól własnych pozycji zamówienia obcego do aktualizacji.
    pub struct ByPositionIdUpdatePositionList<'a> {
        /// Id pozycji zamówienia obcego.
        pub position_id: i32,
        /// Lista obiektów pól własnych pozycji zamówienia obcego do aktualizacji.
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
                "/api/OrderDimensions/UpdatePositionList",
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
                "/api/OrderDimensions/UpdatePositionList",
                query,
                self.position_dimensions,
            )
            .await
        }
    }

    /// `PUT` `/api/OrderDimensions/UpdateMultiPositionsList`
    ///
    /// Body `position_dimension`: Lista obiektów pól własnych wielu pozycji zamówienia obcego do aktualizacji.
    pub struct PositionDimension<'a> {
        /// Lista obiektów pól własnych wielu pozycji zamówienia obcego do aktualizacji.
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
                "/api/OrderDimensions/UpdateMultiPositionsList",
                query,
                self.position_dimension,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/OrderDimensions/UpdateMultiPositionsList",
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
    /// Parameter `order_id`: Id zamówienia obcego.
    pub struct ByOrderId {
        /// Id zamówienia obcego.
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
    /// Parameter `order_number`: Id zamówienia obcego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostaną pobrane wymiary pozycji zamówienia obcego o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub struct ByOrderNumberBuffer {
        /// Id zamówienia obcego.
        pub order_number: String,
        /// Czy dokument znajduje się w buforze.
        /// W przypadku, gdy parametr nie zostanie podany zostaną pobrane wymiary pozycji zamówienia obcego o podanym numerze dokumentu, które nie znajduje się w buforze.
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
impl IOrderDimensionsController {
    /// `GET` `/api/OrderDimensions`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/OrderDimensions`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/OrderDimensions/Update`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    ///
    /// Body `order_dimension`: Obiekt pola obcego zamówienia obcego do aktualizacji.
    pub async fn Update<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: update::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/OrderDimensions/Update`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    ///
    /// Body `order_dimension`: Obiekt pola obcego zamówienia obcego do aktualizacji.
    pub async fn UpdateRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: update::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/OrderDimensions/UpdatePosition`
    ///
    /// Parameter `position_id`: Id pozycji zamówienia obcego.
    ///
    /// Body `position_dimension`: Obiekt pola obcego pozycji zamówienia obcego do aktualizacji.
    pub async fn UpdatePosition<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: update_position::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/OrderDimensions/UpdatePosition`
    ///
    /// Parameter `position_id`: Id pozycji zamówienia obcego.
    ///
    /// Body `position_dimension`: Obiekt pola obcego pozycji zamówienia obcego do aktualizacji.
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
    /// Parameter `order_id`: Id zamówienia obcego.
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
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn GetAspectsRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_aspects::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/OrderDimensions/Positions`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn GetPositionsByOrderId(
        api: &ApiClient,
        order_id: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::PositionDimension>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "orderId", &order_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::PositionDimension>>(HttpMethod::Get, "/api/OrderDimensions/Positions", query).await
    }

    /// `GET` `/api/OrderDimensions/Positions`
    ///
    /// Parameter `order_id`: Id zamówienia obcego.
    pub async fn GetPositionsByOrderIdRaw(
        api: &ApiClient,
        order_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "orderId", &order_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/OrderDimensions/Positions", query)
            .await
    }

    /// `GET` `/api/OrderDimensions/Positions`
    ///
    /// Parameter `order_number`: Numer dokumentu zamówienia obcego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych pozycji zamówienia obcego o podanym numerze dokumentu, które nie znajduje się w buforze.
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
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::PositionDimension>>(HttpMethod::Get, "/api/OrderDimensions/Positions", query).await
    }

    /// `GET` `/api/OrderDimensions/Positions`
    ///
    /// Parameter `order_number`: Numer dokumentu zamówienia obcego.
    ///
    /// Parameter `buffer`: Czy dokument znajduje się w buforze. W przypadku, gdy parametr nie zostanie podany zostanie pobrana lista pól własnych pozycji zamówienia obcego o podanym numerze dokumentu, które nie znajduje się w buforze.
    pub async fn GetPositionsByOrderNumberRaw(
        api: &ApiClient,
        order_number: String,
        buffer: bool,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "orderNumber", &order_number)?;
        push_query_param(&mut query, "buffer", &buffer)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/OrderDimensions/Positions", query)
            .await
    }

    /// `GET` `/api/OrderDimensions/Positions`
    ///
    /// Parameter `position_id`: Id pozycji zamówienia obcego.
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
            "/api/OrderDimensions/Positions",
            query,
        )
        .await
    }

    /// `GET` `/api/OrderDimensions/Positions`
    ///
    /// Parameter `position_id`: Id pozycji zamówienia obcego.
    pub async fn GetPositionRaw(
        api: &ApiClient,
        position_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "positionId", &position_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/OrderDimensions/Positions", query)
            .await
    }

    /// `GET` `/api/OrderDimensions/Classification`
    pub async fn GetClassification(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::common::view_models::DimensionClassification>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DimensionClassification>>(HttpMethod::Get, "/api/OrderDimensions/Classification", query).await
    }

    /// `GET` `/api/OrderDimensions/Classification`
    pub async fn GetClassificationRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/OrderDimensions/Classification",
            query,
        )
        .await
    }

    /// `GET` `/api/OrderDimensions/PositionClassification`
    pub async fn GetPositionClassification(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::common::view_models::DimensionClassification>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DimensionClassification>>(HttpMethod::Get, "/api/OrderDimensions/PositionClassification", query).await
    }

    /// `GET` `/api/OrderDimensions/PositionClassification`
    pub async fn GetPositionClassificationRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/OrderDimensions/PositionClassification",
            query,
        )
        .await
    }

    /// `PUT` `/api/OwnOrderDimensions/Aspects`
    ///
    /// Body `aspect_position`: Obiekt reprezentujący informacje o wymiarach pozycji zamówienia obcego.
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
    /// Body `aspect_position`: Obiekt reprezentujący informacje o wymiarach pozycji zamówienia obcego.
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
