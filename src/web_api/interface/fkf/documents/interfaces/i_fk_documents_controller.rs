use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `FKDocuments`.
pub struct IFKDocumentsController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/FKDocuments`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output =
            Vec<crate::web_api::interface::fkf::documents::view_models::DocumentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<Vec<crate::web_api::interface::fkf::documents::view_models::DocumentListElement>>(HttpMethod::Get, "/api/FKDocuments", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(HttpMethod::Get, "/api/FKDocuments", query)
                .await
        }
    }

    /// `GET` `/api/FKDocuments`
    ///
    /// Parameter `id`: Id dokumentu.
    ///
    /// Parameter `year_id`: Id roku.
    pub struct ByIdYearId {
        /// Id dokumentu.
        pub id: i32,
        /// Id roku.
        pub year_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByIdYearId {
        type Output = crate::web_api::interface::fkf::documents::view_models::Document;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            push_query_param(&mut query, "yearId", &self.year_id)?;
            api.request_no_body::<crate::web_api::interface::fkf::documents::view_models::Document>(
                HttpMethod::Get,
                "/api/FKDocuments",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            push_query_param(&mut query, "yearId", &self.year_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/FKDocuments", query)
                .await
        }
    }
}

pub mod get_from_buffer {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/FKDocuments/Buffer`
    ///
    /// Parameter `year_id`: Id roku.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByYearIdDateFromDateTo {
        /// Id roku.
        pub year_id: i32,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByYearIdDateFromDateTo {
        type Output =
            Vec<crate::web_api::interface::fkf::documents::view_models::DocumentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "yearId", &self.year_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::fkf::documents::view_models::DocumentListElement>>(HttpMethod::Get, "/api/FKDocuments/Buffer", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "yearId", &self.year_id)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/FKDocuments/Buffer", query)
                .await
        }
    }

    /// `GET` `/api/FKDocuments/Buffer`
    ///
    /// Parameter `year_id`: Id roku.
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub struct ByYearIdContractorPositionDateFromDateTo {
        /// Id roku.
        pub year_id: i32,
        /// Pozycja kontrahenta.
        pub contractor_position: i32,
        /// Data od przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty do wskazanej daty do.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_from: Option<chrono::NaiveDateTime>,
        /// Data do przedziału czasowego.
        /// W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty od wskazanej daty od.
        /// Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
        pub date_to: Option<chrono::NaiveDateTime>,
    }

    #[async_trait::async_trait]
    impl Overload for ByYearIdContractorPositionDateFromDateTo {
        type Output =
            Vec<crate::web_api::interface::fkf::documents::view_models::DocumentListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "yearId", &self.year_id)?;
            push_query_param(&mut query, "contractorPosition", &self.contractor_position)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body::<Vec<crate::web_api::interface::fkf::documents::view_models::DocumentListElement>>(HttpMethod::Get, "/api/FKDocuments/Buffer", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "yearId", &self.year_id)?;
            push_query_param(&mut query, "contractorPosition", &self.contractor_position)?;
            push_query_param(&mut query, "dateFrom", &self.date_from)?;
            push_query_param(&mut query, "dateTo", &self.date_to)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/FKDocuments/Buffer", query)
                .await
        }
    }
}

pub mod get_document_features {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/FKDocuments/DocumentFeatures`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output = Vec<crate::web_api::interface::fkf::documents::view_models::Feature>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<Vec<crate::web_api::interface::fkf::documents::view_models::Feature>>(HttpMethod::Get, "/api/FKDocuments/DocumentFeatures", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(HttpMethod::Get, "/api/FKDocuments/DocumentFeatures", query)
                .await
        }
    }

    /// `GET` `/api/FKDocuments/DocumentFeatures`
    ///
    /// Parameter `year_id`: Id roku.
    pub struct ByYearId {
        /// Id roku.
        pub year_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByYearId {
        type Output = Vec<crate::web_api::interface::fkf::documents::view_models::Feature>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "yearId", &self.year_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::fkf::documents::view_models::Feature>>(HttpMethod::Get, "/api/FKDocuments/DocumentFeatures", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "yearId", &self.year_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/FKDocuments/DocumentFeatures", query)
                .await
        }
    }
}

pub mod get_document_record_features {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/FKDocuments/DocumentRecordFeatures`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output = Vec<crate::web_api::interface::fkf::documents::view_models::Feature>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<Vec<crate::web_api::interface::fkf::documents::view_models::Feature>>(HttpMethod::Get, "/api/FKDocuments/DocumentRecordFeatures", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/FKDocuments/DocumentRecordFeatures",
                query,
            )
            .await
        }
    }

    /// `GET` `/api/FKDocuments/DocumentRecordFeatures`
    ///
    /// Parameter `year_id`: Id roku.
    pub struct ByYearId {
        /// Id roku.
        pub year_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByYearId {
        type Output = Vec<crate::web_api::interface::fkf::documents::view_models::Feature>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "yearId", &self.year_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::fkf::documents::view_models::Feature>>(HttpMethod::Get, "/api/FKDocuments/DocumentRecordFeatures", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "yearId", &self.year_id)?;
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/FKDocuments/DocumentRecordFeatures",
                query,
            )
            .await
        }
    }
}

pub mod get_document_types {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/FKDocuments/DocumentTypes`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output = Vec<crate::web_api::interface::fkf::documents::view_models::DocumentType>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<Vec<crate::web_api::interface::fkf::documents::view_models::DocumentType>>(HttpMethod::Get, "/api/FKDocuments/DocumentTypes", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(HttpMethod::Get, "/api/FKDocuments/DocumentTypes", query)
                .await
        }
    }

    /// `GET` `/api/FKDocuments/DocumentTypes`
    ///
    /// Parameter `year_id`: Id roku.
    pub struct ByYearId {
        /// Id roku.
        pub year_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByYearId {
        type Output = Vec<crate::web_api::interface::fkf::documents::view_models::DocumentType>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "yearId", &self.year_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::fkf::documents::view_models::DocumentType>>(HttpMethod::Get, "/api/FKDocuments/DocumentTypes", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "yearId", &self.year_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/FKDocuments/DocumentTypes", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IFKDocumentsController {
    /// `GET` `/api/FKDocuments`
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/FKDocuments`
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/FKDocuments/Buffer`
    ///
    /// Parameter `year_id`: Id roku.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetFromBuffer<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_from_buffer::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/FKDocuments/Buffer`
    ///
    /// Parameter `year_id`: Id roku.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetFromBufferRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_from_buffer::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/FKDocuments/DocumentFeatures`
    pub async fn GetDocumentFeatures<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_document_features::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/FKDocuments/DocumentFeatures`
    pub async fn GetDocumentFeaturesRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_document_features::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/FKDocuments/DocumentRecordFeatures`
    pub async fn GetDocumentRecordFeatures<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_document_record_features::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/FKDocuments/DocumentRecordFeatures`
    pub async fn GetDocumentRecordFeaturesRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_document_record_features::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/FKDocuments/DocumentTypes`
    pub async fn GetDocumentTypes<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_document_types::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/FKDocuments/DocumentTypes`
    pub async fn GetDocumentTypesRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_document_types::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/FKDocuments`
    ///
    /// Parameter `id`: Id dokumentu.
    ///
    /// Parameter `year_id`: Id roku.
    pub async fn GetV2(
        api: &ApiClient,
        id: i32,
        year_id: i32,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::fkf::documents::view_models::DocumentV2>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        push_query_param(&mut query, "yearId", &year_id)?;
        api.request_no_body::<crate::web_api::interface::fkf::documents::view_models::DocumentV2>(
            HttpMethod::Get,
            "/api/FKDocuments",
            query,
        )
        .await
    }

    /// `GET` `/api/FKDocuments`
    ///
    /// Parameter `id`: Id dokumentu.
    ///
    /// Parameter `year_id`: Id roku.
    pub async fn GetV2Raw(
        api: &ApiClient,
        id: i32,
        year_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        push_query_param(&mut query, "yearId", &year_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKDocuments", query)
            .await
    }

    /// `GET` `/api/FKDocuments/AccountingBooks`
    ///
    /// Parameter `year_id`: Id roku.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetFromAccountingBooks(
        api: &ApiClient,
        year_id: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::documents::view_models::DocumentListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "yearId", &year_id)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::documents::view_models::DocumentListElement>>(HttpMethod::Get, "/api/FKDocuments/AccountingBooks", query).await
    }

    /// `GET` `/api/FKDocuments/AccountingBooks`
    ///
    /// Parameter `year_id`: Id roku.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetFromAccountingBooksRaw(
        api: &ApiClient,
        year_id: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "yearId", &year_id)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKDocuments/AccountingBooks", query)
            .await
    }

    /// `GET` `/api/FKDocuments/Schemes`
    ///
    /// Parameter `year_id`: Id roku.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetFromSchemes(
        api: &ApiClient,
        year_id: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::documents::view_models::DocumentListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "yearId", &year_id)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::documents::view_models::DocumentListElement>>(HttpMethod::Get, "/api/FKDocuments/Schemes", query).await
    }

    /// `GET` `/api/FKDocuments/Schemes`
    ///
    /// Parameter `year_id`: Id roku.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie dokumenty od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetFromSchemesRaw(
        api: &ApiClient,
        year_id: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "yearId", &year_id)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKDocuments/Schemes", query)
            .await
    }

    /// `GET` `/api/FKDocuments/Markers`
    pub async fn GetMarkers(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::fkf::common::view_models::Marker>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::fkf::common::view_models::Marker>>(
            HttpMethod::Get,
            "/api/FKDocuments/Markers",
            query,
        )
        .await
    }

    /// `GET` `/api/FKDocuments/Markers`
    pub async fn GetMarkersRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/FKDocuments/Markers", query)
            .await
    }

    /// `GET` `/api/FKDocuments/Page`
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
                crate::web_api::interface::fkf::documents::view_models::DocumentListElement,
            >,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body::<crate::web_api::interface::view_models::Page<
            crate::web_api::interface::fkf::documents::view_models::DocumentListElement,
        >>(HttpMethod::Get, "/api/FKDocuments/Page", query)
            .await
    }

    /// `GET` `/api/FKDocuments/Page`
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
        api.request_no_body_raw(HttpMethod::Get, "/api/FKDocuments/Page", query)
            .await
    }

    /// `GET` `/api/FKDocuments/Transactions`
    pub async fn GetDocumentTransactions(api: &ApiClient) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::fkf::documents::view_models::transactions::TransactionDocument>>, ApiError>{
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::fkf::documents::view_models::transactions::TransactionDocument>>(HttpMethod::Get, "/api/FKDocuments/Transactions", query).await
    }

    /// `GET` `/api/FKDocuments/Transactions`
    pub async fn GetDocumentTransactionsRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/FKDocuments/Transactions", query)
            .await
    }
}
