pub mod criteria_filter;

use serde::{Deserialize, Serialize};

/// WebAPI model `Account`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "Synthetic")]
    pub Synthetic: i32,
    #[serde(rename = "Analytic1")]
    pub Analytic1: i32,
    #[serde(rename = "Analytic2")]
    pub Analytic2: i32,
    #[serde(rename = "Analytic3")]
    pub Analytic3: i32,
    #[serde(rename = "Analytic4")]
    pub Analytic4: i32,
    #[serde(rename = "Analytic5")]
    pub Analytic5: i32,
}

/// WebAPI model `Country`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Active")]
    pub Active: bool,
}

/// WebAPI model `Currency`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Shortcut")]
    pub Shortcut: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Active")]
    pub Active: bool,
}

/// WebAPI model `CurrencyTable`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyTable {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Date")]
    pub Date: chrono::NaiveDateTime,
    #[serde(rename = "CurrencyId")]
    pub CurrencyId: i32,
    #[serde(rename = "Table")]
    pub Table: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "SaleCurrencyRate")]
    pub SaleCurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "PurchaseCurrencyRate")]
    pub PurchaseCurrencyRate: rust_decimal::Decimal,
}

/// WebAPI model `Dimension`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimension {
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Value")]
    pub Value: Option<String>,
    #[serde(rename = "DictionaryValue")]
    pub DictionaryValue: Option<String>,
    #[serde(rename = "Type")]
    pub Type: String,
}

/// WebAPI model `DimensionClassification`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionClassification {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Type")]
    pub Type: String,
    #[serde(rename = "DictionaryId")]
    pub DictionaryId: Option<i32>,
    #[serde(rename = "Index")]
    pub Index: Option<i32>,
}

/// WebAPI model `FilterCriteria`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCriteria {
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Value")]
    pub Value: String,
}

/// WebAPI model `Marker`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker {
    #[serde(rename = "Subtype")]
    pub Subtype: i32,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Active")]
    pub Active: bool,
}

/// WebAPI model `Region`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Active")]
    pub Active: bool,
}

/// WebAPI model `TaxOffice`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxOffice {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Position")]
    pub Position: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "City")]
    pub City: Option<String>,
    #[serde(rename = "IsActive")]
    pub IsActive: bool,
}

/// WebAPI model `VatRate`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatRate {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Rate")]
    pub Rate: String,
    #[serde(rename = "IsActive")]
    pub IsActive: bool,
    #[serde(rename = "DateFrom")]
    pub DateFrom: Option<chrono::NaiveDateTime>,
    #[serde(rename = "DateTo")]
    pub DateTo: Option<chrono::NaiveDateTime>,
}

/// WebAPI model `VatRegister`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatRegister {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumVatRegisterType,
    #[serde(rename = "Kind")]
    pub Kind: crate::web_api::interface::enums::enumVatRegisterKind,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Correction")]
    pub Correction: bool,
    #[serde(rename = "Period")]
    pub Period: Option<i32>,
}

/// WebAPI model `VatRegisterMarker`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatRegisterMarker {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "IsActive")]
    pub IsActive: bool,
}

/// WebAPI model `Year`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Year {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Closed")]
    pub Closed: crate::web_api::interface::enums::enumYearClosingStatus,
    #[serde(rename = "Archived")]
    pub Archived: bool,
    #[serde(rename = "StartDate")]
    pub StartDate: chrono::NaiveDateTime,
    #[serde(rename = "EndDate")]
    pub EndDate: chrono::NaiveDateTime,
    #[serde(rename = "Length")]
    pub Length: i32,
}
