use serde::{Deserialize, Serialize};

/// WebAPI model `PrintData`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintData {
    #[serde(rename = "Disclaimer")]
    pub Disclaimer: String,
    #[serde(rename = "Footer")]
    pub Footer: String,
    #[serde(rename = "IssuedBy")]
    pub IssuedBy: String,
    #[serde(rename = "ReceivedBy")]
    pub ReceivedBy: String,
    #[serde(rename = "ReportText")]
    pub ReportText: crate::web_api::interface::enums::enumRepXReportText,
    #[serde(rename = "ReportParameters")]
    pub ReportParameters: std::collections::HashMap<String, serde_json::Value>,
}

/// WebAPI model `ReportDefinition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDefinition {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "IsActive")]
    pub IsActive: bool,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Location")]
    pub Location: crate::web_api::interface::enums::enumRepXPrintLocation,
}
