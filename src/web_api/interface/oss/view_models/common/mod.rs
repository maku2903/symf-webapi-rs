use serde::{Deserialize, Serialize};

/// WebAPI model `OssCatalog`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OssCatalog {
    #[serde(rename = "FullPath")]
    pub FullPath: String,
    #[serde(rename = "AddIfNotExist")]
    pub AddIfNotExist: bool,
}

/// WebAPI model `OssContractor`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OssContractor {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "RecalculatePrices")]
    pub RecalculatePrices: bool,
    #[serde(rename = "Data")]
    pub Data: OssOrderContractorData,
}

/// WebAPI model `OssDelivery`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OssDelivery {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
}

/// WebAPI model `OssKind`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OssKind {
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "AddIfNotExist")]
    pub AddIfNotExist: bool,
}

/// WebAPI model `OssOrderContractorData`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OssOrderContractorData {
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "NIP")]
    pub NIP: Option<String>,
    #[serde(rename = "City")]
    pub City: Option<String>,
    #[serde(rename = "Street")]
    pub Street: Option<String>,
    #[serde(rename = "HouseNo")]
    pub HouseNo: Option<String>,
    #[serde(rename = "ApartmentNo")]
    pub ApartmentNo: Option<String>,
    #[serde(rename = "PostCode")]
    pub PostCode: Option<String>,
    #[serde(rename = "Country")]
    pub Country: String,
}
