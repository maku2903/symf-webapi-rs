use serde::{Deserialize, Serialize};

/// WebAPI model `Transaction`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: chrono::NaiveDateTime,
    #[serde(rename = "PaymentDate")]
    pub PaymentDate: chrono::NaiveDateTime,
    #[serde(rename = "ValuePln")]
    pub ValuePln: rust_decimal::Decimal,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "SettledValuePln")]
    pub SettledValuePln: rust_decimal::Decimal,
    #[serde(rename = "SettledValue")]
    pub SettledValue: rust_decimal::Decimal,
    #[serde(rename = "LeftToSettledValuePln")]
    pub LeftToSettledValuePln: rust_decimal::Decimal,
    #[serde(rename = "LeftToSettledValue")]
    pub LeftToSettledValue: rust_decimal::Decimal,
    #[serde(rename = "MarkerId")]
    pub MarkerId: Option<i32>,
    #[serde(rename = "Currency")]
    pub Currency: String,
}

/// WebAPI model `TransactionDocument`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDocument {
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "ValuePln")]
    pub ValuePln: rust_decimal::Decimal,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "ContractorId")]
    pub ContractorId: i32,
    #[serde(rename = "ContractorCode")]
    pub ContractorCode: String,
    #[serde(rename = "ContractorName")]
    pub ContractorName: String,
    #[serde(rename = "ContractorNip")]
    pub ContractorNip: String,
    #[serde(rename = "SettledValuePln")]
    pub SettledValuePln: rust_decimal::Decimal,
    #[serde(rename = "SettledValue")]
    pub SettledValue: rust_decimal::Decimal,
    #[serde(rename = "LeftToSettledValuePln")]
    pub LeftToSettledValuePln: rust_decimal::Decimal,
    #[serde(rename = "LeftToSettledValue")]
    pub LeftToSettledValue: rust_decimal::Decimal,
    #[serde(rename = "Transactions")]
    pub Transactions: Vec<Transaction>,
}
