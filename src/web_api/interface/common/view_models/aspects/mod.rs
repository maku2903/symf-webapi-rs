use serde::{Deserialize, Serialize};

/// WebAPI model `Aspect`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aspect {
    #[serde(rename = "Index")]
    pub Index: i32,
    #[serde(rename = "Value")]
    pub Value: String,
    #[serde(rename = "DictionaryValue")]
    pub DictionaryValue: String,
}

/// WebAPI model `AspectDocument`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AspectDocument {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "Buffer")]
    pub Buffer: bool,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Positions")]
    pub Positions: Vec<AspectPositionDetails>,
}

/// WebAPI model `AspectPosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AspectPosition {
    #[serde(rename = "Id")]
    pub Id: i32,
}

/// WebAPI model `AspectPositionDetails`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AspectPositionDetails {
    #[serde(rename = "No")]
    pub No: i32,
    #[serde(rename = "ProductId")]
    pub ProductId: Option<i32>,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
    #[serde(rename = "NetValuePLN")]
    pub NetValuePLN: rust_decimal::Decimal,
    #[serde(rename = "AspectSplits")]
    pub AspectSplits: Vec<AspectSplitDetails>,
    #[serde(rename = "Id")]
    pub Id: i32,
}

/// WebAPI model `AspectPositionEdit`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AspectPositionEdit {
    #[serde(rename = "SplitValueType")]
    pub SplitValueType: crate::web_api::interface::enums::enumSplitValueType,
    #[serde(rename = "AspectSplits")]
    pub AspectSplits: Vec<AspectSplit>,
    #[serde(rename = "Id")]
    pub Id: i32,
}

/// WebAPI model `AspectSplit`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AspectSplit {
    #[serde(rename = "SplitValue")]
    pub SplitValue: rust_decimal::Decimal,
    #[serde(rename = "Aspects")]
    pub Aspects: Vec<Aspect>,
}

/// WebAPI model `AspectSplitDetails`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AspectSplitDetails {
    #[serde(rename = "Percent")]
    pub Percent: rust_decimal::Decimal,
    #[serde(rename = "SplitValue")]
    pub SplitValue: rust_decimal::Decimal,
    #[serde(rename = "Aspects")]
    pub Aspects: Vec<Aspect>,
}
