use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `FKCurrenciesTables`.
pub struct IFKCurrenciesTablesController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/FKCurrenciesTables`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output = Vec<crate::web_api::interface::fkf::common::view_models::CurrencyTable>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::CurrencyTable>>(HttpMethod::Get, "/api/FKCurrenciesTables", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(HttpMethod::Get, "/api/FKCurrenciesTables", query)
                .await
        }
    }

    /// `GET` `/api/FKCurrenciesTables`
    ///
    /// Parameter `currency_id`: Id waluty.
    ///
    /// Parameter `date`: Z dnia.
    pub struct ByCurrencyIdDate {
        /// Id waluty.
        pub currency_id: i32,
        /// Z dnia.
        pub date: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByCurrencyIdDate {
        type Output = Vec<crate::web_api::interface::fkf::common::view_models::CurrencyTable>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "currencyId", &self.currency_id)?;
            push_query_param(&mut query, "date", &self.date)?;
            api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::CurrencyTable>>(HttpMethod::Get, "/api/FKCurrenciesTables", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "currencyId", &self.currency_id)?;
            push_query_param(&mut query, "date", &self.date)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/FKCurrenciesTables", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IFKCurrenciesTablesController {
    /// `GET` `/api/FKCurrenciesTables`
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/FKCurrenciesTables`
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/FKCurrenciesTables/Filter`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie tabele kursów do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie tabele kursów od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetList(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::common::view_models::CurrencyTable>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::CurrencyTable>>(HttpMethod::Get, "/api/FKCurrenciesTables/Filter", query).await
    }

    /// `GET` `/api/FKCurrenciesTables/Filter`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie tabele kursów do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie tabele kursów od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListRaw(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKCurrenciesTables/Filter", query)
            .await
    }

    /// `GET` `/api/FKCurrenciesTables/Filter`
    ///
    /// Parameter `currency_id`: Id waluty.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie tabele kursów do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie tabele kursów od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByCurrency(
        api: &ApiClient,
        currency_id: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::common::view_models::CurrencyTable>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "currencyId", &currency_id)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::CurrencyTable>>(HttpMethod::Get, "/api/FKCurrenciesTables/Filter", query).await
    }

    /// `GET` `/api/FKCurrenciesTables/Filter`
    ///
    /// Parameter `currency_id`: Id waluty.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie tabele kursów do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie tabele kursów od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetListByCurrencyRaw(
        api: &ApiClient,
        currency_id: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "currencyId", &currency_id)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKCurrenciesTables/Filter", query)
            .await
    }
}
