use serde::{Deserialize, Serialize};

/// WebAPI model `AccountChartElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountChartElement {
    #[serde(rename = "Childs")]
    pub Childs: Vec<AccountChartElement>,
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Level")]
    pub Level: i32,
    #[serde(rename = "Account")]
    pub Account: String,
    #[serde(rename = "Account_Full")]
    pub Account_Full: crate::web_api::interface::fkf::common::view_models::Account,
    #[serde(rename = "SubjectType")]
    pub SubjectType: crate::web_api::interface::enums::enumSubjectType,
    #[serde(rename = "SubjectCode")]
    pub SubjectCode: String,
    #[serde(rename = "Settlement")]
    pub Settlement: bool,
}

/// WebAPI model `AccountChartElementSimple`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountChartElementSimple {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Level")]
    pub Level: i32,
    #[serde(rename = "Account")]
    pub Account: String,
    #[serde(rename = "Account_Full")]
    pub Account_Full: crate::web_api::interface::fkf::common::view_models::Account,
    #[serde(rename = "SubjectType")]
    pub SubjectType: crate::web_api::interface::enums::enumSubjectType,
    #[serde(rename = "SubjectCode")]
    pub SubjectCode: String,
    #[serde(rename = "Settlement")]
    pub Settlement: bool,
}
