use serde::{Deserialize, Serialize};

/// WebAPI model `PriceCalculation`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceCalculation {
    #[serde(rename = "DepartmentId")]
    pub DepartmentId: i32,
    #[serde(rename = "ContractorId")]
    pub ContractorId: i32,
    #[serde(rename = "Date")]
    pub Date: chrono::NaiveDateTime,
    #[serde(rename = "PriceType")]
    pub PriceType: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "Currency")]
    pub Currency: String,
}

/// WebAPI model `PriceCalculationCriteria`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceCalculationCriteria {
    #[serde(rename = "CalculationMethod")]
    pub CalculationMethod: crate::web_api::interface::enums::enumPriceCalculationMethod,
    #[serde(rename = "Products")]
    pub Products: Vec<PriceCalculationProductCriteria>,
    #[serde(rename = "DepartmentId")]
    pub DepartmentId: i32,
    #[serde(rename = "ContractorId")]
    pub ContractorId: i32,
    #[serde(rename = "Date")]
    pub Date: chrono::NaiveDateTime,
    #[serde(rename = "PriceType")]
    pub PriceType: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "Currency")]
    pub Currency: String,
}

/// WebAPI model `PriceCalculationProduct`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceCalculationProduct {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
}

/// WebAPI model `PriceCalculationProductCriteria`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceCalculationProductCriteria {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
}

/// WebAPI model `PriceCalculationProductResult`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceCalculationProductResult {
    #[serde(rename = "Price")]
    pub Price: rust_decimal::Decimal,
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
}

/// WebAPI model `PriceCalculationResult`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceCalculationResult {
    #[serde(rename = "Products")]
    pub Products: Vec<PriceCalculationProductResult>,
    #[serde(rename = "DepartmentId")]
    pub DepartmentId: i32,
    #[serde(rename = "ContractorId")]
    pub ContractorId: i32,
    #[serde(rename = "Date")]
    pub Date: chrono::NaiveDateTime,
    #[serde(rename = "PriceType")]
    pub PriceType: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "Currency")]
    pub Currency: String,
}

/// WebAPI model `PriceOrder`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceOrder {
    #[serde(rename = "Department")]
    pub Department: String,
    #[serde(rename = "Date")]
    pub Date: chrono::NaiveDateTime,
    #[serde(rename = "SalePriceType")]
    pub SalePriceType: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "Currency")]
    pub Currency: String,
}

/// WebAPI model `PriceOrderContractor`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceOrderContractor {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
}

/// WebAPI model `PriceOrderContractorCriteria`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceOrderContractorCriteria {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
}

/// WebAPI model `PriceOrderCriteria`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceOrderCriteria {
    #[serde(rename = "Contractor")]
    pub Contractor: PriceOrderContractorCriteria,
    #[serde(rename = "TypeCode")]
    pub TypeCode: String,
    #[serde(rename = "Series")]
    pub Series: String,
    #[serde(rename = "Products")]
    pub Products: Vec<PriceOrderProductCriteria>,
    #[serde(rename = "Department")]
    pub Department: String,
    #[serde(rename = "Date")]
    pub Date: chrono::NaiveDateTime,
    #[serde(rename = "SalePriceType")]
    pub SalePriceType: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "Currency")]
    pub Currency: String,
}

/// WebAPI model `PriceOrderProduct`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceOrderProduct {
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
}

/// WebAPI model `PriceOrderProductCriteria`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceOrderProductCriteria {
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
}

/// WebAPI model `PriceOrderProductResult`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceOrderProductResult {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Price")]
    pub Price: rust_decimal::Decimal,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
}

/// WebAPI model `PriceOrderResult`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceOrderResult {
    #[serde(rename = "Contractor")]
    pub Contractor: PriceOrderContractor,
    #[serde(rename = "Products")]
    pub Products: Vec<PriceOrderProductResult>,
    #[serde(rename = "Department")]
    pub Department: String,
    #[serde(rename = "Date")]
    pub Date: chrono::NaiveDateTime,
    #[serde(rename = "SalePriceType")]
    pub SalePriceType: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "Currency")]
    pub Currency: String,
}
