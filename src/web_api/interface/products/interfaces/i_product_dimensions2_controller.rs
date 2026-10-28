use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `ProductDimensions2`.
pub struct IProductDimensions2Controller;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/v2/ProductDimensions`
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
                "/api/v2/ProductDimensions",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/v2/ProductDimensions", query)
                .await
        }
    }

    /// `GET` `/api/v2/ProductDimensions`
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
                "/api/v2/ProductDimensions",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/v2/ProductDimensions", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IProductDimensions2Controller {
    /// `GET` `/api/v2/ProductDimensions`
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

    /// `GET` `/api/v2/ProductDimensions`
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
}
