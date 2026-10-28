use serde::{Deserialize, Serialize};

/// WebAPI model `AdvancePaymentOptions`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancePaymentOptions {
    #[serde(rename = "Value")]
    pub Value: rust_decimal::Decimal,
    #[serde(rename = "IsLast")]
    pub IsLast: bool,
    #[serde(rename = "AutomaticFillPositions")]
    pub AutomaticFillPositions: bool,
}

/// WebAPI model `FiscalPrinterStatus`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiscalPrinterStatus {
    #[serde(rename = "IsConnected")]
    pub IsConnected: bool,
    #[serde(rename = "ChargerConnected")]
    pub ChargerConnected: Option<bool>,
    #[serde(rename = "CommandQueueEmpty")]
    pub CommandQueueEmpty: Option<bool>,
    #[serde(rename = "PaperRunningOut")]
    pub PaperRunningOut: Option<bool>,
    #[serde(rename = "CommunicationInterfaceStatus")]
    pub CommunicationInterfaceStatus:
        Option<crate::web_api::interface::enums::FiscalCommunicationInterfaceStatus>,
    #[serde(rename = "DeviceStatus")]
    pub DeviceStatus: Option<crate::web_api::interface::enums::FiscalDeviceStatus>,
}

/// WebAPI model `FiscalizationResult`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiscalizationResult {
    #[serde(rename = "Success")]
    pub Success: bool,
    #[serde(rename = "ReceiptNumber")]
    pub ReceiptNumber: Option<i32>,
    #[serde(rename = "FiscalDeviceNo")]
    pub FiscalDeviceNo: String,
    #[serde(rename = "ErrorMessage")]
    pub ErrorMessage: String,
}

/// WebAPI model `SaleCorrection`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleCorrection {
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
    #[serde(rename = "ThirdPartyContractorId")]
    pub ThirdPartyContractorId: Option<i32>,
    #[serde(rename = "ThirdPartyContractorAddressId")]
    pub ThirdPartyContractorAddressId: Option<i32>,
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
    #[serde(rename = "StatusKSeF")]
    pub StatusKSeF: Option<i32>,
    #[serde(rename = "NumberKSeF")]
    pub NumberKSeF: Option<String>,
    #[serde(rename = "IssueDateKSeF")]
    pub IssueDateKSeF: Option<chrono::NaiveDateTime>,
    #[serde(rename = "eArchiveId")]
    pub eArchiveId: Option<String>,
    #[serde(rename = "Positions")]
    pub Positions: Vec<SaleCorrectionPosition>,
    #[serde(rename = "BuyerAddress")]
    pub BuyerAddress: crate::web_api::interface::common::view_models::DocumentAddress,
    #[serde(rename = "RecipientAddress")]
    pub RecipientAddress: crate::web_api::interface::common::view_models::DocumentAddress,
    #[serde(rename = "ThirdPartyContractorAddress")]
    pub ThirdPartyContractorAddress:
        crate::web_api::interface::common::view_models::DocumentAddress,
    #[serde(rename = "DocumentExternalMetadata")]
    pub DocumentExternalMetadata: String,
    #[serde(rename = "IsSmeProcedure")]
    pub IsSmeProcedure: bool,
    #[serde(rename = "InvoiceKind")]
    pub InvoiceKind: serde_json::Value,
}

/// WebAPI model `SaleCorrectionIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleCorrectionIssue {
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
    #[serde(rename = "Buyer")]
    pub Buyer: SaleCorrectionIssueContractor,
    #[serde(rename = "Recipient")]
    pub Recipient: SaleCorrectionIssueContractor,
    #[serde(rename = "PaymentRegistry")]
    pub PaymentRegistry: crate::web_api::interface::common::view_models::PaymentRegistryBase,
    #[serde(rename = "PaymentFormId")]
    pub PaymentFormId: Option<i32>,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "SaleDate")]
    pub SaleDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "MaturityDate")]
    pub MaturityDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "CurrencyRate")]
    pub CurrencyRate: Option<rust_decimal::Decimal>,
    #[serde(rename = "Positions")]
    pub Positions: Vec<SaleCorrectionIssuePosition>,
    #[serde(rename = "Catalog")]
    pub Catalog: SaleDocumentIssueCatalog,
    #[serde(rename = "Kind")]
    pub Kind: SaleDocumentIssueKind,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "ReceivedBy")]
    pub ReceivedBy: String,
    #[serde(rename = "CorrectionReason")]
    pub CorrectionReason: String,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "DocumentExternalMetadata")]
    pub DocumentExternalMetadata: String,
    #[serde(rename = "IsSmeProcedure")]
    pub IsSmeProcedure: bool,
}

/// WebAPI model `SaleCorrectionIssueContractor`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleCorrectionIssueContractor {
    #[serde(rename = "RecalculatePrices")]
    pub RecalculatePrices: bool,
    #[serde(rename = "Data")]
    pub Data: SaleDocumentIssueContractorData,
    #[serde(rename = "DeliveryAddressCode")]
    pub DeliveryAddressCode: String,
}

/// WebAPI model `SaleCorrectionIssuePosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleCorrectionIssuePosition {
    #[serde(rename = "No")]
    pub No: Option<i32>,
    #[serde(rename = "VatRate")]
    pub VatRate: Option<crate::web_api::interface::common::view_models::VatRateBase>,
    #[serde(rename = "Elements")]
    pub Elements: Vec<SaleCorrectionIssuePositionElement>,
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
    #[serde(rename = "ServiceExport")]
    pub ServiceExport: Option<bool>,
}

/// WebAPI model `SaleCorrectionIssuePositionElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleCorrectionIssuePositionElement {
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
    #[serde(rename = "ServiceExport")]
    pub ServiceExport: Option<bool>,
}

/// WebAPI model `SaleCorrectionPosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleCorrectionPosition {
    #[serde(rename = "No")]
    pub No: i32,
    #[serde(rename = "BeforeCorrection")]
    pub BeforeCorrection: SaleCorrectionPositionElement,
    #[serde(rename = "AfterCorrection")]
    pub AfterCorrection: SaleCorrectionPositionElement,
}

/// WebAPI model `SaleCorrectionPositionElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleCorrectionPositionElement {
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

/// WebAPI model `SaleDocument`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleDocument {
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
    #[serde(rename = "ThirdPartyContractorId")]
    pub ThirdPartyContractorId: Option<i32>,
    #[serde(rename = "ThirdPartyContractorAddressId")]
    pub ThirdPartyContractorAddressId: Option<i32>,
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
    #[serde(rename = "eArchiveId")]
    pub eArchiveId: Option<String>,
    #[serde(rename = "Positions")]
    pub Positions: Vec<SaleDocumentPosition>,
    #[serde(rename = "BuyerAddress")]
    pub BuyerAddress: crate::web_api::interface::common::view_models::DocumentAddress,
    #[serde(rename = "RecipientAddress")]
    pub RecipientAddress: crate::web_api::interface::common::view_models::DocumentAddress,
    #[serde(rename = "ThirdPartyContractorAddress")]
    pub ThirdPartyContractorAddress:
        crate::web_api::interface::common::view_models::DocumentAddress,
    #[serde(rename = "TypeExternal")]
    pub TypeExternal: i32,
    #[serde(rename = "IdExternal")]
    pub IdExternal: Option<i32>,
    #[serde(rename = "Id2External")]
    pub Id2External: Option<String>,
    #[serde(rename = "DocumentExternalMetadata")]
    pub DocumentExternalMetadata: String,
    #[serde(rename = "IsSmeProcedure")]
    pub IsSmeProcedure: bool,
    #[serde(rename = "InvoiceKind")]
    pub InvoiceKind: serde_json::Value,
}

/// WebAPI model `SaleDocumentCorrection`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleDocumentCorrection {
    #[serde(rename = "No")]
    pub No: i32,
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "IssueDate")]
    pub IssueDate: Option<chrono::NaiveDateTime>,
    #[serde(rename = "SaleDate")]
    pub SaleDate: Option<chrono::NaiveDateTime>,
}

/// WebAPI model `SaleDocumentIssue`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleDocumentIssue {
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
    pub ReservationType: Option<crate::web_api::interface::enums::enumDocumentReservationType>,
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
    #[serde(rename = "CurrencyRateCIT")]
    pub CurrencyRateCIT: rust_decimal::Decimal,
    #[serde(rename = "SalePriceType")]
    pub SalePriceType: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "PriceKind")]
    pub PriceKind: crate::web_api::interface::enums::enumPriceKind,
    #[serde(rename = "SplitPayment")]
    pub SplitPayment: bool,
    #[serde(rename = "JPK_V7Attributes")]
    pub JPK_V7Attributes: Option<crate::web_api::interface::enums::enumJPK_V7DocumentAttribute>,
    #[serde(rename = "eArchiveId")]
    pub eArchiveId: Option<String>,
    #[serde(rename = "NumberKSeF")]
    pub NumberKSeF: Option<String>,
    #[serde(rename = "IssueDateKSeF")]
    pub IssueDateKSeF: Option<chrono::NaiveDateTime>,
    #[serde(rename = "Buyer")]
    pub Buyer: SaleDocumentIssueContractor,
    #[serde(rename = "Recipient")]
    pub Recipient: SaleDocumentIssueContractor,
    #[serde(rename = "Positions")]
    pub Positions: Vec<SaleDocumentIssuePosition>,
    #[serde(rename = "Catalog")]
    pub Catalog: SaleDocumentIssueCatalog,
    #[serde(rename = "Kind")]
    pub Kind: SaleDocumentIssueKind,
    #[serde(rename = "Marker")]
    pub Marker: i32,
    #[serde(rename = "ReceivedBy")]
    pub ReceivedBy: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "OssProcedure")]
    pub OssProcedure: Option<bool>,
    #[serde(rename = "OssShippingCountry")]
    pub OssShippingCountry: String,
    #[serde(rename = "OssDeliveryCountry")]
    pub OssDeliveryCountry: String,
    #[serde(rename = "TypeExternal")]
    pub TypeExternal: Option<i32>,
    #[serde(rename = "IdExternal")]
    pub IdExternal: Option<i32>,
    #[serde(rename = "Id2External")]
    pub Id2External: Option<String>,
    #[serde(rename = "DocumentExternalMetadata")]
    pub DocumentExternalMetadata: String,
    #[serde(rename = "IsSmeProcedure")]
    pub IsSmeProcedure: bool,
}

/// WebAPI model `SaleDocumentIssueCatalog`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleDocumentIssueCatalog {
    #[serde(rename = "FullPath")]
    pub FullPath: String,
    #[serde(rename = "AddIfNotExist")]
    pub AddIfNotExist: bool,
}

/// WebAPI model `SaleDocumentIssueContractor`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleDocumentIssueContractor {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "RecalculatePrices")]
    pub RecalculatePrices: bool,
    #[serde(rename = "Data")]
    pub Data: SaleDocumentIssueContractorData,
    #[serde(rename = "DeliveryAddressCode")]
    pub DeliveryAddressCode: String,
}

/// WebAPI model `SaleDocumentIssueContractorData`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleDocumentIssueContractorData {
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

/// WebAPI model `SaleDocumentIssueDelivery`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleDocumentIssueDelivery {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Quantity")]
    pub Quantity: rust_decimal::Decimal,
}

/// WebAPI model `SaleDocumentIssueKind`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleDocumentIssueKind {
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "AddIfNotExist")]
    pub AddIfNotExist: bool,
}

/// WebAPI model `SaleDocumentIssuePosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleDocumentIssuePosition {
    #[serde(rename = "VatRate")]
    pub VatRate: Option<crate::web_api::interface::common::view_models::VatRateBase>,
    #[serde(rename = "Elements")]
    pub Elements: Vec<SaleDocumentIssuePositionElement>,
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
    pub Deliveries: Vec<SaleDocumentIssueDelivery>,
    #[serde(rename = "OssService")]
    pub OssService: Option<bool>,
}

/// WebAPI model `SaleDocumentIssuePositionElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleDocumentIssuePositionElement {
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
    pub Deliveries: Vec<SaleDocumentIssueDelivery>,
    #[serde(rename = "OssService")]
    pub OssService: Option<bool>,
}

/// WebAPI model `SaleDocumentListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleDocumentListElement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "DocumentNumber")]
    pub DocumentNumber: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Canceled")]
    pub Canceled: crate::web_api::interface::enums::enumCancelationType,
    #[serde(rename = "Buffer")]
    pub Buffer: i16,
    #[serde(rename = "Settled")]
    pub Settled: i16,
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
    #[serde(rename = "ThirdPartyContractorId")]
    pub ThirdPartyContractorId: Option<i32>,
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
    #[serde(rename = "IncomeValue")]
    pub IncomeValue: rust_decimal::Decimal,
    #[serde(rename = "Currency")]
    pub Currency: String,
    #[serde(rename = "Description")]
    pub Description: String,
    #[serde(rename = "IsSmeProcedure")]
    pub IsSmeProcedure: bool,
    #[serde(rename = "TypeExternal")]
    pub TypeExternal: i32,
    #[serde(rename = "IdExternal")]
    pub IdExternal: Option<i32>,
    #[serde(rename = "Id2External")]
    pub Id2External: Option<String>,
    #[serde(rename = "InvoiceKind")]
    pub InvoiceKind: serde_json::Value,
}

/// WebAPI model `SaleDocumentPosition`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleDocumentPosition {
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

/// WebAPI model `SaleDocumentStatus`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleDocumentStatus {
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

/// WebAPI model `SaleDocumentWZ`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleDocumentWZ {
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

/// WebAPI model `SaleDocumentZMO`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleDocumentZMO {
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
