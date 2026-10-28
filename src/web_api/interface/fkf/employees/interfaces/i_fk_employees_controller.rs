use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `FKEmployees`.
pub struct IFKEmployeesController;

#[allow(non_snake_case)]
impl IFKEmployeesController {
    /// `GET` `/api/FKEmployees`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::employees::view_models::EmployeeListElement>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::fkf::employees::view_models::EmployeeListElement>>(HttpMethod::Get, "/api/FKEmployees", query).await
    }

    /// `GET` `/api/FKEmployees`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/FKEmployees", query)
            .await
    }

    /// `GET` `/api/FKEmployees`
    ///
    /// Parameter `id`: Id pracownika.
    pub async fn GetById(
        api: &ApiClient,
        id: i32,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::fkf::employees::view_models::Employee>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body::<crate::web_api::interface::fkf::employees::view_models::Employee>(
            HttpMethod::Get,
            "/api/FKEmployees",
            query,
        )
        .await
    }

    /// `GET` `/api/FKEmployees`
    ///
    /// Parameter `id`: Id pracownika.
    pub async fn GetByIdRaw(api: &ApiClient, id: i32) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKEmployees", query)
            .await
    }

    /// `GET` `/api/FKEmployees`
    ///
    /// Parameter `position`: Pozycja pracownika.
    pub async fn GetByPosition(
        api: &ApiClient,
        position: i32,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::fkf::employees::view_models::Employee>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "position", &position)?;
        api.request_no_body::<crate::web_api::interface::fkf::employees::view_models::Employee>(
            HttpMethod::Get,
            "/api/FKEmployees",
            query,
        )
        .await
    }

    /// `GET` `/api/FKEmployees`
    ///
    /// Parameter `position`: Pozycja pracownika.
    pub async fn GetByPositionRaw(
        api: &ApiClient,
        position: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "position", &position)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKEmployees", query)
            .await
    }

    /// `GET` `/api/FKEmployees`
    ///
    /// Parameter `code`: Kod pracownika.
    pub async fn GetByCode(
        api: &ApiClient,
        code: String,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::fkf::employees::view_models::Employee>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "code", &code)?;
        api.request_no_body::<crate::web_api::interface::fkf::employees::view_models::Employee>(
            HttpMethod::Get,
            "/api/FKEmployees",
            query,
        )
        .await
    }

    /// `GET` `/api/FKEmployees`
    ///
    /// Parameter `code`: Kod pracownika.
    pub async fn GetByCodeRaw(
        api: &ApiClient,
        code: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "code", &code)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKEmployees", query)
            .await
    }

    /// `POST` `/api/FKEmployees/Create`
    ///
    /// Body `employee`: Obiekt pracownika do utworzenia.
    pub async fn AddNew(
        api: &ApiClient,
        employee: &crate::web_api::interface::fkf::employees::view_models::Employee,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::fkf::employees::view_models::Employee>,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<crate::web_api::interface::fkf::employees::view_models::Employee, _>(HttpMethod::Post, "/api/FKEmployees/Create", query, employee).await
    }

    /// `POST` `/api/FKEmployees/Create`
    ///
    /// Body `employee`: Obiekt pracownika do utworzenia.
    pub async fn AddNewRaw(
        api: &ApiClient,
        employee: &crate::web_api::interface::fkf::employees::view_models::Employee,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(HttpMethod::Post, "/api/FKEmployees/Create", query, employee)
            .await
    }

    /// `PUT` `/api/FKEmployees/Update`
    ///
    /// Body `employee`: Obiekt pracownika do aktualizacji.
    pub async fn Update(
        api: &ApiClient,
        employee: &crate::web_api::interface::fkf::employees::view_models::Employee,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let query = vec![];
        api.request_with_body::<(), _>(HttpMethod::Put, "/api/FKEmployees/Update", query, employee)
            .await
    }

    /// `PUT` `/api/FKEmployees/Update`
    ///
    /// Body `employee`: Obiekt pracownika do aktualizacji.
    pub async fn UpdateRaw(
        api: &ApiClient,
        employee: &crate::web_api::interface::fkf::employees::view_models::Employee,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(HttpMethod::Put, "/api/FKEmployees/Update", query, employee)
            .await
    }

    /// `GET` `/api/FKEmployees/Page`
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
                crate::web_api::interface::fkf::employees::view_models::EmployeeListElement,
            >,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body::<crate::web_api::interface::view_models::Page<
            crate::web_api::interface::fkf::employees::view_models::EmployeeListElement,
        >>(HttpMethod::Get, "/api/FKEmployees/Page", query)
            .await
    }

    /// `GET` `/api/FKEmployees/Page`
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
        api.request_no_body_raw(HttpMethod::Get, "/api/FKEmployees/Page", query)
            .await
    }
}
