use serde::{Deserialize, Serialize};

/// WebAPI model `BusinessDictionary`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessDictionary {
    #[serde(rename = "Id")]
    pub Id: crate::web_api::interface::enums::fk_configurator::DictionaryType,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Elements")]
    pub Elements: Vec<BusinessDictionaryElement>,
}

/// WebAPI model `BusinessDictionaryElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessDictionaryElement {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Position")]
    pub Position: String,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::fk_configurator::DictionaryElementType,
    #[serde(rename = "DimensionValue")]
    pub DimensionValue: String,
}

/// WebAPI model `ContractorPosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractorPosition {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Position")]
    pub Position: i32,
}

/// WebAPI model `Dictionary`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dictionary {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "DefaultElementId")]
    pub DefaultElementId: Option<i32>,
    #[serde(rename = "SuggestedElementId")]
    pub SuggestedElementId: Option<i32>,
    #[serde(rename = "NonSetElementId")]
    pub NonSetElementId: Option<i32>,
    #[serde(rename = "Elements")]
    pub Elements: Vec<DictionaryElement>,
}

/// WebAPI model `DictionaryElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryElement {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Position")]
    pub Position: Option<i32>,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Attributes")]
    pub Attributes: Vec<DictionaryElementAttribute>,
}

/// WebAPI model `DictionaryElementAttribute`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryElementAttribute {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Inherited")]
    pub Inherited: bool,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Value")]
    pub Value: String,
    #[serde(rename = "Type")]
    pub Type: String,
}

/// WebAPI model `DictionaryListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryListElement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Active")]
    pub Active: Option<bool>,
}
