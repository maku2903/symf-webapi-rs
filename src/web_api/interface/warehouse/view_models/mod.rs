use serde::{Deserialize, Serialize};

/// WebAPI model `DeliveryListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryListElement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "QuantityForSale")]
    pub QuantityForSale: rust_decimal::Decimal,
    #[serde(rename = "QuantityInWarehouse")]
    pub QuantityInWarehouse: rust_decimal::Decimal,
    #[serde(rename = "DeliveryDate")]
    pub DeliveryDate: chrono::NaiveDateTime,
    #[serde(rename = "WarehouseId")]
    pub WarehouseId: i32,
    #[serde(rename = "ProductId")]
    pub ProductId: i32,
    #[serde(rename = "ContractorId")]
    pub ContractorId: Option<i32>,
    #[serde(rename = "TypeExternal")]
    pub TypeExternal: i32,
    #[serde(rename = "IdExternal")]
    pub IdExternal: Option<i32>,
    #[serde(rename = "Id2External")]
    pub Id2External: Option<String>,
}

/// WebAPI model `WarehouseDocument`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocument {
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
    #[serde(rename = "OperationDate")]
    pub OperationDate: Option<chrono::NaiveDateTime>,
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
    #[serde(rename = "ContractorId")]
    pub ContractorId: Option<i32>,
    #[serde(rename = "ContractorAddressId")]
    pub ContractorAddressId: Option<i32>,
    #[serde(rename = "ReceivingWarehouseId")]
    pub ReceivingWarehouseId: Option<i32>,
    #[serde(rename = "WarehouseId")]
    pub WarehouseId: i32,
    #[serde(rename = "NetValue")]
    pub NetValue: rust_decimal::Decimal,
    #[serde(rename = "DocumentValue")]
    pub DocumentValue: rust_decimal::Decimal,
    #[serde(rename = "IncomeValue")]
    pub IncomeValue: rust_decimal::Decimal,
    #[serde(rename = "OutcomeValue")]
    pub OutcomeValue: rust_decimal::Decimal,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "CatalogId")]
    pub CatalogId: i32,
    #[serde(rename = "KindId")]
    pub KindId: i32,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "TypeExternal")]
    pub TypeExternal: i32,
    #[serde(rename = "IdExternal")]
    pub IdExternal: Option<i32>,
    #[serde(rename = "Id2External")]
    pub Id2External: Option<String>,
    #[serde(rename = "Positions")]
    pub Positions: Vec<WarehouseDocumentPosition>,
    #[serde(rename = "ContractorAddress")]
    pub ContractorAddress: crate::web_api::interface::common::view_models::DocumentAddress,
}

/// WebAPI model `WarehouseDocumentBase`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocumentBase {
    #[serde(rename = "TypeCode")]
    pub TypeCode: String,
    #[serde(rename = "Series")]
    pub Series: String,
    #[serde(rename = "NumberInSeries")]
    pub NumberInSeries: Option<i32>,
    #[serde(rename = "Warehouse")]
    pub Warehouse: String,
    #[serde(rename = "ReservationType")]
    pub ReservationType: Option<crate::web_api::interface::enums::enumDocumentReservationType>,
    #[serde(rename = "WarehouseRegistry")]
    pub WarehouseRegistry: String,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "OperationDate")]
    pub OperationDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Contractor")]
    pub Contractor: WarehouseDocumentIssueContractor,
    #[serde(rename = "ReceivingWarehouse")]
    pub ReceivingWarehouse: WarehouseDocumentIssueReceivingWarehouse,
    #[serde(rename = "Catalog")]
    pub Catalog: WarehouseDocumentIssueCatalog,
    #[serde(rename = "Kind")]
    pub Kind: WarehouseDocumentssueKind,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "ReceivedBy")]
    pub ReceivedBy: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "TypeExternal")]
    pub TypeExternal: Option<i32>,
    #[serde(rename = "IdExternal")]
    pub IdExternal: Option<i32>,
    #[serde(rename = "Id2External")]
    pub Id2External: Option<String>,
}

/// WebAPI model `WarehouseDocumentEdit`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocumentEdit {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Positions")]
    pub Positions: Vec<WarehouseDocumentEditPosition>,
    #[serde(rename = "TypeCode")]
    pub TypeCode: String,
    #[serde(rename = "Series")]
    pub Series: String,
    #[serde(rename = "NumberInSeries")]
    pub NumberInSeries: Option<i32>,
    #[serde(rename = "Warehouse")]
    pub Warehouse: String,
    #[serde(rename = "ReservationType")]
    pub ReservationType: Option<crate::web_api::interface::enums::enumDocumentReservationType>,
    #[serde(rename = "WarehouseRegistry")]
    pub WarehouseRegistry: String,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "OperationDate")]
    pub OperationDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Contractor")]
    pub Contractor: WarehouseDocumentIssueContractor,
    #[serde(rename = "ReceivingWarehouse")]
    pub ReceivingWarehouse: WarehouseDocumentIssueReceivingWarehouse,
    #[serde(rename = "Catalog")]
    pub Catalog: WarehouseDocumentIssueCatalog,
    #[serde(rename = "Kind")]
    pub Kind: WarehouseDocumentssueKind,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "ReceivedBy")]
    pub ReceivedBy: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "TypeExternal")]
    pub TypeExternal: Option<i32>,
    #[serde(rename = "IdExternal")]
    pub IdExternal: Option<i32>,
    #[serde(rename = "Id2External")]
    pub Id2External: Option<String>,
}

/// WebAPI model `WarehouseDocumentEditPosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocumentEditPosition {
    #[serde(rename = "PositionId")]
    pub PositionId: Option<i32>,
    #[serde(rename = "Delete")]
    pub Delete: bool,
    #[serde(rename = "Elements")]
    pub Elements: Vec<WarehouseDocumentIssuePositionElement>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
    #[serde(rename = "UnitOfMeasurement")]
    pub UnitOfMeasurement: String,
    #[serde(rename = "Value")]
    pub Value: Option<rust_decimal::Decimal>,
    #[serde(rename = "Deliveries")]
    pub Deliveries: Vec<WarehouseDocumentIssueDelivery>,
}

/// WebAPI model `WarehouseDocumentFV`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocumentFV {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "OperationDate")]
    pub OperationDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Contractor1Id")]
    pub Contractor1Id: i32,
    #[serde(rename = "Contractor2Id")]
    pub Contractor2Id: i32,
}

/// WebAPI model `WarehouseDocumentIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocumentIssue {
    #[serde(rename = "Positions")]
    pub Positions: Vec<WarehouseDocumentIssuePosition>,
    #[serde(rename = "TypeCode")]
    pub TypeCode: String,
    #[serde(rename = "Series")]
    pub Series: String,
    #[serde(rename = "NumberInSeries")]
    pub NumberInSeries: Option<i32>,
    #[serde(rename = "Warehouse")]
    pub Warehouse: String,
    #[serde(rename = "ReservationType")]
    pub ReservationType: Option<crate::web_api::interface::enums::enumDocumentReservationType>,
    #[serde(rename = "WarehouseRegistry")]
    pub WarehouseRegistry: String,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "OperationDate")]
    pub OperationDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Contractor")]
    pub Contractor: WarehouseDocumentIssueContractor,
    #[serde(rename = "ReceivingWarehouse")]
    pub ReceivingWarehouse: WarehouseDocumentIssueReceivingWarehouse,
    #[serde(rename = "Catalog")]
    pub Catalog: WarehouseDocumentIssueCatalog,
    #[serde(rename = "Kind")]
    pub Kind: WarehouseDocumentssueKind,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "ReceivedBy")]
    pub ReceivedBy: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "TypeExternal")]
    pub TypeExternal: Option<i32>,
    #[serde(rename = "IdExternal")]
    pub IdExternal: Option<i32>,
    #[serde(rename = "Id2External")]
    pub Id2External: Option<String>,
}

/// WebAPI model `WarehouseDocumentIssueCatalog`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocumentIssueCatalog {
    #[serde(rename = "FullPath")]
    pub FullPath: String,
    #[serde(rename = "AddIfNotExist")]
    pub AddIfNotExist: bool,
}

/// WebAPI model `WarehouseDocumentIssueContractor`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocumentIssueContractor {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Data")]
    pub Data: WarehouseDocumentIssueContractorData,
    #[serde(rename = "DeliveryAddressCode")]
    pub DeliveryAddressCode: String,
}

/// WebAPI model `WarehouseDocumentIssueContractorData`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocumentIssueContractorData {
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

/// WebAPI model `WarehouseDocumentIssueDelivery`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocumentIssueDelivery {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
}

/// WebAPI model `WarehouseDocumentIssuePosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocumentIssuePosition {
    #[serde(rename = "Elements")]
    pub Elements: Vec<WarehouseDocumentIssuePositionElement>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
    #[serde(rename = "UnitOfMeasurement")]
    pub UnitOfMeasurement: String,
    #[serde(rename = "Value")]
    pub Value: Option<rust_decimal::Decimal>,
    #[serde(rename = "Deliveries")]
    pub Deliveries: Vec<WarehouseDocumentIssueDelivery>,
}

/// WebAPI model `WarehouseDocumentIssuePositionElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocumentIssuePositionElement {
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
    #[serde(rename = "UnitOfMeasurement")]
    pub UnitOfMeasurement: String,
    #[serde(rename = "Value")]
    pub Value: Option<rust_decimal::Decimal>,
    #[serde(rename = "Deliveries")]
    pub Deliveries: Vec<WarehouseDocumentIssueDelivery>,
}

/// WebAPI model `WarehouseDocumentIssueReceivingWarehouse`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocumentIssueReceivingWarehouse {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
}

/// WebAPI model `WarehouseDocumentListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocumentListElement {
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
    #[serde(rename = "OperationDate")]
    pub OperationDate: Option<chrono::NaiveDateTime>,
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
    #[serde(rename = "ContractorId")]
    pub ContractorId: Option<i32>,
    #[serde(rename = "ReceivingWarehouseId")]
    pub ReceivingWarehouseId: Option<i32>,
    #[serde(rename = "WarehouseId")]
    pub WarehouseId: i32,
    #[serde(rename = "NetValue")]
    pub NetValue: rust_decimal::Decimal,
    #[serde(rename = "DocumentValue")]
    pub DocumentValue: rust_decimal::Decimal,
    #[serde(rename = "IncomeValue")]
    pub IncomeValue: rust_decimal::Decimal,
    #[serde(rename = "OutcomeValue")]
    pub OutcomeValue: rust_decimal::Decimal,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "TypeExternal")]
    pub TypeExternal: i32,
    #[serde(rename = "IdExternal")]
    pub IdExternal: Option<i32>,
    #[serde(rename = "Id2External")]
    pub Id2External: Option<String>,
}

/// WebAPI model `WarehouseDocumentPosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocumentPosition {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "No")]
    pub No: i32,
    #[serde(rename = "SetHeaderId")]
    pub SetHeaderId: Option<i32>,
    #[serde(rename = "ProductId")]
    pub ProductId: i32,
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
    #[serde(rename = "PriceValue")]
    pub PriceValue: rust_decimal::Decimal,
    #[serde(rename = "NetValue")]
    pub NetValue: rust_decimal::Decimal,
    #[serde(rename = "Reservations")]
    pub Reservations: Option<Vec<WarehouseDocumentPositionReservation>>,
    #[serde(rename = "Deliveries")]
    pub Deliveries: Option<Vec<WarehouseDocumentPositionDelivery>>,
}

/// WebAPI model `WarehouseDocumentPositionDelivery`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocumentPositionDelivery {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
    #[serde(rename = "Date")]
    pub Date: chrono::NaiveDateTime,
    #[serde(rename = "ReservationId")]
    pub ReservationId: Option<i32>,
}

/// WebAPI model `WarehouseDocumentPositionReservation`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocumentPositionReservation {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
}

/// WebAPI model `WarehouseDocumentStatus`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocumentStatus {
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

/// WebAPI model `WarehouseDocumentssueKind`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDocumentssueKind {
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "AddIfNotExist")]
    pub AddIfNotExist: bool,
}

/// WebAPI model `WarehouseRegistry`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseRegistry {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
}
