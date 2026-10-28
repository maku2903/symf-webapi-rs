use serde::{Deserialize, Serialize};

/// WebAPI model `RdfDocumentAttachment`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RdfDocumentAttachment {
    #[serde(rename = "FileName")]
    pub FileName: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Content")]
    pub Content: String,
}

/// WebAPI model `RdfDocumentLink`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RdfDocumentLink {
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "URL")]
    pub URL: String,
}
