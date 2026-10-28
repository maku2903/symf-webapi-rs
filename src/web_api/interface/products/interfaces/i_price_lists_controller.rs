use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `PriceLists`.
pub struct IPriceListsController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/PriceLists`
    ///
    /// Parameter `id`: Id cennika.
    pub struct ById {
        /// Id cennika.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = crate::web_api::interface::products::view_models::price_lists::PriceList;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<crate::web_api::interface::products::view_models::price_lists::PriceList>(HttpMethod::Get, "/api/PriceLists", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/PriceLists", query)
                .await
        }
    }

    /// `GET` `/api/PriceLists`
    ///
    /// Parameter `code`: Kod cennika.
    pub struct ByCode {
        /// Kod cennika.
        pub code: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByCode {
        type Output = crate::web_api::interface::products::view_models::price_lists::PriceList;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "code", &self.code)?;
            api.request_no_body::<crate::web_api::interface::products::view_models::price_lists::PriceList>(HttpMethod::Get, "/api/PriceLists", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "code", &self.code)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/PriceLists", query)
                .await
        }
    }

    /// `GET` `/api/PriceLists`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output = Vec<
            crate::web_api::interface::products::view_models::price_lists::PriceListListElement,
        >;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<Vec<
                crate::web_api::interface::products::view_models::price_lists::PriceListListElement,
            >>(HttpMethod::Get, "/api/PriceLists", query)
                .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(HttpMethod::Get, "/api/PriceLists", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IPriceListsController {
    /// `GET` `/api/PriceLists`
    ///
    /// Parameter `id`: Id cennika.
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/PriceLists`
    ///
    /// Parameter `id`: Id cennika.
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/PriceLists/IncrementalSync`
    ///
    /// Parameter `date_from`: Inferred from route query template.
    pub async fn IncrementalSync(
        api: &ApiClient,
        date_from: chrono::NaiveDateTime,
    ) -> Result<
        ResponseEnvelope<
            Vec<
                crate::web_api::interface::common::view_models::IncrementalSyncListElement<
                    serde_json::Value,
                >,
            >,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        api.request_no_body::<Vec<
            crate::web_api::interface::common::view_models::IncrementalSyncListElement<
                serde_json::Value,
            >,
        >>(HttpMethod::Get, "/api/PriceLists/IncrementalSync", query)
            .await
    }

    /// `GET` `/api/PriceLists/IncrementalSync`
    ///
    /// Parameter `date_from`: Inferred from route query template.
    pub async fn IncrementalSyncRaw(
        api: &ApiClient,
        date_from: chrono::NaiveDateTime,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/PriceLists/IncrementalSync", query)
            .await
    }
}
