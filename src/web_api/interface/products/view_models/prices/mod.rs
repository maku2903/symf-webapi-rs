use serde::{Deserialize, Serialize};

/// WebAPI model `IndividualDiscount`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualDiscount {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumPriceFactorType,
    #[serde(rename = "SubjectType")]
    pub SubjectType: crate::web_api::interface::enums::enumIndividualDiscountSubjectType,
}

/// WebAPI model `IndividualDiscountListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualDiscountListElement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "SubjectType")]
    pub SubjectType: crate::web_api::interface::enums::enumIndividualDiscountSubjectType,
    #[serde(rename = "IndividualDiscounts")]
    pub IndividualDiscounts: Vec<IndividualDiscount>,
}

/// WebAPI model `PriceFactor`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceFactor {
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "PriceType")]
    pub PriceType: crate::web_api::interface::enums::enumPriceType,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumPriceFactorType,
    #[serde(rename = "PriceListId")]
    pub PriceListId: Option<i32>,
    #[serde(rename = "ProductId")]
    pub ProductId: Option<i32>,
    #[serde(rename = "ProductKindId")]
    pub ProductKindId: Option<i32>,
    #[serde(rename = "ContractorId")]
    pub ContractorId: Option<i32>,
    #[serde(rename = "ContractorKindId")]
    pub ContractorKindId: Option<i32>,
    #[serde(rename = "DepartmentId")]
    pub DepartmentId: Option<i32>,
    #[serde(rename = "Priority")]
    pub Priority: i32,
    #[serde(rename = "MinimalPriority")]
    pub MinimalPriority: i32,
    #[serde(rename = "ConnectionType")]
    pub ConnectionType: crate::web_api::interface::enums::enumPriceListConnectionType,
    #[serde(rename = "TimeLimited")]
    pub TimeLimited: bool,
}

/// WebAPI model `PriceFactorCriteria`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceFactorCriteria {
    #[serde(rename = "PriceType")]
    pub PriceType: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
    #[serde(rename = "PeriodPricing")]
    pub PeriodPricing: bool,
    #[serde(rename = "DatePrice")]
    pub DatePrice: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ProductKindId")]
    pub ProductKindId: Option<i32>,
    #[serde(rename = "ProductId")]
    pub ProductId: Option<i32>,
    #[serde(rename = "ContractorKindId")]
    pub ContractorKindId: Option<i32>,
    #[serde(rename = "ContractorId")]
    pub ContractorId: Option<i32>,
    #[serde(rename = "DepartmentId")]
    pub DepartmentId: Option<i32>,
}

/// WebAPI model `ProductBasePrice`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductBasePrice {
    #[serde(rename = "Date")]
    pub Date: chrono::NaiveDateTime,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "Currency")]
    pub Currency: String,
}

/// WebAPI model `ProductListElementSalePrice`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductListElementSalePrice {
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "Kind")]
    pub Kind: crate::web_api::interface::enums::enumPriceKind,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "Currency")]
    pub Currency: String,
}

/// WebAPI model `ProductPrice`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPrice {
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "Currency")]
    pub Currency: String,
}

/// WebAPI model `ProductPriceListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPriceListElement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "SalePrices")]
    pub SalePrices: Vec<ProductSalePriceBase>,
}

/// WebAPI model `ProductPricesEdit`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPricesEdit {
    #[serde(rename = "SalePriceRecalculate")]
    pub SalePriceRecalculate: bool,
    #[serde(rename = "CurrencyMode")]
    pub CurrencyMode: bool,
    #[serde(rename = "CurrencyRateWithoutConversion")]
    pub CurrencyRateWithoutConversion: rust_decimal::Decimal,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "CurrencyRateDate")]
    pub CurrencyRateDate: chrono::NaiveDateTime,
    #[serde(rename = "PriceParameter")]
    pub PriceParameter: Option<crate::web_api::interface::enums::enumPriceParameter>,
    #[serde(rename = "PriceRecalculateType")]
    pub PriceRecalculateType: Option<crate::web_api::interface::enums::enumPriceRecalculateType>,
    #[serde(rename = "BasePrice")]
    pub BasePrice: ProductBasePrice,
    #[serde(rename = "PurchasePrice")]
    pub PurchasePrice: ProductPrice,
    #[serde(rename = "PurchasePriceInCurrency")]
    pub PurchasePriceInCurrency: ProductPrice,
    #[serde(rename = "CalcExcise")]
    pub CalcExcise: ProductPrice,
    #[serde(rename = "CalcDuty")]
    pub CalcDuty: ProductPrice,
    #[serde(rename = "CalcOther")]
    pub CalcOther: ProductPrice,
    #[serde(rename = "SalePrices")]
    pub SalePrices: Vec<ProductSalePrice>,
}

/// WebAPI model `ProductSalePrice`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSalePrice {
    #[serde(rename = "AutomaticValue")]
    pub AutomaticValue: bool,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "ProfitPercent")]
    pub ProfitPercent: rust_decimal::Decimal,
    #[serde(rename = "MarkupPercent")]
    pub MarkupPercent: rust_decimal::Decimal,
    #[serde(rename = "LinkedWithBasePrice")]
    pub LinkedWithBasePrice: bool,
    #[serde(rename = "Kind")]
    pub Kind: crate::web_api::interface::enums::enumPriceKind,
    #[serde(rename = "PriceParameter")]
    pub PriceParameter: crate::web_api::interface::enums::enumPriceParameter,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "Currency")]
    pub Currency: String,
}

/// WebAPI model `ProductSalePriceBase`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSalePriceBase {
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "ProfitPercent")]
    pub ProfitPercent: rust_decimal::Decimal,
    #[serde(rename = "MarkupPercent")]
    pub MarkupPercent: rust_decimal::Decimal,
    #[serde(rename = "LinkedWithBasePrice")]
    pub LinkedWithBasePrice: bool,
    #[serde(rename = "Kind")]
    pub Kind: crate::web_api::interface::enums::enumPriceKind,
    #[serde(rename = "PriceParameter")]
    pub PriceParameter: crate::web_api::interface::enums::enumPriceParameter,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "Currency")]
    pub Currency: String,
}

/// WebAPI model `QuantitativeDiscount`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantitativeDiscount {
    #[serde(rename = "QuantityFrom")]
    pub QuantityFrom: rust_decimal::Decimal,
    #[serde(rename = "QuantityTo")]
    pub QuantityTo: Option<rust_decimal::Decimal>,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumPriceFactorType,
}

/// WebAPI model `QuantitativeDiscountListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantitativeDiscountListElement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Discounts")]
    pub Discounts: Vec<QuantitativeDiscount>,
}
