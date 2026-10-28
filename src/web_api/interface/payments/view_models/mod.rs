pub mod issue;
pub mod issue_kp;

use serde::{Deserialize, Serialize};

/// WebAPI model `Payment`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Canceled")]
    pub Canceled: crate::web_api::interface::enums::enumCancelationType,
    #[serde(rename = "Buffer")]
    pub Buffer: bool,
    #[serde(rename = "Settled")]
    pub Settled: bool,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "SettlementDate")]
    pub SettlementDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "TypeCode")]
    pub TypeCode: String,
    #[serde(rename = "Series")]
    pub Series: String,
    #[serde(rename = "NumberInSeries")]
    pub NumberInSeries: i32,
    #[serde(rename = "IssuerId")]
    pub IssuerId: i32,
    #[serde(rename = "SubjectId")]
    pub SubjectId: Option<i32>,
    #[serde(rename = "ReceivingPaymentRegistryId")]
    pub ReceivingPaymentRegistryId: Option<i32>,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "TotalValue")]
    pub TotalValue: rust_decimal::Decimal,
    #[serde(rename = "SettledValue")]
    pub SettledValue: rust_decimal::Decimal,
    #[serde(rename = "SettledValuePLN")]
    pub SettledValuePLN: rust_decimal::Decimal,
    #[serde(rename = "PaymentRegistryId")]
    pub PaymentRegistryId: i32,
    #[serde(rename = "PaymentType")]
    pub PaymentType: crate::web_api::interface::enums::enumPaymentType,
    #[serde(rename = "SubjectType")]
    pub SubjectType: crate::web_api::interface::enums::enumPaymentSubjectType,
    #[serde(rename = "Positions")]
    pub Positions: Vec<PaymentPosition>,
}

/// WebAPI model `PaymentListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentListElement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "SubjectId")]
    pub SubjectId: Option<i32>,
    #[serde(rename = "ReceivingPaymentRegistryId")]
    pub ReceivingPaymentRegistryId: Option<i32>,
    #[serde(rename = "PaymentRegistryId")]
    pub PaymentRegistryId: Option<i32>,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "SettlementDate")]
    pub SettlementDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "PaymentType")]
    pub PaymentType: crate::web_api::interface::enums::enumPaymentType,
    #[serde(rename = "SubjectType")]
    pub SubjectType: crate::web_api::interface::enums::enumPaymentSubjectType,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "TotalValue")]
    pub TotalValue: rust_decimal::Decimal,
    #[serde(rename = "SettledValue")]
    pub SettledValue: rust_decimal::Decimal,
    #[serde(rename = "SettledValuePLN")]
    pub SettledValuePLN: rust_decimal::Decimal,
}

/// WebAPI model `PaymentPosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentPosition {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "ValuePLN")]
    pub ValuePLN: rust_decimal::Decimal,
    #[serde(rename = "ValueSettlement")]
    pub ValueSettlement: rust_decimal::Decimal,
    #[serde(rename = "ValuePayment")]
    pub ValuePayment: rust_decimal::Decimal,
    #[serde(rename = "CurrencyRateSettlement")]
    pub CurrencyRateSettlement: rust_decimal::Decimal,
    #[serde(rename = "CurrencyRatePayment")]
    pub CurrencyRatePayment: rust_decimal::Decimal,
}

/// WebAPI model `Settlement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settlement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentId")]
    pub DocumentId: Option<i32>,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Canceled")]
    pub Canceled: crate::web_api::interface::enums::enumCancelationType,
    #[serde(rename = "Buffer")]
    pub Buffer: bool,
    #[serde(rename = "Settled")]
    pub Settled: bool,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "SettlementDate")]
    pub SettlementDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "TypeCode")]
    pub TypeCode: String,
    #[serde(rename = "Series")]
    pub Series: String,
    #[serde(rename = "NumberInSeries")]
    pub NumberInSeries: i32,
    #[serde(rename = "IssuerId")]
    pub IssuerId: i32,
    #[serde(rename = "ContractorId")]
    pub ContractorId: Option<i32>,
    #[serde(rename = "SubjectId")]
    pub SubjectId: Option<i32>,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "TotalValue")]
    pub TotalValue: rust_decimal::Decimal,
    #[serde(rename = "PaidValue")]
    pub PaidValue: rust_decimal::Decimal,
    #[serde(rename = "PaidValuePLN")]
    pub PaidValuePLN: rust_decimal::Decimal,
    #[serde(rename = "LeftToPayValue")]
    pub LeftToPayValue: rust_decimal::Decimal,
    #[serde(rename = "LeftToPayValuePLN")]
    pub LeftToPayValuePLN: rust_decimal::Decimal,
    #[serde(rename = "PaymentRegistryId")]
    pub PaymentRegistryId: i32,
    #[serde(rename = "SettlementType")]
    pub SettlementType: crate::web_api::interface::enums::enumSettlementType,
    #[serde(rename = "SubjectType")]
    pub SubjectType: crate::web_api::interface::enums::enumPaymentSubjectType,
    #[serde(rename = "Positions")]
    pub Positions: Vec<SettlementPosition>,
}

/// WebAPI model `SettlementListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementListElement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentId")]
    pub DocumentId: Option<i32>,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "ContractorId")]
    pub ContractorId: Option<i32>,
    #[serde(rename = "SubjectId")]
    pub SubjectId: Option<i32>,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "SettlementDate")]
    pub SettlementDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "SettlementType")]
    pub SettlementType: crate::web_api::interface::enums::enumSettlementType,
    #[serde(rename = "SubjectType")]
    pub SubjectType: crate::web_api::interface::enums::enumPaymentSubjectType,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "TotalValue")]
    pub TotalValue: rust_decimal::Decimal,
    #[serde(rename = "PaidValue")]
    pub PaidValue: rust_decimal::Decimal,
    #[serde(rename = "PaidValuePLN")]
    pub PaidValuePLN: rust_decimal::Decimal,
    #[serde(rename = "LeftToPayValue")]
    pub LeftToPayValue: rust_decimal::Decimal,
    #[serde(rename = "LeftToPayValuePLN")]
    pub LeftToPayValuePLN: rust_decimal::Decimal,
}

/// WebAPI model `SettlementPosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementPosition {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "ValuePLN")]
    pub ValuePLN: rust_decimal::Decimal,
    #[serde(rename = "ValueSettlement")]
    pub ValueSettlement: rust_decimal::Decimal,
    #[serde(rename = "ValuePayment")]
    pub ValuePayment: rust_decimal::Decimal,
    #[serde(rename = "CurrencyRateSettlement")]
    pub CurrencyRateSettlement: rust_decimal::Decimal,
    #[serde(rename = "CurrencyRatePayment")]
    pub CurrencyRatePayment: rust_decimal::Decimal,
}
