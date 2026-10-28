use serde::{Deserialize, Serialize};

/// WebAPI model `AdvancedReservationNew`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedReservationNew {
    #[serde(rename = "DocumentId")]
    pub DocumentId: i32,
    #[serde(rename = "PositionId")]
    pub PositionId: i32,
    #[serde(rename = "DocumentType")]
    pub DocumentType: i16,
    #[serde(rename = "PositionType")]
    pub PositionType: i16,
    #[serde(rename = "Mode")]
    pub Mode: crate::web_api::interface::enums::enumReservationMode,
    #[serde(rename = "Warehouse")]
    pub Warehouse: ReservationNewWarehouse,
    #[serde(rename = "Contractor")]
    pub Contractor: ReservationNewContractor,
    #[serde(rename = "Product")]
    pub Product: ReservationNewProduct,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumDocumentReservationType,
    #[serde(rename = "ReservationDate")]
    pub ReservationDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Quantity")]
    pub Quantity: Option<rust_decimal::Decimal>,
    #[serde(rename = "ReserveToMax")]
    pub ReserveToMax: bool,
    #[serde(rename = "MustReserveAllIndicatedDeliveries")]
    pub MustReserveAllIndicatedDeliveries: bool,
    #[serde(rename = "Deliveries")]
    pub Deliveries: Vec<ReservationNewDelivery>,
}

/// WebAPI model `Reservation`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
    #[serde(rename = "Deliveries")]
    pub Deliveries: Vec<ReservationDelivery>,
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumReservationType,
    #[serde(rename = "Kind")]
    pub Kind: crate::web_api::interface::enums::enumReservationKind,
    #[serde(rename = "ContractorId")]
    pub ContractorId: Option<i32>,
    #[serde(rename = "ProductId")]
    pub ProductId: Option<i32>,
    #[serde(rename = "WarehouseId")]
    pub WarehouseId: Option<i32>,
    #[serde(rename = "ReservationDate")]
    pub ReservationDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "DocumentType")]
    pub DocumentType: i32,
    #[serde(rename = "PositionType")]
    pub PositionType: i32,
    #[serde(rename = "StandardDocumentType")]
    pub StandardDocumentType: Option<crate::web_api::interface::enums::enumReservationDocumentType>,
    #[serde(rename = "StandardPositionType")]
    pub StandardPositionType: Option<crate::web_api::interface::enums::enumReservationPositionType>,
    #[serde(rename = "DocumentId")]
    pub DocumentId: Option<i32>,
    #[serde(rename = "PositionId")]
    pub PositionId: Option<i32>,
}

/// WebAPI model `ReservationDelivery`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationDelivery {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Date")]
    pub Date: chrono::NaiveDateTime,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
    #[serde(rename = "RealizedQuantity")]
    pub RealizedQuantity: rust_decimal::Decimal,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
}

/// WebAPI model `ReservationListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationListElement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumReservationType,
    #[serde(rename = "Kind")]
    pub Kind: crate::web_api::interface::enums::enumReservationKind,
    #[serde(rename = "ContractorId")]
    pub ContractorId: Option<i32>,
    #[serde(rename = "ProductId")]
    pub ProductId: Option<i32>,
    #[serde(rename = "WarehouseId")]
    pub WarehouseId: Option<i32>,
    #[serde(rename = "ReservationDate")]
    pub ReservationDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "DocumentType")]
    pub DocumentType: i32,
    #[serde(rename = "PositionType")]
    pub PositionType: i32,
    #[serde(rename = "StandardDocumentType")]
    pub StandardDocumentType: Option<crate::web_api::interface::enums::enumReservationDocumentType>,
    #[serde(rename = "StandardPositionType")]
    pub StandardPositionType: Option<crate::web_api::interface::enums::enumReservationPositionType>,
    #[serde(rename = "DocumentId")]
    pub DocumentId: Option<i32>,
    #[serde(rename = "PositionId")]
    pub PositionId: Option<i32>,
}

/// WebAPI model `ReservationNew`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationNew {
    #[serde(rename = "Warehouse")]
    pub Warehouse: ReservationNewWarehouse,
    #[serde(rename = "Contractor")]
    pub Contractor: ReservationNewContractor,
    #[serde(rename = "Product")]
    pub Product: ReservationNewProduct,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumDocumentReservationType,
    #[serde(rename = "ReservationDate")]
    pub ReservationDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Quantity")]
    pub Quantity: Option<rust_decimal::Decimal>,
    #[serde(rename = "ReserveToMax")]
    pub ReserveToMax: bool,
    #[serde(rename = "MustReserveAllIndicatedDeliveries")]
    pub MustReserveAllIndicatedDeliveries: bool,
    #[serde(rename = "Deliveries")]
    pub Deliveries: Vec<ReservationNewDelivery>,
}

/// WebAPI model `ReservationNewContractor`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationNewContractor {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
}

/// WebAPI model `ReservationNewDelivery`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationNewDelivery {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
}

/// WebAPI model `ReservationNewProduct`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationNewProduct {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
}

/// WebAPI model `ReservationNewUnitInfo`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationNewUnitInfo {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
}

/// WebAPI model `ReservationNewWarehouse`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationNewWarehouse {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
}
