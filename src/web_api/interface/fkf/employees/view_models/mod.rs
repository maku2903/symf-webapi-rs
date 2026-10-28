use serde::{Deserialize, Serialize};

/// WebAPI model `Employee`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
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
    #[serde(rename = "SecondName")]
    pub SecondName: String,
    #[serde(rename = "Surname")]
    pub Surname: String,
    #[serde(rename = "NIP")]
    pub NIP: Option<String>,
    #[serde(rename = "Pesel")]
    pub Pesel: String,
    #[serde(rename = "Phone")]
    pub Phone: String,
    #[serde(rename = "Address")]
    pub Address: EmployeeAddress,
    #[serde(rename = "BankInfo")]
    pub BankInfo: EmployeeBankInfo,
}

/// WebAPI model `EmployeeAddress`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeAddress {
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

/// WebAPI model `EmployeeBankInfo`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeBankInfo {
    #[serde(rename = "AccountNumber")]
    pub AccountNumber: Option<String>,
    #[serde(rename = "BankName")]
    pub BankName: Option<String>,
}

/// WebAPI model `EmployeeListElement`.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeListElement {
    #[serde(rename = "Id")]
    pub Id: i32,
    #[serde(rename = "Position")]
    pub Position: i32,
    #[serde(rename = "Code")]
    pub Code: String,
    #[serde(rename = "Name")]
    pub Name: String,
    #[serde(rename = "Surname")]
    pub Surname: String,
    #[serde(rename = "NIP")]
    pub NIP: Option<String>,
    #[serde(rename = "Pesel")]
    pub Pesel: String,
    #[serde(rename = "Active")]
    pub Active: bool,
    #[serde(rename = "Place")]
    pub Place: String,
}
