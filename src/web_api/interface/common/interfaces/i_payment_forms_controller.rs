use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `PaymentForms`.
pub struct IPaymentFormsController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/PaymentForms`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output = Vec<crate::web_api::interface::common::view_models::PaymentForm>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<Vec<crate::web_api::interface::common::view_models::PaymentForm>>(
                HttpMethod::Get,
                "/api/PaymentForms",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(HttpMethod::Get, "/api/PaymentForms", query)
                .await
        }
    }

    /// `GET` `/api/PaymentForms`
    ///
    /// Parameter `registry_id`: Id rejestru płatności.
    pub struct ByRegistryId {
        /// Id rejestru płatności.
        pub registry_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByRegistryId {
        type Output = Vec<crate::web_api::interface::common::view_models::PaymentForm>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "registryId", &self.registry_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::common::view_models::PaymentForm>>(
                HttpMethod::Get,
                "/api/PaymentForms",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "registryId", &self.registry_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/PaymentForms", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IPaymentFormsController {
    /// `GET` `/api/PaymentForms`
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/PaymentForms`
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
