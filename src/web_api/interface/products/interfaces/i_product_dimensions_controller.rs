use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `ProductDimensions`.
pub struct IProductDimensionsController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/ProductDimensions`
    ///
    /// Parameter `product_id`: Id towaru.
    pub struct ByProductId {
        /// Id towaru.
        pub product_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByProductId {
        type Output = Vec<crate::web_api::interface::common::view_models::Dimension>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Dimension>>(
                HttpMethod::Get,
                "/api/ProductDimensions",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/ProductDimensions", query)
                .await
        }
    }

    /// `GET` `/api/ProductDimensions`
    ///
    /// Parameter `product_code`: Kod towaru.
    pub struct ByProductCode {
        /// Kod towaru.
        pub product_code: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByProductCode {
        type Output = Vec<crate::web_api::interface::common::view_models::Dimension>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Dimension>>(
                HttpMethod::Get,
                "/api/ProductDimensions",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/ProductDimensions", query)
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

    /// `PUT` `/api/ProductDimensions/Update`
    ///
    /// Parameter `product_id`: Id towaru.
    ///
    /// Body `product_dimension`: Obiekt wymiaru towaru do aktualizacji.
    pub struct ByProductId<'a> {
        /// Id towaru.
        pub product_id: i32,
        /// Obiekt wymiaru towaru do aktualizacji.
        pub product_dimension: &'a crate::web_api::interface::common::view_models::Dimension,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByProductId<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/ProductDimensions/Update",
                query,
                self.product_dimension,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/ProductDimensions/Update",
                query,
                self.product_dimension,
            )
            .await
        }
    }

    /// `PUT` `/api/ProductDimensions/Update`
    ///
    /// Parameter `product_code`: Kod towaru.
    ///
    /// Body `product_dimension`: Obiekt wymiaru towaru do aktualizacji.
    pub struct ByProductCode<'a> {
        /// Kod towaru.
        pub product_code: String,
        /// Obiekt wymiaru towaru do aktualizacji.
        pub product_dimension: &'a crate::web_api::interface::common::view_models::Dimension,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByProductCode<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/ProductDimensions/Update",
                query,
                self.product_dimension,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/ProductDimensions/Update",
                query,
                self.product_dimension,
            )
            .await
        }
    }

    /// `PUT` `/api/ProductDimensions/UpdateList`
    ///
    /// Parameter `product_id`: Id towaru.
    ///
    /// Body `product_dimensions`: Lista obiektów wymiarów towaru do aktualizacji.
    pub struct ByProductIdUpdateList<'a> {
        /// Id towaru.
        pub product_id: i32,
        /// Lista obiektów wymiarów towaru do aktualizacji.
        pub product_dimensions: &'a Vec<crate::web_api::interface::common::view_models::Dimension>,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByProductIdUpdateList<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/ProductDimensions/UpdateList",
                query,
                self.product_dimensions,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/ProductDimensions/UpdateList",
                query,
                self.product_dimensions,
            )
            .await
        }
    }

    /// `PUT` `/api/ProductDimensions/UpdateList`
    ///
    /// Parameter `product_code`: Kod towaru.
    ///
    /// Body `product_dimensions`: Lista obiektów wymiarów towaru do aktualizacji.
    pub struct ByProductCodeUpdateList<'a> {
        /// Kod towaru.
        pub product_code: String,
        /// Lista obiektów wymiarów towaru do aktualizacji.
        pub product_dimensions: &'a Vec<crate::web_api::interface::common::view_models::Dimension>,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByProductCodeUpdateList<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/ProductDimensions/UpdateList",
                query,
                self.product_dimensions,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/ProductDimensions/UpdateList",
                query,
                self.product_dimensions,
            )
            .await
        }
    }
}

#[allow(non_snake_case)]
impl IProductDimensionsController {
    /// `GET` `/api/ProductDimensions`
    ///
    /// Parameter `product_id`: Id towaru.
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/ProductDimensions`
    ///
    /// Parameter `product_id`: Id towaru.
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/ProductDimensions/Update`
    ///
    /// Parameter `product_id`: Id towaru.
    ///
    /// Body `product_dimension`: Obiekt wymiaru towaru do aktualizacji.
    pub async fn Update<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: update::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/ProductDimensions/Update`
    ///
    /// Parameter `product_id`: Id towaru.
    ///
    /// Body `product_dimension`: Obiekt wymiaru towaru do aktualizacji.
    pub async fn UpdateRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: update::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/ProductDimensions/Classification`
    pub async fn GetClassification(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::common::view_models::DimensionClassification>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DimensionClassification>>(HttpMethod::Get, "/api/ProductDimensions/Classification", query).await
    }

    /// `GET` `/api/ProductDimensions/Classification`
    pub async fn GetClassificationRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/ProductDimensions/Classification",
            query,
        )
        .await
    }
}
