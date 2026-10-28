use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `ContractorDimensions`.
pub struct IContractorDimensionsController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/ContractorDimensions`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    pub struct ByContractorId {
        /// Id kontrahenta.
        pub contractor_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorId {
        type Output = Vec<crate::web_api::interface::common::view_models::Dimension>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Dimension>>(
                HttpMethod::Get,
                "/api/ContractorDimensions",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/ContractorDimensions", query)
                .await
        }
    }

    /// `GET` `/api/ContractorDimensions`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    pub struct ByContractorCode {
        /// Kod kontrahenta.
        pub contractor_code: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorCode {
        type Output = Vec<crate::web_api::interface::common::view_models::Dimension>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Dimension>>(
                HttpMethod::Get,
                "/api/ContractorDimensions",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/ContractorDimensions", query)
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

    /// `PUT` `/api/ContractorDimensions/Update`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Body `contractor_dimension`: Obiekt wymiaru kontrahenta do aktualizacji.
    pub struct ByContractorId<'a> {
        /// Id kontrahenta.
        pub contractor_id: i32,
        /// Obiekt wymiaru kontrahenta do aktualizacji.
        pub contractor_dimension: &'a crate::web_api::interface::common::view_models::Dimension,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByContractorId<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/ContractorDimensions/Update",
                query,
                self.contractor_dimension,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/ContractorDimensions/Update",
                query,
                self.contractor_dimension,
            )
            .await
        }
    }

    /// `PUT` `/api/ContractorDimensions/Update`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    ///
    /// Body `contractor_dimension`: Obiekt wymiaru kontrahenta do aktualizacji.
    pub struct ByContractorCode<'a> {
        /// Kod kontrahenta.
        pub contractor_code: String,
        /// Obiekt wymiaru kontrahenta do aktualizacji.
        pub contractor_dimension: &'a crate::web_api::interface::common::view_models::Dimension,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByContractorCode<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/ContractorDimensions/Update",
                query,
                self.contractor_dimension,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/ContractorDimensions/Update",
                query,
                self.contractor_dimension,
            )
            .await
        }
    }

    /// `PUT` `/api/ContractorDimensions/UpdateList`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Body `contractor_dimensions`: Lista obiektów wymiarów kontrahenta do aktualizacji.
    pub struct ByContractorIdUpdateList<'a> {
        /// Id kontrahenta.
        pub contractor_id: i32,
        /// Lista obiektów wymiarów kontrahenta do aktualizacji.
        pub contractor_dimensions:
            &'a Vec<crate::web_api::interface::common::view_models::Dimension>,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByContractorIdUpdateList<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/ContractorDimensions/UpdateList",
                query,
                self.contractor_dimensions,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/ContractorDimensions/UpdateList",
                query,
                self.contractor_dimensions,
            )
            .await
        }
    }

    /// `PUT` `/api/ContractorDimensions/UpdateList`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    ///
    /// Body `contractor_dimensions`: Lista obiektów wymiarów kontrahenta do aktualizacji.
    pub struct ByContractorCodeUpdateList<'a> {
        /// Kod kontrahenta.
        pub contractor_code: String,
        /// Lista obiektów wymiarów kontrahenta do aktualizacji.
        pub contractor_dimensions:
            &'a Vec<crate::web_api::interface::common::view_models::Dimension>,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByContractorCodeUpdateList<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/ContractorDimensions/UpdateList",
                query,
                self.contractor_dimensions,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/ContractorDimensions/UpdateList",
                query,
                self.contractor_dimensions,
            )
            .await
        }
    }
}

#[allow(non_snake_case)]
impl IContractorDimensionsController {
    /// `GET` `/api/ContractorDimensions`
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

    /// `GET` `/api/ContractorDimensions`
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

    /// `PUT` `/api/ContractorDimensions/Update`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Body `contractor_dimension`: Obiekt wymiaru kontrahenta do aktualizacji.
    pub async fn Update<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: update::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/ContractorDimensions/Update`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Body `contractor_dimension`: Obiekt wymiaru kontrahenta do aktualizacji.
    pub async fn UpdateRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: update::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/ContractorDimensions/Classification`
    pub async fn GetClassification(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::common::view_models::DimensionClassification>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DimensionClassification>>(HttpMethod::Get, "/api/ContractorDimensions/Classification", query).await
    }

    /// `GET` `/api/ContractorDimensions/Classification`
    pub async fn GetClassificationRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/ContractorDimensions/Classification",
            query,
        )
        .await
    }
}
