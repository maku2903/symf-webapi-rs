use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `InventoryStates`.
pub struct IInventoryStatesController;

#[allow(non_snake_case)]
impl IInventoryStatesController {
    /// `GET` `/api/InventoryStates`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::inventory_states::view_models::InventoryState>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::inventory_states::view_models::InventoryState>>(HttpMethod::Get, "/api/InventoryStates", query).await
    }

    /// `GET` `/api/InventoryStates`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/InventoryStates", query)
            .await
    }

    /// `GET` `/api/InventoryStates/ByProduct`
    ///
    /// Parameter `id`: Id towaru.
    pub async fn GetByProductId(
        api: &ApiClient,
        id: i32,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::inventory_states::view_models::InventoryState>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body::<Vec<crate::web_api::interface::inventory_states::view_models::InventoryState>>(HttpMethod::Get, "/api/InventoryStates/ByProduct", query).await
    }

    /// `GET` `/api/InventoryStates/ByProduct`
    ///
    /// Parameter `id`: Id towaru.
    pub async fn GetByProductIdRaw(
        api: &ApiClient,
        id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/InventoryStates/ByProduct", query)
            .await
    }

    /// `GET` `/api/InventoryStates/ByProduct`
    ///
    /// Parameter `code`: Kod towaru.
    pub async fn GetByProductCode(
        api: &ApiClient,
        code: String,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::inventory_states::view_models::InventoryState>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "code", &code)?;
        api.request_no_body::<Vec<crate::web_api::interface::inventory_states::view_models::InventoryState>>(HttpMethod::Get, "/api/InventoryStates/ByProduct", query).await
    }

    /// `GET` `/api/InventoryStates/ByProduct`
    ///
    /// Parameter `code`: Kod towaru.
    pub async fn GetByProductCodeRaw(
        api: &ApiClient,
        code: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "code", &code)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/InventoryStates/ByProduct", query)
            .await
    }

    /// `GET` `/api/InventoryStates/ByWarehouse`
    ///
    /// Parameter `id`: Id magazynu.
    pub async fn GetByWarehouseId(
        api: &ApiClient,
        id: i32,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::inventory_states::view_models::InventoryState>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body::<Vec<crate::web_api::interface::inventory_states::view_models::InventoryState>>(HttpMethod::Get, "/api/InventoryStates/ByWarehouse", query).await
    }

    /// `GET` `/api/InventoryStates/ByWarehouse`
    ///
    /// Parameter `id`: Id magazynu.
    pub async fn GetByWarehouseIdRaw(
        api: &ApiClient,
        id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/InventoryStates/ByWarehouse", query)
            .await
    }

    /// `GET` `/api/InventoryStates/ByWarehouse`
    ///
    /// Parameter `code`: Kod magazynu.
    pub async fn GetByWarehouseCode(
        api: &ApiClient,
        code: String,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::inventory_states::view_models::InventoryState>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "code", &code)?;
        api.request_no_body::<Vec<crate::web_api::interface::inventory_states::view_models::InventoryState>>(HttpMethod::Get, "/api/InventoryStates/ByWarehouse", query).await
    }

    /// `GET` `/api/InventoryStates/ByWarehouse`
    ///
    /// Parameter `code`: Kod magazynu.
    pub async fn GetByWarehouseCodeRaw(
        api: &ApiClient,
        code: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "code", &code)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/InventoryStates/ByWarehouse", query)
            .await
    }

    /// `GET` `/api/InventoryStates/ByProductAndWarehouse`
    ///
    /// Parameter `product_id`: Id towaru.
    ///
    /// Parameter `warehouse_id`: Id magazynu.
    pub async fn GetByProductIdAndWarehouseId(
        api: &ApiClient,
        product_id: i32,
        warehouse_id: i32,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::inventory_states::view_models::InventoryState>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "productId", &product_id)?;
        push_query_param(&mut query, "warehouseId", &warehouse_id)?;
        api.request_no_body::<crate::web_api::interface::inventory_states::view_models::InventoryState>(HttpMethod::Get, "/api/InventoryStates/ByProductAndWarehouse", query).await
    }

    /// `GET` `/api/InventoryStates/ByProductAndWarehouse`
    ///
    /// Parameter `product_id`: Id towaru.
    ///
    /// Parameter `warehouse_id`: Id magazynu.
    pub async fn GetByProductIdAndWarehouseIdRaw(
        api: &ApiClient,
        product_id: i32,
        warehouse_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "productId", &product_id)?;
        push_query_param(&mut query, "warehouseId", &warehouse_id)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/InventoryStates/ByProductAndWarehouse",
            query,
        )
        .await
    }

    /// `GET` `/api/InventoryStates/Changes`
    ///
    /// Parameter `date`: Data od kiedy mają zostać pobrane zmiany.
    pub async fn GetChanges(
        api: &ApiClient,
        date: chrono::NaiveDateTime,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::inventory_states::view_models::InventoryState>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "date", &date)?;
        api.request_no_body::<Vec<crate::web_api::interface::inventory_states::view_models::InventoryState>>(HttpMethod::Get, "/api/InventoryStates/Changes", query).await
    }

    /// `GET` `/api/InventoryStates/Changes`
    ///
    /// Parameter `date`: Data od kiedy mają zostać pobrane zmiany.
    pub async fn GetChangesRaw(
        api: &ApiClient,
        date: chrono::NaiveDateTime,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "date", &date)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/InventoryStates/Changes", query)
            .await
    }
}
