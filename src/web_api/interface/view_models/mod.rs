use serde::{Deserialize, Serialize};

/// WebAPI model `CompanyInfo`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyInfo {
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "NIP")]
    pub NIP: Option<String>,
    #[serde(rename = "Address")]
    pub Address: String,
    #[serde(rename = "LicensesInfo")]
    pub LicensesInfo: Vec<LicenceInfo>,
}

/// WebAPI model `InstanceInfo`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceInfo {
    #[serde(rename = "Seed")]
    pub Seed: String,
    #[serde(rename = "ServerName")]
    pub ServerName: String,
    #[serde(rename = "DatabaseName")]
    pub DatabaseName: String,
    #[serde(rename = "Firm")]
    pub Firm: String,
    #[serde(rename = "User")]
    pub User: String,
}

/// WebAPI model `LicenceInfo`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenceInfo {
    #[serde(rename = "Module")]
    pub Module: String,
    #[serde(rename = "Feature")]
    pub Feature: String,
    #[serde(rename = "ValueInt")]
    pub ValueInt: i32,
    #[serde(rename = "ValueDecimal")]
    pub ValueDecimal: rust_decimal::Decimal,
    #[serde(rename = "ValueBool")]
    pub ValueBool: bool,
    #[serde(rename = "ValueString")]
    pub ValueString: String,
    #[serde(rename = "ValueDateTime")]
    pub ValueDateTime: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ValueObject")]
    pub ValueObject: serde_json::Value,
}

/// WebAPI model `LoadedModuleInfo`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadedModuleInfo {
    #[serde(rename = "AssemblyName")]
    pub AssemblyName: String,
    #[serde(rename = "ModuleName")]
    pub ModuleName: String,
    #[serde(rename = "ModuleVersion")]
    pub ModuleVersion: String,
}

/// WebAPI model `Page`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    #[serde(rename = "NextPage")]
    pub NextPage: Option<String>,
    #[serde(rename = "PreviousPage")]
    pub PreviousPage: Option<String>,
    #[serde(rename = "PageNumber")]
    pub PageNumber: i32,
    #[serde(rename = "LimitPerPage")]
    pub LimitPerPage: i32,
    #[serde(rename = "TotalItems")]
    pub TotalItems: i32,
    #[serde(rename = "OrderBy")]
    pub OrderBy: crate::web_api::interface::enums::enumOrderByType,
    #[serde(rename = "Data")]
    pub Data: Vec<T>,
}

/// WebAPI model `Ping`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ping {
    #[serde(rename = "OpenSessionsNumber")]
    pub OpenSessionsNumber: i32,
    #[serde(rename = "ActiveHMFInfo")]
    pub ActiveHMFInfo: Vec<InstanceInfo>,
    #[serde(rename = "ActiveITGInfo")]
    pub ActiveITGInfo: Vec<InstanceInfo>,
    #[serde(rename = "LoadedModuleInfo")]
    pub LoadedModuleInfo: Vec<LoadedModuleInfo>,
}

/// WebAPI model `SessionInformation`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInformation {
    #[serde(rename = "Token")]
    pub Token: String,
    #[serde(rename = "DeviceName")]
    pub DeviceName: String,
    #[serde(rename = "RegisterTime")]
    pub RegisterTime: chrono::NaiveDateTime,
    #[serde(rename = "ExpireTime")]
    pub ExpireTime: chrono::NaiveDateTime,
    #[serde(rename = "TotalSessionsCount")]
    pub TotalSessionsCount: i32,
}
