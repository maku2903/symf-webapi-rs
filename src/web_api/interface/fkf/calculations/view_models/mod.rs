use serde::{Deserialize, Serialize};

/// WebAPI model `BalanceAndTurnoverFilterOptions`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceAndTurnoverFilterOptions {
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
    #[serde(rename = "FromCalculationDate")]
    pub FromCalculationDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ToCalculationDate")]
    pub ToCalculationDate: Option<chrono::NaiveDateTime>,
}

/// WebAPI model `BalanceAndTurnoverListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceAndTurnoverListElement {
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::fk_configurator::AccountBalanceTurnoverType,
    #[serde(rename = "YearId")]
    pub YearId: i32,
    #[serde(rename = "AccountFull")]
    pub AccountFull: String,
    #[serde(rename = "Synthetic")]
    pub Synthetic: i32,
    #[serde(rename = "Analytic1")]
    pub Analytic1: Option<i32>,
    #[serde(rename = "Analytic2")]
    pub Analytic2: Option<i32>,
    #[serde(rename = "Analytic3")]
    pub Analytic3: Option<i32>,
    #[serde(rename = "Analytic4")]
    pub Analytic4: Option<i32>,
    #[serde(rename = "Analytic5")]
    pub Analytic5: Option<i32>,
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
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "BO")]
    pub BO: rust_decimal::Decimal,
    #[serde(rename = "OR")]
    pub OR: rust_decimal::Decimal,
    #[serde(rename = "Value1")]
    pub Value1: rust_decimal::Decimal,
    #[serde(rename = "Value2")]
    pub Value2: rust_decimal::Decimal,
    #[serde(rename = "Value3")]
    pub Value3: rust_decimal::Decimal,
    #[serde(rename = "Value4")]
    pub Value4: rust_decimal::Decimal,
    #[serde(rename = "Value5")]
    pub Value5: rust_decimal::Decimal,
    #[serde(rename = "Value6")]
    pub Value6: rust_decimal::Decimal,
    #[serde(rename = "Value7")]
    pub Value7: rust_decimal::Decimal,
    #[serde(rename = "Value8")]
    pub Value8: rust_decimal::Decimal,
    #[serde(rename = "Value9")]
    pub Value9: rust_decimal::Decimal,
    #[serde(rename = "Value10")]
    pub Value10: rust_decimal::Decimal,
    #[serde(rename = "Value11")]
    pub Value11: rust_decimal::Decimal,
    #[serde(rename = "Value12")]
    pub Value12: rust_decimal::Decimal,
    #[serde(rename = "Value13")]
    pub Value13: rust_decimal::Decimal,
    #[serde(rename = "Value14")]
    pub Value14: rust_decimal::Decimal,
    #[serde(rename = "Value15")]
    pub Value15: rust_decimal::Decimal,
    #[serde(rename = "Value16")]
    pub Value16: rust_decimal::Decimal,
    #[serde(rename = "Value17")]
    pub Value17: rust_decimal::Decimal,
    #[serde(rename = "Value18")]
    pub Value18: rust_decimal::Decimal,
    #[serde(rename = "Value19")]
    pub Value19: rust_decimal::Decimal,
    #[serde(rename = "Value20")]
    pub Value20: rust_decimal::Decimal,
    #[serde(rename = "Value21")]
    pub Value21: rust_decimal::Decimal,
    #[serde(rename = "Value22")]
    pub Value22: rust_decimal::Decimal,
    #[serde(rename = "Value23")]
    pub Value23: rust_decimal::Decimal,
    #[serde(rename = "CalculationDate")]
    pub CalculationDate: chrono::NaiveDateTime,
}

/// WebAPI model `CalculationFilterAccountOptions`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationFilterAccountOptions {
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

/// WebAPI model `CalculationFilterOptions`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationFilterOptions {
    #[serde(rename = "ForDay")]
    pub ForDay: Option<chrono::NaiveDateTime>,
    #[serde(rename = "DateFrom")]
    pub DateFrom: Option<chrono::NaiveDateTime>,
    #[serde(rename = "DateTo")]
    pub DateTo: Option<chrono::NaiveDateTime>,
    #[serde(rename = "WithoutDate")]
    pub WithoutDate: Option<bool>,
    #[serde(rename = "ValueFrom")]
    pub ValueFrom: Option<rust_decimal::Decimal>,
    #[serde(rename = "ValueTo")]
    pub ValueTo: Option<rust_decimal::Decimal>,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "SubjectPosition")]
    pub SubjectPosition: Option<i32>,
    #[serde(rename = "SubjectType")]
    pub SubjectType: Option<crate::web_api::interface::enums::enumSubjectType>,
    #[serde(rename = "DocumentName")]
    pub DocumentName: String,
    #[serde(rename = "Type")]
    pub Type: Option<crate::web_api::interface::enums::enumCalculationType>,
    #[serde(rename = "State")]
    pub State: Option<crate::web_api::interface::enums::enumCalculationState>,
    #[serde(rename = "Account")]
    pub Account: CalculationFilterAccountOptions,
    #[serde(rename = "GroupBy")]
    pub GroupBy: Vec<String>,
    #[serde(rename = "OrderBy")]
    pub OrderBy: Vec<String>,
}

/// WebAPI model `CalculationListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationListElement {
    #[serde(rename = "DocumentId")]
    pub DocumentId: i32,
    #[serde(rename = "TransactionId")]
    pub TransactionId: i32,
    #[serde(rename = "SettlementId")]
    pub SettlementId: i32,
    #[serde(rename = "YearId")]
    pub YearId: i32,
    #[serde(rename = "Buffer")]
    pub Buffer: bool,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: chrono::NaiveDateTime,
    #[serde(rename = "DocumentDate")]
    pub DocumentDate: chrono::NaiveDateTime,
    #[serde(rename = "PeriodDate")]
    pub PeriodDate: chrono::NaiveDateTime,
    #[serde(rename = "DaysToMaturity")]
    pub DaysToMaturity: i32,
    #[serde(rename = "State")]
    pub State: crate::web_api::interface::enums::enumCalculationState,
    #[serde(rename = "Side")]
    pub Side: crate::web_api::interface::enums::enumSideType,
    #[serde(rename = "RecordNumber")]
    pub RecordNumber: String,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "SubjectType")]
    pub SubjectType: crate::web_api::interface::enums::enumSubjectType,
    #[serde(rename = "SubjectPosition")]
    pub SubjectPosition: i32,
    #[serde(rename = "Account")]
    pub Account: String,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "ValuePLN")]
    pub ValuePLN: rust_decimal::Decimal,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
}

/// WebAPI model `CalculationListElementGrouped`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationListElementGrouped {
    #[serde(rename = "SubjectType")]
    pub SubjectType: crate::web_api::interface::enums::enumSubjectType,
    #[serde(rename = "SubjectPosition")]
    pub SubjectPosition: i32,
    #[serde(rename = "ValuePLN")]
    pub ValuePLN: rust_decimal::Decimal,
}
