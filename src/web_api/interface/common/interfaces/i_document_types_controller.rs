use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `DocumentTypes`.
pub struct IDocumentTypesController;

#[allow(non_snake_case)]
impl IDocumentTypesController {
    /// `GET` `/api/DocumentTypes`
    ///
    /// Parameter `id`: Id typu dokumentu.
    pub async fn Get(
        api: &ApiClient,
        id: i32,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::common::view_models::DocumentType>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body::<crate::web_api::interface::common::view_models::DocumentType>(
            HttpMethod::Get,
            "/api/DocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/DocumentTypes`
    ///
    /// Parameter `id`: Id typu dokumentu.
    pub async fn GetRaw(api: &ApiClient, id: i32) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/DocumentTypes", query)
            .await
    }

    /// `GET` `/api/DocumentTypes`
    ///
    /// Parameter `document_character`: Charakter dokumentu.
    pub async fn GetByCharacter(
        api: &ApiClient,
        document_character: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::DocumentType>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentCharacter", &document_character)?;
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::DocumentType>>(
            HttpMethod::Get,
            "/api/DocumentTypes",
            query,
        )
        .await
    }

    /// `GET` `/api/DocumentTypes`
    ///
    /// Parameter `document_character`: Charakter dokumentu.
    pub async fn GetByCharacterRaw(
        api: &ApiClient,
        document_character: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "documentCharacter", &document_character)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/DocumentTypes", query)
            .await
    }

    /// `GET` `/api/DocumentTypes/CharacterRelations`
    pub async fn GetCharacterRelations(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::CharacterRelation>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::CharacterRelation>>(HttpMethod::Get, "/api/DocumentTypes/CharacterRelations", query).await
    }

    /// `GET` `/api/DocumentTypes/CharacterRelations`
    pub async fn GetCharacterRelationsRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/DocumentTypes/CharacterRelations",
            query,
        )
        .await
    }
}
