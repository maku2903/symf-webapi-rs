use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `FKContractors`.
pub struct IFKContractorsController;

#[allow(non_snake_case)]
impl IFKContractorsController {
    /// `GET` `/api/FKContractors`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::contractors::view_models::ContractorListElement>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::fkf::contractors::view_models::ContractorListElement>>(HttpMethod::Get, "/api/FKContractors", query).await
    }

    /// `GET` `/api/FKContractors`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/FKContractors", query)
            .await
    }

    /// `GET` `/api/FKContractors`
    ///
    /// Parameter `id`: Id kontrahenta.
    pub async fn GetById(
        api: &ApiClient,
        id: i32,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::fkf::contractors::view_models::Contractor>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body::<crate::web_api::interface::fkf::contractors::view_models::Contractor>(
            HttpMethod::Get,
            "/api/FKContractors",
            query,
        )
        .await
    }

    /// `GET` `/api/FKContractors`
    ///
    /// Parameter `id`: Id kontrahenta.
    pub async fn GetByIdRaw(api: &ApiClient, id: i32) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKContractors", query)
            .await
    }

    /// `GET` `/api/FKContractors`
    ///
    /// Parameter `position`: Pozycja kontrahenta.
    pub async fn GetByPosition(
        api: &ApiClient,
        position: i32,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::fkf::contractors::view_models::Contractor>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "position", &position)?;
        api.request_no_body::<crate::web_api::interface::fkf::contractors::view_models::Contractor>(
            HttpMethod::Get,
            "/api/FKContractors",
            query,
        )
        .await
    }

    /// `GET` `/api/FKContractors`
    ///
    /// Parameter `position`: Pozycja kontrahenta.
    pub async fn GetByPositionRaw(
        api: &ApiClient,
        position: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "position", &position)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKContractors", query)
            .await
    }

    /// `GET` `/api/FKContractors`
    ///
    /// Parameter `code`: Kod kontrahenta.
    pub async fn GetByCode(
        api: &ApiClient,
        code: String,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::fkf::contractors::view_models::Contractor>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "code", &code)?;
        api.request_no_body::<crate::web_api::interface::fkf::contractors::view_models::Contractor>(
            HttpMethod::Get,
            "/api/FKContractors",
            query,
        )
        .await
    }

    /// `GET` `/api/FKContractors`
    ///
    /// Parameter `code`: Kod kontrahenta.
    pub async fn GetByCodeRaw(
        api: &ApiClient,
        code: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "code", &code)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKContractors", query)
            .await
    }

    /// `GET` `/api/FKContractors`
    ///
    /// Parameter `nip`: NIP kontrahenta.
    pub async fn GetByNIP(
        api: &ApiClient,
        nip: String,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::contractors::view_models::Contractor>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "nip", &nip)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::contractors::view_models::Contractor>>(HttpMethod::Get, "/api/FKContractors", query).await
    }

    /// `GET` `/api/FKContractors`
    ///
    /// Parameter `nip`: NIP kontrahenta.
    pub async fn GetByNIPRaw(
        api: &ApiClient,
        nip: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "nip", &nip)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKContractors", query)
            .await
    }

    /// `GET` `/api/FKContractors`
    ///
    /// Parameter `pesel`: PESEL kontrahenta.
    pub async fn GetByPesel(
        api: &ApiClient,
        pesel: String,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::contractors::view_models::Contractor>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "pesel", &pesel)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::contractors::view_models::Contractor>>(HttpMethod::Get, "/api/FKContractors", query).await
    }

    /// `GET` `/api/FKContractors`
    ///
    /// Parameter `pesel`: PESEL kontrahenta.
    pub async fn GetByPeselRaw(
        api: &ApiClient,
        pesel: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "pesel", &pesel)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKContractors", query)
            .await
    }

    /// `PATCH` `/api/FKContractors/Filter`
    ///
    /// Body `criteria`: Lista kryteriów wyszukiwania.
    pub async fn Filter(
        api: &ApiClient,
        criteria: &Vec<crate::web_api::interface::fkf::common::view_models::FilterCriteria>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::contractors::view_models::ContractorListElement>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<Vec<crate::web_api::interface::fkf::contractors::view_models::ContractorListElement>, _>(HttpMethod::Patch, "/api/FKContractors/Filter", query, criteria).await
    }

    /// `PATCH` `/api/FKContractors/Filter`
    ///
    /// Body `criteria`: Lista kryteriów wyszukiwania.
    pub async fn FilterRaw(
        api: &ApiClient,
        criteria: &Vec<crate::web_api::interface::fkf::common::view_models::FilterCriteria>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Patch,
            "/api/FKContractors/Filter",
            query,
            criteria,
        )
        .await
    }

    /// `POST` `/api/FKContractors/Create`
    ///
    /// Body `contractor`: Obiekt kontrahenta do utworzenia.
    pub async fn AddNew(
        api: &ApiClient,
        contractor: &crate::web_api::interface::fkf::contractors::view_models::Contractor,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::fkf::contractors::view_models::Contractor>,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<crate::web_api::interface::fkf::contractors::view_models::Contractor, _>(HttpMethod::Post, "/api/FKContractors/Create", query, contractor).await
    }

    /// `POST` `/api/FKContractors/Create`
    ///
    /// Body `contractor`: Obiekt kontrahenta do utworzenia.
    pub async fn AddNewRaw(
        api: &ApiClient,
        contractor: &crate::web_api::interface::fkf::contractors::view_models::Contractor,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Post,
            "/api/FKContractors/Create",
            query,
            contractor,
        )
        .await
    }

    /// `PUT` `/api/FKContractors/Update`
    ///
    /// Body `contractor`: Obiekt kontrahenta do aktualizacji.
    pub async fn Update(
        api: &ApiClient,
        contractor: &crate::web_api::interface::fkf::contractors::view_models::Contractor,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let query = vec![];
        api.request_with_body::<(), _>(
            HttpMethod::Put,
            "/api/FKContractors/Update",
            query,
            contractor,
        )
        .await
    }

    /// `PUT` `/api/FKContractors/Update`
    ///
    /// Body `contractor`: Obiekt kontrahenta do aktualizacji.
    pub async fn UpdateRaw(
        api: &ApiClient,
        contractor: &crate::web_api::interface::fkf::contractors::view_models::Contractor,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Put,
            "/api/FKContractors/Update",
            query,
            contractor,
        )
        .await
    }

    /// `GET` `/api/FKContractors/Page`
    ///
    /// Parameter `page`: Numer strony.
    ///
    /// Parameter `size`: Ilość wyników na stronie.
    ///
    /// Parameter `order_by`: Sortowanie wyników na stronie.
    pub async fn GetPagedDocument(
        api: &ApiClient,
        page: i32,
        size: i32,
        order_by: crate::web_api::interface::enums::enumOrderByType,
    ) -> Result<
        ResponseEnvelope<
            crate::web_api::interface::view_models::Page<
                crate::web_api::interface::fkf::contractors::view_models::ContractorListElement,
            >,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body::<crate::web_api::interface::view_models::Page<
            crate::web_api::interface::fkf::contractors::view_models::ContractorListElement,
        >>(HttpMethod::Get, "/api/FKContractors/Page", query)
            .await
    }

    /// `GET` `/api/FKContractors/Page`
    ///
    /// Parameter `page`: Numer strony.
    ///
    /// Parameter `size`: Ilość wyników na stronie.
    ///
    /// Parameter `order_by`: Sortowanie wyników na stronie.
    pub async fn GetPagedDocumentRaw(
        api: &ApiClient,
        page: i32,
        size: i32,
        order_by: crate::web_api::interface::enums::enumOrderByType,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKContractors/Page", query)
            .await
    }

    /// `PATCH` `/api/FKContractors/FilterSql`
    ///
    /// Body `criteria_filter`: Lista kryteriów wyszukiwania.
    pub async fn FilterSql(
        api: &ApiClient,
        criteria_filter: &crate::web_api::interface::fkf::contractors::view_models::ContractorCriteriaFilter,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::contractors::view_models::ContractorListElement>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<Vec<crate::web_api::interface::fkf::contractors::view_models::ContractorListElement>, _>(HttpMethod::Patch, "/api/FKContractors/FilterSql", query, criteria_filter).await
    }

    /// `PATCH` `/api/FKContractors/FilterSql`
    ///
    /// Body `criteria_filter`: Lista kryteriów wyszukiwania.
    pub async fn FilterSqlRaw(
        api: &ApiClient,
        criteria_filter: &crate::web_api::interface::fkf::contractors::view_models::ContractorCriteriaFilter,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Patch,
            "/api/FKContractors/FilterSql",
            query,
            criteria_filter,
        )
        .await
    }
}
