use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// WebAPI controller `Products`.
pub struct IProductsController;

pub mod get {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Products`
    ///
    /// Parameter `id`: Id towaru.
    pub struct ById {
        /// Id towaru.
        pub id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ById {
        type Output = crate::web_api::interface::products::view_models::Product;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body::<crate::web_api::interface::products::view_models::Product>(
                HttpMethod::Get,
                "/api/Products",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "id", &self.id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Products", query)
                .await
        }
    }

    /// `GET` `/api/Products`
    ///
    /// Parameter `code`: Kod towaru.
    pub struct ByCode {
        /// Kod towaru.
        pub code: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByCode {
        type Output = crate::web_api::interface::products::view_models::Product;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "code", &self.code)?;
            api.request_no_body::<crate::web_api::interface::products::view_models::Product>(
                HttpMethod::Get,
                "/api/Products",
                query,
            )
            .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "code", &self.code)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Products", query)
                .await
        }
    }

    /// `GET` `/api/Products`
    pub struct List;

    #[async_trait::async_trait]
    impl Overload for List {
        type Output = Vec<crate::web_api::interface::products::view_models::ProductListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let query = vec![];
            api.request_no_body::<Vec<crate::web_api::interface::products::view_models::ProductListElement>>(HttpMethod::Get, "/api/Products", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let query = vec![];
            api.request_no_body_raw(HttpMethod::Get, "/api/Products", query)
                .await
        }
    }

    /// `GET` `/api/Products`
    ///
    /// Parameter `sale_prices`: Czy uzupełnić informacje o towarze o kolekcję zdefiniowanych cen sprzedaży.
    pub struct BySalePrices {
        /// Czy uzupełnić informacje o towarze o kolekcję zdefiniowanych cen sprzedaży.
        pub sale_prices: bool,
    }

    #[async_trait::async_trait]
    impl Overload for BySalePrices {
        type Output = Vec<crate::web_api::interface::products::view_models::ProductListElement>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "salePrices", &self.sale_prices)?;
            api.request_no_body::<Vec<crate::web_api::interface::products::view_models::ProductListElement>>(HttpMethod::Get, "/api/Products", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "salePrices", &self.sale_prices)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Products", query)
                .await
        }
    }
}

pub mod get_barcodes {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Products/Barcodes`
    ///
    /// Parameter `product_id`: Id towaru.
    pub struct ByProductId {
        /// Id towaru.
        pub product_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByProductId {
        type Output = crate::web_api::interface::products::view_models::ProductBarcodes;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body::<crate::web_api::interface::products::view_models::ProductBarcodes>(HttpMethod::Get, "/api/Products/Barcodes", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Products/Barcodes", query)
                .await
        }
    }

    /// `GET` `/api/Products/Barcodes`
    ///
    /// Parameter `product_code`: Kod towaru.
    pub struct ByProductCode {
        /// Kod towaru.
        pub product_code: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByProductCode {
        type Output = crate::web_api::interface::products::view_models::ProductBarcodes;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body::<crate::web_api::interface::products::view_models::ProductBarcodes>(HttpMethod::Get, "/api/Products/Barcodes", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Products/Barcodes", query)
                .await
        }
    }
}

pub mod update_barcodes {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Products/UpdateBarcodes`
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
            api.request_no_body::<()>(HttpMethod::Get, "/api/Products/UpdateBarcodes", query)
                .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Products/UpdateBarcodes", query)
                .await
        }
    }

    /// `GET` `/api/Products/UpdateBarcodes`
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
            api.request_no_body::<()>(HttpMethod::Get, "/api/Products/UpdateBarcodes", query)
                .await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Products/UpdateBarcodes", query)
                .await
        }
    }
}

pub mod get_logistic_fields {
    use super::*;

    #[async_trait::async_trait]
    pub trait Overload {
        type Output: DeserializeOwned;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError>;
        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError>;
    }

    /// `GET` `/api/Products/LogisticFields`
    ///
    /// Parameter `product_id`: Id towaru.
    pub struct ByProductId {
        /// Id towaru.
        pub product_id: i32,
    }

    #[async_trait::async_trait]
    impl Overload for ByProductId {
        type Output = Vec<crate::web_api::interface::products::view_models::ProductLogisticField>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body::<Vec<crate::web_api::interface::products::view_models::ProductLogisticField>>(HttpMethod::Get, "/api/Products/LogisticFields", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productId", &self.product_id)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Products/LogisticFields", query)
                .await
        }
    }

    /// `GET` `/api/Products/LogisticFields`
    ///
    /// Parameter `product_code`: Kod towaru.
    pub struct ByProductCode {
        /// Kod towaru.
        pub product_code: String,
    }

    #[async_trait::async_trait]
    impl Overload for ByProductCode {
        type Output = Vec<crate::web_api::interface::products::view_models::ProductLogisticField>;

        async fn call(self, api: &ApiClient) -> Result<ResponseEnvelope<Self::Output>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body::<Vec<crate::web_api::interface::products::view_models::ProductLogisticField>>(HttpMethod::Get, "/api/Products/LogisticFields", query).await
        }

        async fn call_raw(self, api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
            let mut query = Vec::<QueryParam>::new();
            push_query_param(&mut query, "productCode", &self.product_code)?;
            api.request_no_body_raw(HttpMethod::Get, "/api/Products/LogisticFields", query)
                .await
        }
    }
}

#[allow(non_snake_case)]
impl IProductsController {
    /// `GET` `/api/Products`
    ///
    /// Parameter `id`: Id towaru.
    pub async fn Get<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Products`
    ///
    /// Parameter `id`: Id towaru.
    pub async fn GetRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Products/Barcodes`
    ///
    /// Parameter `product_id`: Id towaru.
    pub async fn GetBarcodes<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_barcodes::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Products/Barcodes`
    ///
    /// Parameter `product_id`: Id towaru.
    pub async fn GetBarcodesRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_barcodes::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Products/UpdateBarcodes`
    ///
    /// Parameter `product_id`: Id towaru.
    pub async fn UpdateBarcodes<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: update_barcodes::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Products/UpdateBarcodes`
    ///
    /// Parameter `product_id`: Id towaru.
    pub async fn UpdateBarcodesRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: update_barcodes::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Products/LogisticFields`
    ///
    /// Parameter `product_id`: Id towaru.
    pub async fn GetLogisticFields<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<T::Output>, ApiError>
    where
        T: get_logistic_fields::Overload,
    {
        overload.call(api).await
    }

    /// `GET` `/api/Products/LogisticFields`
    ///
    /// Parameter `product_id`: Id towaru.
    pub async fn GetLogisticFieldsRaw<T>(
        api: &ApiClient,
        overload: T,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        T: get_logistic_fields::Overload,
    {
        overload.call_raw(api).await
    }

    /// `GET` `/api/Products/WithSalePrices`
    pub async fn GetWithSalePrices(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::products::view_models::ProductListElementWithSalePrices>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::products::view_models::ProductListElementWithSalePrices>>(HttpMethod::Get, "/api/Products/WithSalePrices", query).await
    }

    /// `GET` `/api/Products/WithSalePrices`
    pub async fn GetWithSalePricesRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Products/WithSalePrices", query)
            .await
    }

    /// `GET` `/api/Products/WithDimensions`
    pub async fn GetWithDimensions(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::products::view_models::ProductListElementWithDimensions>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::products::view_models::ProductListElementWithDimensions>>(HttpMethod::Get, "/api/Products/WithDimensions", query).await
    }

    /// `GET` `/api/Products/WithDimensions`
    pub async fn GetWithDimensionsRaw(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Products/WithDimensions", query)
            .await
    }

    /// `PATCH` `/api/Products/Filter`
    ///
    /// Parameter `sale_prices`: Czy uzupełnić informacje o towarze o kolekcję zdefiniowanych cen sprzedaży.
    ///
    /// Body `criteria`: Lista kryteriów wyszukiwania.
    pub async fn Filter(
        api: &ApiClient,
        sale_prices: bool,
        criteria: &crate::web_api::interface::products::view_models::ProductFilterCriteria,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::products::view_models::ProductListElement>>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "salePrices", &sale_prices)?;
        api.request_with_body::<Vec<crate::web_api::interface::products::view_models::ProductListElement>, _>(HttpMethod::Patch, "/api/Products/Filter", query, criteria).await
    }

    /// `PATCH` `/api/Products/Filter`
    ///
    /// Parameter `sale_prices`: Czy uzupełnić informacje o towarze o kolekcję zdefiniowanych cen sprzedaży.
    ///
    /// Body `criteria`: Lista kryteriów wyszukiwania.
    pub async fn FilterRaw(
        api: &ApiClient,
        sale_prices: bool,
        criteria: &crate::web_api::interface::products::view_models::ProductFilterCriteria,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "salePrices", &sale_prices)?;
        api.request_with_body_raw(HttpMethod::Patch, "/api/Products/Filter", query, criteria)
            .await
    }

    /// `PATCH` `/api/Products/Filter/WithSalePrices`
    ///
    /// Body `criteria`: Lista kryteriów wyszukiwania.
    pub async fn FilterWithSalePrices(
        api: &ApiClient,
        criteria: &crate::web_api::interface::products::view_models::ProductFilterCriteria,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::products::view_models::ProductListElementWithSalePrices>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<Vec<crate::web_api::interface::products::view_models::ProductListElementWithSalePrices>, _>(HttpMethod::Patch, "/api/Products/Filter/WithSalePrices", query, criteria).await
    }

    /// `PATCH` `/api/Products/Filter/WithSalePrices`
    ///
    /// Body `criteria`: Lista kryteriów wyszukiwania.
    pub async fn FilterWithSalePricesRaw(
        api: &ApiClient,
        criteria: &crate::web_api::interface::products::view_models::ProductFilterCriteria,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Patch,
            "/api/Products/Filter/WithSalePrices",
            query,
            criteria,
        )
        .await
    }

    /// `PATCH` `/api/Products/Filter/WithDimensions`
    ///
    /// Body `criteria`: Lista kryteriów wyszukiwania.
    pub async fn FilterWithDimensions(
        api: &ApiClient,
        criteria: &crate::web_api::interface::products::view_models::ProductFilterCriteria,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::products::view_models::ProductListElementWithDimensions>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<Vec<crate::web_api::interface::products::view_models::ProductListElementWithDimensions>, _>(HttpMethod::Patch, "/api/Products/Filter/WithDimensions", query, criteria).await
    }

    /// `PATCH` `/api/Products/Filter/WithDimensions`
    ///
    /// Body `criteria`: Lista kryteriów wyszukiwania.
    pub async fn FilterWithDimensionsRaw(
        api: &ApiClient,
        criteria: &crate::web_api::interface::products::view_models::ProductFilterCriteria,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Patch,
            "/api/Products/Filter/WithDimensions",
            query,
            criteria,
        )
        .await
    }

    /// `POST` `/api/Products/Create`
    ///
    /// Body `product`: Obiekt towaru do utworzenia.
    pub async fn AddNew(
        api: &ApiClient,
        product: &crate::web_api::interface::products::view_models::Product,
    ) -> Result<ResponseEnvelope<crate::web_api::interface::products::view_models::Product>, ApiError>
    {
        let query = vec![];
        api.request_with_body::<crate::web_api::interface::products::view_models::Product, _>(
            HttpMethod::Post,
            "/api/Products/Create",
            query,
            product,
        )
        .await
    }

    /// `POST` `/api/Products/Create`
    ///
    /// Body `product`: Obiekt towaru do utworzenia.
    pub async fn AddNewRaw(
        api: &ApiClient,
        product: &crate::web_api::interface::products::view_models::Product,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(HttpMethod::Post, "/api/Products/Create", query, product)
            .await
    }

    /// `PUT` `/api/Products/Update`
    ///
    /// Body `product`: Obiekt towaru do aktualizacji.
    pub async fn Update(
        api: &ApiClient,
        product: &crate::web_api::interface::products::view_models::Product,
    ) -> Result<ResponseEnvelope<()>, ApiError> {
        let query = vec![];
        api.request_with_body::<(), _>(HttpMethod::Put, "/api/Products/Update", query, product)
            .await
    }

    /// `PUT` `/api/Products/Update`
    ///
    /// Body `product`: Obiekt towaru do aktualizacji.
    pub async fn UpdateRaw(
        api: &ApiClient,
        product: &crate::web_api::interface::products::view_models::Product,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(HttpMethod::Put, "/api/Products/Update", query, product)
            .await
    }

    /// `GET` `/api/Products/Markers`
    pub async fn GetMarkers(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Marker>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Marker>>(
            HttpMethod::Get,
            "/api/Products/Markers",
            query,
        )
        .await
    }

    /// `GET` `/api/Products/Markers`
    pub async fn GetMarkersRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Products/Markers", query)
            .await
    }

    /// `GET` `/api/Products/Kinds`
    pub async fn GetKinds(
        api: &ApiClient,
    ) -> Result<ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Kind>>, ApiError>
    {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Kind>>(
            HttpMethod::Get,
            "/api/Products/Kinds",
            query,
        )
        .await
    }

    /// `GET` `/api/Products/Kinds`
    pub async fn GetKindsRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Products/Kinds", query)
            .await
    }

    /// `GET` `/api/Products/Catalogs`
    pub async fn GetCatalogs(
        api: &ApiClient,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::common::view_models::Catalog>>,
        ApiError,
    > {
        let query = vec![];
        api.request_no_body::<Vec<crate::web_api::interface::common::view_models::Catalog>>(
            HttpMethod::Get,
            "/api/Products/Catalogs",
            query,
        )
        .await
    }

    /// `GET` `/api/Products/Catalogs`
    pub async fn GetCatalogsRaw(api: &ApiClient) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_no_body_raw(HttpMethod::Get, "/api/Products/Catalogs", query)
            .await
    }

    /// `GET` `/api/Products/IncrementalSync`
    ///
    /// Parameter `date_from`: Inferred from route query template.
    pub async fn IncrementalSync(
        api: &ApiClient,
        date_from: chrono::NaiveDateTime,
    ) -> Result<
        ResponseEnvelope<
            Vec<
                crate::web_api::interface::common::view_models::IncrementalSyncListElement<
                    serde_json::Value,
                >,
            >,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        api.request_no_body::<Vec<
            crate::web_api::interface::common::view_models::IncrementalSyncListElement<
                serde_json::Value,
            >,
        >>(HttpMethod::Get, "/api/Products/IncrementalSync", query)
            .await
    }

    /// `GET` `/api/Products/IncrementalSync`
    ///
    /// Parameter `date_from`: Inferred from route query template.
    pub async fn IncrementalSyncRaw(
        api: &ApiClient,
        date_from: chrono::NaiveDateTime,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "dateFrom", &date_from)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Products/IncrementalSync", query)
            .await
    }

    /// `PATCH` `/api/Products/FilterSql`
    ///
    /// Body `criteria_filter`: Lista kryteriów wyszukiwania.
    pub async fn FilterSql(
        api: &ApiClient,
        criteria_filter: &crate::web_api::interface::products::view_models::ProductCriteriaFilter,
    ) -> Result<
        ResponseEnvelope<Vec<crate::web_api::interface::products::view_models::ProductListElement>>,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<Vec<crate::web_api::interface::products::view_models::ProductListElement>, _>(HttpMethod::Patch, "/api/Products/FilterSql", query, criteria_filter).await
    }

    /// `PATCH` `/api/Products/FilterSql`
    ///
    /// Body `criteria_filter`: Lista kryteriów wyszukiwania.
    pub async fn FilterSqlRaw(
        api: &ApiClient,
        criteria_filter: &crate::web_api::interface::products::view_models::ProductCriteriaFilter,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Patch,
            "/api/Products/FilterSql",
            query,
            criteria_filter,
        )
        .await
    }

    /// `GET` `/api/Products/Page`
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
                crate::web_api::interface::products::view_models::ProductListElement,
            >,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body::<crate::web_api::interface::view_models::Page<
            crate::web_api::interface::products::view_models::ProductListElement,
        >>(HttpMethod::Get, "/api/Products/Page", query)
            .await
    }

    /// `GET` `/api/Products/Page`
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
        api.request_no_body_raw(HttpMethod::Get, "/api/Products/Page", query)
            .await
    }

    /// `GET` `/api/Products/Page/WithSalePrices`
    ///
    /// Parameter `page`: Numer strony.
    ///
    /// Parameter `size`: Ilość wyników na stronie.
    ///
    /// Parameter `order_by`: Sortowanie wyników na stronie.
    pub async fn GetPageWithSalePrices(
        api: &ApiClient,
        page: i32,
        size: i32,
        order_by: crate::web_api::interface::enums::enumOrderByType,
    ) -> Result<
        ResponseEnvelope<
            crate::web_api::interface::view_models::Page<
                crate::web_api::interface::products::view_models::ProductListElementWithSalePrices,
            >,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body::<crate::web_api::interface::view_models::Page<
            crate::web_api::interface::products::view_models::ProductListElementWithSalePrices,
        >>(HttpMethod::Get, "/api/Products/Page/WithSalePrices", query)
            .await
    }

    /// `GET` `/api/Products/Page/WithSalePrices`
    ///
    /// Parameter `page`: Numer strony.
    ///
    /// Parameter `size`: Ilość wyników na stronie.
    ///
    /// Parameter `order_by`: Sortowanie wyników na stronie.
    pub async fn GetPageWithSalePricesRaw(
        api: &ApiClient,
        page: i32,
        size: i32,
        order_by: crate::web_api::interface::enums::enumOrderByType,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Products/Page/WithSalePrices", query)
            .await
    }

    /// `GET` `/api/Products/Page/WithDimensions`
    ///
    /// Parameter `page`: Numer strony.
    ///
    /// Parameter `size`: Ilość wyników na stronie.
    ///
    /// Parameter `order_by`: Sortowanie wyników na stronie.
    pub async fn GetPageWithDimensions(
        api: &ApiClient,
        page: i32,
        size: i32,
        order_by: crate::web_api::interface::enums::enumOrderByType,
    ) -> Result<
        ResponseEnvelope<
            crate::web_api::interface::view_models::Page<
                crate::web_api::interface::products::view_models::ProductListElementWithDimensions,
            >,
        >,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body::<crate::web_api::interface::view_models::Page<
            crate::web_api::interface::products::view_models::ProductListElementWithDimensions,
        >>(HttpMethod::Get, "/api/Products/Page/WithDimensions", query)
            .await
    }

    /// `GET` `/api/Products/Page/WithDimensions`
    ///
    /// Parameter `page`: Numer strony.
    ///
    /// Parameter `size`: Ilość wyników na stronie.
    ///
    /// Parameter `order_by`: Sortowanie wyników na stronie.
    pub async fn GetPageWithDimensionsRaw(
        api: &ApiClient,
        page: i32,
        size: i32,
        order_by: crate::web_api::interface::enums::enumOrderByType,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "page", &page)?;
        push_query_param(&mut query, "size", &size)?;
        push_query_param(&mut query, "orderBy", &order_by)?;
        api.request_no_body_raw(HttpMethod::Get, "/api/Products/Page/WithDimensions", query)
            .await
    }

    /// `PATCH` `/api/Products/FilterSql/WithSalePrices`
    ///
    /// Body `criteria_filter`: Lista kryteriów wyszukiwania.
    pub async fn FilterSqlWthSalePrices(
        api: &ApiClient,
        criteria_filter: &crate::web_api::interface::products::view_models::ProductCriteriaFilter,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::products::view_models::ProductListElementWithSalePrices>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<Vec<crate::web_api::interface::products::view_models::ProductListElementWithSalePrices>, _>(HttpMethod::Patch, "/api/Products/FilterSql/WithSalePrices", query, criteria_filter).await
    }

    /// `PATCH` `/api/Products/FilterSql/WithSalePrices`
    ///
    /// Body `criteria_filter`: Lista kryteriów wyszukiwania.
    pub async fn FilterSqlWthSalePricesRaw(
        api: &ApiClient,
        criteria_filter: &crate::web_api::interface::products::view_models::ProductCriteriaFilter,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Patch,
            "/api/Products/FilterSql/WithSalePrices",
            query,
            criteria_filter,
        )
        .await
    }

    /// `PATCH` `/api/Products/FilterSql/WithDimensions`
    ///
    /// Body `criteria_filter`: Lista kryteriów wyszukiwania.
    pub async fn FilterSqlWthDimensions(
        api: &ApiClient,
        criteria_filter: &crate::web_api::interface::products::view_models::ProductCriteriaFilter,
    ) -> Result<
        ResponseEnvelope<
            Vec<crate::web_api::interface::products::view_models::ProductListElementWithDimensions>,
        >,
        ApiError,
    > {
        let query = vec![];
        api.request_with_body::<Vec<crate::web_api::interface::products::view_models::ProductListElementWithDimensions>, _>(HttpMethod::Patch, "/api/Products/FilterSql/WithDimensions", query, criteria_filter).await
    }

    /// `PATCH` `/api/Products/FilterSql/WithDimensions`
    ///
    /// Body `criteria_filter`: Lista kryteriów wyszukiwania.
    pub async fn FilterSqlWthDimensionsRaw(
        api: &ApiClient,
        criteria_filter: &crate::web_api::interface::products::view_models::ProductCriteriaFilter,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let query = vec![];
        api.request_with_body_raw(
            HttpMethod::Patch,
            "/api/Products/FilterSql/WithDimensions",
            query,
            criteria_filter,
        )
        .await
    }
}
