pub mod aspects;
pub mod criteria_filter;

use serde::{Deserialize, Serialize};

/// WebAPI model `AgriculturalPromotionFund`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgriculturalPromotionFund {
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Rate")]
    pub Rate: rust_decimal::Decimal,
    #[serde(rename = "AccountFK")]
    pub AccountFK: String,
}

/// WebAPI model `CNCode`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CNCode {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Active")]
    pub Active: bool,
}

/// WebAPI model `Catalog`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "ParentId")]
    pub ParentId: i32,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "FullPath")]
    pub FullPath: String,
}

/// WebAPI model `CharacterRelation`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterRelation {
    #[serde(rename = "Character")]
    pub Character: crate::web_api::interface::enums::enumDocumentCharacter,
    #[serde(rename = "Corrections")]
    pub Corrections: Vec<crate::web_api::interface::enums::enumDocumentCharacter>,
    #[serde(rename = "Related")]
    pub Related: Vec<crate::web_api::interface::enums::enumDocumentCharacter>,
}

/// WebAPI model `Country`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Active")]
    pub Active: bool,
}

/// WebAPI model `Currency`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Shortcut")]
    pub Shortcut: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Active")]
    pub Active: bool,
}

/// WebAPI model `CurrencyRate`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyRate {
    #[serde(rename = "CurrencyId")]
    pub CurrencyId: i32,
    #[serde(rename = "Date")]
    pub Date: chrono::NaiveDateTime,
    #[serde(rename = "Rate")]
    pub Rate: rust_decimal::Decimal,
}

/// WebAPI model `Department`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Department {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Code")]
    pub Code: String,
}

/// WebAPI model `Dimension`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimension {
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Value")]
    pub Value: Option<String>,
    #[serde(rename = "DictionaryValue")]
    pub DictionaryValue: Option<String>,
    #[serde(rename = "Type")]
    pub Type: String,
}

/// WebAPI model `DimensionClassification`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionClassification {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Type")]
    pub Type: String,
    #[serde(rename = "DictionaryId")]
    pub DictionaryId: Option<i32>,
    #[serde(rename = "Index")]
    pub Index: Option<i32>,
}

/// WebAPI model `DocumentAddress`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentAddress {
    #[serde(rename = "Name")]
    pub Name: Option<String>,
    #[serde(rename = "NIP")]
    pub NIP: Option<String>,
    #[serde(rename = "City")]
    pub City: Option<String>,
    #[serde(rename = "PostCode")]
    pub PostCode: Option<String>,
    #[serde(rename = "Street")]
    pub Street: Option<String>,
    #[serde(rename = "HouseNo")]
    pub HouseNo: Option<String>,
    #[serde(rename = "ApartmentNo")]
    pub ApartmentNo: Option<String>,
    #[serde(rename = "CountryId")]
    pub CountryId: Option<i32>,
}

/// WebAPI model `DocumentSeries`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSeries {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
}

/// WebAPI model `DocumentType`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentType {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "JPKDocumentKind")]
    pub JPKDocumentKind: crate::web_api::interface::enums::enumJPKDocumentKind,
    #[serde(rename = "JPK_V7Attributes")]
    pub JPK_V7Attributes: crate::web_api::interface::enums::enumJPK_V7DocumentAttribute,
    #[serde(rename = "Character")]
    pub Character: crate::web_api::interface::enums::enumDocumentCharacter,
    #[serde(rename = "CorrectionTypeId")]
    pub CorrectionTypeId: Option<i32>,
    #[serde(rename = "CorrectionSerieId")]
    pub CorrectionSerieId: Option<i32>,
    #[serde(rename = "RelatedTypeId")]
    pub RelatedTypeId: Option<i32>,
    #[serde(rename = "RelatedSerieId")]
    pub RelatedSerieId: Option<i32>,
    #[serde(rename = "DefaultDateRegister")]
    pub DefaultDateRegister: Option<i16>,
    #[serde(rename = "IsIssuedByBuyer")]
    pub IsIssuedByBuyer: bool,
    #[serde(rename = "IsThirdPartyContractorEnabled")]
    pub IsThirdPartyContractorEnabled: bool,
    #[serde(rename = "SelfTaxation")]
    pub SelfTaxation: i32,
    #[serde(rename = "IdRejestr")]
    pub IdRejestr: i32,
}

/// WebAPI model `FilterDimensionCriteria`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterDimensionCriteria {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Value")]
    pub Value: Option<String>,
    #[serde(rename = "DictionaryValue")]
    pub DictionaryValue: String,
}

/// WebAPI model `FilterDocumentType`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterDocumentType {
    #[serde(rename = "DocumentTypes")]
    pub DocumentTypes: Vec<String>,
}

/// WebAPI model `IncrementalSyncListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalSyncListElement<T> {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "ModifyType")]
    pub ModifyType: crate::web_api::interface::enums::enumIncrementalSyncModifyType,
    #[serde(rename = "ModifiedAt")]
    pub ModifiedAt: chrono::NaiveDateTime,
    #[serde(rename = "Object")]
    pub Object: T,
}

/// WebAPI model `Kind`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kind {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Active")]
    pub Active: bool,
}

/// WebAPI model `Marker`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker {
    #[serde(rename = "Subtype")]
    pub Subtype: i32,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Active")]
    pub Active: bool,
}

/// WebAPI model `PDF`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PDF {
    #[serde(rename = "Hash_SHA1")]
    pub Hash_SHA1: String,
    #[serde(rename = "ContentFile_Base64")]
    pub ContentFile_Base64: String,
    #[serde(rename = "CreateDate")]
    pub CreateDate: chrono::NaiveDateTime,
}

/// WebAPI model `PDFDimensionSetting`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PDFDimensionSetting {
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "PrintLabel")]
    pub PrintLabel: bool,
}

/// WebAPI model `PDFSettings`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PDFSettings {
    #[serde(rename = "PrintNote")]
    pub PrintNote: bool,
    #[serde(rename = "PrintRemarks")]
    pub PrintRemarks: bool,
    #[serde(rename = "PrintFooter")]
    pub PrintFooter: bool,
    #[serde(rename = "PrintExecutiveSubject")]
    pub PrintExecutiveSubject: bool,
    #[serde(rename = "OperationDateFormat")]
    pub OperationDateFormat:
        crate::web_api::interface::enums::enumPrintParameterOperationDateFormat,
    #[serde(rename = "DocumentStatus")]
    pub DocumentStatus: crate::web_api::interface::enums::enumPrintParameterDocumentStatus,
    #[serde(rename = "NotSettledDocuments")]
    pub NotSettledDocuments:
        crate::web_api::interface::enums::enumPrintParameterNotSettledDocuments,
    #[serde(rename = "DocumentStatusText")]
    pub DocumentStatusText: String,
    #[serde(rename = "IncludedDimensions")]
    pub IncludedDimensions: Vec<PDFDimensionSetting>,
    #[serde(rename = "PrintLeftToPay")]
    pub PrintLeftToPay: bool,
    #[serde(rename = "PrintReceiptNumber")]
    pub PrintReceiptNumber: bool,
    #[serde(rename = "PrintDiscountColumns")]
    pub PrintDiscountColumns: bool,
    #[serde(rename = "MainSubjectAddressDimensionCode")]
    pub MainSubjectAddressDimensionCode: String,
    #[serde(rename = "MainSubjectNameDimensionCode")]
    pub MainSubjectNameDimensionCode: String,
    #[serde(rename = "ExecutiveSubjectAddressDimensionCode")]
    pub ExecutiveSubjectAddressDimensionCode: String,
    #[serde(rename = "ExecutiveSubjectNameDimensionCode")]
    pub ExecutiveSubjectNameDimensionCode: String,
    #[serde(rename = "PositionDescriptionDimensionCode")]
    pub PositionDescriptionDimensionCode: String,
}

/// WebAPI model `PKWiUCode`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PKWiUCode {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Active")]
    pub Active: bool,
}

/// WebAPI model `PaymentForm`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentForm {
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Days")]
    pub Days: i32,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumPaymentRegistryType,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Client")]
    pub Client: i32,
}

/// WebAPI model `PaymentFormBase`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentFormBase {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Client")]
    pub Client: i32,
}

/// WebAPI model `PaymentRegistry`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRegistry {
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumPaymentRegistryType,
    #[serde(rename = "DefaultPaymentFormId")]
    pub DefaultPaymentFormId: Option<i32>,
    #[serde(rename = "BankName")]
    pub BankName: Option<String>,
    #[serde(rename = "BankAccountId")]
    pub BankAccountId: i32,
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "AccountNumber")]
    pub AccountNumber: Option<String>,
}

/// WebAPI model `PaymentRegistryBase`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRegistryBase {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "AccountNumber")]
    pub AccountNumber: Option<String>,
}

/// WebAPI model `PositionDimension`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionDimension {
    #[serde(rename = "PositionId")]
    pub PositionId: i32,
    #[serde(rename = "Dimensions")]
    pub Dimensions: Vec<Dimension>,
}

/// WebAPI model `PriceType`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceType {
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumPriceType,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Kind")]
    pub Kind: crate::web_api::interface::enums::enumPriceKind,
    #[serde(rename = "Active")]
    pub Active: bool,
}

/// WebAPI model `Region`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Active")]
    pub Active: bool,
}

/// WebAPI model `RelatedDocumentPosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedDocumentPosition {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "No")]
    pub No: i32,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
    #[serde(rename = "NetValuePLN")]
    pub NetValuePLN: rust_decimal::Decimal,
    #[serde(rename = "DocumentId")]
    pub DocumentId: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "QuantityCorrected")]
    pub QuantityCorrected: rust_decimal::Decimal,
    #[serde(rename = "NetValuePLNCorrected")]
    pub NetValuePLNCorrected: rust_decimal::Decimal,
}

/// WebAPI model `UnitOfMeasurement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitOfMeasurement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Active")]
    pub Active: bool,
}

/// WebAPI model `UnitOfMeasurementDefinition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitOfMeasurementDefinition {
    #[serde(rename = "Guid")]
    pub Guid: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "No")]
    pub No: i32,
}

/// WebAPI model `VatRate`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatRate {
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "DateTo")]
    pub DateTo: Option<chrono::NaiveDateTime>,
    #[serde(rename = "DateFrom")]
    pub DateFrom: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumVatRateType,
    #[serde(rename = "Fiscal")]
    pub Fiscal: bool,
    #[serde(rename = "RR")]
    pub RR: bool,
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
}

/// WebAPI model `VatRateBase`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatRateBase {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
}

/// WebAPI model `Warehouse`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warehouse {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Code")]
    pub Code: String,
}
