use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `ProductPrices`.
pub struct IProductPricesController;

pub mod update {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PUT` `/api/ProductPrices/Update`
    ///
    /// Parameter `product_id`: Id towaru.
    ///
    /// Body `prices`: Obiekt agregujący ceny towaru do aktualizacji.
    pub struct ByProductId<'a> {
        /// Id towaru.
        pub product_id: i32,
        /// Obiekt agregujący ceny towaru do aktualizacji.
        pub prices: &'a crate::web_api::interface::products::view_models::prices::ProductPricesEdit,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByProductId<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/ProductPrices/Update",
                query,
                self.prices,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/ProductPrices/Update",
                query,
                self.prices,
            )
            .await
        }
    }

    /// `PUT` `/api/ProductPrices/Update`
    ///
    /// Parameter `product_code`: Kod towaru.
    ///
    /// Body `prices`: Obiekt agregujący ceny towaru do aktualizacji.
    pub struct ByProductCode<'a> {
        /// Kod towaru.
        pub product_code: String,
        /// Obiekt agregujący ceny towaru do aktualizacji.
        pub prices: &'a crate::web_api::interface::products::view_models::prices::ProductPricesEdit,
    }

    #[async_trait::async_trait]
    impl<'a> Overload for ByProductCode<'a> {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_with_body::<(), _>(
                HttpMethod::Put,
                "/api/ProductPrices/Update",
                query,
                self.prices,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_with_body_raw(
                HttpMethod::Put,
                "/api/ProductPrices/Update",
                query,
                self.prices,
            )
            .await
        }
    }
}

pub mod recalculate {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `PATCH` `/api/ProductPrices/Recalculate`
    ///
    /// Parameter `product_id`: Id towaru.
    pub struct ByProductId {
        /// Id towaru.
        pub product_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByProductId {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body::<()>(HttpMethod::Patch, "/api/ProductPrices/Recalculate", query)
                .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body_raw(HttpMethod::Patch, "/api/ProductPrices/Recalculate", query)
                .await
        }
    }

    /// `PATCH` `/api/ProductPrices/Recalculate`
    ///
    /// Parameter `product_code`: Kod towaru.
    pub struct ByProductCode {
        /// Kod towaru.
        pub product_code: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByProductCode {
        type Output = ();

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body::<()>(HttpMethod::Patch, "/api/ProductPrices/Recalculate", query)
                .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body_raw(HttpMethod::Patch, "/api/ProductPrices/Recalculate", query)
                .await
        }
    }
}

pub mod get_sale_prices_by_product {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/ProductPrices/SalePrices`
    ///
    /// Parameter `product_id`: Id towaru.
    pub struct ByProductId {
        /// Id towaru.
        pub product_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByProductId {
        type Output =
            Vec<crate::web_api::interface::products::view_models::prices::ProductSalePriceBase>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::products::view_models::prices::ProductSalePriceBase>>(HttpMethod::Get, "/api/ProductPrices/SalePrices", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/ProductPrices/SalePrices", query)
                .await
        }
    }

    /// `GET` `/api/ProductPrices/SalePrices`
    ///
    /// Parameter `product_code`: Kod towaru.
    pub struct ByProductCode {
        /// Kod towaru.
        pub product_code: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByProductCode {
        type Output =
            Vec<crate::web_api::interface::products::view_models::prices::ProductSalePriceBase>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body::<Vec<crate::web_api::interface::products::view_models::prices::ProductSalePriceBase>>(HttpMethod::Get, "/api/ProductPrices/SalePrices", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/ProductPrices/SalePrices", query)
                .await
        }
    }
}

pub mod get_quantitative_discounts_by_product {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/ProductPrices/QuantitativeDiscounts`
    ///
    /// Parameter `product_id`: Id towaru.
    pub struct ByProductId {
        /// Id towaru.
        pub product_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByProductId {
        type Output =
            Vec<crate::web_api::interface::products::view_models::prices::QuantitativeDiscount>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::products::view_models::prices::QuantitativeDiscount>>(HttpMethod::Get, "/api/ProductPrices/QuantitativeDiscounts", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/ProductPrices/QuantitativeDiscounts",
                query,
            )
            .await
        }
    }

    /// `GET` `/api/ProductPrices/QuantitativeDiscounts`
    ///
    /// Parameter `product_code`: Kod towaru.
    pub struct ByProductCode {
        /// Kod towaru.
        pub product_code: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByProductCode {
        type Output =
            Vec<crate::web_api::interface::products::view_models::prices::QuantitativeDiscount>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body::<Vec<crate::web_api::interface::products::view_models::prices::QuantitativeDiscount>>(HttpMethod::Get, "/api/ProductPrices/QuantitativeDiscounts", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/ProductPrices/QuantitativeDiscounts",
                query,
            )
            .await
        }
    }
}

pub mod get_individual_discounts_by_contractor {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/ProductPrices/IndividualDiscounts`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    pub struct ByContractorId {
        /// Id kontrahenta.
        pub contractor_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorId {
        type Output =
            Vec<crate::web_api::interface::products::view_models::prices::IndividualDiscount>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::products::view_models::prices::IndividualDiscount>>(HttpMethod::Get, "/api/ProductPrices/IndividualDiscounts", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorId", &self.contractor_id)?;
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/ProductPrices/IndividualDiscounts",
                query,
            )
            .await
        }
    }

    /// `GET` `/api/ProductPrices/IndividualDiscounts`
    ///
    /// Parameter `contractor_code`: Kod kontrahenta.
    pub struct ByContractorCode {
        /// Kod kontrahenta.
        pub contractor_code: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByContractorCode {
        type Output =
            Vec<crate::web_api::interface::products::view_models::prices::IndividualDiscount>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_no_body::<Vec<crate::web_api::interface::products::view_models::prices::IndividualDiscount>>(HttpMethod::Get, "/api/ProductPrices/IndividualDiscounts", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "contractorCode", &self.contractor_code)?;
            api.request_no_body_raw(
                HttpMethod::Get,
                "/api/ProductPrices/IndividualDiscounts",
                query,
            )
            .await
        }
    }
}

#[allow(non_snake_case)]
impl IProductPricesController {
    /// `PUT` `/api/ProductPrices/Update`
    ///
    /// Parameter `product_id`: Id towaru.
    ///
    /// Body `prices`: Obiekt agregujący ceny towaru do aktualizacji.
    pub async fn Update<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: update::Overload,
    {
        overload.call(api).await
    }

    /// `PUT` `/api/ProductPrices/Update`
    ///
    /// Parameter `product_id`: Id towaru.
    ///
    /// Body `prices`: Obiekt agregujący ceny towaru do aktualizacji.
    pub async fn UpdateRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: update::Overload,
    {
        overload.call_raw(api).await
    }

    /// `PATCH` `/api/ProductPrices/Recalculate`
    ///
    /// Parameter `product_id`: Id towaru.
    pub async fn Recalculate<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: recalculate::Overload,
    {
        overload.call(api).await
    }

    /// `PATCH` `/api/ProductPrices/Recalculate`
    ///
    /// Parameter `product_id`: Id towaru.
    pub async fn RecalculateRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: recalculate::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/ProductPrices/SalePrices`
    ///
    /// Parameter `product_id`: Id towaru.
    pub async fn GetSalePricesByProduct<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_sale_prices_by_product::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/ProductPrices/SalePrices`
    ///
    /// Parameter `product_id`: Id towaru.
    pub async fn GetSalePricesByProductRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_sale_prices_by_product::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/ProductPrices/QuantitativeDiscounts`
    ///
    /// Parameter `product_id`: Id towaru.
    pub async fn GetQuantitativeDiscountsByProduct<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_quantitative_discounts_by_product::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/ProductPrices/QuantitativeDiscounts`
    ///
    /// Parameter `product_id`: Id towaru.
    pub async fn GetQuantitativeDiscountsByProductRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_quantitative_discounts_by_product::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/ProductPrices/IndividualDiscounts`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    pub async fn GetIndividualDiscountsByContractor<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_individual_discounts_by_contractor::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/ProductPrices/IndividualDiscounts`
    ///
    /// Parameter `contractor_id`: Id kontrahenta.
    pub async fn GetIndividualDiscountsByContractorRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_individual_discounts_by_contractor::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/ProductPrices/SalePrices`
    ///
    /// Parameter `product_kind_id`: Id rodzaju towaru.
    pub async fn GetSalePricesByProductKind(
        api: &ApiClient,
        product_kind_id: i32,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::products::view_models::prices::ProductPriceListElement>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "productKindId", &product_kind_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::products::view_models::prices::ProductPriceListElement>>(HttpMethod::Get, "/api/ProductPrices/SalePrices", query).await
    }

    /// `GET` `/api/ProductPrices/SalePrices`
    ///
    /// Parameter `product_kind_id`: Id rodzaju towaru.
    pub async fn GetSalePricesByProductKindRaw(
        api: &ApiClient,
        product_kind_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "productKindId", &product_kind_id)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/ProductPrices/SalePrices", query)
            .await
    }

    /// `GET` `/api/ProductPrices/SalePrices`
    pub async fn GetSalePrices(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::products::view_models::prices::ProductPriceListElement>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::products::view_models::prices::ProductPriceListElement>>(HttpMethod::Get, "/api/ProductPrices/SalePrices", query).await
    }

    /// `GET` `/api/ProductPrices/SalePrices`
    pub async fn GetSalePricesRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/ProductPrices/SalePrices", query)
            .await
    }

    /// `GET` `/api/ProductPrices/QuantitativeDiscounts`
    ///
    /// Parameter `product_kind_id`: Id rodzaju towaru.
    pub async fn GetQuantitativeDiscountsByProductKind(api: &ApiClient, product_kind_id: i32) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::products::view_models::prices::QuantitativeDiscountListElement>>, ApiError>{
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "productKindId", &product_kind_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::products::view_models::prices::QuantitativeDiscountListElement>>(HttpMethod::Get, "/api/ProductPrices/QuantitativeDiscounts", query).await
    }

    /// `GET` `/api/ProductPrices/QuantitativeDiscounts`
    ///
    /// Parameter `product_kind_id`: Id rodzaju towaru.
    pub async fn GetQuantitativeDiscountsByProductKindRaw(
        api: &ApiClient,
        product_kind_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "productKindId", &product_kind_id)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/ProductPrices/QuantitativeDiscounts",
            query,
        )
        .await
    }

    /// `GET` `/api/ProductPrices/QuantitativeDiscounts`
    pub async fn GetQuantitativeDiscounts(api: &ApiClient) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::products::view_models::prices::QuantitativeDiscountListElement>>, ApiError>{
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::products::view_models::prices::QuantitativeDiscountListElement>>(HttpMethod::Get, "/api/ProductPrices/QuantitativeDiscounts", query).await
    }

    /// `GET` `/api/ProductPrices/QuantitativeDiscounts`
    pub async fn GetQuantitativeDiscountsRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/ProductPrices/QuantitativeDiscounts",
            query,
        )
        .await
    }

    /// `GET` `/api/ProductPrices/IndividualDiscounts`
    ///
    /// Parameter `contractor_kind_id`: Id rodzaju kontrahenta.
    pub async fn GetIndividualDiscountsByContractorKind(
        api: &ApiClient,
        contractor_kind_id: i32,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::products::view_models::prices::IndividualDiscount>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorKindId", &contractor_kind_id)?;
        api.request_no_body::<Vec<crate::web_api::interface::products::view_models::prices::IndividualDiscount>>(HttpMethod::Get, "/api/ProductPrices/IndividualDiscounts", query).await
    }

    /// `GET` `/api/ProductPrices/IndividualDiscounts`
    ///
    /// Parameter `contractor_kind_id`: Id rodzaju kontrahenta.
    pub async fn GetIndividualDiscountsByContractorKindRaw(
        api: &ApiClient,
        contractor_kind_id: i32,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "contractorKindId", &contractor_kind_id)?;
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/ProductPrices/IndividualDiscounts",
            query,
        )
        .await
    }

    /// `GET` `/api/ProductPrices/IndividualDiscounts`
    pub async fn GetIndividualDiscounts(api: &ApiClient) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::products::view_models::prices::IndividualDiscountListElement>>, ApiError>{
        let query = vec![];
        api.request_no_body::<Vec<
            crate::web_api::interface::products::view_models::prices::IndividualDiscountListElement,
        >>(
            HttpMethod::Get,
            "/api/ProductPrices/IndividualDiscounts",
            query,
        )
        .await
    }

    /// `GET` `/api/ProductPrices/IndividualDiscounts`
    pub async fn GetIndividualDiscountsRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(
            HttpMethod::Get,
            "/api/ProductPrices/IndividualDiscounts",
            query,
        )
        .await
    }

    /// `GET` `/api/ProductPrices/PriceFactors`
    ///
    /// Parameter `criteria`: Kryteria wyszukana współczynników ceny.
    pub async fn GetPriceFactors(
        api: &ApiClient,
        criteria: crate::web_api::interface::products::view_models::prices::PriceFactorCriteria,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::products::view_models::prices::PriceFactor>,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "criteria", &criteria)?;
        api.request_no_body::<Vec<crate::web_api::interface::products::view_models::prices::PriceFactor>>(HttpMethod::Get, "/api/ProductPrices/PriceFactors", query).await
    }

    /// `GET` `/api/ProductPrices/PriceFactors`
    ///
    /// Parameter `criteria`: Kryteria wyszukana współczynników ceny.
    pub async fn GetPriceFactorsRaw(
        api: &ApiClient,
        criteria: crate::web_api::interface::products::view_models::prices::PriceFactorCriteria,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "criteria", &criteria)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/ProductPrices/PriceFactors", query)
            .await
    }

    /// `PATCH` `/api/ProductPrices/CalculatePrices`
    ///
    /// Body `criteria`: Kryteria wyliczania ceny towaru.
    pub async fn CalculatePrices(api: &ApiClient, criteria: &crate::web_api::interface::products::view_models::price_calculations::PriceCalculationCriteria) -> Result<ResponseEnvelope<crate::web_api::interface::products::view_models::price_calculations::PriceCalculationResult>, ApiError>{
        let query = vec![];
        api.request_with_body::<crate::web_api::interface::products::view_models::price_calculations::PriceCalculationResult, _>(HttpMethod::Patch, "/api/ProductPrices/CalculatePrices", query, criteria).await
    }

    /// `PATCH` `/api/ProductPrices/CalculatePrices`
    ///
    /// Body `criteria`: Kryteria wyliczania ceny towaru.
    pub async fn CalculatePricesRaw(
        api: &ApiClient,
        criteria: &crate::web_api::interface::products::view_models::price_calculations::PriceCalculationCriteria,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Patch,
            "/api/ProductPrices/CalculatePrices",
            query,
            criteria,
        )
        .await
    }

    /// `PATCH` `/api/ProductPrices/OrderPrices`
    ///
    /// Body `criteria`: Kryteria wyliczania ceny towaru.
    pub async fn OrderPrices(
        api: &ApiClient,
        criteria: &crate::web_api::interface::products::view_models::price_calculations::PriceOrderCriteria,
    ) -> Result<
        ResponseEnvelope<
            crate::web_api::interface::products::view_models::price_calculations::PriceOrderResult,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<crate::web_api::interface::products::view_models::price_calculations::PriceOrderResult, _>(HttpMethod::Patch, "/api/ProductPrices/OrderPrices", query, criteria).await
    }

    /// `PATCH` `/api/ProductPrices/OrderPrices`
    ///
    /// Body `criteria`: Kryteria wyliczania ceny towaru.
    pub async fn OrderPricesRaw(
        api: &ApiClient,
        criteria: &crate::web_api::interface::products::view_models::price_calculations::PriceOrderCriteria,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Patch,
            "/api/ProductPrices/OrderPrices",
            query,
            criteria,
        )
        .await
    }
}
