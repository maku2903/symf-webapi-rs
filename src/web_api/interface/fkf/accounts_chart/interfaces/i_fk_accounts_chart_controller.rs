use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `FKAccountsChart`.
pub struct IFKAccountsChartController;

#[allow(non_snake_case)]
impl IFKAccountsChartController {
    /// `GET` `/api/FKAccountsChart/FlatList`
    ///
    /// Parameter `year_id`: Id roku.
    pub async fn GetFlatList(api: &ApiClient, year_id: i32) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::fkf::accounts_chart::view_models::AccountChartElementSimple>>, ApiError>{
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "yearId", &year_id)?;
        api.request_no_body::<Vec<
            crate::web_api::interface::fkf::accounts_chart::view_models::AccountChartElementSimple,
        >>(HttpMethod::Get, "/api/FKAccountsChart/FlatList", query)
            .await
    }

    /// `GET` `/api/FKAccountsChart/FlatList`
    ///
    /// Parameter `year_id`: Id roku.
    pub async fn GetFlatListRaw(
        api: &ApiClient,
        year_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "yearId", &year_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKAccountsChart/FlatList", query)
            .await
    }

    /// `GET` `/api/FKAccountsChart/FlatList`
    ///
    /// Parameter `year_id`: Id roku.
    pub async fn GetTreeList(
        api: &ApiClient,
        year_id: i32,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::fkf::accounts_chart::view_models::AccountChartElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "yearId", &year_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::accounts_chart::view_models::AccountChartElement>>(HttpMethod::Get, "/api/FKAccountsChart/FlatList", query).await
    }

    /// `GET` `/api/FKAccountsChart/FlatList`
    ///
    /// Parameter `year_id`: Id roku.
    pub async fn GetTreeListRaw(
        api: &ApiClient,
        year_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "yearId", &year_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/FKAccountsChart/FlatList", query)
            .await
    }
}
