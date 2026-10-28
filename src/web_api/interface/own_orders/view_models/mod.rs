use serde::{Deserialize, Serialize};

/// WebAPI model `OwnOrder`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnOrder {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Canceled")]
    pub Canceled: crate::web_api::interface::enums::enumCancelationType,
    #[serde(rename = "Buffer")]
    pub Buffer: bool,
    #[serde(rename = "Settled")]
    pub Settled: bool,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ReceiveDate")]
    pub ReceiveDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "TypeCode")]
    pub TypeCode: String,
    #[serde(rename = "Series")]
    pub Series: String,
    #[serde(rename = "NumberInSeries")]
    pub NumberInSeries: i32,
    #[serde(rename = "IssuerId")]
    pub IssuerId: i32,
    #[serde(rename = "SellerId")]
    pub SellerId: Option<i32>,
    #[serde(rename = "SellerAddressId")]
    pub SellerAddressId: Option<i32>,
    #[serde(rename = "DelivererId")]
    pub DelivererId: Option<i32>,
    #[serde(rename = "DelivererAddressId")]
    pub DelivererAddressId: Option<i32>,
    #[serde(rename = "DepartmentId")]
    pub DepartmentId: i32,
    #[serde(rename = "PriceKind")]
    pub PriceKind: crate::web_api::interface::enums::enumPriceKind,
    #[serde(rename = "NetValuePLN")]
    pub NetValuePLN: rust_decimal::Decimal,
    #[serde(rename = "VatValuePLN")]
    pub VatValuePLN: rust_decimal::Decimal,
    #[serde(rename = "NetValue")]
    pub NetValue: rust_decimal::Decimal,
    #[serde(rename = "GrossValue")]
    pub GrossValue: rust_decimal::Decimal,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "PaymentRegistryId")]
    pub PaymentRegistryId: i32,
    #[serde(rename = "PaymentFormId")]
    pub PaymentFormId: i32,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "CatalogId")]
    pub CatalogId: i32,
    #[serde(rename = "KindId")]
    pub KindId: i32,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "Positions")]
    pub Positions: Vec<OwnOrderPosition>,
    #[serde(rename = "SellerAddress")]
    pub SellerAddress: crate::web_api::interface::common::view_models::DocumentAddress,
    #[serde(rename = "DelivererAddress")]
    pub DelivererAddress: crate::web_api::interface::common::view_models::DocumentAddress,
}

/// WebAPI model `OwnOrderFV`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnOrderFV {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "BuyDate")]
    pub BuyDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "SellerId")]
    pub SellerId: i32,
    #[serde(rename = "DelivererId")]
    pub DelivererId: i32,
}

/// WebAPI model `OwnOrderIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnOrderIssue {
    #[serde(rename = "TypeCode")]
    pub TypeCode: String,
    #[serde(rename = "Series")]
    pub Series: String,
    #[serde(rename = "NumberInSeries")]
    pub NumberInSeries: Option<i32>,
    #[serde(rename = "Department")]
    pub Department: String,
    #[serde(rename = "PaymentRegistry")]
    pub PaymentRegistry: crate::web_api::interface::common::view_models::PaymentRegistryBase,
    #[serde(rename = "PaymentFormId")]
    pub PaymentFormId: i32,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ReceiveDate")]
    pub ReceiveDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "PriceKind")]
    pub PriceKind: crate::web_api::interface::enums::enumPriceKind,
    #[serde(rename = "Seller")]
    pub Seller: OwnOrderIssueContractor,
    #[serde(rename = "Deliverer")]
    pub Deliverer: OwnOrderIssueContractor,
    #[serde(rename = "Positions")]
    pub Positions: Vec<OwnOrderIssuePosition>,
    #[serde(rename = "Catalog")]
    pub Catalog: OwnOrderIssueCatalog,
    #[serde(rename = "Kind")]
    pub Kind: OwnOrderIssueKind,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
}

/// WebAPI model `OwnOrderIssueCatalog`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnOrderIssueCatalog {
    #[serde(rename = "FullPath")]
    pub FullPath: String,
    #[serde(rename = "AddIfNotExist")]
    pub AddIfNotExist: bool,
}

/// WebAPI model `OwnOrderIssueContractor`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnOrderIssueContractor {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "RecalculatePrices")]
    pub RecalculatePrices: bool,
    #[serde(rename = "Data")]
    pub Data: OwnOrderIssueContractorData,
    #[serde(rename = "DeliveryAddressCode")]
    pub DeliveryAddressCode: String,
}

/// WebAPI model `OwnOrderIssueContractorData`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnOrderIssueContractorData {
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "NIP")]
    pub NIP: Option<String>,
    #[serde(rename = "Country")]
    pub Country: String,
    #[serde(rename = "City")]
    pub City: Option<String>,
    #[serde(rename = "Street")]
    pub Street: Option<String>,
    #[serde(rename = "HouseNo")]
    pub HouseNo: Option<String>,
    #[serde(rename = "ApartmentNo")]
    pub ApartmentNo: Option<String>,
    #[serde(rename = "PostCode")]
    pub PostCode: Option<String>,
}

/// WebAPI model `OwnOrderIssueKind`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnOrderIssueKind {
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "AddIfNotExist")]
    pub AddIfNotExist: bool,
}

/// WebAPI model `OwnOrderIssuePosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnOrderIssuePosition {
    #[serde(rename = "VatRate")]
    pub VatRate: Option<crate::web_api::interface::common::view_models::VatRateBase>,
    #[serde(rename = "Elements")]
    pub Elements: Vec<OwnOrderIssuePositionElement>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
    #[serde(rename = "UnitOfMeasurement")]
    pub UnitOfMeasurement: String,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
}

/// WebAPI model `OwnOrderIssuePositionElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnOrderIssuePositionElement {
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
    #[serde(rename = "UnitOfMeasurement")]
    pub UnitOfMeasurement: String,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
}

/// WebAPI model `OwnOrderListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnOrderListElement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Canceled")]
    pub Canceled: crate::web_api::interface::enums::enumCancelationType,
    #[serde(rename = "Buffer")]
    pub Buffer: bool,
    #[serde(rename = "Settled")]
    pub Settled: bool,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ReceiveDate")]
    pub ReceiveDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "TypeCode")]
    pub TypeCode: String,
    #[serde(rename = "Series")]
    pub Series: String,
    #[serde(rename = "NumberInSeries")]
    pub NumberInSeries: i32,
    #[serde(rename = "IssuerId")]
    pub IssuerId: i32,
    #[serde(rename = "SellerId")]
    pub SellerId: Option<i32>,
    #[serde(rename = "DelivererId")]
    pub DelivererId: Option<i32>,
    #[serde(rename = "DepartmentId")]
    pub DepartmentId: i32,
    #[serde(rename = "NetValuePLN")]
    pub NetValuePLN: rust_decimal::Decimal,
    #[serde(rename = "VatValuePLN")]
    pub VatValuePLN: rust_decimal::Decimal,
    #[serde(rename = "NetValue")]
    pub NetValue: rust_decimal::Decimal,
    #[serde(rename = "GrossValue")]
    pub GrossValue: rust_decimal::Decimal,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "Description")]
    pub Description: String,
}

/// WebAPI model `OwnOrderPZ`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnOrderPZ {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "OperationDate")]
    pub OperationDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "DelivererId")]
    pub DelivererId: i32,
}

/// WebAPI model `OwnOrderPosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnOrderPosition {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "No")]
    pub No: i32,
    #[serde(rename = "SetHeaderId")]
    pub SetHeaderId: Option<i32>,
    #[serde(rename = "ProductId")]
    pub ProductId: Option<i32>,
    #[serde(rename = "ProductCode")]
    pub ProductCode: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
    #[serde(rename = "WrittenQuantity")]
    pub WrittenQuantity: rust_decimal::Decimal,
    #[serde(rename = "EnteredQuantity")]
    pub EnteredQuantity: rust_decimal::Decimal,
    #[serde(rename = "UnitOfMeasurement")]
    pub UnitOfMeasurement: String,
    #[serde(rename = "WrittenUnitOfMeasurement")]
    pub WrittenUnitOfMeasurement: String,
    #[serde(rename = "EnteredUnitOfMeasurement")]
    pub EnteredUnitOfMeasurement: String,
    #[serde(rename = "VatRate")]
    pub VatRate: Option<crate::web_api::interface::common::view_models::VatRateBase>,
    #[serde(rename = "PriceKind")]
    pub PriceKind: crate::web_api::interface::enums::enumPriceKind,
    #[serde(rename = "PriceValuePLN")]
    pub PriceValuePLN: rust_decimal::Decimal,
    #[serde(rename = "PriceValue")]
    pub PriceValue: rust_decimal::Decimal,
    #[serde(rename = "NetValuePLN")]
    pub NetValuePLN: rust_decimal::Decimal,
    #[serde(rename = "NetValue")]
    pub NetValue: rust_decimal::Decimal,
    #[serde(rename = "VatValuePLN")]
    pub VatValuePLN: rust_decimal::Decimal,
    #[serde(rename = "GrossValue")]
    pub GrossValue: rust_decimal::Decimal,
}

/// WebAPI model `OwnOrderPositionRelation`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnOrderPositionRelation {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "No")]
    pub No: i32,
    #[serde(rename = "ProductId")]
    pub ProductId: Option<i32>,
    #[serde(rename = "ProductCode")]
    pub ProductCode: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
    #[serde(rename = "NetValuePLN")]
    pub NetValuePLN: rust_decimal::Decimal,
    #[serde(rename = "OwnOrderId")]
    pub OwnOrderId: i32,
    #[serde(rename = "OwnOrderNumber")]
    pub OwnOrderNumber: String,
    #[serde(rename = "RelatedFV")]
    pub RelatedFV: Vec<crate::web_api::interface::common::view_models::RelatedDocumentPosition>,
    #[serde(rename = "RelatedPZ")]
    pub RelatedPZ: Vec<crate::web_api::interface::common::view_models::RelatedDocumentPosition>,
}

/// WebAPI model `OwnOrderStatus`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnOrderStatus {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "Buffer")]
    pub Buffer: bool,
    #[serde(rename = "PaymentSettled")]
    pub PaymentSettled: i32,
    #[serde(rename = "WarehouseSettled")]
    pub WarehouseSettled: i32,
    #[serde(rename = "RDFStatus")]
    pub RDFStatus: crate::web_api::interface::enums::enumRDFStatus,
    #[serde(rename = "ManualSettled")]
    pub ManualSettled: crate::web_api::interface::enums::enumManualSettledState,
    #[serde(rename = "DocumentStatus")]
    pub DocumentStatus: crate::web_api::interface::enums::enumDocumentStatus,
    #[serde(rename = "DocumentStatusText")]
    pub DocumentStatusText: String,
}
