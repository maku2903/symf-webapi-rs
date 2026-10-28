pub mod issue;
pub mod rdf;
pub mod transactions;

use serde::{Deserialize, Serialize};

/// WebAPI model `Document`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "Source")]
    pub Source: crate::web_api::interface::enums::enumDocumentSource,
    #[serde(rename = "YearId")]
    pub YearId: i32,
    #[serde(rename = "SettlementId")]
    pub SettlementId: i32,
    #[serde(rename = "IssueDate")]
    pub IssueDate: chrono::NaiveDateTime,
    #[serde(rename = "DocumentDate")]
    pub DocumentDate: chrono::NaiveDateTime,
    #[serde(rename = "PeriodDate")]
    pub PeriodDate: chrono::NaiveDateTime,
    #[serde(rename = "OperationDate")]
    pub OperationDate: chrono::NaiveDateTime,
    #[serde(rename = "ReceiptDate")]
    pub ReceiptDate: chrono::NaiveDateTime,
    #[serde(rename = "TypeCode")]
    pub TypeCode: String,
    #[serde(rename = "NumberInSeries")]
    pub NumberInSeries: i32,
    #[serde(rename = "Content")]
    pub Content: String,
    #[serde(rename = "ContractorPosition")]
    pub ContractorPosition: i32,
    #[serde(rename = "TrilateralContractorPosition")]
    pub TrilateralContractorPosition: i32,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "ValuePLN")]
    pub ValuePLN: rust_decimal::Decimal,
    #[serde(rename = "ValueParallel")]
    pub ValueParallel: rust_decimal::Decimal,
    #[serde(rename = "ValueOffVatRegistry")]
    pub ValueOffVatRegistry: rust_decimal::Decimal,
    #[serde(rename = "OpeningBalance")]
    pub OpeningBalance: rust_decimal::Decimal,
    #[serde(rename = "RecordsBalance")]
    pub RecordsBalance: rust_decimal::Decimal,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "CurrencyRateTableId")]
    pub CurrencyRateTableId: i32,
    #[serde(rename = "CurrencyRateType")]
    pub CurrencyRateType: crate::web_api::interface::enums::enumCurrencyRateType,
    #[serde(rename = "CorrectionNumber")]
    pub CorrectionNumber: String,
    #[serde(rename = "CorrectionDate")]
    pub CorrectionDate: chrono::NaiveDateTime,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "SplitPaymentType")]
    pub SplitPaymentType: crate::web_api::interface::enums::enumFKSplitPaymentType,
    #[serde(rename = "JPK_V7Attributes")]
    pub JPK_V7Attributes: crate::web_api::interface::enums::enumJPK_V7DocumentAttribute,
    #[serde(rename = "ClearenceDate")]
    pub ClearenceDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "StatusKSeF")]
    pub StatusKSeF: crate::web_api::interface::enums::enumKSeFInvoiceStatus,
    #[serde(rename = "eArchiveId")]
    pub eArchiveId: Option<String>,
    #[serde(rename = "NumberKSeF")]
    pub NumberKSeF: Option<String>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "PaymentDate")]
    pub PaymentDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Origin")]
    pub Origin: String,
    #[serde(rename = "IssueDateKSeF")]
    pub IssueDateKSeF: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Records")]
    pub Records: Vec<DocumentRecord>,
    #[serde(rename = "Settlements")]
    pub Settlements: Vec<DocumentSettlement>,
    #[serde(rename = "Transactions")]
    pub Transactions: Vec<DocumentTransaction>,
    #[serde(rename = "VatRegisters")]
    pub VatRegisters: Vec<DocumentVatRegister>,
    #[serde(rename = "CurrencyVAT")]
    pub CurrencyVAT: DocumentVatRegisterCurrency,
    #[serde(rename = "CurrencyCIT_PIT")]
    pub CurrencyCIT_PIT: DocumentVatRegisterCurrency,
    #[serde(rename = "Features")]
    pub Features: Vec<i32>,
    #[serde(rename = "FKMessages")]
    pub FKMessages: Vec<DocumentMessage>,
    #[serde(rename = "Relations")]
    pub Relations: Vec<DocumentRelation>,
}

/// WebAPI model `DocumentListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentListElement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "Source")]
    pub Source: crate::web_api::interface::enums::enumDocumentSource,
    #[serde(rename = "YearId")]
    pub YearId: i32,
    #[serde(rename = "DocumentDate")]
    pub DocumentDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "PeriodDate")]
    pub PeriodDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "OperationDate")]
    pub OperationDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ReceiptDate")]
    pub ReceiptDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "TypeCode")]
    pub TypeCode: String,
    #[serde(rename = "NumberInSeries")]
    pub NumberInSeries: i32,
    #[serde(rename = "Content")]
    pub Content: String,
    #[serde(rename = "ContractorPosition")]
    pub ContractorPosition: Option<i32>,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "ValuePLN")]
    pub ValuePLN: rust_decimal::Decimal,
    #[serde(rename = "Currency")]
    pub Currency: String,
}

/// WebAPI model `DocumentMessage`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMessage {
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumFKDocumentMessageType,
    #[serde(rename = "Message")]
    pub Message: String,
}

/// WebAPI model `DocumentRecord`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRecord {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "No")]
    pub No: i32,
    #[serde(rename = "SplitNo")]
    pub SplitNo: i32,
    #[serde(rename = "Account")]
    pub Account: String,
    #[serde(rename = "Account_Full")]
    pub Account_Full: crate::web_api::interface::fkf::common::view_models::Account,
    #[serde(rename = "SplitType")]
    pub SplitType: crate::web_api::interface::enums::enumSplitType,
    #[serde(rename = "SplitOpposingAccountType")]
    pub SplitOpposingAccountType: crate::web_api::interface::enums::enumSplitOpposingAccountType,
    #[serde(rename = "ParallelType")]
    pub ParallelType: crate::web_api::interface::enums::enumParallelType,
    #[serde(rename = "Side")]
    pub Side: crate::web_api::interface::enums::enumSideType,
    #[serde(rename = "RecordVat")]
    pub RecordVat: bool,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "DescriptionKind")]
    pub DescriptionKind: crate::web_api::interface::enums::enumRecordDecsriptionKind,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "CurrencyRateTableId")]
    pub CurrencyRateTableId: i32,
    #[serde(rename = "CurrencyRateType")]
    pub CurrencyRateType: crate::web_api::interface::enums::enumCurrencyRateType,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "ValuePLN")]
    pub ValuePLN: rust_decimal::Decimal,
    #[serde(rename = "ReportAccount")]
    pub ReportAccount: bool,
    #[serde(rename = "KPKWDate")]
    pub KPKWDate: chrono::NaiveDateTime,
    #[serde(rename = "Settlements")]
    pub Settlements: Vec<DocumentSettlement>,
    #[serde(rename = "Transactions")]
    pub Transactions: Vec<DocumentTransaction>,
    #[serde(rename = "Features")]
    pub Features: Vec<i32>,
}

/// WebAPI model `DocumentRelation`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRelation {
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "DocumentDate")]
    pub DocumentDate: chrono::NaiveDateTime,
    #[serde(rename = "DocumentId")]
    pub DocumentId: i32,
    #[serde(rename = "YearId")]
    pub YearId: i32,
}

/// WebAPI model `DocumentSettlement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSettlement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "TransactionId")]
    pub TransactionId: i32,
    #[serde(rename = "SettledDocument")]
    pub SettledDocument: String,
    #[serde(rename = "Side")]
    pub Side: crate::web_api::interface::enums::enumSideType,
    #[serde(rename = "Account")]
    pub Account: String,
    #[serde(rename = "Account_Full")]
    pub Account_Full: crate::web_api::interface::fkf::common::view_models::Account,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "ValuePLN")]
    pub ValuePLN: rust_decimal::Decimal,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "CurrencyForeign")]
    pub CurrencyForeign: String,
    #[serde(rename = "CurrencyRateForeign")]
    pub CurrencyRateForeign: rust_decimal::Decimal,
    #[serde(rename = "ValueForeign")]
    pub ValueForeign: rust_decimal::Decimal,
}

/// WebAPI model `DocumentTransaction`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentTransaction {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Side")]
    pub Side: crate::web_api::interface::enums::enumSideType,
    #[serde(rename = "Account")]
    pub Account: String,
    #[serde(rename = "Account_Full")]
    pub Account_Full: crate::web_api::interface::fkf::common::view_models::Account,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: chrono::NaiveDateTime,
    #[serde(rename = "Advance")]
    pub Advance: bool,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "ValuePLN")]
    pub ValuePLN: rust_decimal::Decimal,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "InterestType")]
    pub InterestType: crate::web_api::interface::enums::enumTransactionInterestType,
    #[serde(rename = "InterestRate")]
    pub InterestRate: rust_decimal::Decimal,
    #[serde(rename = "RKP")]
    pub RKP: rust_decimal::Decimal,
    #[serde(rename = "BankAccountId")]
    pub BankAccountId: Option<i32>,
}

/// WebAPI model `DocumentType`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentType {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "YearId")]
    pub YearId: i32,
    #[serde(rename = "Character")]
    pub Character: crate::web_api::interface::enums::enumFKDocumentCharacter,
}

/// WebAPI model `DocumentV2`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentV2 {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "Source")]
    pub Source: crate::web_api::interface::enums::enumDocumentSource,
    #[serde(rename = "YearId")]
    pub YearId: i32,
    #[serde(rename = "SettlementId")]
    pub SettlementId: i32,
    #[serde(rename = "IssueDate")]
    pub IssueDate: chrono::NaiveDateTime,
    #[serde(rename = "DocumentDate")]
    pub DocumentDate: chrono::NaiveDateTime,
    #[serde(rename = "PeriodDate")]
    pub PeriodDate: chrono::NaiveDateTime,
    #[serde(rename = "OperationDate")]
    pub OperationDate: chrono::NaiveDateTime,
    #[serde(rename = "ReceiptDate")]
    pub ReceiptDate: chrono::NaiveDateTime,
    #[serde(rename = "TypeCode")]
    pub TypeCode: String,
    #[serde(rename = "NumberInSeries")]
    pub NumberInSeries: i32,
    #[serde(rename = "Content")]
    pub Content: String,
    #[serde(rename = "ContractorPosition")]
    pub ContractorPosition: i32,
    #[serde(rename = "TrilateralContractorPosition")]
    pub TrilateralContractorPosition: i32,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "ValuePLN")]
    pub ValuePLN: rust_decimal::Decimal,
    #[serde(rename = "ValueParallel")]
    pub ValueParallel: rust_decimal::Decimal,
    #[serde(rename = "ValueOffVatRegistry")]
    pub ValueOffVatRegistry: rust_decimal::Decimal,
    #[serde(rename = "OpeningBalance")]
    pub OpeningBalance: rust_decimal::Decimal,
    #[serde(rename = "RecordsBalance")]
    pub RecordsBalance: rust_decimal::Decimal,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "CurrencyRateTableId")]
    pub CurrencyRateTableId: i32,
    #[serde(rename = "CurrencyRateType")]
    pub CurrencyRateType: crate::web_api::interface::enums::enumCurrencyRateType,
    #[serde(rename = "CorrectionNumber")]
    pub CorrectionNumber: String,
    #[serde(rename = "CorrectionDate")]
    pub CorrectionDate: chrono::NaiveDateTime,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "SplitPaymentType")]
    pub SplitPaymentType: crate::web_api::interface::enums::enumFKSplitPaymentType,
    #[serde(rename = "JpkV7Attributes")]
    pub JpkV7Attributes: crate::web_api::interface::enums::enumJPK_V7DocumentAttribute,
    #[serde(rename = "ClearanceDate")]
    pub ClearanceDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "StatusKSeF")]
    pub StatusKSeF: crate::web_api::interface::enums::enumKSeFInvoiceStatus,
    #[serde(rename = "EArchiveId")]
    pub EArchiveId: Option<String>,
    #[serde(rename = "NumberKSeF")]
    pub NumberKSeF: Option<String>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "PaymentDate")]
    pub PaymentDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Origin")]
    pub Origin: String,
    #[serde(rename = "IssueDateKSeF")]
    pub IssueDateKSeF: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Records")]
    pub Records: Vec<DocumentRecord>,
    #[serde(rename = "Settlements")]
    pub Settlements: Vec<DocumentSettlement>,
    #[serde(rename = "Transactions")]
    pub Transactions: Vec<DocumentTransaction>,
    #[serde(rename = "VatRegisters")]
    pub VatRegisters: Vec<DocumentVatRegister>,
    #[serde(rename = "CurrencyVAT")]
    pub CurrencyVAT: DocumentVatRegisterCurrency,
    #[serde(rename = "CurrencyCIT_PIT")]
    pub CurrencyCIT_PIT: DocumentVatRegisterCurrency,
    #[serde(rename = "Features")]
    pub Features: Vec<i32>,
    #[serde(rename = "FKMessages")]
    pub FKMessages: Vec<DocumentMessage>,
    #[serde(rename = "Relations")]
    pub Relations: Vec<DocumentRelation>,
}

/// WebAPI model `DocumentVatRegister`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVatRegister {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DefinitionId")]
    pub DefinitionId: i32,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumVatRegisterTypeABCD,
    #[serde(rename = "Period")]
    pub Period: chrono::NaiveDateTime,
    #[serde(rename = "PeriodType")]
    pub PeriodType: crate::web_api::interface::enums::enumVatRegisterPeriodType,
    #[serde(rename = "Service")]
    pub Service: bool,
    #[serde(rename = "UE")]
    pub UE: bool,
    #[serde(rename = "Chargable")]
    pub Chargable: bool,
    #[serde(rename = "VatRate")]
    pub VatRate: Option<String>,
    #[serde(rename = "GrossValue")]
    pub GrossValue: rust_decimal::Decimal,
    #[serde(rename = "GrossValuePLN")]
    pub GrossValuePLN: rust_decimal::Decimal,
    #[serde(rename = "NetValue")]
    pub NetValue: rust_decimal::Decimal,
    #[serde(rename = "NetValuePLN")]
    pub NetValuePLN: rust_decimal::Decimal,
    #[serde(rename = "VatValue")]
    pub VatValue: rust_decimal::Decimal,
    #[serde(rename = "VatValuePLN")]
    pub VatValuePLN: rust_decimal::Decimal,
    #[serde(rename = "ToPay")]
    pub ToPay: rust_decimal::Decimal,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "CorrectionNumber")]
    pub CorrectionNumber: String,
    #[serde(rename = "CorrectionDate")]
    pub CorrectionDate: chrono::NaiveDateTime,
    #[serde(rename = "JPK_V7ProductGroups")]
    pub JPK_V7ProductGroups: crate::web_api::interface::enums::enumJPK_V7ProductGroup,
    #[serde(rename = "ContractorPosition")]
    pub ContractorPosition: i32,
    #[serde(rename = "SplitNo")]
    pub SplitNo: i32,
    #[serde(rename = "DocumentDate")]
    pub DocumentDate: chrono::NaiveDateTime,
    #[serde(rename = "ReceiptDate")]
    pub ReceiptDate: chrono::NaiveDateTime,
}

/// WebAPI model `DocumentVatRegisterCurrency`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVatRegisterCurrency {
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: Option<rust_decimal::Decimal>,
    #[serde(rename = "CurrencyRateTableId")]
    pub CurrencyRateTableId: Option<i32>,
    #[serde(rename = "CurrencyRateTableDate")]
    pub CurrencyRateTableDate: Option<chrono::NaiveDateTime>,
}

/// WebAPI model `Feature`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "YearId")]
    pub YearId: i32,
    #[serde(rename = "BufferOnly")]
    pub BufferOnly: bool,
}
