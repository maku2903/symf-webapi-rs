use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `FKYears`.
pub struct IFKYearsController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/FKYears`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output = Vec<crate::web_api::interface::fkf::common::view_models::Year>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::Year>>(
                HttpMethod::Get,
                "/api/FKYears",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(HttpMethod::Get, "/api/FKYears", query)
                .await
        }
    }

    /// `GET` `/api/FKYears`
    ///
    /// Parameter `date`: Inferred from route query template.
    pub struct ByDate {
        /// Inferred from route query template.
        pub date: chrono::NaiveDateTime,
    }

    #[async_trait::async_trait]
    impl Overload for ByDate {
        type Output = crate::web_api::interface::fkf::common::view_models::Year;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "date", &self.date)?;
            api.request_no_body::<crate::web_api::interface::fkf::common::view_models::Year>(
                HttpMethod::Get,
                "/api/FKYears",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "date", &self.date)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/FKYears", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IFKYearsController {
    /// `GET` `/api/FKYears`
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/FKYears`
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
