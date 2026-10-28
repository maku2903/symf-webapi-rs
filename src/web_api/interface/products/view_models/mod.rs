pub mod price_calculations;
pub mod price_lists;
pub mod prices;

use serde::{Deserialize, Serialize};

/// WebAPI model `Product`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumProductType,
    #[serde(rename = "Barcode")]
    pub Barcode: String,
    #[serde(rename = "PKWiU")]
    pub PKWiU: String,
    #[serde(rename = "CNFull")]
    pub CNFull: String,
    #[serde(rename = "CN")]
    pub CN: String,
    #[serde(rename = "DeliveryScheme")]
    pub DeliveryScheme: String,
    #[serde(rename = "FiscalName")]
    pub FiscalName: String,
    #[serde(rename = "AgriculturalPromotionFund")]
    pub AgriculturalPromotionFund: String,
    #[serde(rename = "PriceNegotiation")]
    pub PriceNegotiation: bool,
    #[serde(rename = "PriceRounding")]
    pub PriceRounding: Option<crate::web_api::interface::enums::enumPriceRounding>,
    #[serde(rename = "PriceRecalculateType")]
    pub PriceRecalculateType: Option<crate::web_api::interface::enums::enumPriceRecalculateType>,
    #[serde(rename = "SettlementMethod")]
    pub SettlementMethod: Option<crate::web_api::interface::enums::enumSettlementMethod>,
    #[serde(rename = "PriceParameter")]
    pub PriceParameter: Option<crate::web_api::interface::enums::enumPriceParameter>,
    #[serde(rename = "VAT50Minus")]
    pub VAT50Minus: bool,
    #[serde(rename = "ReverseCharge")]
    pub ReverseCharge: bool,
    #[serde(rename = "SplitPayment")]
    pub SplitPayment: bool,
    #[serde(rename = "JPK_V7ProductGroup")]
    pub JPK_V7ProductGroup: Option<crate::web_api::interface::enums::enumJPK_V7ProductGroup>,
    #[serde(rename = "StaticSet")]
    pub StaticSet: bool,
    #[serde(rename = "PriceSetMode")]
    pub PriceSetMode: bool,
    #[serde(rename = "CollationSetMode")]
    pub CollationSetMode: bool,
    #[serde(rename = "SalePriceRecalculate")]
    pub SalePriceRecalculate: bool,
    #[serde(rename = "CurrencyMode")]
    pub CurrencyMode: bool,
    #[serde(rename = "CurrencyRateWithoutConversion")]
    pub CurrencyRateWithoutConversion: rust_decimal::Decimal,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "CurrencyConversion")]
    pub CurrencyConversion: i32,
    #[serde(rename = "CurrencyRateDate")]
    pub CurrencyRateDate: chrono::NaiveDateTime,
    #[serde(rename = "MinState")]
    pub MinState: rust_decimal::Decimal,
    #[serde(rename = "MaxState")]
    pub MaxState: rust_decimal::Decimal,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "Marker")]
    pub Marker: i16,
    #[serde(rename = "FK")]
    pub FK: ProductFK,
    #[serde(rename = "Intrastat")]
    pub Intrastat: ProductIntrastat,
    #[serde(rename = "UnitsOfMeasurement")]
    pub UnitsOfMeasurement: ProductUnitsOfMeasurement,
    #[serde(rename = "BasePrice")]
    pub BasePrice: crate::web_api::interface::products::view_models::prices::ProductBasePrice,
    #[serde(rename = "PurchasePrice")]
    pub PurchasePrice: crate::web_api::interface::products::view_models::prices::ProductPrice,
    #[serde(rename = "PurchasePriceInCurrency")]
    pub PurchasePriceInCurrency:
        crate::web_api::interface::products::view_models::prices::ProductPrice,
    #[serde(rename = "SalePrices")]
    pub SalePrices: Vec<crate::web_api::interface::products::view_models::prices::ProductSalePrice>,
    #[serde(rename = "CalcOther")]
    pub CalcOther: crate::web_api::interface::products::view_models::prices::ProductPrice,
    #[serde(rename = "CalcExcise")]
    pub CalcExcise: crate::web_api::interface::products::view_models::prices::ProductPrice,
    #[serde(rename = "CalcDuty")]
    pub CalcDuty: crate::web_api::interface::products::view_models::prices::ProductPrice,
    #[serde(rename = "Kind")]
    pub Kind: crate::web_api::interface::common::view_models::Kind,
    #[serde(rename = "Catalog")]
    pub Catalog: crate::web_api::interface::common::view_models::Catalog,
    #[serde(rename = "VatRate")]
    pub VatRate: Option<crate::web_api::interface::common::view_models::VatRateBase>,
    #[serde(rename = "SetElements")]
    pub SetElements: Option<Vec<ProductSetElement>>,
    #[serde(rename = "Dimensions")]
    pub Dimensions: Vec<crate::web_api::interface::common::view_models::Dimension>,
    #[serde(rename = "Barcodes")]
    pub Barcodes: Vec<ProductBarcode>,
    #[serde(rename = "LogisticFields")]
    pub LogisticFields: Vec<ProductLogisticField>,
    #[serde(rename = "RtfNote")]
    pub RtfNote: Option<String>,
}

/// WebAPI model `ProductBarcode`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductBarcode {
    #[serde(rename = "Barcode")]
    pub Barcode: Option<String>,
    #[serde(rename = "UnitName")]
    pub UnitName: String,
    #[serde(rename = "UnitOfMeasurement")]
    pub UnitOfMeasurement: String,
    #[serde(rename = "UnitOfMeasurementGuid")]
    pub UnitOfMeasurementGuid: Option<String>,
    #[serde(rename = "SendToKSeF")]
    pub SendToKSeF: Option<bool>,
}

/// WebAPI model `ProductBarcodes`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductBarcodes {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Barcodes")]
    pub Barcodes: Vec<ProductBarcode>,
}

/// WebAPI model `ProductCriteriaFilter`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCriteriaFilter {
    #[serde(rename = "Active")]
    pub Active: Option<bool>,
    #[serde(rename = "Marker")]
    pub Marker: crate::web_api::interface::common::view_models::criteria_filter::IntCriteriaFilter,
    #[serde(rename = "FiscalName")]
    pub FiscalName:
        crate::web_api::interface::common::view_models::criteria_filter::StringCriteriaFilter,
    #[serde(rename = "CN")]
    pub CN: crate::web_api::interface::common::view_models::criteria_filter::StringCriteriaFilter,
    #[serde(rename = "Barcode")]
    pub Barcode:
        crate::web_api::interface::common::view_models::criteria_filter::StringCriteriaFilter,
    #[serde(rename = "Name")]
    pub Name: crate::web_api::interface::common::view_models::criteria_filter::StringCriteriaFilter,
    #[serde(rename = "Code")]
    pub Code: crate::web_api::interface::common::view_models::criteria_filter::StringCriteriaFilter,
    #[serde(rename = "Type")]
    pub Type: Option<crate::web_api::interface::enums::enumProductType>,
    #[serde(rename = "KindId")]
    pub KindId: Option<i32>,
    #[serde(rename = "CatalogId")]
    pub CatalogId: Option<i32>,
}

/// WebAPI model `ProductFK`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFK {
    #[serde(rename = "IdFK")]
    pub IdFK: String,
    #[serde(rename = "ParameterFK")]
    pub ParameterFK: String,
}

/// WebAPI model `ProductFilterCriteria`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFilterCriteria {
    #[serde(rename = "Code_From")]
    pub Code_From: String,
    #[serde(rename = "Code_To")]
    pub Code_To: String,
    #[serde(rename = "Code_Mode")]
    pub Code_Mode: Option<crate::web_api::interface::enums::enumFilterCriteriaMode>,
    #[serde(rename = "Name_From")]
    pub Name_From: String,
    #[serde(rename = "Name_To")]
    pub Name_To: String,
    #[serde(rename = "Name_Mode")]
    pub Name_Mode: Option<crate::web_api::interface::enums::enumFilterCriteriaMode>,
    #[serde(rename = "Barcode_From")]
    pub Barcode_From: String,
    #[serde(rename = "Barcode_To")]
    pub Barcode_To: String,
    #[serde(rename = "Barcode_Mode")]
    pub Barcode_Mode: Option<crate::web_api::interface::enums::enumFilterCriteriaMode>,
    #[serde(rename = "CN_From")]
    pub CN_From: i32,
    #[serde(rename = "CN_To")]
    pub CN_To: i32,
    #[serde(rename = "CN_Mode")]
    pub CN_Mode: Option<crate::web_api::interface::enums::enumFilterCriteriaMode>,
    #[serde(rename = "FiscalName_From")]
    pub FiscalName_From: String,
    #[serde(rename = "FiscalName_To")]
    pub FiscalName_To: String,
    #[serde(rename = "FiscalName_Mode")]
    pub FiscalName_Mode: Option<crate::web_api::interface::enums::enumFilterCriteriaMode>,
    #[serde(rename = "Marker_From")]
    pub Marker_From: i16,
    #[serde(rename = "Marker_To")]
    pub Marker_To: i16,
    #[serde(rename = "Marker_Mode")]
    pub Marker_Mode: Option<crate::web_api::interface::enums::enumFilterCriteriaMode>,
    #[serde(rename = "Active")]
    pub Active: Option<bool>,
    #[serde(rename = "Dimensions")]
    pub Dimensions: Vec<crate::web_api::interface::common::view_models::FilterDimensionCriteria>,
}

/// WebAPI model `ProductIntrastat`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductIntrastat {
    #[serde(rename = "RatioOfWeight")]
    pub RatioOfWeight: rust_decimal::Decimal,
    #[serde(rename = "RatioOfComplementaryUM")]
    pub RatioOfComplementaryUM: rust_decimal::Decimal,
    #[serde(rename = "ComplementaryUM")]
    pub ComplementaryUM: String,
    #[serde(rename = "UseComplementaryUM")]
    pub UseComplementaryUM: bool,
}

/// WebAPI model `ProductListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductListElement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Unit")]
    pub Unit: Option<String>,
    #[serde(rename = "DefaultUnit")]
    pub DefaultUnit: Option<String>,
    #[serde(rename = "VatRate")]
    pub VatRate: Option<String>,
    #[serde(rename = "Type")]
    pub Type: Option<crate::web_api::interface::enums::enumProductType>,
    #[serde(rename = "Barcode")]
    pub Barcode: String,
    #[serde(rename = "CN")]
    pub CN: String,
    #[serde(rename = "PKWiU")]
    pub PKWiU: String,
    #[serde(rename = "FiscalName")]
    pub FiscalName: String,
}

/// WebAPI model `ProductListElementWithDimensions`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductListElementWithDimensions {
    #[serde(rename = "Dimensions")]
    pub Dimensions: Vec<crate::web_api::interface::common::view_models::Dimension>,
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Unit")]
    pub Unit: Option<String>,
    #[serde(rename = "DefaultUnit")]
    pub DefaultUnit: Option<String>,
    #[serde(rename = "VatRate")]
    pub VatRate: Option<String>,
    #[serde(rename = "Type")]
    pub Type: Option<crate::web_api::interface::enums::enumProductType>,
    #[serde(rename = "Barcode")]
    pub Barcode: String,
    #[serde(rename = "CN")]
    pub CN: String,
    #[serde(rename = "PKWiU")]
    pub PKWiU: String,
    #[serde(rename = "FiscalName")]
    pub FiscalName: String,
}

/// WebAPI model `ProductListElementWithSalePrices`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductListElementWithSalePrices {
    #[serde(rename = "SalePrices")]
    pub SalePrices:
        Vec<crate::web_api::interface::products::view_models::prices::ProductListElementSalePrice>,
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Unit")]
    pub Unit: Option<String>,
    #[serde(rename = "DefaultUnit")]
    pub DefaultUnit: Option<String>,
    #[serde(rename = "VatRate")]
    pub VatRate: Option<String>,
    #[serde(rename = "Type")]
    pub Type: Option<crate::web_api::interface::enums::enumProductType>,
    #[serde(rename = "Barcode")]
    pub Barcode: String,
    #[serde(rename = "CN")]
    pub CN: String,
    #[serde(rename = "PKWiU")]
    pub PKWiU: String,
    #[serde(rename = "FiscalName")]
    pub FiscalName: String,
}

/// WebAPI model `ProductLogisticField`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductLogisticField {
    #[serde(rename = "UnitTypeName")]
    pub UnitTypeName: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Value")]
    pub Value: Option<rust_decimal::Decimal>,
    #[serde(rename = "Unit")]
    pub Unit: String,
}

/// WebAPI model `ProductPhoto`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPhoto {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "CreateDate")]
    pub CreateDate: chrono::NaiveDateTime,
    #[serde(rename = "ContentFile_Base64")]
    pub ContentFile_Base64: String,
    #[serde(rename = "PhotoQuality")]
    pub PhotoQuality: crate::web_api::interface::enums::enumPhotoQuality,
}

/// WebAPI model `ProductPhotoListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPhotoListElement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "CreateDate")]
    pub CreateDate: chrono::NaiveDateTime,
}

/// WebAPI model `ProductSetElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSetElement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
    #[serde(rename = "DisplayQuantity")]
    pub DisplayQuantity: rust_decimal::Decimal,
    #[serde(rename = "DisplayUM")]
    pub DisplayUM: String,
}

/// WebAPI model `ProductUnitsOfMeasurement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductUnitsOfMeasurement {
    #[serde(rename = "RecordUM")]
    pub RecordUM: String,
    #[serde(rename = "AdditionalUM1")]
    pub AdditionalUM1: String,
    #[serde(rename = "AdditionalUM2")]
    pub AdditionalUM2: String,
    #[serde(rename = "DefaultUM")]
    pub DefaultUM: String,
    #[serde(rename = "DisplayUM")]
    pub DisplayUM: String,
    #[serde(rename = "DefaultUMKind")]
    pub DefaultUMKind: crate::web_api::interface::enums::enumDefaultUnitOfMeasurementKind,
    #[serde(rename = "RatioOfAdditionalUM1")]
    pub RatioOfAdditionalUM1: rust_decimal::Decimal,
    #[serde(rename = "RatioOfAdditionalUM2")]
    pub RatioOfAdditionalUM2: rust_decimal::Decimal,
    #[serde(rename = "RatioOfDefaultUM")]
    pub RatioOfDefaultUM: rust_decimal::Decimal,
}
