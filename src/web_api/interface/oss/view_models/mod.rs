pub mod common;
pub mod orders;
pub mod sales;

use serde::{Deserialize, Serialize};

/// WebAPI model `CountryItem`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryItem {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Products")]
    pub Products: Vec<ProductItem>,
}

/// WebAPI model `CountryProductList`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryProductList {
    #[serde(rename = "Countries")]
    pub Countries: Vec<CountryItem>,
}

/// WebAPI model `ProductItem`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductItem {
    #[serde(rename = "ErpId")]
    pub ErpId: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "VatRateId")]
    pub VatRateId: i32,
    #[serde(rename = "ForeignName")]
    pub ForeignName: String,
    #[serde(rename = "ForeignDescription")]
    pub ForeignDescription: String,
    #[serde(rename = "ShopProducts")]
    pub ShopProducts: Vec<ProductShopItem>,
}

/// WebAPI model `ProductMap`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductMap {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "ShopName")]
    pub ShopName: String,
    #[serde(rename = "ErpProductId")]
    pub ErpProductId: i32,
    #[serde(rename = "ShopProductId")]
    pub ShopProductId: String,
    #[serde(rename = "CountryId")]
    pub CountryId: i32,
}

/// WebAPI model `ProductMapCreate`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductMapCreate {
    #[serde(rename = "ErpProductId")]
    pub ErpProductId: i32,
    #[serde(rename = "ShopProductId")]
    pub ShopProductId: String,
    #[serde(rename = "CountryId")]
    pub CountryId: i32,
}

/// WebAPI model `ProductShopItem`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductShopItem {
    #[serde(rename = "ShopId")]
    pub ShopId: String,
}
