use serde::{Deserialize, Serialize};

/// WebAPI model `DocumentIssueSettlement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentIssueSettlement {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "DocumentId")]
    pub DocumentId: Option<i32>,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "SettlementDate")]
    pub SettlementDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
}

/// WebAPI model `DocumentIssueSubject`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentIssueSubject {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
}

/// WebAPI model `PaymentIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentIssue {
    #[serde(rename = "Series")]
    pub Series: String,
    #[serde(rename = "TypeCode")]
    pub TypeCode: String,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "TotalValue")]
    pub TotalValue: Option<rust_decimal::Decimal>,
    #[serde(rename = "PaymentDate")]
    pub PaymentDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Subject")]
    pub Subject: DocumentIssueSubject,
    #[serde(rename = "PaymentRegistry")]
    pub PaymentRegistry: crate::web_api::interface::common::view_models::PaymentRegistryBase,
    #[serde(rename = "Settlements")]
    pub Settlements: Vec<DocumentIssueSettlement>,
}

/// WebAPI model `SettlementIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementIssue {
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Series")]
    pub Series: String,
    #[serde(rename = "TypeCode")]
    pub TypeCode: String,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "TotalValue")]
    pub TotalValue: Option<rust_decimal::Decimal>,
    #[serde(rename = "PaymentDate")]
    pub PaymentDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Subject")]
    pub Subject: DocumentIssueSubject,
    #[serde(rename = "PaymentRegistry")]
    pub PaymentRegistry: crate::web_api::interface::common::view_models::PaymentRegistryBase,
    #[serde(rename = "Settlements")]
    pub Settlements: Vec<DocumentIssueSettlement>,
}
