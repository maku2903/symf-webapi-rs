use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `OssContractors`.
pub struct IOssContractorsController;

#[allow(non_snake_case)]
impl IOssContractorsController {
    /// `GET` `/api/OssContractors`
    ///
    /// Parameter `id`: Id kontrahenta.
    pub async fn Get(
        api: &ApiClient,
        id: i32,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::contractors::view_models::Contractor>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body::<crate::web_api::interface::contractors::view_models::Contractor>(
            HttpMethod::Get,
            "/api/OssContractors",
            query,
        )
        .await
    }

    /// `GET` `/api/OssContractors`
    ///
    /// Parameter `id`: Id kontrahenta.
    pub async fn GetRaw(api: &ApiClient, id: i32) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/OssContractors", query)
            .await
    }

    /// `POST` `/api/OssContractors/Create`
    ///
    /// Parameter `sync_fk`: Czy zsynchronizować kontrahenta z FK.
    ///
    /// Body `contractor`: Obiekt kontrahenta do utworzenia.
    pub async fn AddNew(
        api: &ApiClient,
        sync_fk: bool,
        contractor: &crate::web_api::interface::contractors::view_models::Contractor,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::contractors::view_models::Contractor>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "syncFk", &sync_fk)?;
        api.request_with_body::<crate::web_api::interface::contractors::view_models::Contractor, _>(
            HttpMethod::Post,
            "/api/OssContractors/Create",
            query,
            contractor,
        )
        .await
    }

    /// `POST` `/api/OssContractors/Create`
    ///
    /// Parameter `sync_fk`: Czy zsynchronizować kontrahenta z FK.
    ///
    /// Body `contractor`: Obiekt kontrahenta do utworzenia.
    pub async fn AddNewRaw(
        api: &ApiClient,
        sync_fk: bool,
        contractor: &crate::web_api::interface::contractors::view_models::Contractor,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "syncFk", &sync_fk)?;
        api.request_with_body_raw(
            HttpMethod::Post,
            "/api/OssContractors/Create",
            query,
            contractor,
        )
        .await
    }

    /// `PUT` `/api/OssContractors/Update`
    ///
    /// Parameter `sync_fk`: Czy zsynchronizować kontrahenta z FK.
    ///
    /// Body `contractor`: Obiekt kontrahenta do aktualizacji.
    pub async fn Update(
        api: &ApiClient,
        sync_fk: bool,
        contractor: &crate::web_api::interface::contractors::view_models::Contractor,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "syncFk", &sync_fk)?;
        api.request_with_body::<(), _>(
            HttpMethod::Put,
            "/api/OssContractors/Update",
            query,
            contractor,
        )
        .await
    }

    /// `PUT` `/api/OssContractors/Update`
    ///
    /// Parameter `sync_fk`: Czy zsynchronizować kontrahenta z FK.
    ///
    /// Body `contractor`: Obiekt kontrahenta do aktualizacji.
    pub async fn UpdateRaw(
        api: &ApiClient,
        sync_fk: bool,
        contractor: &crate::web_api::interface::contractors::view_models::Contractor,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "syncFk", &sync_fk)?;
        api.request_with_body_raw(
            HttpMethod::Put,
            "/api/OssContractors/Update",
            query,
            contractor,
        )
        .await
    }
}
