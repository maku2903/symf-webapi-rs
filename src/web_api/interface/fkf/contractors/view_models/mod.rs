use serde::{Deserialize, Serialize};

/// WebAPI model `Contractor`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contractor {
    #[serde(rename = "Id")]
    pub Id: Option<i32>,
    #[serde(rename = "Position")]
    pub Position: Option<i32>,
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
    #[serde(rename = "SplitPayment")]
    pub SplitPayment: crate::web_api::interface::enums::enumContractorSplitPayment,
    #[serde(rename = "NIP")]
    pub NIP: Option<String>,
    #[serde(rename = "Regon")]
    pub Regon: String,
    #[serde(rename = "Pesel")]
    pub Pesel: String,
    #[serde(rename = "CreditLimit")]
    pub CreditLimit: bool,
    #[serde(rename = "MaxCreditValue")]
    pub MaxCreditValue: rust_decimal::Decimal,
    #[serde(rename = "CreditCurrency")]
    pub CreditCurrency: String,
    #[serde(rename = "Type")]
    pub Type: crate::web_api::interface::enums::enumContractorType,
    #[serde(rename = "Contact")]
    pub Contact: ContractorContact,
    #[serde(rename = "Address")]
    pub Address: ContractorAddress,
    #[serde(rename = "BankInfo")]
    pub BankInfo: ContractorBankInfo,
    #[serde(rename = "Dimensions")]
    pub Dimensions: Vec<crate::web_api::interface::fkf::common::view_models::Dimension>,
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
}

/// WebAPI model `ContractorBankInfo`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractorBankInfo {
    #[serde(rename = "AccountNumber")]
    pub AccountNumber: Option<String>,
    #[serde(rename = "BankName")]
    pub BankName: Option<String>,
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
    #[serde(rename = "Telex")]
    pub Telex: String,
    #[serde(rename = "Email")]
    pub Email: String,
    #[serde(rename = "WWW")]
    pub WWW: String,
    #[serde(rename = "Facebook")]
    pub Facebook: String,
}

/// WebAPI model `ContractorCriteriaFilter`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractorCriteriaFilter {
    #[serde(rename = "Code")]
    pub Code:
        crate::web_api::interface::fkf::common::view_models::criteria_filter::StringCriteriaFilter,
    #[serde(rename = "Name")]
    pub Name:
        crate::web_api::interface::fkf::common::view_models::criteria_filter::StringCriteriaFilter,
    #[serde(rename = "NIP")]
    pub NIP: Option<
        crate::web_api::interface::fkf::common::view_models::criteria_filter::StringCriteriaFilter,
    >,
    #[serde(rename = "City")]
    pub City: Option<
        crate::web_api::interface::fkf::common::view_models::criteria_filter::StringCriteriaFilter,
    >,
    #[serde(rename = "Province")]
    pub Province:
        crate::web_api::interface::fkf::common::view_models::criteria_filter::StringCriteriaFilter,
    #[serde(rename = "Marker")]
    pub Marker:
        crate::web_api::interface::fkf::common::view_models::criteria_filter::IntCriteriaFilter,
    #[serde(rename = "Active")]
    pub Active: Option<bool>,
    #[serde(rename = "Country")]
    pub Country:
        crate::web_api::interface::fkf::common::view_models::criteria_filter::StringCriteriaFilter,
    #[serde(rename = "EmailDomain")]
    pub EmailDomain:
        crate::web_api::interface::fkf::common::view_models::criteria_filter::StringCriteriaFilter,
    #[serde(rename = "KindId")]
    pub KindId: Option<i32>,
    #[serde(rename = "CatalogId")]
    pub CatalogId: Option<i32>,
}

/// WebAPI model `ContractorListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractorListElement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Position")]
    pub Position: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Type")]
    pub Type: Option<crate::web_api::interface::enums::enumContractorType>,
    #[serde(rename = "NIP")]
    pub NIP: Option<String>,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Place")]
    pub Place: String,
    #[serde(rename = "PostCode")]
    pub PostCode: Option<String>,
}
