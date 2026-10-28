use serde::{Deserialize, Serialize};

/// WebAPI model `PurchaseCorrection`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseCorrection {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "MasterDocumentOid")]
    pub MasterDocumentOid: Option<i32>,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Canceled")]
    pub Canceled: crate::web_api::interface::enums::enumCancelationType,
    #[serde(rename = "Buffer")]
    pub Buffer: bool,
    #[serde(rename = "Settled")]
    pub Settled: bool,
    #[serde(rename = "Fisacal")]
    pub Fisacal: crate::web_api::interface::enums::enumFiscalizationStatus,
    #[serde(rename = "eInvoice")]
    pub eInvoice: bool,
    #[serde(rename = "SplitPayment")]
    pub SplitPayment: bool,
    #[serde(rename = "JPK_V7Attributes")]
    pub JPK_V7Attributes: crate::web_api::interface::enums::enumJPK_V7DocumentAttribute,
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
    #[serde(rename = "ReceivedBy")]
    pub ReceivedBy: String,
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
    #[serde(rename = "CurrencyRateCIT")]
    pub CurrencyRateCIT: rust_decimal::Decimal,
    #[serde(rename = "PaymentRegistryId")]
    pub PaymentRegistryId: i32,
    #[serde(rename = "PaymentFormId")]
    pub PaymentFormId: i32,
    #[serde(rename = "CorrectionReason")]
    pub CorrectionReason: String,
    #[serde(rename = "CatalogId")]
    pub CatalogId: i32,
    #[serde(rename = "KindId")]
    pub KindId: i32,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "ForeignDocumentDate")]
    pub ForeignDocumentDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "PurchaseDate")]
    pub PurchaseDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "VoluntarySplitPayment")]
    pub VoluntarySplitPayment: bool,
    #[serde(rename = "WhiteList")]
    pub WhiteList: bool,
    #[serde(rename = "ForeignDocumentNumber")]
    pub ForeignDocumentNumber: String,
    #[serde(rename = "StatusKSeF")]
    pub StatusKSeF: Option<i32>,
    #[serde(rename = "NumberKSeF")]
    pub NumberKSeF: Option<String>,
    #[serde(rename = "IssueDateKSeF")]
    pub IssueDateKSeF: Option<chrono::NaiveDateTime>,
    #[serde(rename = "eArchiveId")]
    pub eArchiveId: Option<String>,
    #[serde(rename = "IsSmeProcedure")]
    pub IsSmeProcedure: bool,
    #[serde(rename = "IsIssuedByBuyer")]
    pub IsIssuedByBuyer: bool,
    #[serde(rename = "InvoiceKind")]
    pub InvoiceKind: serde_json::Value,
    #[serde(rename = "Positions")]
    pub Positions: Vec<PurchaseCorrectionPosition>,
    #[serde(rename = "SellerAddress")]
    pub SellerAddress: crate::web_api::interface::common::view_models::DocumentAddress,
    #[serde(rename = "DelivererAddress")]
    pub DelivererAddress: crate::web_api::interface::common::view_models::DocumentAddress,
}

/// WebAPI model `PurchaseCorrectionIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseCorrectionIssue {
    #[serde(rename = "MasterDocumentId")]
    pub MasterDocumentId: Option<i32>,
    #[serde(rename = "MasterDocumentNumber")]
    pub MasterDocumentNumber: String,
    #[serde(rename = "TypeCode")]
    pub TypeCode: String,
    #[serde(rename = "Series")]
    pub Series: String,
    #[serde(rename = "NumberInSeries")]
    pub NumberInSeries: Option<i32>,
    #[serde(rename = "SplitPayment")]
    pub SplitPayment: bool,
    #[serde(rename = "JPK_V7Attributes")]
    pub JPK_V7Attributes: Option<crate::web_api::interface::enums::enumJPK_V7DocumentAttribute>,
    #[serde(rename = "Seller")]
    pub Seller: PurchaseCorrectionIssueContractor,
    #[serde(rename = "Deliverer")]
    pub Deliverer: PurchaseCorrectionIssueContractor,
    #[serde(rename = "PaymentRegistry")]
    pub PaymentRegistry: crate::web_api::interface::common::view_models::PaymentRegistryBase,
    #[serde(rename = "PaymentFormId")]
    pub PaymentFormId: Option<i32>,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ReceiveDate")]
    pub ReceiveDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "PurchaseDate")]
    pub PurchaseDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ForeignDocumentDate")]
    pub ForeignDocumentDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: Option<rust_decimal::Decimal>,
    #[serde(rename = "Positions")]
    pub Positions: Vec<PurchaseCorrectionIssuePosition>,
    #[serde(rename = "Catalog")]
    pub Catalog: PurchaseDocumentIssueCatalog,
    #[serde(rename = "Kind")]
    pub Kind: PurchaseDocumentIssueKind,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "CorrectionReason")]
    pub CorrectionReason: String,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "ForeignDocumentNumber")]
    pub ForeignDocumentNumber: String,
    #[serde(rename = "VoluntarySplitPayment")]
    pub VoluntarySplitPayment: bool,
    #[serde(rename = "WhiteList")]
    pub WhiteList: bool,
}

/// WebAPI model `PurchaseCorrectionIssueContractor`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseCorrectionIssueContractor {
    #[serde(rename = "RecalculatePrices")]
    pub RecalculatePrices: bool,
    #[serde(rename = "Data")]
    pub Data: PurchaseDocumentIssueContractorData,
    #[serde(rename = "DeliveryAddressCode")]
    pub DeliveryAddressCode: String,
}

/// WebAPI model `PurchaseCorrectionIssuePosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseCorrectionIssuePosition {
    #[serde(rename = "No")]
    pub No: Option<i32>,
    #[serde(rename = "VatRate")]
    pub VatRate: Option<crate::web_api::interface::common::view_models::VatRateBase>,
    #[serde(rename = "Elements")]
    pub Elements: Vec<PurchaseCorrectionIssuePositionElement>,
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
    #[serde(rename = "Duty")]
    pub Duty: rust_decimal::Decimal,
    #[serde(rename = "Excise")]
    pub Excise: rust_decimal::Decimal,
    #[serde(rename = "PriceValue")]
    pub PriceValue: rust_decimal::Decimal,
}

/// WebAPI model `PurchaseCorrectionIssuePositionElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseCorrectionIssuePositionElement {
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
    #[serde(rename = "Duty")]
    pub Duty: rust_decimal::Decimal,
    #[serde(rename = "Excise")]
    pub Excise: rust_decimal::Decimal,
    #[serde(rename = "PriceValue")]
    pub PriceValue: rust_decimal::Decimal,
}

/// WebAPI model `PurchaseCorrectionPosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseCorrectionPosition {
    #[serde(rename = "No")]
    pub No: i32,
    #[serde(rename = "BeforeCorrection")]
    pub BeforeCorrection: PurchaseCorrectionPositionElement,
    #[serde(rename = "AfterCorrection")]
    pub AfterCorrection: PurchaseCorrectionPositionElement,
}

/// WebAPI model `PurchaseCorrectionPositionElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseCorrectionPositionElement {
    #[serde(rename = "Id")]
    pub Id: i32,
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
    #[serde(rename = "UnitOfMeasurement")]
    pub UnitOfMeasurement: String,
    #[serde(rename = "WrittenUnitOfMeasurement")]
    pub WrittenUnitOfMeasurement: String,
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
    #[serde(rename = "Duty")]
    pub Duty: rust_decimal::Decimal,
    #[serde(rename = "Excise")]
    pub Excise: rust_decimal::Decimal,
}

/// WebAPI model `PurchaseDocument`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseDocument {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "ForeignDocumentNumber")]
    pub ForeignDocumentNumber: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Canceled")]
    pub Canceled: crate::web_api::interface::enums::enumCancelationType,
    #[serde(rename = "Buffer")]
    pub Buffer: bool,
    #[serde(rename = "Settled")]
    pub Settled: bool,
    #[serde(rename = "Fisacal")]
    pub Fisacal: crate::web_api::interface::enums::enumFiscalizationStatus,
    #[serde(rename = "eInvoice")]
    pub eInvoice: bool,
    #[serde(rename = "SplitPayment")]
    pub SplitPayment: bool,
    #[serde(rename = "VoluntarySplitPayment")]
    pub VoluntarySplitPayment: bool,
    #[serde(rename = "WhiteList")]
    pub WhiteList: bool,
    #[serde(rename = "JPK_V7Attributes")]
    pub JPK_V7Attributes: crate::web_api::interface::enums::enumJPK_V7DocumentAttribute,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ReceiveDate")]
    pub ReceiveDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ForeignDocumentDate")]
    pub ForeignDocumentDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "PurchaseDate")]
    pub PurchaseDate: Option<chrono::NaiveDateTime>,
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
    #[serde(rename = "ReceivedBy")]
    pub ReceivedBy: String,
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
    #[serde(rename = "CurrencyRateCIT")]
    pub CurrencyRateCIT: rust_decimal::Decimal,
    #[serde(rename = "PaymentRegistryId")]
    pub PaymentRegistryId: i32,
    #[serde(rename = "PaymentFormId")]
    pub PaymentFormId: i32,
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
    #[serde(rename = "StatusKSeF")]
    pub StatusKSeF: Option<i32>,
    #[serde(rename = "NumberKSeF")]
    pub NumberKSeF: Option<String>,
    #[serde(rename = "IssueDateKSeF")]
    pub IssueDateKSeF: Option<chrono::NaiveDateTime>,
    #[serde(rename = "InvoiceKind")]
    pub InvoiceKind: serde_json::Value,
    #[serde(rename = "eArchiveId")]
    pub eArchiveId: Option<String>,
    #[serde(rename = "IsSmeProcedure")]
    pub IsSmeProcedure: bool,
    #[serde(rename = "IsIssuedByBuyer")]
    pub IsIssuedByBuyer: bool,
    #[serde(rename = "Positions")]
    pub Positions: Vec<PurchaseDocumentPosition>,
    #[serde(rename = "SellerAddress")]
    pub SellerAddress: crate::web_api::interface::common::view_models::DocumentAddress,
    #[serde(rename = "DelivererAddress")]
    pub DelivererAddress: crate::web_api::interface::common::view_models::DocumentAddress,
}

/// WebAPI model `PurchaseDocumentCorrection`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseDocumentCorrection {
    #[serde(rename = "No")]
    pub No: i32,
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ReceiveDate")]
    pub ReceiveDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ForeignDocumentDate")]
    pub ForeignDocumentDate: Option<chrono::NaiveDateTime>,
}

/// WebAPI model `PurchaseDocumentIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseDocumentIssue {
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
    #[serde(rename = "PurchaseDate")]
    pub PurchaseDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ForeignDocumentDate")]
    pub ForeignDocumentDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: rust_decimal::Decimal,
    #[serde(rename = "CurrencyRateCIT")]
    pub CurrencyRateCIT: rust_decimal::Decimal,
    #[serde(rename = "PriceKind")]
    pub PriceKind: crate::web_api::interface::enums::enumPriceKind,
    #[serde(rename = "SplitPayment")]
    pub SplitPayment: bool,
    #[serde(rename = "VoluntarySplitPayment")]
    pub VoluntarySplitPayment: bool,
    #[serde(rename = "WhiteList")]
    pub WhiteList: bool,
    #[serde(rename = "JPK_V7Attributes")]
    pub JPK_V7Attributes: Option<crate::web_api::interface::enums::enumJPK_V7DocumentAttribute>,
    #[serde(rename = "eArchiveId")]
    pub eArchiveId: Option<String>,
    #[serde(rename = "NumberKSeF")]
    pub NumberKSeF: Option<String>,
    #[serde(rename = "IssueDateKSeF")]
    pub IssueDateKSeF: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Seller")]
    pub Seller: PurchaseDocumentIssueContractor,
    #[serde(rename = "Deliverer")]
    pub Deliverer: PurchaseDocumentIssueContractor,
    #[serde(rename = "Positions")]
    pub Positions: Vec<PurchaseDocumentIssuePosition>,
    #[serde(rename = "Catalog")]
    pub Catalog: PurchaseDocumentIssueCatalog,
    #[serde(rename = "Kind")]
    pub Kind: PurchaseDocumentIssueKind,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "ForeignDocumentNumber")]
    pub ForeignDocumentNumber: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "IsSmeProcedure")]
    pub IsSmeProcedure: bool,
}

/// WebAPI model `PurchaseDocumentIssueCatalog`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseDocumentIssueCatalog {
    #[serde(rename = "FullPath")]
    pub FullPath: String,
    #[serde(rename = "AddIfNotExist")]
    pub AddIfNotExist: bool,
}

/// WebAPI model `PurchaseDocumentIssueContractor`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseDocumentIssueContractor {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "RecalculatePrices")]
    pub RecalculatePrices: bool,
    #[serde(rename = "Data")]
    pub Data: PurchaseDocumentIssueContractorData,
    #[serde(rename = "DeliveryAddressCode")]
    pub DeliveryAddressCode: String,
}

/// WebAPI model `PurchaseDocumentIssueContractorData`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseDocumentIssueContractorData {
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

/// WebAPI model `PurchaseDocumentIssueKind`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseDocumentIssueKind {
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "AddIfNotExist")]
    pub AddIfNotExist: bool,
}

/// WebAPI model `PurchaseDocumentIssuePosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseDocumentIssuePosition {
    #[serde(rename = "VatRate")]
    pub VatRate: Option<crate::web_api::interface::common::view_models::VatRateBase>,
    #[serde(rename = "Elements")]
    pub Elements: Vec<PurchaseDocumentIssuePositionElement>,
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
    #[serde(rename = "Duty")]
    pub Duty: rust_decimal::Decimal,
    #[serde(rename = "Excise")]
    pub Excise: rust_decimal::Decimal,
    #[serde(rename = "PriceValue")]
    pub PriceValue: rust_decimal::Decimal,
}

/// WebAPI model `PurchaseDocumentIssuePositionElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseDocumentIssuePositionElement {
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
    #[serde(rename = "Duty")]
    pub Duty: rust_decimal::Decimal,
    #[serde(rename = "Excise")]
    pub Excise: rust_decimal::Decimal,
    #[serde(rename = "PriceValue")]
    pub PriceValue: rust_decimal::Decimal,
}

/// WebAPI model `PurchaseDocumentListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseDocumentListElement {
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
    #[serde(rename = "PurchaseDate")]
    pub PurchaseDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ReceiveDate")]
    pub ReceiveDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "ForeignDocumentDate")]
    pub ForeignDocumentDate: Option<chrono::NaiveDateTime>,
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
    #[serde(rename = "IsSmeProcedure")]
    pub IsSmeProcedure: bool,
    #[serde(rename = "IsIssuedByBuyer")]
    pub IsIssuedByBuyer: bool,
    #[serde(rename = "InvoiceKind")]
    pub InvoiceKind: serde_json::Value,
}

/// WebAPI model `PurchaseDocumentPZ`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseDocumentPZ {
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

/// WebAPI model `PurchaseDocumentPosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseDocumentPosition {
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
    #[serde(rename = "UnitOfMeasurement")]
    pub UnitOfMeasurement: String,
    #[serde(rename = "WrittenUnitOfMeasurement")]
    pub WrittenUnitOfMeasurement: String,
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
    #[serde(rename = "Duty")]
    pub Duty: rust_decimal::Decimal,
    #[serde(rename = "Excise")]
    pub Excise: rust_decimal::Decimal,
}

/// WebAPI model `PurchaseDocumentStatus`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseDocumentStatus {
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
