use serde::{Deserialize, Serialize};

/// WebAPI model `InventoryState`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryState {
    #[serde(rename = "ProductId")]
    pub ProductId: i32,
    #[serde(rename = "WarehouseId")]
    pub WarehouseId: i32,
    #[serde(rename = "AmountInStore")]
    pub AmountInStore: rust_decimal::Decimal,
    #[serde(rename = "AmountToSale")]
    pub AmountToSale: rust_decimal::Decimal,
}
