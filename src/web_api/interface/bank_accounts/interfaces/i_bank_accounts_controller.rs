use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `BankAccounts`.
pub struct IBankAccountsController;

pub mod get_list_by_contractor {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/BankAccounts`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    pub struct ByContractorId {
        /// Id kontrahenta.
        pub contractor_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorId {
        type Output = Vec<crate::web_api::interface::bank_accounts::view_models::BankAccount>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::bank_accounts::view_models::BankAccount>>(HttpMethod::Get, "/api/BankAccounts", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/BankAccounts", query)
                .await
        }
    }

    /// `GET` `/api/BankAccounts`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    pub struct ByContractorCode {
        /// Kod kontrahenta.
        pub contractor_code: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorCode {
        type Output = Vec<crate::web_api::interface::bank_accounts::view_models::BankAccount>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_no_body::<Vec<crate::web_api::interface::bank_accounts::view_models::BankAccount>>(HttpMethod::Get, "/api/BankAccounts", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/BankAccounts", query)
                .await
        }
    }
}

pub mod insert {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `POST` `/api/BankAccounts/Create`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Body `bank_account`: Obiekt rachunku bankowego do utworzenia.
    pub struct ByContractorId<'a> {
        /// Id kontrahenta.
        pub contractor_id: i32,
        /// Obiekt rachunku bankowego do utworzenia.
        pub bank_account: &'a crate::web_api::interface::bank_accounts::view_models::BankAccount,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByContractorId<'a> {
        type Output = crate::web_api::interface::bank_accounts::view_models::BankAccount;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_with_body::<crate::web_api::interface::bank_accounts::view_models::BankAccount, _>(HttpMethod::Post, "/api/BankAccounts/Create", query, self.bank_account).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_with_body_raw(
                HttpMethod::Post,
                "/api/BankAccounts/Create",
                query,
                self.bank_account,
            )
            .await
        }
    }

    /// `POST` `/api/BankAccounts/Create`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    ///
    /// Body `bank_account`: Obiekt rachunku bankowego do utworzenia.
    pub struct ByContractorCode<'a> {
        /// Kod kontrahenta.
        pub contractor_code: String,
        /// Obiekt rachunku bankowego do utworzenia.
        pub bank_account: &'a crate::web_api::interface::bank_accounts::view_models::BankAccount,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByContractorCode<'a> {
        type Output = crate::web_api::interface::bank_accounts::view_models::BankAccount;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_with_body::<crate::web_api::interface::bank_accounts::view_models::BankAccount, _>(HttpMethod::Post, "/api/BankAccounts/Create", query, self.bank_account).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_with_body_raw(
                HttpMethod::Post,
                "/api/BankAccounts/Create",
                query,
                self.bank_account,
            )
            .await
        }
    }
}

#[allow(non_snake_case)]
impl IBankAccountsController {
    /// `GET` `/api/BankAccounts`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    pub async fn GetListByContractor<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_list_by_contractor::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/BankAccounts`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    pub async fn GetListByContractorRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_list_by_contractor::Overload,
    {
        overload.call_raw(api).await
    }

    /// `POST` `/api/BankAccounts/Create`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Body `bank_account`: Obiekt rachunku bankowego do utworzenia.
    pub async fn Insert<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: insert::Overload,
    {
        overload.call(api).await
    }

    /// `POST` `/api/BankAccounts/Create`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    ///
    /// Body `bank_account`: Obiekt rachunku bankowego do utworzenia.
    pub async fn InsertRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: insert::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/BankAccounts`
    ///
    /// Parameter `id`: Id rachunku bankowego.
    pub async fn Get(
        api: &ApiClient,
        id: i32,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::bank_accounts::view_models::BankAccount>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body::<crate::web_api::interface::bank_accounts::view_models::BankAccount>(
            HttpMethod::Get,
            "/api/BankAccounts",
            query,
        )
        .await
    }

    /// `GET` `/api/BankAccounts`
    ///
    /// Parameter `id`: Id rachunku bankowego.
    pub async fn GetRaw(api: &ApiClient, id: i32) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/BankAccounts", query)
            .await
    }

    /// `PUT` `/api/BankAccounts/Update`
    ///
    /// Body `bank_account`: Obiekt rachunku bankowego do aktualizacji.
    pub async fn Update(
        api: &ApiClient,
        bank_account: &crate::web_api::interface::bank_accounts::view_models::BankAccount,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let query = vec![];
        api.request_with_body::<(), _>(
            HttpMethod::Put,
            "/api/BankAccounts/Update",
            query,
            bank_account,
        )
        .await
    }

    /// `PUT` `/api/BankAccounts/Update`
    ///
    /// Body `bank_account`: Obiekt rachunku bankowego do aktualizacji.
    pub async fn UpdateRaw(
        api: &ApiClient,
        bank_account: &crate::web_api::interface::bank_accounts::view_models::BankAccount,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Put,
            "/api/BankAccounts/Update",
            query,
            bank_account,
        )
        .await
    }

    /// `PATCH` `/api/BankAccounts/SetAsMain`
    ///
    /// Parameter `id`: Id rachunku bankowego.
    pub async fn SetAsMain(api: &ApiClient, id: i32) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body::<()>(HttpMethod::Patch, "/api/BankAccounts/SetAsMain", query)
            .await
    }

    /// `PATCH` `/api/BankAccounts/SetAsMain`
    ///
    /// Parameter `id`: Id rachunku bankowego.
    pub async fn SetAsMainRaw(
        api: &ApiClient,
        id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body_raw(HttpMethod::Patch, "/api/BankAccounts/SetAsMain", query)
            .await
    }
}
