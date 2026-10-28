pub mod business;
pub mod custom;
pub mod purchase_invoice;

use serde::{Deserialize, Serialize};

/// WebAPI model `DocumentRecordAccountIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRecordAccountIssue {
    #[serde(rename = "Synthetic")]
    pub Synthetic: i32,
    #[serde(rename = "Analytic1")]
    pub Analytic1: i32,
    #[serde(rename = "Analytic2")]
    pub Analytic2: i32,
    #[serde(rename = "Analytic3")]
    pub Analytic3: i32,
    #[serde(rename = "Analytic4")]
    pub Analytic4: i32,
    #[serde(rename = "Analytic5")]
    pub Analytic5: i32,
}
