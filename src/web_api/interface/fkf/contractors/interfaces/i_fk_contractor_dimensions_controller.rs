use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `FKContractorDimensions`.
pub struct IFKContractorDimensionsController;

pub mod update_by_id {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/FKContractorDimensions/Update`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Body `contractor_dimension`: Obiekt wymiaru kontrahenta do aktualizacji.
    pub struct ByContractorId<'a> {
        /// Id kontrahenta.
        pub contractor_id: i32,
        /// Obiekt wymiaru kontrahenta do aktualizacji.
        pub contractor_dimension:
            &'a crate::web_api::interface::fkf::common::view_models::Dimension,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByContractorId<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/FKContractorDimensions/Update",
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
                "/api/FKContractorDimensions/Update",
                query,
                self.contractor_dimension,
            )
            .await
        }
    }

    /// `PUT` `/api/FKContractorDimensions/UpdateList`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Body `contractor_dimensions`: Lista obiektów wymiarów kontrahenta do aktualizacji.
    pub struct ByContractorIdUpdateList<'a> {
        /// Id kontrahenta.
        pub contractor_id: i32,
        /// Lista obiektów wymiarów kontrahenta do aktualizacji.
        pub contractor_dimensions:
            &'a Vec<crate::web_api::interface::fkf::common::view_models::Dimension>,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByContractorIdUpdateList<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/FKContractorDimensions/UpdateList",
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
                "/api/FKContractorDimensions/UpdateList",
                query,
                self.contractor_dimensions,
            )
            .await
        }
    }
}

pub mod update_by_position {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/FKContractorDimensions/Update`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Body `contractor_dimension`: Obiekt wymiaru kontrahenta do aktualizacji.
    pub struct ByContractorPosition<'a> {
        /// Pozycja kontrahenta.
        pub contractor_position: i32,
        /// Obiekt wymiaru kontrahenta do aktualizacji.
        pub contractor_dimension:
            &'a crate::web_api::interface::fkf::common::view_models::Dimension,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByContractorPosition<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorPosition", &self.contractor_position)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/FKContractorDimensions/Update",
                query,
                self.contractor_dimension,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorPosition", &self.contractor_position)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/FKContractorDimensions/Update",
                query,
                self.contractor_dimension,
            )
            .await
        }
    }

    /// `PUT` `/api/FKContractorDimensions/UpdateList`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Body `contractor_dimensions`: Lista obiektów wymiarów kontrahenta do aktualizacji.
    pub struct ByContractorPositionUpdateList<'a> {
        /// Pozycja kontrahenta.
        pub contractor_position: i32,
        /// Lista obiektów wymiarów kontrahenta do aktualizacji.
        pub contractor_dimensions:
            &'a Vec<crate::web_api::interface::fkf::common::view_models::Dimension>,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByContractorPositionUpdateList<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorPosition", &self.contractor_position)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/FKContractorDimensions/UpdateList",
                query,
                self.contractor_dimensions,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorPosition", &self.contractor_position)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/FKContractorDimensions/UpdateList",
                query,
                self.contractor_dimensions,
            )
            .await
        }
    }
}

pub mod update_by_code {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/FKContractorDimensions/Update`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    ///
    /// Body `contractor_dimension`: Obiekt wymiaru kontrahenta do aktualizacji.
    pub struct ByContractorCode<'a> {
        /// Kod kontrahenta.
        pub contractor_code: String,
        /// Obiekt wymiaru kontrahenta do aktualizacji.
        pub contractor_dimension:
            &'a crate::web_api::interface::fkf::common::view_models::Dimension,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByContractorCode<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/FKContractorDimensions/Update",
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
                "/api/FKContractorDimensions/Update",
                query,
                self.contractor_dimension,
            )
            .await
        }
    }

    /// `PUT` `/api/FKContractorDimensions/UpdateList`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    ///
    /// Body `contractor_dimensions`: Lista obiektów wymiarów kontrahenta do aktualizacji.
    pub struct ByContractorCodeUpdateList<'a> {
        /// Kod kontrahenta.
        pub contractor_code: String,
        /// Lista obiektów wymiarów kontrahenta do aktualizacji.
        pub contractor_dimensions:
            &'a Vec<crate::web_api::interface::fkf::common::view_models::Dimension>,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByContractorCodeUpdateList<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/FKContractorDimensions/UpdateList",
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
                "/api/FKContractorDimensions/UpdateList",
                query,
                self.contractor_dimensions,
            )
            .await
        }
    }
}

#[allow(non_snake_case)]
impl IFKContractorDimensionsController {
    /// `PUT` `/api/FKContractorDimensions/Update`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Body `contractor_dimension`: Obiekt wymiaru kontrahenta do aktualizacji.
    pub async fn UpdateById<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: update_by_id::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/FKContractorDimensions/Update`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Body `contractor_dimension`: Obiekt wymiaru kontrahenta do aktualizacji.
    pub async fn UpdateByIdRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: update_by_id::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/FKContractorDimensions/Update`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Body `contractor_dimension`: Obiekt wymiaru kontrahenta do aktualizacji.
    pub async fn UpdateByPosition<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: update_by_position::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/FKContractorDimensions/Update`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Body `contractor_dimension`: Obiekt wymiaru kontrahenta do aktualizacji.
    pub async fn UpdateByPositionRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: update_by_position::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PUT` `/api/FKContractorDimensions/Update`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    ///
    /// Body `contractor_dimension`: Obiekt wymiaru kontrahenta do aktualizacji.
    pub async fn UpdateByCode<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: update_by_code::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/FKContractorDimensions/Update`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    ///
    /// Body `contractor_dimension`: Obiekt wymiaru kontrahenta do aktualizacji.
    pub async fn UpdateByCodeRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: update_by_code::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/FKContractorDimensions`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    pub async fn GetById(
        api: &ApiClient,
        contractor_id: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::common::view_models::Dimension>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorId", &contractor_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::Dimension>>(
            HttpMethod::Get,
            "/api/FKContractorDimensions",
            query,
        )
        .await
    }

    /// `GET` `/api/FKContractorDimensions`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    pub async fn GetByIdRaw(
        api: &ApiClient,
        contractor_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorId", &contractor_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKContractorDimensions", query)
            .await
    }

    /// `GET` `/api/FKContractorDimensions`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    pub async fn GetByPosition(
        api: &ApiClient,
        contractor_position: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::common::view_models::Dimension>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::Dimension>>(
            HttpMethod::Get,
            "/api/FKContractorDimensions",
            query,
        )
        .await
    }

    /// `GET` `/api/FKContractorDimensions`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    pub async fn GetByPositionRaw(
        api: &ApiClient,
        contractor_position: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKContractorDimensions", query)
            .await
    }

    /// `GET` `/api/FKContractorDimensions`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    pub async fn GetByCode(
        api: &ApiClient,
        contractor_code: String,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::common::view_models::Dimension>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorCode", &contractor_code)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::Dimension>>(
            HttpMethod::Get,
            "/api/FKContractorDimensions",
            query,
        )
        .await
    }

    /// `GET` `/api/FKContractorDimensions`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    pub async fn GetByCodeRaw(
        api: &ApiClient,
        contractor_code: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorCode", &contractor_code)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKContractorDimensions", query)
            .await
    }

    /// `GET` `/api/FKContractorDimensions/Classification`
    pub async fn GetClassification(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::common::view_models::DimensionClassification>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::DimensionClassification>>(HttpMethod::Get, "/api/FKContractorDimensions/Classification", query).await
    }

    /// `GET` `/api/FKContractorDimensions/Classification`
    pub async fn GetClassificationRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKContractorDimensions/Classification",
            query,
        )
        .await
    }
}
