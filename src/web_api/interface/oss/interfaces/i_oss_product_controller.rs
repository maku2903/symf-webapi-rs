use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `OssProduct`.
pub struct IOssProductController;

#[allow(non_snake_case)]
impl IOssProductController {
    /// `GET` `/api/OssProduct`
    pub async fn Get(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::oss::view_models::CountryProductList>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<crate::web_api::interface::oss::view_models::CountryProductList>(
            HttpMethod::Get,
            "/api/OssProduct",
            query,
        )
        .await
    }

    /// `GET` `/api/OssProduct`
    pub async fn GetRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/OssProduct", query)
            .await
    }

    /// `GET` `/api/OssProduct`
    ///
    /// Parameter `country_id`: Id kraju.
    pub async fn GetByCountry(
        api: &ApiClient,
        country_id: i32,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::oss::view_models::CountryProductList>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "countryId", &country_id)?;
        api.request_no_body::<crate::web_api::interface::oss::view_models::CountryProductList>(
            HttpMethod::Get,
            "/api/OssProduct",
            query,
        )
        .await
    }

    /// `GET` `/api/OssProduct`
    ///
    /// Parameter `country_id`: Id kraju.
    pub async fn GetByCountryRaw(
        api: &ApiClient,
        country_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "countryId", &country_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/OssProduct", query)
            .await
    }

    /// `GET` `/api/OssProduct`
    ///
    /// Parameter `erp_product_id`: Id towaru z programu Handel.
    pub async fn GetByErpProduct(
        api: &ApiClient,
        erp_product_id: i32,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::oss::view_models::CountryProductList>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "erpProductId", &erp_product_id)?;
        api.request_no_body::<crate::web_api::interface::oss::view_models::CountryProductList>(
            HttpMethod::Get,
            "/api/OssProduct",
            query,
        )
        .await
    }

    /// `GET` `/api/OssProduct`
    ///
    /// Parameter `erp_product_id`: Id towaru z programu Handel.
    pub async fn GetByErpProductRaw(
        api: &ApiClient,
        erp_product_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "erpProductId", &erp_product_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/OssProduct", query)
            .await
    }

    /// `GET` `/api/OssProduct`
    ///
    /// Parameter `shop_product_id`: Id towaru ze sklepu internetowego.
    pub async fn GetByShopProduct(
        api: &ApiClient,
        shop_product_id: String,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::oss::view_models::CountryProductList>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "shopProductId", &shop_product_id)?;
        api.request_no_body::<crate::web_api::interface::oss::view_models::CountryProductList>(
            HttpMethod::Get,
            "/api/OssProduct",
            query,
        )
        .await
    }

    /// `GET` `/api/OssProduct`
    ///
    /// Parameter `shop_product_id`: Id towaru ze sklepu internetowego.
    pub async fn GetByShopProductRaw(
        api: &ApiClient,
        shop_product_id: String,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "shopProductId", &shop_product_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/OssProduct", query)
            .await
    }
}
