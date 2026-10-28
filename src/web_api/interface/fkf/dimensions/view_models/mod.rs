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
    #[serde(rename = "DocumentDate")]
    pub DocumentDate: chrono::NaiveDateTime,
    #[serde(rename = "ContractorPosition")]
    pub ContractorPosition: i32,
    #[serde(rename = "Records")]
    pub Records: Vec<DocumentRecord>,
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
    #[serde(rename = "Side")]
    pub Side: crate::web_api::interface::enums::enumSideType,
    #[serde(rename = "Account")]
    pub Account: String,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "ValuePLN")]
    pub ValuePLN: rust_decimal::Decimal,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "DimensionSplits")]
    pub DimensionSplits: Vec<DocumentRecordDimensionSplit>,
}

/// WebAPI model `DocumentRecordDimension`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRecordDimension {
    #[serde(rename = "Index")]
    pub Index: i32,
    #[serde(rename = "Value")]
    pub Value: Option<String>,
}

/// WebAPI model `DocumentRecordDimensionSplit`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRecordDimensionSplit {
    #[serde(rename = "SplitValue")]
    pub SplitValue: rust_decimal::Decimal,
    #[serde(rename = "Percent")]
    pub Percent: rust_decimal::Decimal,
    #[serde(rename = "Dimensions")]
    pub Dimensions: Vec<DocumentRecordDimension>,
}
