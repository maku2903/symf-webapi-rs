use serde::{Deserialize, Serialize};

/// WebAPI model `Order`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
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
    #[serde(rename = "SplitPayment")]
    pub SplitPayment: bool,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "SaleDate")]
    pub SaleDate: Option<chrono::NaiveDateTime>,
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
    #[serde(rename = "BuyerId")]
    pub BuyerId: Option<i32>,
    #[serde(rename = "BuyerAddressId")]
    pub BuyerAddressId: Option<i32>,
    #[serde(rename = "RecipientId")]
    pub RecipientId: Option<i32>,
    #[serde(rename = "RecipientAddressId")]
    pub RecipientAddressId: Option<i32>,
    #[serde(rename = "ReceivedBy")]
    pub ReceivedBy: String,
    #[serde(rename = "DepartmentId")]
    pub DepartmentId: i32,
    #[serde(rename = "PriceKind")]
    pub PriceKind: crate::web_api::interface::enums::enumPriceKind,
    #[serde(rename = "SalePriceType")]
    pub SalePriceType: crate::web_api::interface::enums::enumSalePriceType,
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
    pub Positions: Vec<OrderPosition>,
    #[serde(rename = "BuyerAddress")]
    pub BuyerAddress: crate::web_api::interface::common::view_models::DocumentAddress,
    #[serde(rename = "RecipientAddress")]
    pub RecipientAddress: crate::web_api::interface::common::view_models::DocumentAddress,
    #[serde(rename = "TypeExternal")]
    pub TypeExternal: i32,
    #[serde(rename = "IdExternal")]
    pub IdExternal: Option<i32>,
    #[serde(rename = "Id2External")]
    pub Id2External: Option<String>,
    #[serde(rename = "DocumentExternalMetadata")]
    pub DocumentExternalMetadata: String,
}

/// WebAPI model `OrderBase`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBase {
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
    pub ReservationType: crate::web_api::interface::enums::enumDocumentReservationType,
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
    #[serde(rename = "SalePriceType")]
    pub SalePriceType: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "PriceKind")]
    pub PriceKind: crate::web_api::interface::enums::enumPriceKind,
    #[serde(rename = "SplitPayment")]
    pub SplitPayment: bool,
    #[serde(rename = "Buyer")]
    pub Buyer: OrderIssueContractor,
    #[serde(rename = "Recipient")]
    pub Recipient: OrderIssueContractor,
    #[serde(rename = "Catalog")]
    pub Catalog: OrderIssueCatalog,
    #[serde(rename = "Kind")]
    pub Kind: OrderIssueKind,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "ReceivedBy")]
    pub ReceivedBy: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "DocumentExternalMetadata")]
    pub DocumentExternalMetadata: String,
}

/// WebAPI model `OrderEdit`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderEdit {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Positions")]
    pub Positions: Vec<OrderEditPosition>,
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
    pub ReservationType: crate::web_api::interface::enums::enumDocumentReservationType,
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
    #[serde(rename = "SalePriceType")]
    pub SalePriceType: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "PriceKind")]
    pub PriceKind: crate::web_api::interface::enums::enumPriceKind,
    #[serde(rename = "SplitPayment")]
    pub SplitPayment: bool,
    #[serde(rename = "Buyer")]
    pub Buyer: OrderIssueContractor,
    #[serde(rename = "Recipient")]
    pub Recipient: OrderIssueContractor,
    #[serde(rename = "Catalog")]
    pub Catalog: OrderIssueCatalog,
    #[serde(rename = "Kind")]
    pub Kind: OrderIssueKind,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "ReceivedBy")]
    pub ReceivedBy: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "DocumentExternalMetadata")]
    pub DocumentExternalMetadata: String,
}

/// WebAPI model `OrderEditPosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderEditPosition {
    #[serde(rename = "PositionId")]
    pub PositionId: Option<i32>,
    #[serde(rename = "Delete")]
    pub Delete: bool,
    #[serde(rename = "VatRate")]
    pub VatRate: Option<crate::web_api::interface::common::view_models::VatRateBase>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "Elements")]
    pub Elements: Vec<OrderIssuePositionElement>,
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
    #[serde(rename = "Deliveries")]
    pub Deliveries: Vec<OrderIssueDelivery>,
}

/// WebAPI model `OrderFV`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderFV {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "SaleDate")]
    pub SaleDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "BuyerId")]
    pub BuyerId: i32,
    #[serde(rename = "RecipientId")]
    pub RecipientId: i32,
}

/// WebAPI model `OrderIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderIssue {
    #[serde(rename = "Positions")]
    pub Positions: Vec<OrderIssuePosition>,
    #[serde(rename = "TypeExternal")]
    pub TypeExternal: Option<i32>,
    #[serde(rename = "IdExternal")]
    pub IdExternal: Option<i32>,
    #[serde(rename = "Id2External")]
    pub Id2External: Option<String>,
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
    pub ReservationType: crate::web_api::interface::enums::enumDocumentReservationType,
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
    #[serde(rename = "SalePriceType")]
    pub SalePriceType: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "PriceKind")]
    pub PriceKind: crate::web_api::interface::enums::enumPriceKind,
    #[serde(rename = "SplitPayment")]
    pub SplitPayment: bool,
    #[serde(rename = "Buyer")]
    pub Buyer: OrderIssueContractor,
    #[serde(rename = "Recipient")]
    pub Recipient: OrderIssueContractor,
    #[serde(rename = "Catalog")]
    pub Catalog: OrderIssueCatalog,
    #[serde(rename = "Kind")]
    pub Kind: OrderIssueKind,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "ReceivedBy")]
    pub ReceivedBy: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "DocumentExternalMetadata")]
    pub DocumentExternalMetadata: String,
}

/// WebAPI model `OrderIssueCatalog`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderIssueCatalog {
    #[serde(rename = "FullPath")]
    pub FullPath: String,
    #[serde(rename = "AddIfNotExist")]
    pub AddIfNotExist: bool,
}

/// WebAPI model `OrderIssueContractor`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderIssueContractor {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "RecalculatePrices")]
    pub RecalculatePrices: bool,
    #[serde(rename = "Data")]
    pub Data: OrderIssueContractorData,
    #[serde(rename = "DeliveryAddressCode")]
    pub DeliveryAddressCode: String,
}

/// WebAPI model `OrderIssueContractorData`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderIssueContractorData {
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

/// WebAPI model `OrderIssueDelivery`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderIssueDelivery {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
}

/// WebAPI model `OrderIssueKind`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderIssueKind {
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "AddIfNotExist")]
    pub AddIfNotExist: bool,
}

/// WebAPI model `OrderIssuePosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderIssuePosition {
    #[serde(rename = "VatRate")]
    pub VatRate: Option<crate::web_api::interface::common::view_models::VatRateBase>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "Elements")]
    pub Elements: Vec<OrderIssuePositionElement>,
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
    #[serde(rename = "Deliveries")]
    pub Deliveries: Vec<OrderIssueDelivery>,
}

/// WebAPI model `OrderIssuePositionElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderIssuePositionElement {
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
    #[serde(rename = "Deliveries")]
    pub Deliveries: Vec<OrderIssueDelivery>,
}

/// WebAPI model `OrderListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderListElement {
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
    #[serde(rename = "SaleDate")]
    pub SaleDate: Option<chrono::NaiveDateTime>,
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
    #[serde(rename = "BuyerId")]
    pub BuyerId: Option<i32>,
    #[serde(rename = "RecipientId")]
    pub RecipientId: Option<i32>,
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
    #[serde(rename = "TypeExternal")]
    pub TypeExternal: i32,
    #[serde(rename = "IdExternal")]
    pub IdExternal: Option<i32>,
    #[serde(rename = "Id2External")]
    pub Id2External: Option<String>,
}

/// WebAPI model `OrderPosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderPosition {
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
    #[serde(rename = "SalePriceType")]
    pub SalePriceType: crate::web_api::interface::enums::enumSalePriceType,
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

/// WebAPI model `OrderPositionRelation`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderPositionRelation {
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
    #[serde(rename = "OrderId")]
    pub OrderId: i32,
    #[serde(rename = "OrderNumber")]
    pub OrderNumber: String,
    #[serde(rename = "RelatedFV")]
    pub RelatedFV: Vec<crate::web_api::interface::common::view_models::RelatedDocumentPosition>,
    #[serde(rename = "RelatedWZ")]
    pub RelatedWZ: Vec<crate::web_api::interface::common::view_models::RelatedDocumentPosition>,
}

/// WebAPI model `OrderStatus`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatus {
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

/// WebAPI model `OrderWZ`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderWZ {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "OperationDate")]
    pub OperationDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "RecipientId")]
    pub RecipientId: i32,
}
