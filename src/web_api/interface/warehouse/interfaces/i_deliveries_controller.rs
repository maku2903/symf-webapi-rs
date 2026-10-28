use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `Deliveries`.
pub struct IDeliveriesController;

pub mod get_in_warehouse {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Deliveries/InWarehouse`
    ///
    /// Parameter `warehouse_id`: Id magazynu.
    ///
    /// Parameter `product_id`: Id towaru.
    pub struct ByWarehouseIdProductId {
        /// Id magazynu.
        pub warehouse_id: i32,
        /// Id towaru.
        pub product_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByWarehouseIdProductId {
        type Output = Vec<crate::web_api::interface::warehouse::view_models::DeliveryListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "warehouseId", &self.warehouse_id)?;
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::warehouse::view_models::DeliveryListElement>>(HttpMethod::Get, "/api/Deliveries/InWarehouse", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "warehouseId", &self.warehouse_id)?;
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Deliveries/InWarehouse", query)
                .await
        }
    }

    /// `GET` `/api/Deliveries/InWarehouse`
    ///
    /// Parameter `warehouse_code`: Kod magazynu.
    ///
    /// Parameter `product_code`: Kod towaru.
    pub struct ByWarehouseCodeProductCode {
        /// Kod magazynu.
        pub warehouse_code: String,
        /// Kod towaru.
        pub product_code: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByWarehouseCodeProductCode {
        type Output = Vec<crate::web_api::interface::warehouse::view_models::DeliveryListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "warehouseCode", &self.warehouse_code)?;
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body::<Vec<crate::web_api::interface::warehouse::view_models::DeliveryListElement>>(HttpMethod::Get, "/api/Deliveries/InWarehouse", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "warehouseCode", &self.warehouse_code)?;
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Deliveries/InWarehouse", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IDeliveriesController {
    /// `GET` `/api/Deliveries/InWarehouse`
    ///
    /// Parameter `warehouse_id`: Id magazynu.
    ///
    /// Parameter `product_id`: Id towaru.
    pub async fn GetInWarehouse<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_in_warehouse::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Deliveries/InWarehouse`
    ///
    /// Parameter `warehouse_id`: Id magazynu.
    ///
    /// Parameter `product_id`: Id towaru.
    pub async fn GetInWarehouseRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_in_warehouse::Overload,
    {
        overload.call_raw(api).await
    }
}
