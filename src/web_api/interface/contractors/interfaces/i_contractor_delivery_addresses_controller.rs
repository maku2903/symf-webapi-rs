use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `ContractorDeliveryAddresses`.
pub struct IContractorDeliveryAddressesController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/ContractorDeliveryAddresses`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    pub struct ByContractorId {
        /// Id kontrahenta.
        pub contractor_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorId {
        type Output =
            Vec<crate::web_api::interface::contractors::view_models::ContractorDeliveryAddress>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::contractors::view_models::ContractorDeliveryAddress>>(HttpMethod::Get, "/api/ContractorDeliveryAddresses", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/ContractorDeliveryAddresses", query)
                .await
        }
    }

    /// `GET` `/api/ContractorDeliveryAddresses`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    pub struct ByContractorCode {
        /// Kod kontrahenta.
        pub contractor_code: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorCode {
        type Output =
            Vec<crate::web_api::interface::contractors::view_models::ContractorDeliveryAddress>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_no_body::<Vec<crate::web_api::interface::contractors::view_models::ContractorDeliveryAddress>>(HttpMethod::Get, "/api/ContractorDeliveryAddresses", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/ContractorDeliveryAddresses", query)
                .await
        }
    }
}

pub mod add_new {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `POST` `/api/ContractorDeliveryAddresses/Create`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Body `contractor_delivery_address`: Obiekt adresu dostawy kontrahenta do utworzenia.
    pub struct ByContractorId<'a> {
        /// Id kontrahenta.
        pub contractor_id: i32,
        /// Obiekt adresu dostawy kontrahenta do utworzenia.
        pub contractor_delivery_address:
            &'a crate::web_api::interface::contractors::view_models::ContractorDeliveryAddress,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByContractorId<'a> {
        type Output =
            Vec<crate::web_api::interface::contractors::view_models::ContractorDeliveryAddress>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_with_body::<Vec<crate::web_api::interface::contractors::view_models::ContractorDeliveryAddress>, _>(HttpMethod::Post, "/api/ContractorDeliveryAddresses/Create", query, self.contractor_delivery_address).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_with_body_raw(
                HttpMethod::Post,
                "/api/ContractorDeliveryAddresses/Create",
                query,
                self.contractor_delivery_address,
            )
            .await
        }
    }

    /// `POST` `/api/ContractorDeliveryAddresses/Create`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    ///
    /// Body `contractor_delivery_address`: Obiekt adresu dostawy kontrahenta do utworzenia.
    pub struct ByContractorCode<'a> {
        /// Kod kontrahenta.
        pub contractor_code: String,
        /// Obiekt adresu dostawy kontrahenta do utworzenia.
        pub contractor_delivery_address:
            &'a crate::web_api::interface::contractors::view_models::ContractorDeliveryAddress,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByContractorCode<'a> {
        type Output =
            Vec<crate::web_api::interface::contractors::view_models::ContractorDeliveryAddress>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_with_body::<Vec<crate::web_api::interface::contractors::view_models::ContractorDeliveryAddress>, _>(HttpMethod::Post, "/api/ContractorDeliveryAddresses/Create", query, self.contractor_delivery_address).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_with_body_raw(
                HttpMethod::Post,
                "/api/ContractorDeliveryAddresses/Create",
                query,
                self.contractor_delivery_address,
            )
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

    /// `PUT` `/api/ContractorDeliveryAddresses/Update`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Body `contractor_delivery_address`: Obiekt adresu dostawy kontrahenta do aktualizacji.
    pub struct ByContractorId<'a> {
        /// Id kontrahenta.
        pub contractor_id: i32,
        /// Obiekt adresu dostawy kontrahenta do aktualizacji.
        pub contractor_delivery_address:
            &'a crate::web_api::interface::contractors::view_models::ContractorDeliveryAddress,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByContractorId<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/ContractorDeliveryAddresses/Update",
                query,
                self.contractor_delivery_address,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/ContractorDeliveryAddresses/Update",
                query,
                self.contractor_delivery_address,
            )
            .await
        }
    }

    /// `PUT` `/api/ContractorDeliveryAddresses/Update`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    ///
    /// Body `contractor_delivery_address`: Obiekt adresu dostawy kontrahenta do aktualizacji.
    pub struct ByContractorCode<'a> {
        /// Kod kontrahenta.
        pub contractor_code: String,
        /// Obiekt adresu dostawy kontrahenta do aktualizacji.
        pub contractor_delivery_address:
            &'a crate::web_api::interface::contractors::view_models::ContractorDeliveryAddress,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByContractorCode<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/ContractorDeliveryAddresses/Update",
                query,
                self.contractor_delivery_address,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/ContractorDeliveryAddresses/Update",
                query,
                self.contractor_delivery_address,
            )
            .await
        }
    }
}

#[allow(non_snake_case)]
impl IContractorDeliveryAddressesController {
    /// `GET` `/api/ContractorDeliveryAddresses`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/ContractorDeliveryAddresses`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `POST` `/api/ContractorDeliveryAddresses/Create`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Body `contractor_delivery_address`: Obiekt adresu dostawy kontrahenta do utworzenia.
    pub async fn AddNew<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: add_new::Overload,
    {
        overload.call(api).await
    }

    /// `POST` `/api/ContractorDeliveryAddresses/Create`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Body `contractor_delivery_address`: Obiekt adresu dostawy kontrahenta do utworzenia.
    pub async fn AddNewRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: add_new::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/ContractorDeliveryAddresses/Update`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Body `contractor_delivery_address`: Obiekt adresu dostawy kontrahenta do aktualizacji.
    pub async fn Update<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: update::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/ContractorDeliveryAddresses/Update`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Body `contractor_delivery_address`: Obiekt adresu dostawy kontrahenta do aktualizacji.
    pub async fn UpdateRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: update::Overload,
    {
        overload.call_raw(api).await
    }
}
