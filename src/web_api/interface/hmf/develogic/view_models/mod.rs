use serde::{Deserialize, Serialize};

/// WebAPI model `DevelogicAdvancePayment`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelogicAdvancePayment {
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "IsFinal")]
    pub IsFinal: bool,
}

/// WebAPI model `DevelogicContractInvoiceIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelogicContractInvoiceIssue {
    #[serde(rename = "TypeCode")]
    pub TypeCode: String,
    #[serde(rename = "Series")]
    pub Series: String,
    #[serde(rename = "CorrectionTypeCode")]
    pub CorrectionTypeCode: String,
    #[serde(rename = "CorrectionSeries")]
    pub CorrectionSeries: String,
    #[serde(rename = "NumberInSeries")]
    pub NumberInSeries: Option<i32>,
    #[serde(rename = "ReservationType")]
    pub ReservationType: Option<crate::web_api::interface::enums::enumDocumentReservationType>,
    #[serde(rename = "PaymentRegistry")]
    pub PaymentRegistry: crate::web_api::interface::common::view_models::PaymentRegistryBase,
    #[serde(rename = "PaymentFormId")]
    pub PaymentFormId: i32,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "SaleDate")]
    pub SaleDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "SalePriceType")]
    pub SalePriceType: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "SplitPayment")]
    pub SplitPayment: bool,
    #[serde(rename = "JPK_V7Attributes")]
    pub JPK_V7Attributes: Option<crate::web_api::interface::enums::enumJPK_V7DocumentAttribute>,
    #[serde(rename = "Catalog")]
    pub Catalog: crate::web_api::interface::sales::view_models::SaleDocumentIssueCatalog,
    #[serde(rename = "Kind")]
    pub Kind: crate::web_api::interface::sales::view_models::SaleDocumentIssueKind,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "ReceivedBy")]
    pub ReceivedBy: String,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "Positions")]
    pub Positions: Vec<DevelogicContractInvoiceIssuePosition>,
}

/// WebAPI model `DevelogicContractInvoiceIssuePosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelogicContractInvoiceIssuePosition {
    #[serde(rename = "ContractPositionId")]
    pub ContractPositionId: i32,
    #[serde(rename = "Quantity")]
    pub Quantity: i32,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
}

/// WebAPI model `DevelogicDocumentIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelogicDocumentIssue {
    #[serde(rename = "SubAccountNumber")]
    pub SubAccountNumber: String,
    #[serde(rename = "TypeCode")]
    pub TypeCode: String,
    #[serde(rename = "Series")]
    pub Series: String,
    #[serde(rename = "NumberInSeries")]
    pub NumberInSeries: Option<i32>,
    #[serde(rename = "Department")]
    pub Department: String,
    #[serde(rename = "Warehouse")]
    pub Warehouse: String,
    #[serde(rename = "ReservationType")]
    pub ReservationType: Option<crate::web_api::interface::enums::enumDocumentReservationType>,
    #[serde(rename = "PaymentRegistry")]
    pub PaymentRegistry: crate::web_api::interface::common::view_models::PaymentRegistryBase,
    #[serde(rename = "PaymentFormId")]
    pub PaymentFormId: i32,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "SaleDate")]
    pub SaleDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "CurrencyRateCIT")]
    pub CurrencyRateCIT: rust_decimal::Decimal,
    #[serde(rename = "SalePriceType")]
    pub SalePriceType: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "PriceKind")]
    pub PriceKind: crate::web_api::interface::enums::enumPriceKind,
    #[serde(rename = "SplitPayment")]
    pub SplitPayment: bool,
    #[serde(rename = "JPK_V7Attributes")]
    pub JPK_V7Attributes: Option<crate::web_api::interface::enums::enumJPK_V7DocumentAttribute>,
    #[serde(rename = "eArchiveId")]
    pub eArchiveId: Option<String>,
    #[serde(rename = "NumberKSeF")]
    pub NumberKSeF: Option<String>,
    #[serde(rename = "IssueDateKSeF")]
    pub IssueDateKSeF: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Buyer")]
    pub Buyer: crate::web_api::interface::sales::view_models::SaleDocumentIssueContractor,
    #[serde(rename = "Recipient")]
    pub Recipient: crate::web_api::interface::sales::view_models::SaleDocumentIssueContractor,
    #[serde(rename = "Positions")]
    pub Positions: Vec<crate::web_api::interface::sales::view_models::SaleDocumentIssuePosition>,
    #[serde(rename = "Catalog")]
    pub Catalog: crate::web_api::interface::sales::view_models::SaleDocumentIssueCatalog,
    #[serde(rename = "Kind")]
    pub Kind: crate::web_api::interface::sales::view_models::SaleDocumentIssueKind,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "ReceivedBy")]
    pub ReceivedBy: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "OssProcedure")]
    pub OssProcedure: Option<bool>,
    #[serde(rename = "OssShippingCountry")]
    pub OssShippingCountry: String,
    #[serde(rename = "OssDeliveryCountry")]
    pub OssDeliveryCountry: String,
    #[serde(rename = "TypeExternal")]
    pub TypeExternal: Option<i32>,
    #[serde(rename = "IdExternal")]
    pub IdExternal: Option<i32>,
    #[serde(rename = "Id2External")]
    pub Id2External: Option<String>,
    #[serde(rename = "DocumentExternalMetadata")]
    pub DocumentExternalMetadata: String,
    #[serde(rename = "IsSmeProcedure")]
    pub IsSmeProcedure: bool,
}
