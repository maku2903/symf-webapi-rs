use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// WebAPI enum `AccountBalanceTurnoverType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountBalanceTurnoverType {
    /// Wire value `0`.
    Balance,
    /// Wire value `1`.
    TurnoverDebit,
    /// Wire value `2`.
    TurnoverCredit,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl AccountBalanceTurnoverType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Balance => 0,
            Self::TurnoverDebit => 1,
            Self::TurnoverCredit => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for AccountBalanceTurnoverType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for AccountBalanceTurnoverType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Balance,
            1 => Self::TurnoverDebit,
            2 => Self::TurnoverCredit,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `DictionaryElementType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DictionaryElementType {
    /// Wire value `0`.
    Record,
    /// Wire value `1`.
    RecordOrDimension,
    /// Wire value `2`.
    Dimension,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl DictionaryElementType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Record => 0,
            Self::RecordOrDimension => 1,
            Self::Dimension => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for DictionaryElementType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for DictionaryElementType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Record,
            1 => Self::RecordOrDimension,
            2 => Self::Dimension,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `DictionaryType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DictionaryType {
    /// Wire value `1`.
    ProductType,
    /// Wire value `2`.
    Product,
    /// Wire value `3`.
    ContractorType,
    /// Wire value `4`.
    Contractor,
    /// Wire value `5`.
    Department,
    /// Wire value `6`.
    CostType,
    /// Wire value `101`.
    A01,
    /// Wire value `102`.
    A02,
    /// Wire value `103`.
    A03,
    /// Wire value `104`.
    A04,
    /// Wire value `105`.
    A05,
    /// Wire value `106`.
    A06,
    /// Wire value `107`.
    A07,
    /// Wire value `108`.
    A08,
    /// Wire value `109`.
    A09,
    /// Wire value `110`.
    A10,
    /// Wire value `111`.
    A11,
    /// Wire value `112`.
    A12,
    /// Wire value `113`.
    A13,
    /// Wire value `114`.
    A14,
    /// Wire value `115`.
    A15,
    /// Wire value `116`.
    A16,
    /// Wire value `117`.
    A17,
    /// Wire value `118`.
    A18,
    /// Wire value `119`.
    A19,
    /// Wire value `120`.
    A20,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl DictionaryType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::ProductType => 1,
            Self::Product => 2,
            Self::ContractorType => 3,
            Self::Contractor => 4,
            Self::Department => 5,
            Self::CostType => 6,
            Self::A01 => 101,
            Self::A02 => 102,
            Self::A03 => 103,
            Self::A04 => 104,
            Self::A05 => 105,
            Self::A06 => 106,
            Self::A07 => 107,
            Self::A08 => 108,
            Self::A09 => 109,
            Self::A10 => 110,
            Self::A11 => 111,
            Self::A12 => 112,
            Self::A13 => 113,
            Self::A14 => 114,
            Self::A15 => 115,
            Self::A16 => 116,
            Self::A17 => 117,
            Self::A18 => 118,
            Self::A19 => 119,
            Self::A20 => 120,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for DictionaryType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for DictionaryType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            1 => Self::ProductType,
            2 => Self::Product,
            3 => Self::ContractorType,
            4 => Self::Contractor,
            5 => Self::Department,
            6 => Self::CostType,
            101 => Self::A01,
            102 => Self::A02,
            103 => Self::A03,
            104 => Self::A04,
            105 => Self::A05,
            106 => Self::A06,
            107 => Self::A07,
            108 => Self::A08,
            109 => Self::A09,
            110 => Self::A10,
            111 => Self::A11,
            112 => Self::A12,
            113 => Self::A13,
            114 => Self::A14,
            115 => Self::A15,
            116 => Self::A16,
            117 => Self::A17,
            118 => Self::A18,
            119 => Self::A19,
            120 => Self::A20,
            other => Self::UnknownValue(other),
        })
    }
}
