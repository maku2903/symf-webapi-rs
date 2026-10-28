use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `OssProductMaps`.
pub struct IOssProductMapsController;

#[allow(non_snake_case)]
impl IOssProductMapsController {
    /// `POST` `/api/OssProductMap/Create`
    ///
    /// Body `map`: Korelacja towarów.
    pub async fn Create(
        api: &ApiClient,
        map: &crate::web_api::interface::oss::view_models::ProductMapCreate,
    ) -> Result<ResponseEnvelope<crate::web_api::interface::oss::view_models::ProductMap>, ApiError>
    {
        let query = vec![];
        api.request_with_body::<crate::web_api::interface::oss::view_models::ProductMap, _>(
            HttpMethod::Post,
            "/api/OssProductMap/Create",
            query,
            map,
        )
        .await
    }

    /// `POST` `/api/OssProductMap/Create`
    ///
    /// Body `map`: Korelacja towarów.
    pub async fn CreateRaw(
        api: &ApiClient,
        map: &crate::web_api::interface::oss::view_models::ProductMapCreate,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(HttpMethod::Post, "/api/OssProductMap/Create", query, map)
            .await
    }

    /// `DELETE` `/api/OssProductMap/Delete`
    ///
    /// Parameter `id`: Id korelacji.
    pub async fn Delete(api: &ApiClient, id: i32) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body::<()>(HttpMethod::Delete, "/api/OssProductMap/Delete", query)
            .await
    }

    /// `DELETE` `/api/OssProductMap/Delete`
    ///
    /// Parameter `id`: Id korelacji.
    pub async fn DeleteRaw(api: &ApiClient, id: i32) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body_raw(HttpMethod::Delete, "/api/OssProductMap/Delete", query)
            .await
    }

    /// `GET` `/api/OssProductMap`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::oss::view_models::ProductMap>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::oss::view_models::ProductMap>>(
            HttpMethod::Get,
            "/api/OssProductMap",
            query,
        )
        .await
    }

    /// `GET` `/api/OssProductMap`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/OssProductMap", query)
            .await
    }

    /// `GET` `/api/OssProductMap`
    ///
    /// Parameter `country_id`: Id kraju.
    pub async fn GetByCountry(
        api: &ApiClient,
        country_id: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::oss::view_models::ProductMap>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "countryId", &country_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::oss::view_models::ProductMap>>(
            HttpMethod::Get,
            "/api/OssProductMap",
            query,
        )
        .await
    }

    /// `GET` `/api/OssProductMap`
    ///
    /// Parameter `country_id`: Id kraju.
    pub async fn GetByCountryRaw(
        api: &ApiClient,
        country_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "countryId", &country_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/OssProductMap", query)
            .await
    }

    /// `GET` `/api/OssProductMap`
    ///
    /// Parameter `erp_product_id`: Id towaru z programu Handel.
    pub async fn GetByErpProduct(
        api: &ApiClient,
        erp_product_id: i32,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::oss::view_models::ProductMap>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "erpProductId", &erp_product_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::oss::view_models::ProductMap>>(
            HttpMethod::Get,
            "/api/OssProductMap",
            query,
        )
        .await
    }

    /// `GET` `/api/OssProductMap`
    ///
    /// Parameter `erp_product_id`: Id towaru z programu Handel.
    pub async fn GetByErpProductRaw(
        api: &ApiClient,
        erp_product_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "erpProductId", &erp_product_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/OssProductMap", query)
            .await
    }

    /// `GET` `/api/OssProductMap`
    ///
    /// Parameter `shop_product_id`: Id towaru ze sklepu internetowego.
    pub async fn GetByShopProduct(
        api: &ApiClient,
        shop_product_id: String,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::oss::view_models::ProductMap>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "shopProductId", &shop_product_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::oss::view_models::ProductMap>>(
            HttpMethod::Get,
            "/api/OssProductMap",
            query,
        )
        .await
    }

    /// `GET` `/api/OssProductMap`
    ///
    /// Parameter `shop_product_id`: Id towaru ze sklepu internetowego.
    pub async fn GetByShopProductRaw(
        api: &ApiClient,
        shop_product_id: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "shopProductId", &shop_product_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/OssProductMap", query)
            .await
    }
}
