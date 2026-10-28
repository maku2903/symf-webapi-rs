use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `FKCalculationsBusiness`.
pub struct IFKCalculationsBusinessController;

#[allow(non_snake_case)]
impl IFKCalculationsBusinessController {
    /// `GET` `/api/FKCalculationsBusiness/BalanceAndTurnover`
    ///
    /// Parameter `options`: Obiekt parametrów filtru elementów słowników.
    pub async fn BalanceAndTurnoverAsync(api: &ApiClient, options: crate::web_api::interface::fkf::calculations::view_models::BalanceAndTurnoverFilterOptions) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::fkf::calculations::view_models::BalanceAndTurnoverListElement>>, ApiError>{
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "options", &options)?;
        api.request_no_body::<Vec<crate::web_api::interface::fkf::calculations::view_models::BalanceAndTurnoverListElement>>(HttpMethod::Get, "/api/FKCalculationsBusiness/BalanceAndTurnover", query).await
    }

    /// `GET` `/api/FKCalculationsBusiness/BalanceAndTurnover`
    ///
    /// Parameter `options`: Obiekt parametrów filtru elementów słowników.
    pub async fn BalanceAndTurnoverAsyncRaw(
        api: &ApiClient,
        options: crate::web_api::interface::fkf::calculations::view_models::BalanceAndTurnoverFilterOptions,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "options", &options)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/FKCalculationsBusiness/BalanceAndTurnover",
            query,
        )
        .await
    }
}
