use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `Reservations`.
pub struct IReservationsController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Reservations`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output =
            Vec<crate::web_api::interface::reservations::view_models::ReservationListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<Vec<crate::web_api::interface::reservations::view_models::ReservationListElement>>(HttpMethod::Get, "/api/Reservations", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(HttpMethod::Get, "/api/Reservations", query)
                .await
        }
    }

    /// `GET` `/api/Reservations`
    ///
    /// Parameter `id`: Id rezerwacji.
    pub struct ById {
        /// Id rezerwacji.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = crate::web_api::interface::reservations::view_models::Reservation;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<crate::web_api::interface::reservations::view_models::Reservation>(HttpMethod::Get, "/api/Reservations", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Reservations", query)
                .await
        }
    }
}

pub mod get_list_by_contractor {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Reservations/Filter`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    pub struct ByContractorId {
        /// Id kontrahenta.
        pub contractor_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorId {
        type Output =
            Vec<crate::web_api::interface::reservations::view_models::ReservationListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::reservations::view_models::ReservationListElement>>(HttpMethod::Get, "/api/Reservations/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Reservations/Filter", query)
                .await
        }
    }

    /// `GET` `/api/Reservations/Filter`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    pub struct ByContractorCode {
        /// Kod kontrahenta.
        pub contractor_code: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorCode {
        type Output =
            Vec<crate::web_api::interface::reservations::view_models::ReservationListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_no_body::<Vec<crate::web_api::interface::reservations::view_models::ReservationListElement>>(HttpMethod::Get, "/api/Reservations/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Reservations/Filter", query)
                .await
        }
    }
}

pub mod get_list_by_product {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Reservations/Filter`
    ///
    /// Parameter `product_id`: Id towaru.
    pub struct ByProductId {
        /// Id towaru.
        pub product_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByProductId {
        type Output =
            Vec<crate::web_api::interface::reservations::view_models::ReservationListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::reservations::view_models::ReservationListElement>>(HttpMethod::Get, "/api/Reservations/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Reservations/Filter", query)
                .await
        }
    }

    /// `GET` `/api/Reservations/Filter`
    ///
    /// Parameter `product_code`: Kod towaru.
    pub struct ByProductCode {
        /// Kod towaru.
        pub product_code: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByProductCode {
        type Output =
            Vec<crate::web_api::interface::reservations::view_models::ReservationListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body::<Vec<crate::web_api::interface::reservations::view_models::ReservationListElement>>(HttpMethod::Get, "/api/Reservations/Filter", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Reservations/Filter", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IReservationsController {
    /// `GET` `/api/Reservations`
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Reservations`
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Reservations/Filter`
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

    /// `GET` `/api/Reservations/Filter`
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

    /// `GET` `/api/Reservations/Filter`
    ///
    /// Parameter `product_id`: Id towaru.
    pub async fn GetListByProduct<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_list_by_product::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Reservations/Filter`
    ///
    /// Parameter `product_id`: Id towaru.
    pub async fn GetListByProductRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_list_by_product::Overload,
    {
        overload.call_raw(api).await
    }

    /// `POST` `/api/Reservations/Create`
    ///
    /// Body `reservation`: Obiekt rezerwacji do założenia.
    pub async fn AddNew(
        api: &ApiClient,
        reservation: &crate::web_api::interface::reservations::view_models::ReservationNew,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::reservations::view_models::Reservation>,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<crate::web_api::interface::reservations::view_models::Reservation, _>(HttpMethod::Post, "/api/Reservations/Create", query, reservation).await
    }

    /// `POST` `/api/Reservations/Create`
    ///
    /// Body `reservation`: Obiekt rezerwacji do założenia.
    pub async fn AddNewRaw(
        api: &ApiClient,
        reservation: &crate::web_api::interface::reservations::view_models::ReservationNew,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Post,
            "/api/Reservations/Create",
            query,
            reservation,
        )
        .await
    }

    /// `POST` `/api/Reservations/AdvancedCreate`
    ///
    /// Body `reservation`: Obiekt rezerwacji do założenia.
    pub async fn AdvancedAddNew(
        api: &ApiClient,
        reservation: &crate::web_api::interface::reservations::view_models::AdvancedReservationNew,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::reservations::view_models::Reservation>,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<crate::web_api::interface::reservations::view_models::Reservation, _>(HttpMethod::Post, "/api/Reservations/AdvancedCreate", query, reservation).await
    }

    /// `POST` `/api/Reservations/AdvancedCreate`
    ///
    /// Body `reservation`: Obiekt rezerwacji do założenia.
    pub async fn AdvancedAddNewRaw(
        api: &ApiClient,
        reservation: &crate::web_api::interface::reservations::view_models::AdvancedReservationNew,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Post,
            "/api/Reservations/AdvancedCreate",
            query,
            reservation,
        )
        .await
    }

    /// `DELETE` `/api/Reservations/Delete`
    ///
    /// Parameter `id`: Id rezerwacji.
    pub async fn Delete(api: &ApiClient, id: i32) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body::<()>(HttpMethod::Delete, "/api/Reservations/Delete", query)
            .await
    }

    /// `DELETE` `/api/Reservations/Delete`
    ///
    /// Parameter `id`: Id rezerwacji.
    pub async fn DeleteRaw(api: &ApiClient, id: i32) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body_raw(HttpMethod::Delete, "/api/Reservations/Delete", query)
            .await
    }

    /// `DELETE` `/api/Reservations/AdvancedDelete`
    ///
    /// Parameter `id`: Id rezerwacji.
    pub async fn AdvancedDelete(
        api: &ApiClient,
        id: i32,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body::<()>(
            HttpMethod::Delete,
            "/api/Reservations/AdvancedDelete",
            query,
        )
        .await
    }

    /// `DELETE` `/api/Reservations/AdvancedDelete`
    ///
    /// Parameter `id`: Id rezerwacji.
    pub async fn AdvancedDeleteRaw(
        api: &ApiClient,
        id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "id", &id)?;
        api.request_no_body_raw(
            HttpMethod::Delete,
            "/api/Reservations/AdvancedDelete",
            query,
        )
        .await
    }
}
