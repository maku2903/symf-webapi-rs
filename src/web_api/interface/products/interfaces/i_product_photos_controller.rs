use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `ProductPhotos`.
pub struct IProductPhotosController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/ProductPhotos`
    ///
    /// Parameter `product_id`: Id towaru
    pub struct ByProductId {
        /// Id towaru
        pub product_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByProductId {
        type Output =
            Vec<crate::web_api::interface::products::view_models::ProductPhotoListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::products::view_models::ProductPhotoListElement>>(HttpMethod::Get, "/api/ProductPhotos", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/ProductPhotos", query)
                .await
        }
    }

    /// `GET` `/api/ProductPhotos`
    ///
    /// Parameter `product_code`: Kod towaru
    pub struct ByProductCode {
        /// Kod towaru
        pub product_code: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByProductCode {
        type Output =
            Vec<crate::web_api::interface::products::view_models::ProductPhotoListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body::<Vec<crate::web_api::interface::products::view_models::ProductPhotoListElement>>(HttpMethod::Get, "/api/ProductPhotos", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/ProductPhotos", query)
                .await
        }
    }

    /// `GET` `/api/ProductPhotos`
    ///
    /// Parameter `photo_id`: Id zdjęcia.
    ///
    /// Parameter `quality`: Jakość zdjęcia.
    pub struct ByPhotoIdQuality {
        /// Id zdjęcia.
        pub photo_id: i32,
        /// Jakość zdjęcia.
        pub quality: crate::web_api::interface::enums::enumPhotoQuality,
    }

    #[async_trait::async_trait]
    impl Overload for ByPhotoIdQuality {
        type Output = crate::web_api::interface::products::view_models::ProductPhoto;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "photoId", &self.photo_id)?;
            push_query_param(&mut query, "quality", &self.quality)?;
            api.request_no_body::<crate::web_api::interface::products::view_models::ProductPhoto>(
                HttpMethod::Get,
                "/api/ProductPhotos",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "photoId", &self.photo_id)?;
            push_query_param(&mut query, "quality", &self.quality)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/ProductPhotos", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IProductPhotosController {
    /// `GET` `/api/ProductPhotos`
    ///
    /// Parameter `product_id`: Id towaru
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/ProductPhotos`
    ///
    /// Parameter `product_id`: Id towaru
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
