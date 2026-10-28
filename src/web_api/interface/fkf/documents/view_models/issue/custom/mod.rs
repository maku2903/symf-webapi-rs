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
    #[serde(rename = "ClearanceDate")]
    pub ClearanceDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ContractorPosition")]
    pub ContractorPosition: Option<i32>,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: Option<rust_decimal::Decimal>,
    #[serde(rename = "CurrencyRateTableId")]
    pub CurrencyRateTableId: Option<i32>,
    #[serde(rename = "CurrencyRateType")]
    pub CurrencyRateType: Option<crate::web_api::interface::enums::enumCurrencyRateType>,
    #[serde(rename = "Marker")]
    pub Marker: Option<i32>,
    #[serde(rename = "SplitPaymentType")]
    pub SplitPaymentType: Option<crate::web_api::interface::enums::enumFKSplitPaymentType>,
    #[serde(rename = "JPK_V7Attributes")]
    pub JPK_V7Attributes: Option<crate::web_api::interface::enums::enumJPK_V7DocumentAttribute>,
    #[serde(rename = "Value")]
    pub Value: Option<rust_decimal::Decimal>,
    #[serde(rename = "ValuePLN")]
    pub ValuePLN: Option<rust_decimal::Decimal>,
    #[serde(rename = "ValueParallel")]
    pub ValueParallel: Option<rust_decimal::Decimal>,
    #[serde(rename = "ValueOffVatRegistry")]
    pub ValueOffVatRegistry: Option<rust_decimal::Decimal>,
    #[serde(rename = "OpeningBalance")]
    pub OpeningBalance: Option<rust_decimal::Decimal>,
    #[serde(rename = "RecordsBalance")]
    pub RecordsBalance: Option<rust_decimal::Decimal>,
    #[serde(rename = "eArchiveId")]
    pub eArchiveId: Option<String>,
    #[serde(rename = "NumberKSeF")]
    pub NumberKSeF: Option<String>,
    #[serde(rename = "IssueDateKSeF")]
    pub IssueDateKSeF: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Records")]
    pub Records: Vec<DocumentRecordIssue>,
    #[serde(rename = "Settlements")]
    pub Settlements: Vec<DocumentSettlementIssue>,
    #[serde(rename = "Transactions")]
    pub Transactions: Vec<DocumentTransactionIssue>,
    #[serde(rename = "VatRegisters")]
    pub VatRegisters: Vec<DocumentVatRegisterIssue>,
    #[serde(rename = "CurrencyVAT")]
    pub CurrencyVAT: DocumentVatRegisterCurrencyIssue,
    #[serde(rename = "CurrencyCIT_PIT")]
    pub CurrencyCIT_PIT: DocumentVatRegisterCurrencyIssue,
    #[serde(rename = "Relations")]
    pub Relations: Vec<DocumentRelationIssue>,
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
    pub ParallelType: Option<crate::web_api::interface::enums::enumParallelType>,
    #[serde(rename = "Side")]
    pub Side: crate::web_api::interface::enums::enumSideType,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "DescriptionKind")]
    pub DescriptionKind: Option<crate::web_api::interface::enums::enumRecordDecsriptionKind>,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: Option<rust_decimal::Decimal>,
    #[serde(rename = "CurrencyRateTableId")]
    pub CurrencyRateTableId: Option<i32>,
    #[serde(rename = "CurrencyRateType")]
    pub CurrencyRateType: Option<crate::web_api::interface::enums::enumCurrencyRateType>,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "Value")]
    pub Value: Option<rust_decimal::Decimal>,
    #[serde(rename = "ValuePLN")]
    pub ValuePLN: Option<rust_decimal::Decimal>,
    #[serde(rename = "ReportAccount")]
    pub ReportAccount: Option<bool>,
    #[serde(rename = "KPKWDate")]
    pub KPKWDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Transactions")]
    pub Transactions: Vec<DocumentTransactionIssue>,
    #[serde(rename = "Settlements")]
    pub Settlements: Vec<DocumentSettlementIssue>,
    #[serde(rename = "Features")]
    pub Features: Vec<i32>,
    #[serde(rename = "DimensionSplits")]
    pub DimensionSplits: Vec<DocumentRecordDimensionSplitIssue>,
}

/// WebAPI model `DocumentRelationIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRelationIssue {
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "DocumentDate")]
    pub DocumentDate: chrono::NaiveDateTime,
    #[serde(rename = "ContractorPosition")]
    pub ContractorPosition: i32,
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
    pub Value: Option<rust_decimal::Decimal>,
    #[serde(rename = "ValuePLN")]
    pub ValuePLN: Option<rust_decimal::Decimal>,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: Option<rust_decimal::Decimal>,
    #[serde(rename = "CurrencyForeign")]
    pub CurrencyForeign: String,
    #[serde(rename = "CurrencyRateForeign")]
    pub CurrencyRateForeign: Option<rust_decimal::Decimal>,
    #[serde(rename = "ValueForeign")]
    pub ValueForeign: Option<rust_decimal::Decimal>,
}

/// WebAPI model `DocumentTransactionIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentTransactionIssue {
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Advance")]
    pub Advance: Option<bool>,
    #[serde(rename = "Value")]
    pub Value: Option<rust_decimal::Decimal>,
    #[serde(rename = "ValuePLN")]
    pub ValuePLN: Option<rust_decimal::Decimal>,
    #[serde(rename = "TransactionId")]
    pub TransactionId: Option<i32>,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: Option<rust_decimal::Decimal>,
    #[serde(rename = "InterestType")]
    pub InterestType: Option<crate::web_api::interface::enums::enumTransactionInterestType>,
    #[serde(rename = "InterestRate")]
    pub InterestRate: Option<rust_decimal::Decimal>,
    #[serde(rename = "RKP")]
    pub RKP: Option<rust_decimal::Decimal>,
    #[serde(rename = "BankAccountId")]
    pub BankAccountId: Option<i32>,
    #[serde(rename = "Marker")]
    pub Marker: Option<i32>,
}

/// WebAPI model `DocumentVatRegisterCurrencyIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVatRegisterCurrencyIssue {
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: Option<rust_decimal::Decimal>,
    #[serde(rename = "CurrencyRateTableId")]
    pub CurrencyRateTableId: Option<i32>,
    #[serde(rename = "CurrencyRateTableDate")]
    pub CurrencyRateTableDate: Option<chrono::NaiveDateTime>,
}

/// WebAPI model `DocumentVatRegisterIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVatRegisterIssue {
    #[serde(rename = "DefinitionId")]
    pub DefinitionId: i32,
    #[serde(rename = "Type")]
    pub Type: Option<crate::web_api::interface::enums::enumVatRegisterTypeABCD>,
    #[serde(rename = "Period")]
    pub Period: Option<chrono::NaiveDateTime>,
    #[serde(rename = "PeriodType")]
    pub PeriodType: Option<crate::web_api::interface::enums::enumVatRegisterPeriodType>,
    #[serde(rename = "Service")]
    pub Service: Option<bool>,
    #[serde(rename = "UE")]
    pub UE: Option<bool>,
    #[serde(rename = "Chargable")]
    pub Chargable: Option<bool>,
    #[serde(rename = "VatRate")]
    pub VatRate: Option<String>,
    #[serde(rename = "GrossValuePLN")]
    pub GrossValuePLN: Option<rust_decimal::Decimal>,
    #[serde(rename = "NetValuePLN")]
    pub NetValuePLN: Option<rust_decimal::Decimal>,
    #[serde(rename = "VatValuePLN")]
    pub VatValuePLN: Option<rust_decimal::Decimal>,
    #[serde(rename = "GrossValue")]
    pub GrossValue: Option<rust_decimal::Decimal>,
    #[serde(rename = "NetValue")]
    pub NetValue: Option<rust_decimal::Decimal>,
    #[serde(rename = "VatValue")]
    pub VatValue: Option<rust_decimal::Decimal>,
    #[serde(rename = "Marker")]
    pub Marker: Option<i32>,
    #[serde(rename = "JPK_V7ProductGroups")]
    pub JPK_V7ProductGroups: Option<crate::web_api::interface::enums::enumJPK_V7ProductGroup>,
    #[serde(rename = "ContractorPosition")]
    pub ContractorPosition: Option<i32>,
    #[serde(rename = "SplitNo")]
    pub SplitNo: Option<i32>,
    #[serde(rename = "DocumentDate")]
    pub DocumentDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ReceiptDate")]
    pub ReceiptDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "OssShippingCountry")]
    pub OssShippingCountry: String,
    #[serde(rename = "OssDeliveryCountry")]
    pub OssDeliveryCountry: String,
    #[serde(rename = "OssDeliveryKind")]
    pub OssDeliveryKind: Option<crate::web_api::interface::enums::enumOssDeliveryKind>,
    #[serde(rename = "OssCorrectionDate")]
    pub OssCorrectionDate: Option<chrono::NaiveDateTime>,
}
