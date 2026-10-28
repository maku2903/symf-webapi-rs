use serde::{Deserialize, Serialize};

/// WebAPI model `BusinessDocumentCorrectionIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessDocumentCorrectionIssue {
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "DocumentDate")]
    pub DocumentDate: chrono::NaiveDateTime,
}

/// WebAPI model `BusinessDocumentIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessDocumentIssue {
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "DocumentType")]
    pub DocumentType: String,
    #[serde(rename = "DocumentTypeFeature")]
    pub DocumentTypeFeature: String,
    #[serde(rename = "AccountingScheme")]
    pub AccountingScheme: String,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Contractor")]
    pub Contractor: String,
    #[serde(rename = "ContractorPosition")]
    pub ContractorPosition: Option<i32>,
    #[serde(rename = "ContractorType")]
    pub ContractorType: String,
    #[serde(rename = "Department")]
    pub Department: String,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "DocumentDate")]
    pub DocumentDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "OperationDate")]
    pub OperationDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ReceiptDate")]
    pub ReceiptDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: Option<rust_decimal::Decimal>,
    #[serde(rename = "ForManualVerification")]
    pub ForManualVerification: bool,
    #[serde(rename = "SetTransactionMarker")]
    pub SetTransactionMarker: bool,
    #[serde(rename = "SplitPaymentType")]
    pub SplitPaymentType: crate::web_api::interface::enums::enumFKSplitPaymentType,
    #[serde(rename = "Positions")]
    pub Positions: Vec<BusinessDocumentPositionIssue>,
    #[serde(rename = "Corrections")]
    pub Corrections: Vec<BusinessDocumentCorrectionIssue>,
}

/// WebAPI model `BusinessDocumentPositionIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessDocumentPositionIssue {
    #[serde(rename = "PositionType")]
    pub PositionType: Option<crate::web_api::interface::enums::enumAccountingSchemePositionType>,
    #[serde(rename = "JPK_V7ProductGroups")]
    pub JPK_V7ProductGroups: Option<crate::web_api::interface::enums::enumJPK_V7ProductGroup>,
    #[serde(rename = "VatRate")]
    pub VatRate: Option<String>,
    #[serde(rename = "Product")]
    pub Product: String,
    #[serde(rename = "ProductType")]
    pub ProductType: String,
    #[serde(rename = "CostType")]
    pub CostType: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "GrossValue")]
    pub GrossValue: rust_decimal::Decimal,
    #[serde(rename = "NetValue")]
    pub NetValue: rust_decimal::Decimal,
    #[serde(rename = "VatValue")]
    pub VatValue: rust_decimal::Decimal,
    #[serde(rename = "GrossValuePLN")]
    pub GrossValuePLN: rust_decimal::Decimal,
    #[serde(rename = "NetValuePLN")]
    pub NetValuePLN: rust_decimal::Decimal,
    #[serde(rename = "VatValuePLN")]
    pub VatValuePLN: rust_decimal::Decimal,
    #[serde(rename = "Account01")]
    pub Account01: String,
    #[serde(rename = "Account02")]
    pub Account02: String,
    #[serde(rename = "Account03")]
    pub Account03: String,
    #[serde(rename = "Account04")]
    pub Account04: String,
    #[serde(rename = "Account05")]
    pub Account05: String,
    #[serde(rename = "Account06")]
    pub Account06: String,
    #[serde(rename = "Account07")]
    pub Account07: String,
    #[serde(rename = "Account08")]
    pub Account08: String,
    #[serde(rename = "Account09")]
    pub Account09: String,
    #[serde(rename = "Account10")]
    pub Account10: String,
    #[serde(rename = "Account11")]
    pub Account11: String,
    #[serde(rename = "Account12")]
    pub Account12: String,
    #[serde(rename = "Account13")]
    pub Account13: String,
    #[serde(rename = "Account14")]
    pub Account14: String,
    #[serde(rename = "Account15")]
    pub Account15: String,
    #[serde(rename = "Account16")]
    pub Account16: String,
    #[serde(rename = "Account17")]
    pub Account17: String,
    #[serde(rename = "Account18")]
    pub Account18: String,
    #[serde(rename = "Account19")]
    pub Account19: String,
    #[serde(rename = "Account20")]
    pub Account20: String,
}
