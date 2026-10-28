use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `Session`.
pub struct ISessionController;

#[allow(non_snake_case)]
impl ISessionController {
    /// `GET` `/api/Sessions/CloseSession`
    pub async fn CloseSession(api: &ApiClient) -> Result<ResponseEnvelope<()>, ApiError> {
        let query = vec![];
        api.request_no_body::<()>(HttpMethod::Get, "/api/Sessions/CloseSession", query)
            .await
    }

    /// `GET` `/api/Sessions/CloseSession`
    pub async fn CloseSessionRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Sessions/CloseSession", query)
            .await
    }

    /// `GET` `/api/Sessions/OpenNewSession`
    ///
    /// Parameter `device_name`: Nazwa urządzenia.
    pub async fn OpenNewSession(
        api: &ApiClient,
        device_name: String,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "deviceName", &device_name)?;
        api.request_no_body::<()>(HttpMethod::Get, "/api/Sessions/OpenNewSession", query)
            .await
    }

    /// `GET` `/api/Sessions/OpenNewSession`
    ///
    /// Parameter `device_name`: Nazwa urządzenia.
    pub async fn OpenNewSessionRaw(
        api: &ApiClient,
        device_name: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "deviceName", &device_name)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Sessions/OpenNewSession", query)
            .await
    }

    /// `GET` `/api/Sessions/SessionInformation`
    ///
    /// Parameter `cancellation_token`: Token anulowania operacji
    pub async fn SessionInformationAsync(
        api: &ApiClient,
        cancellation_token: serde_json::Value,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::view_models::SessionInformation>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "cancellationToken", &cancellation_token)?;
        api.request_no_body::<crate::web_api::interface::view_models::SessionInformation>(
            HttpMethod::Get,
            "/api/Sessions/SessionInformation",
            query,
        )
        .await
    }

    /// `GET` `/api/Sessions/SessionInformation`
    ///
    /// Parameter `cancellation_token`: Token anulowania operacji
    pub async fn SessionInformationAsyncRaw(
        api: &ApiClient,
        cancellation_token: serde_json::Value,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "cancellationToken", &cancellation_token)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Sessions/SessionInformation", query)
            .await
    }
}
