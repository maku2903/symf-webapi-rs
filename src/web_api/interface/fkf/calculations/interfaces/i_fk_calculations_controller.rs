use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `FKCalculations`.
pub struct IFKCalculationsController;

#[allow(non_snake_case)]
impl IFKCalculationsController {
    /// `GET` `/api/FKCalculations`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations", query).await
    }

    /// `GET` `/api/FKCalculations`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/FKCalculations", query)
            .await
    }

    /// `GET` `/api/FKCalculations/CustomFilter`
    ///
    /// Parameter `options`: Obiekt parametrów filtru.
    pub async fn Custom(
        api: &ApiClient,
        options: crate::web_api::interface::fkf::calculations::view_models::CalculationFilterOptions,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "options", &options)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/CustomFilter", query).await
    }

    /// `GET` `/api/FKCalculations/CustomFilter`
    ///
    /// Parameter `options`: Obiekt parametrów filtru.
    pub async fn CustomRaw(
        api: &ApiClient,
        options: crate::web_api::interface::fkf::calculations::view_models::CalculationFilterOptions,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "options", &options)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKCalculations/CustomFilter", query)
            .await
    }

    /// `GET` `/api/FKCalculations/Filter`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetByContractor(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter", query).await
    }

    /// `GET` `/api/FKCalculations/Filter`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetByContractorRaw(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKCalculations/Filter", query)
            .await
    }

    /// `GET` `/api/FKCalculations/Filter/NotSettled`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetNotSettledByContractor(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/NotSettled", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/NotSettled`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetNotSettledByContractorRaw(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/NotSettled",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter/BeforeMaturityTerm`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetBeforeMaturityTermByContractor(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/BeforeMaturityTerm", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/BeforeMaturityTerm`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetBeforeMaturityTermByContractorRaw(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/BeforeMaturityTerm",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter/AfterMaturityTerm`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetAfterMaturityTermByContractor(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/AfterMaturityTerm", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/AfterMaturityTerm`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetAfterMaturityTermByContractorRaw(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/AfterMaturityTerm",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter/Due`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueByContractor(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/Due", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/Due`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueByContractorRaw(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKCalculations/Filter/Due", query)
            .await
    }

    /// `GET` `/api/FKCalculations/Filter/Due/NotSettled`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueNotSettledByContractor(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/Due/NotSettled", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/Due/NotSettled`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueNotSettledByContractorRaw(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/Due/NotSettled",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter/Due/BeforeMaturityTerm`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueBeforeMaturityTermByContractor(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/Due/BeforeMaturityTerm", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/Due/BeforeMaturityTerm`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueBeforeMaturityTermByContractorRaw(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/Due/BeforeMaturityTerm",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter/Due/AfterMaturityTerm`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueAfterMaturityTermByContractor(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/Due/AfterMaturityTerm", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/Due/AfterMaturityTerm`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueAfterMaturityTermByContractorRaw(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/Due/AfterMaturityTerm",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter/Obligation`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationByContractor(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/Obligation", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/Obligation`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationByContractorRaw(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/Obligation",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter/Obligation/NotSettled`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationNotSettledByContractor(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/Obligation/NotSettled", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/Obligation/NotSettled`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationNotSettledByContractorRaw(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/Obligation/NotSettled",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter/Obligation/BeforeMaturityTerm`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationBeforeMaturityTermByContractor(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/Obligation/BeforeMaturityTerm", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/Obligation/BeforeMaturityTerm`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationBeforeMaturityTermByContractorRaw(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/Obligation/BeforeMaturityTerm",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter/Obligation/AfterMaturityTerm`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationAfterMaturityTermByContractor(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/Obligation/AfterMaturityTerm", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/Obligation/AfterMaturityTerm`
    ///
    /// Parameter `contractor_position`: Pozycja kontrahenta.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationAfterMaturityTermByContractorRaw(
        api: &ApiClient,
        contractor_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorPosition", &contractor_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/Obligation/AfterMaturityTerm",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetByEmployee(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter", query).await
    }

    /// `GET` `/api/FKCalculations/Filter`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetByEmployeeRaw(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKCalculations/Filter", query)
            .await
    }

    /// `GET` `/api/FKCalculations/Filter/NotSettled`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetNotSettledByEmployee(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/NotSettled", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/NotSettled`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetNotSettledByEmployeeRaw(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/NotSettled",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter/BeforeMaturityTerm`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetBeforeMaturityTermByEmployee(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/BeforeMaturityTerm", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/BeforeMaturityTerm`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetBeforeMaturityTermByEmployeeRaw(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/BeforeMaturityTerm",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter/AfterMaturityTerm`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetAfterMaturityTermByEmployee(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/AfterMaturityTerm", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/AfterMaturityTerm`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetAfterMaturityTermByEmployeeRaw(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/AfterMaturityTerm",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter/Due`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueByEmployee(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/Due", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/Due`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueByEmployeeRaw(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKCalculations/Filter/Due", query)
            .await
    }

    /// `GET` `/api/FKCalculations/Filter/Due/NotSettled`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueNotSettledByEmployee(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/Due/NotSettled", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/Due/NotSettled`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueNotSettledByEmployeeRaw(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/Due/NotSettled",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter/Due/BeforeMaturityTerm`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueBeforeMaturityTermByEmployee(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/Due/BeforeMaturityTerm", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/Due/BeforeMaturityTerm`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueBeforeMaturityTermByEmployeeRaw(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/Due/BeforeMaturityTerm",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter/Due/AfterMaturityTerm`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueAfterMaturityTermByEmployee(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/Due/AfterMaturityTerm", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/Due/AfterMaturityTerm`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueAfterMaturityTermByEmployeeRaw(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/Due/AfterMaturityTerm",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter/Obligation`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationByEmployee(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/Obligation", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/Obligation`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationByEmployeeRaw(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/Obligation",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter/Obligation/NotSettled`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationNotSettledByEmployee(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/Obligation/NotSettled", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/Obligation/NotSettled`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationNotSettledByEmployeeRaw(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/Obligation/NotSettled",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter/Obligation/BeforeMaturityTerm`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationBeforeMaturityTermByEmployee(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/Obligation/BeforeMaturityTerm", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/Obligation/BeforeMaturityTerm`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationBeforeMaturityTermByEmployeeRaw(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/Obligation/BeforeMaturityTerm",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Filter/Obligation/AfterMaturityTerm`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationAfterMaturityTermByEmployee(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElement>>(HttpMethod::Get, "/api/FKCalculations/Filter/Obligation/AfterMaturityTerm", query).await
    }

    /// `GET` `/api/FKCalculations/Filter/Obligation/AfterMaturityTerm`
    ///
    /// Parameter `employee_position`: Pozycja pracownika.
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationAfterMaturityTermByEmployeeRaw(
        api: &ApiClient,
        employee_position: i32,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "employeePosition", &employee_position)?;
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Filter/Obligation/AfterMaturityTerm",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Grouped/Due`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueGroupedBySubject(api: &ApiClient, date_from: Option<chrono::NaiveDateTime>, date_to: Option<chrono::NaiveDateTime>) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElementGrouped>>, ApiError>{
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElementGrouped>>(HttpMethod::Get, "/api/FKCalculations/Grouped/Due", query).await
    }

    /// `GET` `/api/FKCalculations/Grouped/Due`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueGroupedBySubjectRaw(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKCalculations/Grouped/Due", query)
            .await
    }

    /// `GET` `/api/FKCalculations/Grouped/Due/NotSettled`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueNotSettledGroupedBySubject(api: &ApiClient, date_from: Option<chrono::NaiveDateTime>, date_to: Option<chrono::NaiveDateTime>) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElementGrouped>>, ApiError>{
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElementGrouped>>(HttpMethod::Get, "/api/FKCalculations/Grouped/Due/NotSettled", query).await
    }

    /// `GET` `/api/FKCalculations/Grouped/Due/NotSettled`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueNotSettledGroupedBySubjectRaw(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Grouped/Due/NotSettled",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Grouped/Due/BeforeMaturityTerm`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueBeforeMaturityTermGroupedBySubject(api: &ApiClient, date_from: Option<chrono::NaiveDateTime>, date_to: Option<chrono::NaiveDateTime>) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElementGrouped>>, ApiError>{
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElementGrouped>>(HttpMethod::Get, "/api/FKCalculations/Grouped/Due/BeforeMaturityTerm", query).await
    }

    /// `GET` `/api/FKCalculations/Grouped/Due/BeforeMaturityTerm`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueBeforeMaturityTermGroupedBySubjectRaw(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Grouped/Due/BeforeMaturityTerm",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Grouped/Due/AfterMaturityTerm`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueAfterMaturityTermGroupedBySubject(api: &ApiClient, date_from: Option<chrono::NaiveDateTime>, date_to: Option<chrono::NaiveDateTime>) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElementGrouped>>, ApiError>{
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElementGrouped>>(HttpMethod::Get, "/api/FKCalculations/Grouped/Due/AfterMaturityTerm", query).await
    }

    /// `GET` `/api/FKCalculations/Grouped/Due/AfterMaturityTerm`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetDueAfterMaturityTermGroupedBySubjectRaw(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Grouped/Due/AfterMaturityTerm",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Grouped/Obligation`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationGroupedBySubject(api: &ApiClient, date_from: Option<chrono::NaiveDateTime>, date_to: Option<chrono::NaiveDateTime>) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElementGrouped>>, ApiError>{
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElementGrouped>>(HttpMethod::Get, "/api/FKCalculations/Grouped/Obligation", query).await
    }

    /// `GET` `/api/FKCalculations/Grouped/Obligation`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationGroupedBySubjectRaw(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Grouped/Obligation",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Grouped/Obligation/NotSettled`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationNotSettledGroupedBySubject(api: &ApiClient, date_from: Option<chrono::NaiveDateTime>, date_to: Option<chrono::NaiveDateTime>) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElementGrouped>>, ApiError>{
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElementGrouped>>(HttpMethod::Get, "/api/FKCalculations/Grouped/Obligation/NotSettled", query).await
    }

    /// `GET` `/api/FKCalculations/Grouped/Obligation/NotSettled`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationNotSettledGroupedBySubjectRaw(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Grouped/Obligation/NotSettled",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Grouped/Obligation/BeforeMaturityTerm`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationBeforeMaturityTermGroupedBySubject(api: &ApiClient, date_from: Option<chrono::NaiveDateTime>, date_to: Option<chrono::NaiveDateTime>) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElementGrouped>>, ApiError>{
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElementGrouped>>(HttpMethod::Get, "/api/FKCalculations/Grouped/Obligation/BeforeMaturityTerm", query).await
    }

    /// `GET` `/api/FKCalculations/Grouped/Obligation/BeforeMaturityTerm`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationBeforeMaturityTermGroupedBySubjectRaw(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Grouped/Obligation/BeforeMaturityTerm",
            query,
        )
        .await
    }

    /// `GET` `/api/FKCalculations/Grouped/Obligation/AfterMaturityTerm`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationAfterMaturityTermGroupedBySubject(api: &ApiClient, date_from: Option<chrono::NaiveDateTime>, date_to: Option<chrono::NaiveDateTime>) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElementGrouped>>, ApiError>{
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::CalculationListElementGrouped>>(HttpMethod::Get, "/api/FKCalculations/Grouped/Obligation/AfterMaturityTerm", query).await
    }

    /// `GET` `/api/FKCalculations/Grouped/Obligation/AfterMaturityTerm`
    ///
    /// Parameter `date_from`: Data od przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki do wskazanej daty do. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    ///
    /// Parameter `date_to`: Data do przedziału czasowego. W przypadku, gdy parametr nie zostanie podany wyszukane zostaną wszystkie rozrachunki od wskazanej daty od. Data paramteru powinna zostać podana w formacie yyyy-MM-dd.
    pub async fn GetObligationAfterMaturityTermGroupedBySubjectRaw(
        api: &ApiClient,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        push_query_param(&mut query, "dateTo", &date_to)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculations/Grouped/Obligation/AfterMaturityTerm",
            query,
        )
        .await
    }
}
