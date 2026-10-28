use serde::{Deserialize, Serialize};

/// WebAPI model `DocumentIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentIssue {
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "YearId")]
    pub YearId: i32,
    #[serde(rename = "TypeCode")]
    pub TypeCode: String,
    #[serde(rename = "Content")]
    pub Content: String,
    #[serde(rename = "IssueDate")]
    pub IssueDate: chrono::NaiveDateTime,
    #[serde(rename = "DocumentDate")]
    pub DocumentDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "PeriodDate")]
    pub PeriodDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "OperationDate")]
    pub OperationDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ReceiptDate")]
    pub ReceiptDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ContractorPosition")]
    pub ContractorPosition: i32,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "CurrencyRateTableId")]
    pub CurrencyRateTableId: i32,
    #[serde(rename = "CurrencyRateType")]
    pub CurrencyRateType: crate::web_api::interface::enums::enumCurrencyRateType,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "SplitPaymentType")]
    pub SplitPaymentType: crate::web_api::interface::enums::enumFKSplitPaymentType,
    #[serde(rename = "JPK_V7Attributes")]
    pub JPK_V7Attributes: Option<crate::web_api::interface::enums::enumJPK_V7DocumentAttribute>,
    #[serde(rename = "Records")]
    pub Records: Vec<DocumentRecordIssue>,
    #[serde(rename = "Settlements")]
    pub Settlements: Vec<DocumentSettlementIssue>,
    #[serde(rename = "Transactions")]
    pub Transactions: Vec<DocumentTransactionIssue>,
    #[serde(rename = "VatRegisters")]
    pub VatRegisters: Vec<DocumentVatRegisterIssue>,
    #[serde(rename = "Features")]
    pub Features: Vec<i32>,
    #[serde(rename = "IsSmeProcedure")]
    pub IsSmeProcedure: bool,
}

/// WebAPI model `DocumentRecordDimensionIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRecordDimensionIssue {
    #[serde(rename = "Index")]
    pub Index: i32,
    #[serde(rename = "Value")]
    pub Value: Option<String>,
}

/// WebAPI model `DocumentRecordDimensionSplitIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRecordDimensionSplitIssue {
    #[serde(rename = "SplitValue")]
    pub SplitValue: rust_decimal::Decimal,
    #[serde(rename = "Dimensions")]
    pub Dimensions: Vec<DocumentRecordDimensionIssue>,
}

/// WebAPI model `DocumentRecordIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRecordIssue {
    #[serde(rename = "No")]
    pub No: i32,
    #[serde(rename = "SplitNo")]
    pub SplitNo: i32,
    #[serde(rename = "Account")]
    pub Account:
        crate::web_api::interface::fkf::documents::view_models::issue::DocumentRecordAccountIssue,
    #[serde(rename = "ParallelType")]
    pub ParallelType: crate::web_api::interface::enums::enumParallelType,
    #[serde(rename = "Side")]
    pub Side: crate::web_api::interface::enums::enumSideType,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "DescriptionKind")]
    pub DescriptionKind: crate::web_api::interface::enums::enumRecordDecsriptionKind,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "ValuePLN")]
    pub ValuePLN: rust_decimal::Decimal,
    #[serde(rename = "Settlements")]
    pub Settlements: Vec<DocumentSettlementIssue>,
    #[serde(rename = "Transactions")]
    pub Transactions: Vec<DocumentTransactionIssue>,
    #[serde(rename = "Features")]
    pub Features: Vec<i32>,
    #[serde(rename = "DimensionSplits")]
    pub DimensionSplits: Vec<DocumentRecordDimensionSplitIssue>,
}

/// WebAPI model `DocumentSettlementIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSettlementIssue {
    #[serde(rename = "TransactionId")]
    pub TransactionId: Option<i32>,
    #[serde(rename = "SettledDocument")]
    pub SettledDocument: String,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "ValuePLN")]
    pub ValuePLN: rust_decimal::Decimal,
}

/// WebAPI model `DocumentTransactionIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentTransactionIssue {
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: chrono::NaiveDateTime,
    #[serde(rename = "Advance")]
    pub Advance: bool,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "ValuePLN")]
    pub ValuePLN: rust_decimal::Decimal,
    #[serde(rename = "InterestType")]
    pub InterestType: crate::web_api::interface::enums::enumTransactionInterestType,
    #[serde(rename = "InterestRate")]
    pub InterestRate: rust_decimal::Decimal,
    #[serde(rename = "RKP")]
    pub RKP: rust_decimal::Decimal,
}

/// WebAPI model `DocumentVatRegisterIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVatRegisterIssue {
    #[serde(rename = "DefinitionId")]
    pub DefinitionId: i32,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumVatRegisterTypeABCD,
    #[serde(rename = "Period")]
    pub Period: chrono::NaiveDateTime,
    #[serde(rename = "PeriodType")]
    pub PeriodType: crate::web_api::interface::enums::enumVatRegisterPeriodType,
    #[serde(rename = "VatRate")]
    pub VatRate: Option<String>,
    #[serde(rename = "GrossValuePLN")]
    pub GrossValuePLN: rust_decimal::Decimal,
    #[serde(rename = "NetValuePLN")]
    pub NetValuePLN: rust_decimal::Decimal,
    #[serde(rename = "VatValuePLN")]
    pub VatValuePLN: rust_decimal::Decimal,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "JPK_V7ProductGroups")]
    pub JPK_V7ProductGroups: Option<crate::web_api::interface::enums::enumJPK_V7ProductGroup>,
}
