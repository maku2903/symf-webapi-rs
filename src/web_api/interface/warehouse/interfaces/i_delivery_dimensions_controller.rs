use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `DeliveryDimensions`.
pub struct IDeliveryDimensionsController;

pub mod update {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/DeliveryDimensions/Update`
    ///
    /// Parameter `delivery_id`: Id dostawy.
    ///
    /// Body `delivery_dimension`: Obiekt pola własnego dostawy do aktualizacji.
    pub struct ByDeliveryId<'a> {
        /// Id dostawy.
        pub delivery_id: i32,
        /// Obiekt pola własnego dostawy do aktualizacji.
        pub delivery_dimension: &'a crate::web_api::interface::common::view_models::Dimension,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByDeliveryId<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "deliveryId", &self.delivery_id)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/DeliveryDimensions/Update",
                query,
                self.delivery_dimension,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "deliveryId", &self.delivery_id)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/DeliveryDimensions/Update",
                query,
                self.delivery_dimension,
            )
            .await
        }
    }

    /// `PUT` `/api/DeliveryDimensions/UpdateList`
    ///
    /// Parameter `delivery_id`: Id dostawy.
    ///
    /// Body `delivery_dimensions`: Lista obiektów pól własnych dostawy do aktualizacji.
    pub struct ByDeliveryIdUpdateList<'a> {
        /// Id dostawy.
        pub delivery_id: i32,
        /// Lista obiektów pól własnych dostawy do aktualizacji.
        pub delivery_dimensions: &'a Vec<crate::web_api::interface::common::view_models::Dimension>,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByDeliveryIdUpdateList<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "deliveryId", &self.delivery_id)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/DeliveryDimensions/UpdateList",
                query,
                self.delivery_dimensions,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "deliveryId", &self.delivery_id)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/DeliveryDimensions/UpdateList",
                query,
                self.delivery_dimensions,
            )
            .await
        }
    }
}

#[allow(non_snake_case)]
impl IDeliveryDimensionsController {
    /// `PUT` `/api/DeliveryDimensions/Update`
    ///
    /// Parameter `delivery_id`: Id dostawy.
    ///
    /// Body `delivery_dimension`: Obiekt pola własnego dostawy do aktualizacji.
    pub async fn Update<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: update::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/DeliveryDimensions/Update`
    ///
    /// Parameter `delivery_id`: Id dostawy.
    ///
    /// Body `delivery_dimension`: Obiekt pola własnego dostawy do aktualizacji.
    pub async fn UpdateRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: update::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/DeliveryDimensions`
    ///
    /// Parameter `delivery_id`: Id dostawy.
    pub async fn Get(
        api: &ApiClient,
        delivery_id: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Dimension>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "deliveryId", &delivery_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Dimension>>(
            HttpMethod::Get,
            "/api/DeliveryDimensions",
            query,
        )
        .await
    }

    /// `GET` `/api/DeliveryDimensions`
    ///
    /// Parameter `delivery_id`: Id dostawy.
    pub async fn GetRaw(
        api: &ApiClient,
        delivery_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "deliveryId", &delivery_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/DeliveryDimensions", query)
            .await
    }

    /// `GET` `/api/DeliveryDimensions/Classification`
    pub async fn GetClassification(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::common::view_models::DimensionClassification>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DimensionClassification>>(HttpMethod::Get, "/api/DeliveryDimensions/Classification", query).await
    }

    /// `GET` `/api/DeliveryDimensions/Classification`
    pub async fn GetClassificationRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/DeliveryDimensions/Classification",
            query,
        )
        .await
    }
}
