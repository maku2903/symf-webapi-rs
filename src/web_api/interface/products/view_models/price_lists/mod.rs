use serde::{Deserialize, Serialize};

/// WebAPI model `PriceList`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceList {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "DateFrom")]
    pub DateFrom: Option<chrono::NaiveDateTime>,
    #[serde(rename = "DateTo")]
    pub DateTo: Option<chrono::NaiveDateTime>,
    #[serde(rename = "DepartmentId")]
    pub DepartmentId: Option<i32>,
    #[serde(rename = "IsPriceListHistoryEnabled")]
    pub IsPriceListHistoryEnabled: bool,
    #[serde(rename = "IsCurrencyPriceList")]
    pub IsCurrencyPriceList: bool,
    #[serde(rename = "PriceListCurrency")]
    pub PriceListCurrency: String,
    #[serde(rename = "ForAllContractors")]
    pub ForAllContractors: bool,
    #[serde(rename = "Priority")]
    pub Priority: i32,
    #[serde(rename = "MinimalPriority")]
    pub MinimalPriority: i32,
    #[serde(rename = "ConnectionType")]
    pub ConnectionType: crate::web_api::interface::enums::enumPriceListConnectionType,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumPriceFactorType,
    #[serde(rename = "SpecificWeekdays")]
    pub SpecificWeekdays: bool,
    #[serde(rename = "AvailableOnMonday")]
    pub AvailableOnMonday: bool,
    #[serde(rename = "AvailableOnTuesday")]
    pub AvailableOnTuesday: bool,
    #[serde(rename = "AvailableOnWednesday")]
    pub AvailableOnWednesday: bool,
    #[serde(rename = "AvailableOnThursday")]
    pub AvailableOnThursday: bool,
    #[serde(rename = "AvailableOnFriday")]
    pub AvailableOnFriday: bool,
    #[serde(rename = "AvailableOnSaturday")]
    pub AvailableOnSaturday: bool,
    #[serde(rename = "AvailableOnSunday")]
    pub AvailableOnSunday: bool,
    #[serde(rename = "Products")]
    pub Products: Vec<PriceListSubjectDetails>,
    #[serde(rename = "ProductKinds")]
    pub ProductKinds: Vec<PriceListSubjectDetails>,
    #[serde(rename = "Contractors")]
    pub Contractors: Vec<PriceListSubject>,
    #[serde(rename = "ContractorKinds")]
    pub ContractorKinds: Vec<PriceListSubject>,
}

/// WebAPI model `PriceListDetails`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceListDetails {
    #[serde(rename = "Value")]
    pub Value: Option<rust_decimal::Decimal>,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "PriceType")]
    pub PriceType: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "PriceKind")]
    pub PriceKind: crate::web_api::interface::enums::enumPriceKind,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumPriceFactorType,
}

/// WebAPI model `PriceListListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceListListElement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "DateFrom")]
    pub DateFrom: Option<chrono::NaiveDateTime>,
    #[serde(rename = "DateTo")]
    pub DateTo: Option<chrono::NaiveDateTime>,
    #[serde(rename = "DepartmentId")]
    pub DepartmentId: Option<i32>,
    #[serde(rename = "IsPriceListHistoryEnabled")]
    pub IsPriceListHistoryEnabled: bool,
    #[serde(rename = "IsCurrencyPriceList")]
    pub IsCurrencyPriceList: bool,
    #[serde(rename = "PriceListCurrency")]
    pub PriceListCurrency: String,
}

/// WebAPI model `PriceListSubject`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceListSubject {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
}

/// WebAPI model `PriceListSubjectDetails`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceListSubjectDetails {
    #[serde(rename = "Prices")]
    pub Prices: Vec<PriceListDetails>,
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
}
