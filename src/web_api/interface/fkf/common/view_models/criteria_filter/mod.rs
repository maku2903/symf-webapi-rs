use serde::{Deserialize, Serialize};

/// WebAPI model `IntCriteriaFilter`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntCriteriaFilter {
    #[serde(rename = "Value_From")]
    pub Value_From: i32,
    #[serde(rename = "Value_To")]
    pub Value_To: i32,
    #[serde(rename = "Mode")]
    pub Mode: crate::web_api::interface::enums::enumFilterSqlCriteriaMode,
}

/// WebAPI model `StringCriteriaFilter`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringCriteriaFilter {
    #[serde(rename = "Value")]
    pub Value: String,
    #[serde(rename = "Mode")]
    pub Mode: crate::web_api::interface::enums::enumFilterSqlCriteriaMode,
}
