use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `ContractorCardIndex`.
pub struct IContractorCardIndexController;

#[allow(non_snake_case)]
impl IContractorCardIndexController {
    /// `GET` `/api/Contractors`
    ///
    /// Parameter `id`: Id kontrahenta.
    pub async fn GetByIdAsync(api: &ApiClient, id: i32) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body::<()>(HttpMethod::Get, "/api/Contractors", query)
            .await
    }

    /// `GET` `/api/Contractors`
    ///
    /// Parameter `id`: Id kontrahenta.
    pub async fn GetByIdAsyncRaw(
        api: &ApiClient,
        id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Contractors", query)
            .await
    }

    /// `GET` `/api/Contractors`
    ///
    /// Parameter `code`: Kod kontrahenta.
    pub async fn GetByCodeAsync(
        api: &ApiClient,
        code: String,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "code", &code)?;
        api.request_no_body::<()>(HttpMethod::Get, "/api/Contractors", query)
            .await
    }

    /// `GET` `/api/Contractors`
    ///
    /// Parameter `code`: Kod kontrahenta.
    pub async fn GetByCodeAsyncRaw(
        api: &ApiClient,
        code: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "code", &code)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Contractors", query)
            .await
    }

    /// `GET` `/api/Contractors`
    ///
    /// Parameter `nip`: NIP kontrahenta.
    pub async fn GetByNIPAsync(
        api: &ApiClient,
        nip: String,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "nip", &nip)?;
        api.request_no_body::<()>(HttpMethod::Get, "/api/Contractors", query)
            .await
    }

    /// `GET` `/api/Contractors`
    ///
    /// Parameter `nip`: NIP kontrahenta.
    pub async fn GetByNIPAsyncRaw(
        api: &ApiClient,
        nip: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "nip", &nip)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Contractors", query)
            .await
    }
}
