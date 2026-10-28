use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `Ksef`.
pub struct IKsefController;

#[allow(non_snake_case)]
impl IKsefController {
    /// `POST` `/api/Ksef/AddSendToKsefEvent`
    ///
    /// Parameter `cancellation_token`: Token anulujący operację asynchroniczną.
    ///
    /// Parameter `doc_id`: Identyfikator dokumentu, który ma zostać przesłany do KSeF.
    pub async fn AddSendToKsefEventAsync(
        api: &ApiClient,
        cancellation_token: i32,
        doc_id: serde_json::Value,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "cancellationToken", &cancellation_token)?;
        push_query_param(&mut query, "docId", &doc_id)?;
        api.request_no_body::<()>(HttpMethod::Post, "/api/Ksef/AddSendToKsefEvent", query)
            .await
    }

    /// `POST` `/api/Ksef/AddSendToKsefEvent`
    ///
    /// Parameter `cancellation_token`: Token anulujący operację asynchroniczną.
    ///
    /// Parameter `doc_id`: Identyfikator dokumentu, który ma zostać przesłany do KSeF.
    pub async fn AddSendToKsefEventAsyncRaw(
        api: &ApiClient,
        cancellation_token: i32,
        doc_id: serde_json::Value,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "cancellationToken", &cancellation_token)?;
        push_query_param(&mut query, "docId", &doc_id)?;
        api.request_no_body_raw(HttpMethod::Post, "/api/Ksef/AddSendToKsefEvent", query)
            .await
    }

    /// `POST` `/api/Ksef/ChangeKsefStatusAsync`
    ///
    /// Parameter `id`: Identyfikator dokumentu, dla którego należy zmienić status KSeF.
    ///
    /// Parameter `ksef_status`: Nowy status KSeF do ustawienia dla dokumentu.
    ///
    /// Parameter `cancellation_token`: Token anulujący operację asynchroniczną.
    pub async fn ChangeKsefStatusAsync(
        api: &ApiClient,
        id: i32,
        ksef_status: i32,
        cancellation_token: serde_json::Value,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        push_query_param(&mut query, "ksefStatus", &ksef_status)?;
        push_query_param(&mut query, "cancellationToken", &cancellation_token)?;
        api.request_no_body::<()>(HttpMethod::Post, "/api/Ksef/ChangeKsefStatusAsync", query)
            .await
    }

    /// `POST` `/api/Ksef/ChangeKsefStatusAsync`
    ///
    /// Parameter `id`: Identyfikator dokumentu, dla którego należy zmienić status KSeF.
    ///
    /// Parameter `ksef_status`: Nowy status KSeF do ustawienia dla dokumentu.
    ///
    /// Parameter `cancellation_token`: Token anulujący operację asynchroniczną.
    pub async fn ChangeKsefStatusAsyncRaw(
        api: &ApiClient,
        id: i32,
        ksef_status: i32,
        cancellation_token: serde_json::Value,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        push_query_param(&mut query, "ksefStatus", &ksef_status)?;
        push_query_param(&mut query, "cancellationToken", &cancellation_token)?;
        api.request_no_body_raw(HttpMethod::Post, "/api/Ksef/ChangeKsefStatusAsync", query)
            .await
    }
}
