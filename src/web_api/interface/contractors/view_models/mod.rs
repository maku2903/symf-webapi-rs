use serde::{Deserialize, Serialize};

/// WebAPI model `Contractor`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contractor {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Vies")]
    pub Vies: bool,
    #[serde(rename = "VATTaxPayer")]
    pub VATTaxPayer: bool,
    #[serde(rename = "NIP")]
    pub NIP: Option<String>,
    #[serde(rename = "Pesel")]
    pub Pesel: String,
    #[serde(rename = "Regon")]
    pub Regon: String,
    #[serde(rename = "CreditLimit")]
    pub CreditLimit: bool,
    #[serde(rename = "MaxCreditValue")]
    pub MaxCreditValue: rust_decimal::Decimal,
    #[serde(rename = "CreditCurrency")]
    pub CreditCurrency: String,
    #[serde(rename = "PriceNegotiation")]
    pub PriceNegotiation: bool,
    #[serde(rename = "DefaultDiscountPercent")]
    pub DefaultDiscountPercent: rust_decimal::Decimal,
    #[serde(rename = "PriceType")]
    pub PriceType: crate::web_api::interface::enums::enumSalePriceType,
    #[serde(rename = "PriceKind")]
    pub PriceKind: crate::web_api::interface::enums::enumPriceKind,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumContractorType,
    #[serde(rename = "SplitPayment")]
    pub SplitPayment: crate::web_api::interface::enums::enumContractorSplitPayment,
    #[serde(rename = "Note")]
    pub Note: Option<String>,
    #[serde(rename = "Marker")]
    pub Marker: i16,
    #[serde(rename = "Due")]
    pub Due: rust_decimal::Decimal,
    #[serde(rename = "Obligation")]
    pub Obligation: rust_decimal::Decimal,
    #[serde(rename = "PaymentRegistry")]
    pub PaymentRegistry: crate::web_api::interface::common::view_models::PaymentRegistryBase,
    #[serde(rename = "PaymentForm")]
    pub PaymentForm: crate::web_api::interface::common::view_models::PaymentFormBase,
    #[serde(rename = "Contact")]
    pub Contact: ContractorContact,
    #[serde(rename = "DefaultAddress")]
    pub DefaultAddress: ContractorDefaultAddress,
    #[serde(rename = "CorrespondenceAddress")]
    pub CorrespondenceAddress: ContractorCorrespondenceAddress,
    #[serde(rename = "BankInfo")]
    pub BankInfo: ContractorBankInfo,
    #[serde(rename = "FK")]
    pub FK: ContractorFK,
    #[serde(rename = "Catalog")]
    pub Catalog: crate::web_api::interface::common::view_models::Catalog,
    #[serde(rename = "Kind")]
    pub Kind: crate::web_api::interface::common::view_models::Kind,
    #[serde(rename = "Dimensions")]
    pub Dimensions: Vec<crate::web_api::interface::common::view_models::Dimension>,
    #[serde(rename = "DeliveryAddresses")]
    pub DeliveryAddresses: Vec<ContractorDeliveryAddress>,
    #[serde(rename = "RtfNote")]
    pub RtfNote: Option<String>,
}

/// WebAPI model `ContractorAddress`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractorAddress {
    #[serde(rename = "Country")]
    pub Country: String,
    #[serde(rename = "City")]
    pub City: Option<String>,
    #[serde(rename = "Province")]
    pub Province: String,
    #[serde(rename = "Street")]
    pub Street: Option<String>,
    #[serde(rename = "HouseNo")]
    pub HouseNo: Option<String>,
    #[serde(rename = "ApartmentNo")]
    pub ApartmentNo: Option<String>,
    #[serde(rename = "PostCode")]
    pub PostCode: Option<String>,
    #[serde(rename = "FullAddress")]
    pub FullAddress: String,
}

/// WebAPI model `ContractorBankInfo`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractorBankInfo {
    #[serde(rename = "AccountNumber")]
    pub AccountNumber: Option<String>,
    #[serde(rename = "AccountNumer")]
    pub AccountNumer: String,
    #[serde(rename = "BankName")]
    pub BankName: Option<String>,
    #[serde(rename = "SWIFT_BIC")]
    pub SWIFT_BIC: String,
}

/// WebAPI model `ContractorContact`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractorContact {
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Surname")]
    pub Surname: String,
    #[serde(rename = "Phone1")]
    pub Phone1: String,
    #[serde(rename = "Phone2")]
    pub Phone2: String,
    #[serde(rename = "Fax")]
    pub Fax: String,
    #[serde(rename = "Email")]
    pub Email: String,
    #[serde(rename = "WWW")]
    pub WWW: String,
    #[serde(rename = "Facebook")]
    pub Facebook: String,
}

/// WebAPI model `ContractorCorrespondenceAddress`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractorCorrespondenceAddress {
    #[serde(rename = "Country")]
    pub Country: String,
    #[serde(rename = "City")]
    pub City: Option<String>,
    #[serde(rename = "Province")]
    pub Province: String,
    #[serde(rename = "Street")]
    pub Street: Option<String>,
    #[serde(rename = "HouseNo")]
    pub HouseNo: Option<String>,
    #[serde(rename = "ApartmentNo")]
    pub ApartmentNo: Option<String>,
    #[serde(rename = "PostCode")]
    pub PostCode: Option<String>,
    #[serde(rename = "FullAddress")]
    pub FullAddress: String,
}

/// WebAPI model `ContractorCriteriaFilter`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractorCriteriaFilter {
    #[serde(rename = "Code")]
    pub Code: crate::web_api::interface::common::view_models::criteria_filter::StringCriteriaFilter,
    #[serde(rename = "Name")]
    pub Name: crate::web_api::interface::common::view_models::criteria_filter::StringCriteriaFilter,
    #[serde(rename = "NIP")]
    pub NIP: Option<
        crate::web_api::interface::common::view_models::criteria_filter::StringCriteriaFilter,
    >,
    #[serde(rename = "City")]
    pub City: Option<
        crate::web_api::interface::common::view_models::criteria_filter::StringCriteriaFilter,
    >,
    #[serde(rename = "Province")]
    pub Province:
        crate::web_api::interface::common::view_models::criteria_filter::StringCriteriaFilter,
    #[serde(rename = "Marker")]
    pub Marker: crate::web_api::interface::common::view_models::criteria_filter::IntCriteriaFilter,
    #[serde(rename = "Active")]
    pub Active: Option<bool>,
    #[serde(rename = "Country")]
    pub Country:
        crate::web_api::interface::common::view_models::criteria_filter::StringCriteriaFilter,
    #[serde(rename = "EmailDomain")]
    pub EmailDomain:
        crate::web_api::interface::common::view_models::criteria_filter::StringCriteriaFilter,
    #[serde(rename = "KindId")]
    pub KindId: Option<i32>,
    #[serde(rename = "CatalogId")]
    pub CatalogId: Option<i32>,
}

/// WebAPI model `ContractorDefaultAddress`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractorDefaultAddress {
    #[serde(rename = "Country")]
    pub Country: String,
    #[serde(rename = "City")]
    pub City: Option<String>,
    #[serde(rename = "Province")]
    pub Province: String,
    #[serde(rename = "Street")]
    pub Street: Option<String>,
    #[serde(rename = "HouseNo")]
    pub HouseNo: Option<String>,
    #[serde(rename = "ApartmentNo")]
    pub ApartmentNo: Option<String>,
    #[serde(rename = "PostCode")]
    pub PostCode: Option<String>,
    #[serde(rename = "FullAddress")]
    pub FullAddress: String,
}

/// WebAPI model `ContractorDeliveryAddress`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractorDeliveryAddress {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Country")]
    pub Country: String,
    #[serde(rename = "City")]
    pub City: Option<String>,
    #[serde(rename = "Province")]
    pub Province: String,
    #[serde(rename = "Street")]
    pub Street: Option<String>,
    #[serde(rename = "HouseNo")]
    pub HouseNo: Option<String>,
    #[serde(rename = "ApartmentNo")]
    pub ApartmentNo: Option<String>,
    #[serde(rename = "PostCode")]
    pub PostCode: Option<String>,
    #[serde(rename = "FullAddress")]
    pub FullAddress: String,
}

/// WebAPI model `ContractorFK`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractorFK {
    #[serde(rename = "IdFK")]
    pub IdFK: String,
    #[serde(rename = "IdFKSync")]
    pub IdFKSync: Option<i32>,
    #[serde(rename = "IdentFK")]
    pub IdentFK: String,
    #[serde(rename = "ParameterFK")]
    pub ParameterFK: String,
}

/// WebAPI model `ContractorFilterCriteria`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractorFilterCriteria {
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
    #[serde(rename = "NIP_From")]
    pub NIP_From: String,
    #[serde(rename = "NIP_To")]
    pub NIP_To: String,
    #[serde(rename = "NIP_Mode")]
    pub NIP_Mode: Option<crate::web_api::interface::enums::enumFilterCriteriaMode>,
    #[serde(rename = "City_From")]
    pub City_From: String,
    #[serde(rename = "City_To")]
    pub City_To: String,
    #[serde(rename = "City_Mode")]
    pub City_Mode: Option<crate::web_api::interface::enums::enumFilterCriteriaMode>,
    #[serde(rename = "Province_From")]
    pub Province_From: String,
    #[serde(rename = "Province_To")]
    pub Province_To: String,
    #[serde(rename = "Province_Mode")]
    pub Province_Mode: Option<crate::web_api::interface::enums::enumFilterCriteriaMode>,
    #[serde(rename = "Marker_From")]
    pub Marker_From: i16,
    #[serde(rename = "Marker_To")]
    pub Marker_To: i16,
    #[serde(rename = "Marker_Mode")]
    pub Marker_Mode: Option<crate::web_api::interface::enums::enumFilterCriteriaMode>,
    #[serde(rename = "Active")]
    pub Active: Option<bool>,
    #[serde(rename = "Country")]
    pub Country: String,
    #[serde(rename = "Dimensions")]
    pub Dimensions: Vec<crate::web_api::interface::common::view_models::FilterDimensionCriteria>,
}

/// WebAPI model `ContractorListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractorListElement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Type")]
    pub Type: Option<crate::web_api::interface::enums::enumContractorType>,
    #[serde(rename = "NIP")]
    pub NIP: Option<String>,
    #[serde(rename = "Pesel")]
    pub Pesel: Option<String>,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "PriceType")]
    pub PriceType: Option<crate::web_api::interface::enums::enumSalePriceType>,
    #[serde(rename = "PriceKind")]
    pub PriceKind: Option<crate::web_api::interface::enums::enumPriceKind>,
    #[serde(rename = "PaymentFormOid")]
    pub PaymentFormOid: i32,
    #[serde(rename = "PaymentRegistryOid")]
    pub PaymentRegistryOid: i32,
    #[serde(rename = "Place")]
    pub Place: Option<String>,
    #[serde(rename = "PostCode")]
    pub PostCode: Option<String>,
}

/// WebAPI model `ContractorListElementWithDimensions`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractorListElementWithDimensions {
    #[serde(rename = "Dimensions")]
    pub Dimensions: Vec<crate::web_api::interface::common::view_models::Dimension>,
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Type")]
    pub Type: Option<crate::web_api::interface::enums::enumContractorType>,
    #[serde(rename = "NIP")]
    pub NIP: Option<String>,
    #[serde(rename = "Pesel")]
    pub Pesel: Option<String>,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "PriceType")]
    pub PriceType: Option<crate::web_api::interface::enums::enumSalePriceType>,
    #[serde(rename = "PriceKind")]
    pub PriceKind: Option<crate::web_api::interface::enums::enumPriceKind>,
    #[serde(rename = "PaymentFormOid")]
    pub PaymentFormOid: i32,
    #[serde(rename = "PaymentRegistryOid")]
    pub PaymentRegistryOid: i32,
    #[serde(rename = "Place")]
    pub Place: Option<String>,
    #[serde(rename = "PostCode")]
    pub PostCode: Option<String>,
}
