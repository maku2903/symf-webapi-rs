use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `FKDictionaries`.
pub struct IFKDictionariesController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/FKDictionaries`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output =
            Vec<crate::web_api::interface::dictionaries::view_models::DictionaryListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<Vec<crate::web_api::interface::dictionaries::view_models::DictionaryListElement>>(HttpMethod::Get, "/api/FKDictionaries", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(HttpMethod::Get, "/api/FKDictionaries", query)
                .await
        }
    }

    /// `GET` `/api/FKDictionaries`
    ///
    /// Parameter `id`: Inferred from route query template.
    pub struct ById {
        /// Inferred from route query template.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = crate::web_api::interface::dictionaries::view_models::Dictionary;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<crate::web_api::interface::dictionaries::view_models::Dictionary>(
                HttpMethod::Get,
                "/api/FKDictionaries",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/FKDictionaries", query)
                .await
        }
    }
}

pub mod set_element_position {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PATCH` `/api/FKDictionaries/SetElementPosition`
    ///
    /// Parameter `id`: Inferred from route query template.
    pub struct ById {
        /// Inferred from route query template.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<()>(
                HttpMethod::Patch,
                "/api/FKDictionaries/SetElementPosition",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(
                HttpMethod::Patch,
                "/api/FKDictionaries/SetElementPosition",
                query,
            )
            .await
        }
    }

    /// `PATCH` `/api/FKDictionaries/SetElementPosition`
    ///
    /// Parameter `id`: Id elementu słownika.
    ///
    /// Parameter `position`: Numer pozycji do której ma zostać przypisanym element słownika.
    pub struct ByIdPosition {
        /// Id elementu słownika.
        pub id: i32,
        /// Numer pozycji do której ma zostać przypisanym element słownika.
        pub position: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByIdPosition {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            push_query_param(&mut query, "position", &self.position)?;
            api.request_no_body::<()>(
                HttpMethod::Patch,
                "/api/FKDictionaries/SetElementPosition",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            push_query_param(&mut query, "position", &self.position)?;
            api.request_no_body_raw(
                HttpMethod::Patch,
                "/api/FKDictionaries/SetElementPosition",
                query,
            )
            .await
        }
    }
}

pub mod set_contractor_position {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PATCH` `/api/FKDictionaries/SetContractorPosition`
    ///
    /// Parameter `id`: Inferred from route query template.
    pub struct ById {
        /// Inferred from route query template.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = crate::web_api::interface::dictionaries::view_models::ContractorPosition;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<crate::web_api::interface::dictionaries::view_models::ContractorPosition>(HttpMethod::Patch, "/api/FKDictionaries/SetContractorPosition", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(
                HttpMethod::Patch,
                "/api/FKDictionaries/SetContractorPosition",
                query,
            )
            .await
        }
    }

    /// `PATCH` `/api/FKDictionaries/SetContractorPosition`
    ///
    /// Parameter `id`: Id kontrahenta.
    ///
    /// Parameter `position`: Numer pozycji do której ma zostać przypisany kontrahent.
    pub struct ByIdPosition {
        /// Id kontrahenta.
        pub id: i32,
        /// Numer pozycji do której ma zostać przypisany kontrahent.
        pub position: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByIdPosition {
        type Output = crate::web_api::interface::dictionaries::view_models::ContractorPosition;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            push_query_param(&mut query, "position", &self.position)?;
            api.request_no_body::<crate::web_api::interface::dictionaries::view_models::ContractorPosition>(HttpMethod::Patch, "/api/FKDictionaries/SetContractorPosition", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            push_query_param(&mut query, "position", &self.position)?;
            api.request_no_body_raw(
                HttpMethod::Patch,
                "/api/FKDictionaries/SetContractorPosition",
                query,
            )
            .await
        }
    }
}

#[allow(non_snake_case)]
impl IFKDictionariesController {
    /// `GET` `/api/FKDictionaries`
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/FKDictionaries`
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PATCH` `/api/FKDictionaries/SetElementPosition`
    ///
    /// Parameter `id`: Inferred from route query template.
    pub async fn SetElementPosition<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: set_element_position::Overload,
    {
        overload.call(api).await
    }

    /// `PATCH` `/api/FKDictionaries/SetElementPosition`
    ///
    /// Parameter `id`: Inferred from route query template.
    pub async fn SetElementPositionRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: set_element_position::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PATCH` `/api/FKDictionaries/SetContractorPosition`
    ///
    /// Parameter `id`: Inferred from route query template.
    pub async fn SetContractorPosition<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: set_contractor_position::Overload,
    {
        overload.call(api).await
    }

    /// `PATCH` `/api/FKDictionaries/SetContractorPosition`
    ///
    /// Parameter `id`: Inferred from route query template.
    pub async fn SetContractorPositionRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: set_contractor_position::Overload,
    {
        overload.call_raw(api).await
    }

    /// `POST` `/api/FKDictionaries/Create`
    ///
    /// Parameter `dictionary_id`: Id słownika.
    ///
    /// Body `element`: Obiekt elementu słownika do utworzenia.
    pub async fn AddNew(
        api: &ApiClient,
        dictionary_id: i32,
        element: &crate::web_api::interface::dictionaries::view_models::DictionaryElement,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dictionaryId", &dictionary_id)?;
        api.request_with_body::<(), _>(
            HttpMethod::Post,
            "/api/FKDictionaries/Create",
            query,
            element,
        )
        .await
    }

    /// `POST` `/api/FKDictionaries/Create`
    ///
    /// Parameter `dictionary_id`: Id słownika.
    ///
    /// Body `element`: Obiekt elementu słownika do utworzenia.
    pub async fn AddNewRaw(
        api: &ApiClient,
        dictionary_id: i32,
        element: &crate::web_api::interface::dictionaries::view_models::DictionaryElement,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dictionaryId", &dictionary_id)?;
        api.request_with_body_raw(
            HttpMethod::Post,
            "/api/FKDictionaries/Create",
            query,
            element,
        )
        .await
    }

    /// `PUT` `/api/FKDictionaries/Update`
    ///
    /// Parameter `dictionary_id`: Id słownika.
    ///
    /// Body `element`: Obiekt elementu słownika do aktualizacji.
    pub async fn Update(
        api: &ApiClient,
        dictionary_id: i32,
        element: &crate::web_api::interface::dictionaries::view_models::DictionaryElement,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dictionaryId", &dictionary_id)?;
        api.request_with_body::<(), _>(
            HttpMethod::Put,
            "/api/FKDictionaries/Update",
            query,
            element,
        )
        .await
    }

    /// `PUT` `/api/FKDictionaries/Update`
    ///
    /// Parameter `dictionary_id`: Id słownika.
    ///
    /// Body `element`: Obiekt elementu słownika do aktualizacji.
    pub async fn UpdateRaw(
        api: &ApiClient,
        dictionary_id: i32,
        element: &crate::web_api::interface::dictionaries::view_models::DictionaryElement,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dictionaryId", &dictionary_id)?;
        api.request_with_body_raw(
            HttpMethod::Put,
            "/api/FKDictionaries/Update",
            query,
            element,
        )
        .await
    }

    /// `GET` `/api/FKDictionaries/Business`
    ///
    /// Parameter `id`: Inferred from route query template.
    pub async fn GetBusiness(
        api: &ApiClient,
        id: i32,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::dictionaries::view_models::BusinessDictionary>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body::<crate::web_api::interface::dictionaries::view_models::BusinessDictionary>(HttpMethod::Get, "/api/FKDictionaries/Business", query).await
    }

    /// `GET` `/api/FKDictionaries/Business`
    ///
    /// Parameter `id`: Inferred from route query template.
    pub async fn GetBusinessRaw(
        api: &ApiClient,
        id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKDictionaries/Business", query)
            .await
    }

    /// `POST` `/api/FKDictionaries/CreateBusiness`
    ///
    /// Parameter `dictionary_id`: Id słownika.
    ///
    /// Body `element`: Obiekt elementu słownika do utworzenia.
    pub async fn AddNewBusiness(
        api: &ApiClient,
        dictionary_id: i32,
        element: &crate::web_api::interface::dictionaries::view_models::BusinessDictionaryElement,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dictionaryId", &dictionary_id)?;
        api.request_with_body::<(), _>(
            HttpMethod::Post,
            "/api/FKDictionaries/CreateBusiness",
            query,
            element,
        )
        .await
    }

    /// `POST` `/api/FKDictionaries/CreateBusiness`
    ///
    /// Parameter `dictionary_id`: Id słownika.
    ///
    /// Body `element`: Obiekt elementu słownika do utworzenia.
    pub async fn AddNewBusinessRaw(
        api: &ApiClient,
        dictionary_id: i32,
        element: &crate::web_api::interface::dictionaries::view_models::BusinessDictionaryElement,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dictionaryId", &dictionary_id)?;
        api.request_with_body_raw(
            HttpMethod::Post,
            "/api/FKDictionaries/CreateBusiness",
            query,
            element,
        )
        .await
    }

    /// `PUT` `/api/FKDictionaries/UpdateBusiness`
    ///
    /// Parameter `dictionary_id`: Id słownika.
    ///
    /// Body `element`: Obiekt elementu słownika do aktualizacji.
    pub async fn UpdateBusiness(
        api: &ApiClient,
        dictionary_id: i32,
        element: &crate::web_api::interface::dictionaries::view_models::BusinessDictionaryElement,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dictionaryId", &dictionary_id)?;
        api.request_with_body::<(), _>(
            HttpMethod::Put,
            "/api/FKDictionaries/UpdateBusiness",
            query,
            element,
        )
        .await
    }

    /// `PUT` `/api/FKDictionaries/UpdateBusiness`
    ///
    /// Parameter `dictionary_id`: Id słownika.
    ///
    /// Body `element`: Obiekt elementu słownika do aktualizacji.
    pub async fn UpdateBusinessRaw(
        api: &ApiClient,
        dictionary_id: i32,
        element: &crate::web_api::interface::dictionaries::view_models::BusinessDictionaryElement,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dictionaryId", &dictionary_id)?;
        api.request_with_body_raw(
            HttpMethod::Put,
            "/api/FKDictionaries/UpdateBusiness",
            query,
            element,
        )
        .await
    }
}
