use serde::{Deserialize, Serialize};

/// WebAPI model `BankAccount`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "AccountNumber")]
    pub AccountNumber: Option<String>,
    #[serde(rename = "BankName")]
    pub BankName: Option<String>,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Main")]
    pub Main: bool,
    #[serde(rename = "SWIFT_BIC")]
    pub SWIFT_BIC: String,
    #[serde(rename = "WhiteList")]
    pub WhiteList: Option<bool>,
    #[serde(rename = "WhiteListDate")]
    pub WhiteListDate: Option<chrono::NaiveDateTime>,
}
