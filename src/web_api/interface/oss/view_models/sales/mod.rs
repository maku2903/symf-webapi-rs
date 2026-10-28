use serde::{Deserialize, Serialize};

/// WebAPI model `OssDomesticSaleDocument`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OssDomesticSaleDocument {
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "SaleDate")]
    pub SaleDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "PaymentRegistry")]
    pub PaymentRegistry: crate::web_api::interface::common::view_models::PaymentRegistryBase,
    #[serde(rename = "PaymentFormId")]
    pub PaymentFormId: i32,
    #[serde(rename = "SalePriceType")]
    pub SalePriceType: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "PriceKind")]
    pub PriceKind: crate::web_api::interface::enums::enumPriceKind,
    #[serde(rename = "ReservationType")]
    pub ReservationType: crate::web_api::interface::enums::enumDocumentReservationType,
    #[serde(rename = "Buyer")]
    pub Buyer: crate::web_api::interface::oss::view_models::common::OssContractor,
    #[serde(rename = "Recipient")]
    pub Recipient: crate::web_api::interface::oss::view_models::common::OssContractor,
    #[serde(rename = "Catalog")]
    pub Catalog: crate::web_api::interface::oss::view_models::common::OssCatalog,
    #[serde(rename = "Kind")]
    pub Kind: crate::web_api::interface::oss::view_models::common::OssKind,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "ReceivedBy")]
    pub ReceivedBy: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "ShopOrderId")]
    pub ShopOrderId: String,
    #[serde(rename = "ShippingCountry")]
    pub ShippingCountry: String,
    #[serde(rename = "TypeExternal")]
    pub TypeExternal: i32,
    #[serde(rename = "IdExternal")]
    pub IdExternal: Option<i32>,
    #[serde(rename = "Id2External")]
    pub Id2External: Option<String>,
    #[serde(rename = "Positions")]
    pub Positions: Vec<OssSaleDocumentPosition>,
}

/// WebAPI model `OssSaleDocument`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OssSaleDocument {
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "Country")]
    pub Country: String,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "SaleDate")]
    pub SaleDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "PaymentRegistry")]
    pub PaymentRegistry: crate::web_api::interface::common::view_models::PaymentRegistryBase,
    #[serde(rename = "PaymentFormId")]
    pub PaymentFormId: i32,
    #[serde(rename = "SalePriceType")]
    pub SalePriceType: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "PriceKind")]
    pub PriceKind: crate::web_api::interface::enums::enumPriceKind,
    #[serde(rename = "ReservationType")]
    pub ReservationType: crate::web_api::interface::enums::enumDocumentReservationType,
    #[serde(rename = "Buyer")]
    pub Buyer: crate::web_api::interface::oss::view_models::common::OssContractor,
    #[serde(rename = "Recipient")]
    pub Recipient: crate::web_api::interface::oss::view_models::common::OssContractor,
    #[serde(rename = "Catalog")]
    pub Catalog: crate::web_api::interface::oss::view_models::common::OssCatalog,
    #[serde(rename = "Kind")]
    pub Kind: crate::web_api::interface::oss::view_models::common::OssKind,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "ReceivedBy")]
    pub ReceivedBy: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "ShopOrderId")]
    pub ShopOrderId: String,
    #[serde(rename = "ShippingCountry")]
    pub ShippingCountry: String,
    #[serde(rename = "TypeExternal")]
    pub TypeExternal: i32,
    #[serde(rename = "IdExternal")]
    pub IdExternal: Option<i32>,
    #[serde(rename = "Id2External")]
    pub Id2External: Option<String>,
    #[serde(rename = "Positions")]
    pub Positions: Vec<OssSaleDocumentPosition>,
}

/// WebAPI model `OssSaleDocumentPosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OssSaleDocumentPosition {
    #[serde(rename = "Elements")]
    pub Elements: Vec<OssSaleDocumentPositionElement>,
    #[serde(rename = "ErpProductId")]
    pub ErpProductId: Option<i32>,
    #[serde(rename = "ShopProductId")]
    pub ShopProductId: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
    #[serde(rename = "UnitOfMeasurement")]
    pub UnitOfMeasurement: String,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Deliveries")]
    pub Deliveries: Vec<crate::web_api::interface::oss::view_models::common::OssDelivery>,
    #[serde(rename = "OssService")]
    pub OssService: Option<bool>,
}

/// WebAPI model `OssSaleDocumentPositionElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OssSaleDocumentPositionElement {
    #[serde(rename = "ErpProductId")]
    pub ErpProductId: Option<i32>,
    #[serde(rename = "ShopProductId")]
    pub ShopProductId: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
    #[serde(rename = "UnitOfMeasurement")]
    pub UnitOfMeasurement: String,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Deliveries")]
    pub Deliveries: Vec<crate::web_api::interface::oss::view_models::common::OssDelivery>,
    #[serde(rename = "OssService")]
    pub OssService: Option<bool>,
}
