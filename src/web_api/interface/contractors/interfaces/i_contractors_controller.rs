use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `Contractors`.
pub struct IContractorsController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Contractors`
    ///
    /// Parameter `id`: Id kontrahenta.
    pub struct ById {
        /// Id kontrahenta.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = crate::web_api::interface::contractors::view_models::Contractor;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<crate::web_api::interface::contractors::view_models::Contractor>(
                HttpMethod::Get,
                "/api/Contractors",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Contractors", query)
                .await
        }
    }

    /// `GET` `/api/Contractors`
    ///
    /// Parameter `code`: Kod kontrahenta.
    pub struct ByCode {
        /// Kod kontrahenta.
        pub code: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByCode {
        type Output = crate::web_api::interface::contractors::view_models::Contractor;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "code", &self.code)?;
            api.request_no_body::<crate::web_api::interface::contractors::view_models::Contractor>(
                HttpMethod::Get,
                "/api/Contractors",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "code", &self.code)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Contractors", query)
                .await
        }
    }

    /// `GET` `/api/Contractors`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output =
            Vec<crate::web_api::interface::contractors::view_models::ContractorListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<Vec<crate::web_api::interface::contractors::view_models::ContractorListElement>>(HttpMethod::Get, "/api/Contractors", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(HttpMethod::Get, "/api/Contractors", query)
                .await
        }
    }
}

pub mod sync_fk {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PATCH` `/api/Contractors/SyncFK`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Parameter `is_new`: Czy nowy kontrahent.
    pub struct ByContractorIdIsNew {
        /// Id kontrahenta.
        pub contractor_id: i32,
        /// Czy nowy kontrahent.
        pub is_new: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorIdIsNew {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            push_query_param(&mut query, "isNew", &self.is_new)?;
            api.request_no_body::<()>(HttpMethod::Patch, "/api/Contractors/SyncFK", query)
                .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            push_query_param(&mut query, "isNew", &self.is_new)?;
            api.request_no_body_raw(HttpMethod::Patch, "/api/Contractors/SyncFK", query)
                .await
        }
    }

    /// `PATCH` `/api/Contractors/SyncFK`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    ///
    /// Parameter `is_new`: Czy nowy kontrahent.
    pub struct ByContractorCodeIsNew {
        /// Kod kontrahenta.
        pub contractor_code: String,
        /// Czy nowy kontrahent.
        pub is_new: bool,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorCodeIsNew {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            push_query_param(&mut query, "isNew", &self.is_new)?;
            api.request_no_body::<()>(HttpMethod::Patch, "/api/Contractors/SyncFK", query)
                .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            push_query_param(&mut query, "isNew", &self.is_new)?;
            api.request_no_body_raw(HttpMethod::Patch, "/api/Contractors/SyncFK", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IContractorsController {
    /// `GET` `/api/Contractors`
    ///
    /// Parameter `id`: Id kontrahenta.
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Contractors`
    ///
    /// Parameter `id`: Id kontrahenta.
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PATCH` `/api/Contractors/SyncFK`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Parameter `is_new`: Czy nowy kontrahent.
    pub async fn SyncFK<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: sync_fk::Overload,
    {
        overload.call(api).await
    }

    /// `PATCH` `/api/Contractors/SyncFK`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Parameter `is_new`: Czy nowy kontrahent.
    pub async fn SyncFKRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: sync_fk::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Contractors`
    ///
    /// Parameter `nip`: NIP kontrahenta.
    pub async fn GetByNIP(
        api: &ApiClient,
        nip: String,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::contractors::view_models::Contractor>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "nip", &nip)?;
        api.request_no_body::<Vec<crate::web_api::interface::contractors::view_models::Contractor>>(
            HttpMethod::Get,
            "/api/Contractors",
            query,
        )
        .await
    }

    /// `GET` `/api/Contractors`
    ///
    /// Parameter `nip`: NIP kontrahenta.
    pub async fn GetByNIPRaw(
        api: &ApiClient,
        nip: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "nip", &nip)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Contractors", query)
            .await
    }

    /// `GET` `/api/Contractors`
    ///
    /// Parameter `pesel`: PESEL kontrahenta.
    pub async fn GetByPesel(
        api: &ApiClient,
        pesel: String,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::contractors::view_models::Contractor>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "pesel", &pesel)?;
        api.request_no_body::<Vec<crate::web_api::interface::contractors::view_models::Contractor>>(
            HttpMethod::Get,
            "/api/Contractors",
            query,
        )
        .await
    }

    /// `GET` `/api/Contractors`
    ///
    /// Parameter `pesel`: PESEL kontrahenta.
    pub async fn GetByPeselRaw(
        api: &ApiClient,
        pesel: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "pesel", &pesel)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Contractors", query)
            .await
    }

    /// `GET` `/api/Contractors`
    ///
    /// Parameter `position`: Pozycja FK kontrahenta.
    pub async fn GetByPosition(
        api: &ApiClient,
        position: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::contractors::view_models::Contractor>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "position", &position)?;
        api.request_no_body::<Vec<crate::web_api::interface::contractors::view_models::Contractor>>(
            HttpMethod::Get,
            "/api/Contractors",
            query,
        )
        .await
    }

    /// `GET` `/api/Contractors`
    ///
    /// Parameter `position`: Pozycja FK kontrahenta.
    pub async fn GetByPositionRaw(
        api: &ApiClient,
        position: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "position", &position)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Contractors", query)
            .await
    }

    /// `GET` `/api/Contractors`
    ///
    /// Parameter `nip`: NIP kontrahenta.
    ///
    /// Parameter `position`: Pozycja FK kontrahenta.
    pub async fn GetByNIPAndPosition(
        api: &ApiClient,
        nip: String,
        position: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::contractors::view_models::Contractor>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "nip", &nip)?;
        push_query_param(&mut query, "position", &position)?;
        api.request_no_body::<Vec<crate::web_api::interface::contractors::view_models::Contractor>>(
            HttpMethod::Get,
            "/api/Contractors",
            query,
        )
        .await
    }

    /// `GET` `/api/Contractors`
    ///
    /// Parameter `nip`: NIP kontrahenta.
    ///
    /// Parameter `position`: Pozycja FK kontrahenta.
    pub async fn GetByNIPAndPositionRaw(
        api: &ApiClient,
        nip: String,
        position: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "nip", &nip)?;
        push_query_param(&mut query, "position", &position)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Contractors", query)
            .await
    }

    /// `GET` `/api/Contractors/WithDimensions`
    pub async fn GetWithDimensions(api: &ApiClient) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::contractors::view_models::ContractorListElementWithDimensions>>, ApiError>{
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::contractors::view_models::ContractorListElementWithDimensions>>(HttpMethod::Get, "/api/Contractors/WithDimensions", query).await
    }

    /// `GET` `/api/Contractors/WithDimensions`
    pub async fn GetWithDimensionsRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Contractors/WithDimensions", query)
            .await
    }

    /// `PATCH` `/api/Contractors/Filter`
    ///
    /// Body `criteria`: Lista kryteriów wyszukiwania.
    pub async fn Filter(
        api: &ApiClient,
        criteria: &crate::web_api::interface::contractors::view_models::ContractorFilterCriteria,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::contractors::view_models::ContractorListElement>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<Vec<crate::web_api::interface::contractors::view_models::ContractorListElement>, _>(HttpMethod::Patch, "/api/Contractors/Filter", query, criteria).await
    }

    /// `PATCH` `/api/Contractors/Filter`
    ///
    /// Body `criteria`: Lista kryteriów wyszukiwania.
    pub async fn FilterRaw(
        api: &ApiClient,
        criteria: &crate::web_api::interface::contractors::view_models::ContractorFilterCriteria,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Patch,
            "/api/Contractors/Filter",
            query,
            criteria,
        )
        .await
    }

    /// `PATCH` `/api/Contractors/Filter/WithDimensions`
    ///
    /// Body `criteria`: Lista kryteriów wyszukiwania.
    pub async fn FilterWithDimensions(api: &ApiClient, criteria: &crate::web_api::interface::contractors::view_models::ContractorFilterCriteria) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::contractors::view_models::ContractorListElementWithDimensions>>, ApiError>{
        let query = vec![];
        api.request_with_body::<Vec<crate::web_api::interface::contractors::view_models::ContractorListElementWithDimensions>, _>(HttpMethod::Patch, "/api/Contractors/Filter/WithDimensions", query, criteria).await
    }

    /// `PATCH` `/api/Contractors/Filter/WithDimensions`
    ///
    /// Body `criteria`: Lista kryteriów wyszukiwania.
    pub async fn FilterWithDimensionsRaw(
        api: &ApiClient,
        criteria: &crate::web_api::interface::contractors::view_models::ContractorFilterCriteria,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Patch,
            "/api/Contractors/Filter/WithDimensions",
            query,
            criteria,
        )
        .await
    }

    /// `POST` `/api/Contractors/Create`
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
            "/api/Contractors/Create",
            query,
            contractor,
        )
        .await
    }

    /// `POST` `/api/Contractors/Create`
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
            "/api/Contractors/Create",
            query,
            contractor,
        )
        .await
    }

    /// `PUT` `/api/Contractors/Update`
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
            "/api/Contractors/Update",
            query,
            contractor,
        )
        .await
    }

    /// `PUT` `/api/Contractors/Update`
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
            "/api/Contractors/Update",
            query,
            contractor,
        )
        .await
    }

    /// `GET` `/api/Contractors/Markers`
    pub async fn GetMarkers(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Marker>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Marker>>(
            HttpMethod::Get,
            "/api/Contractors/Markers",
            query,
        )
        .await
    }

    /// `GET` `/api/Contractors/Markers`
    pub async fn GetMarkersRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Contractors/Markers", query)
            .await
    }

    /// `GET` `/api/Contractors/Kinds`
    pub async fn GetKinds(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Kind>>, ApiError>
    {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Kind>>(
            HttpMethod::Get,
            "/api/Contractors/Kinds",
            query,
        )
        .await
    }

    /// `GET` `/api/Contractors/Kinds`
    pub async fn GetKindsRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Contractors/Kinds", query)
            .await
    }

    /// `GET` `/api/Contractors/Catalogs`
    pub async fn GetCatalogs(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Catalog>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Catalog>>(
            HttpMethod::Get,
            "/api/Contractors/Catalogs",
            query,
        )
        .await
    }

    /// `GET` `/api/Contractors/Catalogs`
    pub async fn GetCatalogsRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Contractors/Catalogs", query)
            .await
    }

    /// `GET` `/api/Contractors/IncrementalSync`
    ///
    /// Parameter `date_from`: Inferred from route query template.
    pub async fn IncrementalSync(
        api: &ApiClient,
        date_from: chrono::NaiveDateTime,
    ) -> Result<
        ResponseEnvelope<
            Vec<
                crate::web_api::interface::common::view_models::IncrementalSyncListElement<
                    serde_json::Value,
                >,
            >,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        api.request_no_body::<Vec<
            crate::web_api::interface::common::view_models::IncrementalSyncListElement<
                serde_json::Value,
            >,
        >>(HttpMethod::Get, "/api/Contractors/IncrementalSync", query)
            .await
    }

    /// `GET` `/api/Contractors/IncrementalSync`
    ///
    /// Parameter `date_from`: Inferred from route query template.
    pub async fn IncrementalSyncRaw(
        api: &ApiClient,
        date_from: chrono::NaiveDateTime,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Contractors/IncrementalSync", query)
            .await
    }

    /// `PATCH` `/api/Contractors/FilterSql`
    ///
    /// Body `criteria_filter`: Lista kryteriów wyszukiwania.
    pub async fn FilterSql(
        api: &ApiClient,
        criteria_filter: &crate::web_api::interface::contractors::view_models::ContractorCriteriaFilter,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::contractors::view_models::ContractorListElement>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<Vec<crate::web_api::interface::contractors::view_models::ContractorListElement>, _>(HttpMethod::Patch, "/api/Contractors/FilterSql", query, criteria_filter).await
    }

    /// `PATCH` `/api/Contractors/FilterSql`
    ///
    /// Body `criteria_filter`: Lista kryteriów wyszukiwania.
    pub async fn FilterSqlRaw(
        api: &ApiClient,
        criteria_filter: &crate::web_api::interface::contractors::view_models::ContractorCriteriaFilter,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Patch,
            "/api/Contractors/FilterSql",
            query,
            criteria_filter,
        )
        .await
    }

    /// `GET` `/api/Contractors/Page`
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
                crate::web_api::interface::contractors::view_models::ContractorListElement,
            >,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body::<crate::web_api::interface::view_models::Page<
            crate::web_api::interface::contractors::view_models::ContractorListElement,
        >>(HttpMethod::Get, "/api/Contractors/Page", query)
            .await
    }

    /// `GET` `/api/Contractors/Page`
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
        api.request_no_body_raw(HttpMethod::Get, "/api/Contractors/Page", query)
            .await
    }

    /// `GET` `/api/Contractors/PageWithDimensions`
    ///
    /// Parameter `page`: Numer strony.
    ///
    /// Parameter `size`: Ilość wyników na stronie.
    ///
    /// Parameter `order_by`: Sortowanie wyników na stronie.
    pub async fn GetPagedDocumentWithDimensions(
        api: &ApiClient,
        page: i32,
        size: i32,
        order_by: crate::web_api::interface::enums::enumOrderByType,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::view_models::Page<serde_json::Value>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body::<crate::web_api::interface::view_models::Page<serde_json::Value>>(
            HttpMethod::Get,
            "/api/Contractors/PageWithDimensions",
            query,
        )
        .await
    }

    /// `GET` `/api/Contractors/PageWithDimensions`
    ///
    /// Parameter `page`: Numer strony.
    ///
    /// Parameter `size`: Ilość wyników na stronie.
    ///
    /// Parameter `order_by`: Sortowanie wyników na stronie.
    pub async fn GetPagedDocumentWithDimensionsRaw(
        api: &ApiClient,
        page: i32,
        size: i32,
        order_by: crate::web_api::interface::enums::enumOrderByType,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/Contractors/PageWithDimensions",
            query,
        )
        .await
    }

    /// `PATCH` `/api/Contractors/FilterSql/WithDimensions`
    ///
    /// Body `criteria_filter`: Lista kryteriów wyszukiwania.
    pub async fn FilterSqlWithDimensions(api: &ApiClient, criteria_filter: &crate::web_api::interface::contractors::view_models::ContractorCriteriaFilter) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::contractors::view_models::ContractorListElementWithDimensions>>, ApiError>{
        let query = vec![];
        api.request_with_body::<Vec<crate::web_api::interface::contractors::view_models::ContractorListElementWithDimensions>, _>(HttpMethod::Patch, "/api/Contractors/FilterSql/WithDimensions", query, criteria_filter).await
    }

    /// `PATCH` `/api/Contractors/FilterSql/WithDimensions`
    ///
    /// Body `criteria_filter`: Lista kryteriów wyszukiwania.
    pub async fn FilterSqlWithDimensionsRaw(
        api: &ApiClient,
        criteria_filter: &crate::web_api::interface::contractors::view_models::ContractorCriteriaFilter,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Patch,
            "/api/Contractors/FilterSql/WithDimensions",
            query,
            criteria_filter,
        )
        .await
    }
}
