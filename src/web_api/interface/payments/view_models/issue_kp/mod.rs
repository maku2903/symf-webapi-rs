use serde::{Deserialize, Serialize};

/// WebAPI model `PaymentKPIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentKPIssue {
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
    #[serde(rename = "Contractor")]
    pub Contractor: PaymentKPIssueContractor,
    #[serde(rename = "PaymentRegistry")]
    pub PaymentRegistry: crate::web_api::interface::common::view_models::PaymentRegistryBase,
    #[serde(rename = "Invoices")]
    pub Invoices: Vec<PaymentKPIssueInvoice>,
}

/// WebAPI model `PaymentKPIssueContractor`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentKPIssueContractor {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
}

/// WebAPI model `PaymentKPIssueInvoice`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentKPIssueInvoice {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "SettlementDate")]
    pub SettlementDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
}
