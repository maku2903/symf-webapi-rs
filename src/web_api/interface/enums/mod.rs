pub mod fk_configurator;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// WebAPI enum `FiscalCommunicationInterfaceStatus`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FiscalCommunicationInterfaceStatus {
    /// Wire value `0`.
    TransactionResourcesFree,
    /// Wire value `1`.
    TransactionResourcesBusyByPC1,
    /// Wire value `2`.
    TransactionResourcesBusyByPC2,
    /// Wire value `3`.
    TransactionResourcesBusyBySystem,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl FiscalCommunicationInterfaceStatus {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::TransactionResourcesFree => 0,
            Self::TransactionResourcesBusyByPC1 => 1,
            Self::TransactionResourcesBusyByPC2 => 2,
            Self::TransactionResourcesBusyBySystem => 3,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for FiscalCommunicationInterfaceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for FiscalCommunicationInterfaceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::TransactionResourcesFree,
            1 => Self::TransactionResourcesBusyByPC1,
            2 => Self::TransactionResourcesBusyByPC2,
            3 => Self::TransactionResourcesBusyBySystem,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `FiscalDeviceStatus`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FiscalDeviceStatus {
    /// Wire value `0`.
    Ready,
    /// Wire value `1`.
    InMenu,
    /// Wire value `2`.
    WaitingForKey,
    /// Wire value `3`.
    WaitingForUser,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl FiscalDeviceStatus {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Ready => 0,
            Self::InMenu => 1,
            Self::WaitingForKey => 2,
            Self::WaitingForUser => 3,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for FiscalDeviceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for FiscalDeviceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Ready,
            1 => Self::InMenu,
            2 => Self::WaitingForKey,
            3 => Self::WaitingForUser,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumAccountingSchemePositionType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumAccountingSchemePositionType {
    /// Wire value `0`.
    Basic,
    /// Wire value `1`.
    Additional01,
    /// Wire value `2`.
    Additional02,
    /// Wire value `3`.
    Additional03,
    /// Wire value `4`.
    Additional04,
    /// Wire value `5`.
    Additional05,
    /// Wire value `6`.
    Additional06,
    /// Wire value `7`.
    Additional07,
    /// Wire value `8`.
    Additional08,
    /// Wire value `9`.
    Additional09,
    /// Wire value `10`.
    Additional10,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumAccountingSchemePositionType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Basic => 0,
            Self::Additional01 => 1,
            Self::Additional02 => 2,
            Self::Additional03 => 3,
            Self::Additional04 => 4,
            Self::Additional05 => 5,
            Self::Additional06 => 6,
            Self::Additional07 => 7,
            Self::Additional08 => 8,
            Self::Additional09 => 9,
            Self::Additional10 => 10,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumAccountingSchemePositionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumAccountingSchemePositionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Basic,
            1 => Self::Additional01,
            2 => Self::Additional02,
            3 => Self::Additional03,
            4 => Self::Additional04,
            5 => Self::Additional05,
            6 => Self::Additional06,
            7 => Self::Additional07,
            8 => Self::Additional08,
            9 => Self::Additional09,
            10 => Self::Additional10,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumCalculationState`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumCalculationState {
    /// Wire value `0`.
    All,
    /// Wire value `1`.
    Settled,
    /// Wire value `2`.
    NotSettled,
    /// Wire value `3`.
    NotSettledBeforeMaturityTerm,
    /// Wire value `4`.
    NotSettledAfterMaturityTerm,
    /// Wire value `5`.
    NotSettledWithoutMaturityTerm,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumCalculationState {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::All => 0,
            Self::Settled => 1,
            Self::NotSettled => 2,
            Self::NotSettledBeforeMaturityTerm => 3,
            Self::NotSettledAfterMaturityTerm => 4,
            Self::NotSettledWithoutMaturityTerm => 5,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumCalculationState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumCalculationState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::All,
            1 => Self::Settled,
            2 => Self::NotSettled,
            3 => Self::NotSettledBeforeMaturityTerm,
            4 => Self::NotSettledAfterMaturityTerm,
            5 => Self::NotSettledWithoutMaturityTerm,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumCalculationType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumCalculationType {
    /// Wire value `0`.
    Due,
    /// Wire value `1`.
    Obligation,
    /// Wire value `3`.
    All,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumCalculationType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Due => 0,
            Self::Obligation => 1,
            Self::All => 3,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumCalculationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumCalculationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Due,
            1 => Self::Obligation,
            3 => Self::All,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumCancelationType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumCancelationType {
    /// Wire value `0`.
    Other,
    /// Wire value `1`.
    Standard,
    /// Wire value `2`.
    ImportedFromArchive,
    /// Wire value `3`.
    CanceledReceipt,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumCancelationType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Other => 0,
            Self::Standard => 1,
            Self::ImportedFromArchive => 2,
            Self::CanceledReceipt => 3,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumCancelationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumCancelationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Other,
            1 => Self::Standard,
            2 => Self::ImportedFromArchive,
            3 => Self::CanceledReceipt,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumContractorSplitPayment`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumContractorSplitPayment {
    /// Wire value `0`.
    None,
    /// Wire value `1`.
    Blocked,
    /// Wire value `2`.
    Required,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumContractorSplitPayment {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Blocked => 1,
            Self::Required => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumContractorSplitPayment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumContractorSplitPayment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::None,
            1 => Self::Blocked,
            2 => Self::Required,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumContractorType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumContractorType {
    /// Wire value `0`.
    Firm,
    /// Wire value `1`.
    PersonFirm,
    /// Wire value `2`.
    Person,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumContractorType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Firm => 0,
            Self::PersonFirm => 1,
            Self::Person => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumContractorType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumContractorType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Firm,
            1 => Self::PersonFirm,
            2 => Self::Person,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumCurrencyRateType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumCurrencyRateType {
    /// Wire value `0`.
    WithoutCurrency,
    /// Wire value `1`.
    Purchase,
    /// Wire value `2`.
    Sale,
    /// Wire value `3`.
    Mean,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumCurrencyRateType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::WithoutCurrency => 0,
            Self::Purchase => 1,
            Self::Sale => 2,
            Self::Mean => 3,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumCurrencyRateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumCurrencyRateType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::WithoutCurrency,
            1 => Self::Purchase,
            2 => Self::Sale,
            3 => Self::Mean,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumDefaultUnitOfMeasurementKind`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumDefaultUnitOfMeasurementKind {
    /// Wire value `0`.
    None,
    /// Wire value `1`.
    Record,
    /// Wire value `2`.
    Additional1,
    /// Wire value `3`.
    Additional2,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumDefaultUnitOfMeasurementKind {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Record => 1,
            Self::Additional1 => 2,
            Self::Additional2 => 3,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumDefaultUnitOfMeasurementKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumDefaultUnitOfMeasurementKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::None,
            1 => Self::Record,
            2 => Self::Additional1,
            3 => Self::Additional2,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumDocumentCharacter`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumDocumentCharacter {
    /// Wire value `86`.
    ZMO,
    /// Wire value `103`.
    ZWO,
    /// Wire value `87`.
    ZMW,
    /// Wire value `104`.
    ZWW,
    /// Wire value `58`.
    NAL,
    /// Wire value `59`.
    ZOB,
    /// Wire value `60`.
    KP,
    /// Wire value `61`.
    KW,
    /// Wire value `62`.
    BP,
    /// Wire value `63`.
    BW,
    /// Wire value `64`.
    IP,
    /// Wire value `65`.
    IW,
    /// Wire value `66`.
    TRPlus,
    /// Wire value `67`.
    TRMinus,
    /// Wire value `110`.
    RK,
    /// Wire value `68`.
    FVZ,
    /// Wire value `70`.
    RUZ,
    /// Wire value `72`.
    DIM,
    /// Wire value `97`.
    ZRZ,
    /// Wire value `99`.
    FVR,
    /// Wire value `106`.
    FVM,
    /// Wire value `115`.
    WNT,
    /// Wire value `138`.
    FWZ,
    /// Wire value `69`.
    FZK,
    /// Wire value `139`.
    FWZK,
    /// Wire value `71`.
    RKZ,
    /// Wire value `73`.
    DMK,
    /// Wire value `98`.
    ZRK,
    /// Wire value `100`.
    FRK,
    /// Wire value `107`.
    FMK,
    /// Wire value `121`.
    WNK,
    /// Wire value `46`.
    DEX,
    /// Wire value `50`.
    PAR,
    /// Wire value `79`.
    SRS,
    /// Wire value `42`.
    RUS,
    /// Wire value `126`.
    REX,
    /// Wire value `124`.
    FVSM,
    /// Wire value `40`.
    FVS,
    /// Wire value `101`.
    FVW,
    /// Wire value `134`.
    SKO,
    /// Wire value `150`.
    SKW,
    /// Wire value `114`.
    WDT,
    /// Wire value `154`.
    RO,
    /// Wire value `155`.
    RKO,
    /// Wire value `136`.
    SZA,
    /// Wire value `152`.
    SZW,
    /// Wire value `159`.
    NZZ,
    /// Wire value `160`.
    NZZK,
    /// Wire value `157`.
    NZW,
    /// Wire value `158`.
    NZWK,
    /// Wire value `45`.
    DXK,
    /// Wire value `51`.
    PRK,
    /// Wire value `80`.
    SRK,
    /// Wire value `43`.
    RKS,
    /// Wire value `127`.
    RXK,
    /// Wire value `125`.
    FVMK,
    /// Wire value `41`.
    FKS,
    /// Wire value `102`.
    FKW,
    /// Wire value `135`.
    SKK,
    /// Wire value `151`.
    SKWK,
    /// Wire value `118`.
    WKS,
    /// Wire value `120`.
    WDTK,
    /// Wire value `91`.
    PWM,
    /// Wire value `74`.
    PW,
    /// Wire value `76`.
    RW,
    /// Wire value `78`.
    WZ,
    /// Wire value `89`.
    PZ,
    /// Wire value `82`.
    MMPlus,
    /// Wire value `84`.
    MMMinus,
    /// Wire value `75`.
    PWK,
    /// Wire value `77`.
    RWK,
    /// Wire value `88`.
    WZK,
    /// Wire value `83`.
    MKPlus,
    /// Wire value `85`.
    MKMinus,
    /// Wire value `81`.
    PZK,
    /// Wire value `90`.
    KWM,
    /// Wire value `108`.
    FZS,
    /// Wire value `112`.
    ZKS,
    /// Wire value `128`.
    FZZ,
    /// Wire value `113`.
    ZKZ,
    /// Wire value `109`.
    KZS,
    /// Wire value `122`.
    ZSK,
    /// Wire value `111`.
    KZZ,
    /// Wire value `123`.
    ZZK,
    /// Wire value `116`.
    FWS,
    /// Wire value `117`.
    FWEZ,
    /// Wire value `119`.
    WKZ,
    /// Wire value `129`.
    WDP,
    /// Wire value `130`.
    WPK,
    /// Wire value `131`.
    PZP,
    /// Wire value `132`.
    PPK,
    /// Wire value `156`.
    LP,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumDocumentCharacter {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::ZMO => 86,
            Self::ZWO => 103,
            Self::ZMW => 87,
            Self::ZWW => 104,
            Self::NAL => 58,
            Self::ZOB => 59,
            Self::KP => 60,
            Self::KW => 61,
            Self::BP => 62,
            Self::BW => 63,
            Self::IP => 64,
            Self::IW => 65,
            Self::TRPlus => 66,
            Self::TRMinus => 67,
            Self::RK => 110,
            Self::FVZ => 68,
            Self::RUZ => 70,
            Self::DIM => 72,
            Self::ZRZ => 97,
            Self::FVR => 99,
            Self::FVM => 106,
            Self::WNT => 115,
            Self::FWZ => 138,
            Self::FZK => 69,
            Self::FWZK => 139,
            Self::RKZ => 71,
            Self::DMK => 73,
            Self::ZRK => 98,
            Self::FRK => 100,
            Self::FMK => 107,
            Self::WNK => 121,
            Self::DEX => 46,
            Self::PAR => 50,
            Self::SRS => 79,
            Self::RUS => 42,
            Self::REX => 126,
            Self::FVSM => 124,
            Self::FVS => 40,
            Self::FVW => 101,
            Self::SKO => 134,
            Self::SKW => 150,
            Self::WDT => 114,
            Self::RO => 154,
            Self::RKO => 155,
            Self::SZA => 136,
            Self::SZW => 152,
            Self::NZZ => 159,
            Self::NZZK => 160,
            Self::NZW => 157,
            Self::NZWK => 158,
            Self::DXK => 45,
            Self::PRK => 51,
            Self::SRK => 80,
            Self::RKS => 43,
            Self::RXK => 127,
            Self::FVMK => 125,
            Self::FKS => 41,
            Self::FKW => 102,
            Self::SKK => 135,
            Self::SKWK => 151,
            Self::WKS => 118,
            Self::WDTK => 120,
            Self::PWM => 91,
            Self::PW => 74,
            Self::RW => 76,
            Self::WZ => 78,
            Self::PZ => 89,
            Self::MMPlus => 82,
            Self::MMMinus => 84,
            Self::PWK => 75,
            Self::RWK => 77,
            Self::WZK => 88,
            Self::MKPlus => 83,
            Self::MKMinus => 85,
            Self::PZK => 81,
            Self::KWM => 90,
            Self::FZS => 108,
            Self::ZKS => 112,
            Self::FZZ => 128,
            Self::ZKZ => 113,
            Self::KZS => 109,
            Self::ZSK => 122,
            Self::KZZ => 111,
            Self::ZZK => 123,
            Self::FWS => 116,
            Self::FWEZ => 117,
            Self::WKZ => 119,
            Self::WDP => 129,
            Self::WPK => 130,
            Self::PZP => 131,
            Self::PPK => 132,
            Self::LP => 156,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumDocumentCharacter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumDocumentCharacter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            86 => Self::ZMO,
            103 => Self::ZWO,
            87 => Self::ZMW,
            104 => Self::ZWW,
            58 => Self::NAL,
            59 => Self::ZOB,
            60 => Self::KP,
            61 => Self::KW,
            62 => Self::BP,
            63 => Self::BW,
            64 => Self::IP,
            65 => Self::IW,
            66 => Self::TRPlus,
            67 => Self::TRMinus,
            110 => Self::RK,
            68 => Self::FVZ,
            70 => Self::RUZ,
            72 => Self::DIM,
            97 => Self::ZRZ,
            99 => Self::FVR,
            106 => Self::FVM,
            115 => Self::WNT,
            138 => Self::FWZ,
            69 => Self::FZK,
            139 => Self::FWZK,
            71 => Self::RKZ,
            73 => Self::DMK,
            98 => Self::ZRK,
            100 => Self::FRK,
            107 => Self::FMK,
            121 => Self::WNK,
            46 => Self::DEX,
            50 => Self::PAR,
            79 => Self::SRS,
            42 => Self::RUS,
            126 => Self::REX,
            124 => Self::FVSM,
            40 => Self::FVS,
            101 => Self::FVW,
            134 => Self::SKO,
            150 => Self::SKW,
            114 => Self::WDT,
            154 => Self::RO,
            155 => Self::RKO,
            136 => Self::SZA,
            152 => Self::SZW,
            159 => Self::NZZ,
            160 => Self::NZZK,
            157 => Self::NZW,
            158 => Self::NZWK,
            45 => Self::DXK,
            51 => Self::PRK,
            80 => Self::SRK,
            43 => Self::RKS,
            127 => Self::RXK,
            125 => Self::FVMK,
            41 => Self::FKS,
            102 => Self::FKW,
            135 => Self::SKK,
            151 => Self::SKWK,
            118 => Self::WKS,
            120 => Self::WDTK,
            91 => Self::PWM,
            74 => Self::PW,
            76 => Self::RW,
            78 => Self::WZ,
            89 => Self::PZ,
            82 => Self::MMPlus,
            84 => Self::MMMinus,
            75 => Self::PWK,
            77 => Self::RWK,
            88 => Self::WZK,
            83 => Self::MKPlus,
            85 => Self::MKMinus,
            81 => Self::PZK,
            90 => Self::KWM,
            108 => Self::FZS,
            112 => Self::ZKS,
            128 => Self::FZZ,
            113 => Self::ZKZ,
            109 => Self::KZS,
            122 => Self::ZSK,
            111 => Self::KZZ,
            123 => Self::ZZK,
            116 => Self::FWS,
            117 => Self::FWEZ,
            119 => Self::WKZ,
            129 => Self::WDP,
            130 => Self::WPK,
            131 => Self::PZP,
            132 => Self::PPK,
            156 => Self::LP,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumDocumentReservationType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumDocumentReservationType {
    /// Wire value `0`.
    None,
    /// Wire value `1`.
    Qunatity,
    /// Wire value `2`.
    IndicationDelivery,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumDocumentReservationType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Qunatity => 1,
            Self::IndicationDelivery => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumDocumentReservationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumDocumentReservationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::None,
            1 => Self::Qunatity,
            2 => Self::IndicationDelivery,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumDocumentSource`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumDocumentSource {
    /// Wire value `0`.
    AccountingBooks,
    /// Wire value `1`.
    Buffer,
    /// Wire value `2`.
    Schemes,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumDocumentSource {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::AccountingBooks => 0,
            Self::Buffer => 1,
            Self::Schemes => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumDocumentSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumDocumentSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::AccountingBooks,
            1 => Self::Buffer,
            2 => Self::Schemes,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumDocumentStatus`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumDocumentStatus {
    /// Wire value `0`.
    Other,
    /// Wire value `4`.
    New,
    /// Wire value `8`.
    Partial,
    /// Wire value `16`.
    Realized,
    /// Wire value `32`.
    Closed,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumDocumentStatus {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Other => 0,
            Self::New => 4,
            Self::Partial => 8,
            Self::Realized => 16,
            Self::Closed => 32,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumDocumentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumDocumentStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Other,
            4 => Self::New,
            8 => Self::Partial,
            16 => Self::Realized,
            32 => Self::Closed,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumFKDocumentCharacter`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumFKDocumentCharacter {
    /// Wire value `1`.
    DP,
    /// Wire value `2`.
    FVZ,
    /// Wire value `3`.
    FVS,
    /// Wire value `4`.
    RUZ,
    /// Wire value `5`.
    RUS,
    /// Wire value `6`.
    RK,
    /// Wire value `7`.
    RKOld,
    /// Wire value `8`.
    DEX,
    /// Wire value `9`.
    DIM,
    /// Wire value `10`.
    FKZ,
    /// Wire value `11`.
    FKS,
    /// Wire value `12`.
    RKZ,
    /// Wire value `13`.
    RKS,
    /// Wire value `14`.
    WB,
    /// Wire value `15`.
    RZL,
    /// Wire value `16`.
    FWN,
    /// Wire value `17`.
    WNT,
    /// Wire value `18`.
    WDT,
    /// Wire value `19`.
    PWN,
    /// Wire value `20`.
    FWV,
    /// Wire value `21`.
    RKW,
    /// Wire value `22`.
    FWZ,
    /// Wire value `23`.
    FWS,
    /// Wire value `24`.
    KFWZ,
    /// Wire value `25`.
    KFWS,
    /// Wire value `26`.
    DS,
    /// Wire value `27`.
    WBW,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumFKDocumentCharacter {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::DP => 1,
            Self::FVZ => 2,
            Self::FVS => 3,
            Self::RUZ => 4,
            Self::RUS => 5,
            Self::RK => 6,
            Self::RKOld => 7,
            Self::DEX => 8,
            Self::DIM => 9,
            Self::FKZ => 10,
            Self::FKS => 11,
            Self::RKZ => 12,
            Self::RKS => 13,
            Self::WB => 14,
            Self::RZL => 15,
            Self::FWN => 16,
            Self::WNT => 17,
            Self::WDT => 18,
            Self::PWN => 19,
            Self::FWV => 20,
            Self::RKW => 21,
            Self::FWZ => 22,
            Self::FWS => 23,
            Self::KFWZ => 24,
            Self::KFWS => 25,
            Self::DS => 26,
            Self::WBW => 27,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumFKDocumentCharacter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumFKDocumentCharacter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            1 => Self::DP,
            2 => Self::FVZ,
            3 => Self::FVS,
            4 => Self::RUZ,
            5 => Self::RUS,
            6 => Self::RK,
            7 => Self::RKOld,
            8 => Self::DEX,
            9 => Self::DIM,
            10 => Self::FKZ,
            11 => Self::FKS,
            12 => Self::RKZ,
            13 => Self::RKS,
            14 => Self::WB,
            15 => Self::RZL,
            16 => Self::FWN,
            17 => Self::WNT,
            18 => Self::WDT,
            19 => Self::PWN,
            20 => Self::FWV,
            21 => Self::RKW,
            22 => Self::FWZ,
            23 => Self::FWS,
            24 => Self::KFWZ,
            25 => Self::KFWS,
            26 => Self::DS,
            27 => Self::WBW,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumFKDocumentMessageType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumFKDocumentMessageType {
    /// Wire value `0`.
    Comments,
    /// Wire value `1`.
    Warning,
    /// Wire value `2`.
    Error,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumFKDocumentMessageType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Comments => 0,
            Self::Warning => 1,
            Self::Error => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumFKDocumentMessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumFKDocumentMessageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Comments,
            1 => Self::Warning,
            2 => Self::Error,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumFKSplitPaymentType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumFKSplitPaymentType {
    /// Wire value `0`.
    None,
    /// Wire value `1`.
    SplitPayment,
    /// Wire value `3`.
    VoluntarySplitPayment,
    /// Wire value `4`.
    TaxPayerRegister,
    /// Wire value `5`.
    TaxPayerRegisterSplitPayment,
    /// Wire value `7`.
    TaxPayerRegisterVoluntarySplitPayment,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumFKSplitPaymentType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::None => 0,
            Self::SplitPayment => 1,
            Self::VoluntarySplitPayment => 3,
            Self::TaxPayerRegister => 4,
            Self::TaxPayerRegisterSplitPayment => 5,
            Self::TaxPayerRegisterVoluntarySplitPayment => 7,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumFKSplitPaymentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumFKSplitPaymentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::None,
            1 => Self::SplitPayment,
            3 => Self::VoluntarySplitPayment,
            4 => Self::TaxPayerRegister,
            5 => Self::TaxPayerRegisterSplitPayment,
            7 => Self::TaxPayerRegisterVoluntarySplitPayment,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumFilterCriteriaMode`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumFilterCriteriaMode {
    /// Wire value `0`.
    Within,
    /// Wire value `1`.
    StartsWith,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumFilterCriteriaMode {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Within => 0,
            Self::StartsWith => 1,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumFilterCriteriaMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumFilterCriteriaMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Within,
            1 => Self::StartsWith,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumFilterSqlCriteriaMode`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumFilterSqlCriteriaMode {
    /// Wire value `0`.
    Within,
    /// Wire value `1`.
    Equals,
    /// Wire value `2`.
    Like,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumFilterSqlCriteriaMode {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Within => 0,
            Self::Equals => 1,
            Self::Like => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumFilterSqlCriteriaMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumFilterSqlCriteriaMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Within,
            1 => Self::Equals,
            2 => Self::Like,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumFiscalizationStatus`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumFiscalizationStatus {
    /// Wire value `0`.
    NonFiscal,
    /// Wire value `1`.
    ToFiscalization,
    /// Wire value `2`.
    Fiscializated,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumFiscalizationStatus {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::NonFiscal => 0,
            Self::ToFiscalization => 1,
            Self::Fiscializated => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumFiscalizationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumFiscalizationStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::NonFiscal,
            1 => Self::ToFiscalization,
            2 => Self::Fiscializated,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumIncrementalSyncModifyType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumIncrementalSyncModifyType {
    /// Wire value `1`.
    InsertOrUpdate,
    /// Wire value `2`.
    Delete,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumIncrementalSyncModifyType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::InsertOrUpdate => 1,
            Self::Delete => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumIncrementalSyncModifyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumIncrementalSyncModifyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            1 => Self::InsertOrUpdate,
            2 => Self::Delete,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumIndividualDiscountSubjectType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumIndividualDiscountSubjectType {
    /// Wire value `0`.
    Contractor,
    /// Wire value `1`.
    ContractorKind,
    /// Wire value `2`.
    Product,
    /// Wire value `3`.
    ProductKind,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumIndividualDiscountSubjectType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Contractor => 0,
            Self::ContractorKind => 1,
            Self::Product => 2,
            Self::ProductKind => 3,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumIndividualDiscountSubjectType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumIndividualDiscountSubjectType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Contractor,
            1 => Self::ContractorKind,
            2 => Self::Product,
            3 => Self::ProductKind,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumJPKDocumentKind`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumJPKDocumentKind {
    /// Wire value `0`.
    None,
    /// Wire value `1`.
    VAT,
    /// Wire value `2`.
    KOREKTA,
    /// Wire value `3`.
    ZAL,
    /// Wire value `21`.
    PZ,
    /// Wire value `22`.
    WZ,
    /// Wire value `23`.
    RW,
    /// Wire value `24`.
    MM,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumJPKDocumentKind {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::None => 0,
            Self::VAT => 1,
            Self::KOREKTA => 2,
            Self::ZAL => 3,
            Self::PZ => 21,
            Self::WZ => 22,
            Self::RW => 23,
            Self::MM => 24,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumJPKDocumentKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumJPKDocumentKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::None,
            1 => Self::VAT,
            2 => Self::KOREKTA,
            3 => Self::ZAL,
            21 => Self::PZ,
            22 => Self::WZ,
            23 => Self::RW,
            24 => Self::MM,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumJPK_V7DocumentAttribute`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumJPK_V7DocumentAttribute {
    /// Wire value `0`.
    None,
    /// Wire value `1`.
    RO,
    /// Wire value `2`.
    WEW,
    /// Wire value `4`.
    FP,
    /// Wire value `8`.
    MK,
    /// Wire value `16`.
    VATRR,
    /// Wire value `32`.
    SW,
    /// Wire value `64`.
    EE,
    /// Wire value `128`.
    TP,
    /// Wire value `256`.
    TTWNT,
    /// Wire value `512`.
    TTD,
    /// Wire value `1024`.
    MRT,
    /// Wire value `2048`.
    MRUZ,
    /// Wire value `4096`.
    I42,
    /// Wire value `8192`.
    I63,
    /// Wire value `16384`.
    BSPV,
    /// Wire value `32768`.
    BSPVDOSTAWA,
    /// Wire value `65536`.
    BMPVPROWIZJA,
    /// Wire value `131072`.
    MPP,
    /// Wire value `262144`.
    IMP,
    /// Wire value `524288`.
    IED,
    /// Wire value `1048576`.
    WSTOEE,
    /// Wire value `2097152`.
    BFK,
    /// Wire value `4194304`.
    DI,
    /// Wire value `8388608`.
    OFF,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumJPK_V7DocumentAttribute {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::None => 0,
            Self::RO => 1,
            Self::WEW => 2,
            Self::FP => 4,
            Self::MK => 8,
            Self::VATRR => 16,
            Self::SW => 32,
            Self::EE => 64,
            Self::TP => 128,
            Self::TTWNT => 256,
            Self::TTD => 512,
            Self::MRT => 1024,
            Self::MRUZ => 2048,
            Self::I42 => 4096,
            Self::I63 => 8192,
            Self::BSPV => 16384,
            Self::BSPVDOSTAWA => 32768,
            Self::BMPVPROWIZJA => 65536,
            Self::MPP => 131072,
            Self::IMP => 262144,
            Self::IED => 524288,
            Self::WSTOEE => 1048576,
            Self::BFK => 2097152,
            Self::DI => 4194304,
            Self::OFF => 8388608,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumJPK_V7DocumentAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumJPK_V7DocumentAttribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::None,
            1 => Self::RO,
            2 => Self::WEW,
            4 => Self::FP,
            8 => Self::MK,
            16 => Self::VATRR,
            32 => Self::SW,
            64 => Self::EE,
            128 => Self::TP,
            256 => Self::TTWNT,
            512 => Self::TTD,
            1024 => Self::MRT,
            2048 => Self::MRUZ,
            4096 => Self::I42,
            8192 => Self::I63,
            16384 => Self::BSPV,
            32768 => Self::BSPVDOSTAWA,
            65536 => Self::BMPVPROWIZJA,
            131072 => Self::MPP,
            262144 => Self::IMP,
            524288 => Self::IED,
            1048576 => Self::WSTOEE,
            2097152 => Self::BFK,
            4194304 => Self::DI,
            8388608 => Self::OFF,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumJPK_V7ProductGroup`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumJPK_V7ProductGroup {
    /// Wire value `0`.
    None,
    /// Wire value `1`.
    GTU01,
    /// Wire value `2`.
    GTU02,
    /// Wire value `4`.
    GTU03,
    /// Wire value `8`.
    GTU04,
    /// Wire value `16`.
    GTU05,
    /// Wire value `32`.
    GTU06,
    /// Wire value `64`.
    GTU07,
    /// Wire value `128`.
    GTU08,
    /// Wire value `256`.
    GTU09,
    /// Wire value `512`.
    GTU10,
    /// Wire value `1024`.
    GTU11,
    /// Wire value `2048`.
    GTU12,
    /// Wire value `4096`.
    GTU13,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumJPK_V7ProductGroup {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::None => 0,
            Self::GTU01 => 1,
            Self::GTU02 => 2,
            Self::GTU03 => 4,
            Self::GTU04 => 8,
            Self::GTU05 => 16,
            Self::GTU06 => 32,
            Self::GTU07 => 64,
            Self::GTU08 => 128,
            Self::GTU09 => 256,
            Self::GTU10 => 512,
            Self::GTU11 => 1024,
            Self::GTU12 => 2048,
            Self::GTU13 => 4096,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumJPK_V7ProductGroup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumJPK_V7ProductGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::None,
            1 => Self::GTU01,
            2 => Self::GTU02,
            4 => Self::GTU03,
            8 => Self::GTU04,
            16 => Self::GTU05,
            32 => Self::GTU06,
            64 => Self::GTU07,
            128 => Self::GTU08,
            256 => Self::GTU09,
            512 => Self::GTU10,
            1024 => Self::GTU11,
            2048 => Self::GTU12,
            4096 => Self::GTU13,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumKSeFInvoiceStatus`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumKSeFInvoiceStatus {
    /// Wire value `0`.
    NOTKSEFFORMAT,
    /// Wire value `10`.
    WAITINGTOBESENTTOKSEFMODULE,
    /// Wire value `20`.
    ERRORSENDINGTOKSEFMODULE,
    /// Wire value `25`.
    SENDING,
    /// Wire value `30`.
    NOTSENTTOKSEF,
    /// Wire value `40`.
    ERROR,
    /// Wire value `50`.
    PENDINGFORVALIDATIONBYKSEF,
    /// Wire value `60`.
    APPROVEDBYKSEF,
    /// Wire value `70`.
    REJECTEDBYKSEF,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumKSeFInvoiceStatus {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::NOTKSEFFORMAT => 0,
            Self::WAITINGTOBESENTTOKSEFMODULE => 10,
            Self::ERRORSENDINGTOKSEFMODULE => 20,
            Self::SENDING => 25,
            Self::NOTSENTTOKSEF => 30,
            Self::ERROR => 40,
            Self::PENDINGFORVALIDATIONBYKSEF => 50,
            Self::APPROVEDBYKSEF => 60,
            Self::REJECTEDBYKSEF => 70,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumKSeFInvoiceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumKSeFInvoiceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::NOTKSEFFORMAT,
            10 => Self::WAITINGTOBESENTTOKSEFMODULE,
            20 => Self::ERRORSENDINGTOKSEFMODULE,
            25 => Self::SENDING,
            30 => Self::NOTSENTTOKSEF,
            40 => Self::ERROR,
            50 => Self::PENDINGFORVALIDATIONBYKSEF,
            60 => Self::APPROVEDBYKSEF,
            70 => Self::REJECTEDBYKSEF,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumManualSettledState`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumManualSettledState {
    /// Wire value `0`.
    Automatic,
    /// Wire value `1`.
    ManualNotSettled,
    /// Wire value `2`.
    ManualSettled,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumManualSettledState {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Automatic => 0,
            Self::ManualNotSettled => 1,
            Self::ManualSettled => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumManualSettledState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumManualSettledState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Automatic,
            1 => Self::ManualNotSettled,
            2 => Self::ManualSettled,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumOrderByType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumOrderByType {
    /// Wire value `0`.
    Desc,
    /// Wire value `1`.
    Asc,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumOrderByType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Desc => 0,
            Self::Asc => 1,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumOrderByType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumOrderByType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Desc,
            1 => Self::Asc,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumOssDeliveryKind`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumOssDeliveryKind {
    /// Wire value `1`.
    Product,
    /// Wire value `2`.
    Service,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumOssDeliveryKind {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Product => 1,
            Self::Service => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumOssDeliveryKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumOssDeliveryKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            1 => Self::Product,
            2 => Self::Service,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumParallelType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumParallelType {
    /// Wire value `0`.
    Undefined,
    /// Wire value `1`.
    Parallel,
    /// Wire value `2`.
    StandardWithOnRequestParallel,
    /// Wire value `4`.
    StandardWithAutomaticParallel,
    /// Wire value `8`.
    ParallelOnDebit,
    /// Wire value `16`.
    ParallelOnCredit,
    /// Wire value `32`.
    OnRequestParallel,
    /// Wire value `64`.
    StuckParallel,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumParallelType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Undefined => 0,
            Self::Parallel => 1,
            Self::StandardWithOnRequestParallel => 2,
            Self::StandardWithAutomaticParallel => 4,
            Self::ParallelOnDebit => 8,
            Self::ParallelOnCredit => 16,
            Self::OnRequestParallel => 32,
            Self::StuckParallel => 64,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumParallelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumParallelType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Undefined,
            1 => Self::Parallel,
            2 => Self::StandardWithOnRequestParallel,
            4 => Self::StandardWithAutomaticParallel,
            8 => Self::ParallelOnDebit,
            16 => Self::ParallelOnCredit,
            32 => Self::OnRequestParallel,
            64 => Self::StuckParallel,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumPaymentRegistryType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumPaymentRegistryType {
    /// Wire value `0`.
    Cash,
    /// Wire value `1`.
    Bank,
    /// Wire value `2`.
    Other,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumPaymentRegistryType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Cash => 0,
            Self::Bank => 1,
            Self::Other => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumPaymentRegistryType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumPaymentRegistryType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Cash,
            1 => Self::Bank,
            2 => Self::Other,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumPaymentSubjectType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumPaymentSubjectType {
    /// Wire value `0`.
    Contractor,
    /// Wire value `104`.
    PaymentRegistry,
    /// Wire value `106`.
    Employee,
    /// Wire value `107`.
    Office,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumPaymentSubjectType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Contractor => 0,
            Self::PaymentRegistry => 104,
            Self::Employee => 106,
            Self::Office => 107,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumPaymentSubjectType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumPaymentSubjectType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Contractor,
            104 => Self::PaymentRegistry,
            106 => Self::Employee,
            107 => Self::Office,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumPaymentType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumPaymentType {
    /// Wire value `0`.
    Income,
    /// Wire value `1`.
    Expenditure,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumPaymentType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Income => 0,
            Self::Expenditure => 1,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumPaymentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumPaymentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Income,
            1 => Self::Expenditure,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumPhotoQuality`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumPhotoQuality {
    /// Wire value `0`.
    Original,
    /// Wire value `1`.
    Medium,
    /// Wire value `2`.
    Low,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumPhotoQuality {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Original => 0,
            Self::Medium => 1,
            Self::Low => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumPhotoQuality {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumPhotoQuality {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Original,
            1 => Self::Medium,
            2 => Self::Low,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumPriceCalculationMethod`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumPriceCalculationMethod {
    /// Wire value `0`.
    Handel,
    /// Wire value `1`.
    Algorithm,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumPriceCalculationMethod {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Handel => 0,
            Self::Algorithm => 1,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumPriceCalculationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumPriceCalculationMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Handel,
            1 => Self::Algorithm,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumPriceFactorType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumPriceFactorType {
    /// Wire value `0`.
    Discount,
    /// Wire value `1`.
    Price,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumPriceFactorType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Discount => 0,
            Self::Price => 1,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumPriceFactorType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumPriceFactorType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Discount,
            1 => Self::Price,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumPriceKind`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumPriceKind {
    /// Wire value `0`.
    Undefined,
    /// Wire value `1`.
    Gross,
    /// Wire value `2`.
    Net,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumPriceKind {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Undefined => 0,
            Self::Gross => 1,
            Self::Net => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumPriceKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumPriceKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Undefined,
            1 => Self::Gross,
            2 => Self::Net,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumPriceListConnectionType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumPriceListConnectionType {
    /// Wire value `1`.
    Individual,
    /// Wire value `2`.
    LinkedWithCardIndexAndIndividualDiscounts,
    /// Wire value `3`.
    LinkedWithCardIndexAndIndividualDiscountsAndOtherPriceLists,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumPriceListConnectionType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Individual => 1,
            Self::LinkedWithCardIndexAndIndividualDiscounts => 2,
            Self::LinkedWithCardIndexAndIndividualDiscountsAndOtherPriceLists => 3,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumPriceListConnectionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumPriceListConnectionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            1 => Self::Individual,
            2 => Self::LinkedWithCardIndexAndIndividualDiscounts,
            3 => Self::LinkedWithCardIndexAndIndividualDiscountsAndOtherPriceLists,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumPriceParameter`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumPriceParameter {
    /// Wire value `1`.
    Profit,
    /// Wire value `2`.
    Markup,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumPriceParameter {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Profit => 1,
            Self::Markup => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumPriceParameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumPriceParameter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            1 => Self::Profit,
            2 => Self::Markup,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumPriceRecalculateType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumPriceRecalculateType {
    /// Wire value `0`.
    PurchasePriceAndBasePriceAreNotAgreed,
    /// Wire value `1`.
    AutomaticallyAtEveryPurchasePriceChange,
    /// Wire value `2`.
    AutomaticallyWhenPurchasePriceIsHigherThanBasePrice,
    /// Wire value `3`.
    ManuallyAtEveryPurchasePriceChange,
    /// Wire value `4`.
    ManuallyWhenPurchasePriceIsHigherThanBasePrice,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumPriceRecalculateType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::PurchasePriceAndBasePriceAreNotAgreed => 0,
            Self::AutomaticallyAtEveryPurchasePriceChange => 1,
            Self::AutomaticallyWhenPurchasePriceIsHigherThanBasePrice => 2,
            Self::ManuallyAtEveryPurchasePriceChange => 3,
            Self::ManuallyWhenPurchasePriceIsHigherThanBasePrice => 4,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumPriceRecalculateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumPriceRecalculateType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::PurchasePriceAndBasePriceAreNotAgreed,
            1 => Self::AutomaticallyAtEveryPurchasePriceChange,
            2 => Self::AutomaticallyWhenPurchasePriceIsHigherThanBasePrice,
            3 => Self::ManuallyAtEveryPurchasePriceChange,
            4 => Self::ManuallyWhenPurchasePriceIsHigherThanBasePrice,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumPriceRounding`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumPriceRounding {
    /// Wire value `6`.
    Value00001,
    /// Wire value `5`.
    Value0001,
    /// Wire value `0`.
    Value001,
    /// Wire value `1`.
    Value010,
    /// Wire value `2`.
    Value100,
    /// Wire value `3`.
    Value1000,
    /// Wire value `4`.
    Value10000,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumPriceRounding {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Value00001 => 6,
            Self::Value0001 => 5,
            Self::Value001 => 0,
            Self::Value010 => 1,
            Self::Value100 => 2,
            Self::Value1000 => 3,
            Self::Value10000 => 4,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumPriceRounding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumPriceRounding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            6 => Self::Value00001,
            5 => Self::Value0001,
            0 => Self::Value001,
            1 => Self::Value010,
            2 => Self::Value100,
            3 => Self::Value1000,
            4 => Self::Value10000,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumPriceType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumPriceType {
    /// Wire value `0`.
    Undefined,
    /// Wire value `1`.
    PriceA,
    /// Wire value `2`.
    PriceB,
    /// Wire value `3`.
    PriceC,
    /// Wire value `4`.
    PriceD,
    /// Wire value `5`.
    PriceIndividual,
    /// Wire value `6`.
    PricePurchase,
    /// Wire value `7`.
    PricePurchaseInCurrency,
    /// Wire value `8`.
    PriceBase,
    /// Wire value `9`.
    Other,
    /// Wire value `10`.
    Duty,
    /// Wire value `11`.
    Excise,
    /// Wire value `12`.
    QuantitativeDiscount,
    /// Wire value `13`.
    SaleCurrencyExchangeRate,
    /// Wire value `14`.
    PurchaseCurrencyExchangeRate,
    /// Wire value `15`.
    ContractorDiscount,
    /// Wire value `16`.
    PriceE,
    /// Wire value `17`.
    PriceF,
    /// Wire value `18`.
    PriceG,
    /// Wire value `19`.
    PriceH,
    /// Wire value `20`.
    PriceI,
    /// Wire value `21`.
    PriceJ,
    /// Wire value `22`.
    PriceK,
    /// Wire value `23`.
    PriceL,
    /// Wire value `24`.
    PriceM,
    /// Wire value `25`.
    PriceN,
    /// Wire value `26`.
    PriceO,
    /// Wire value `27`.
    PriceP,
    /// Wire value `28`.
    PriceQ,
    /// Wire value `29`.
    PriceR,
    /// Wire value `30`.
    PriceS,
    /// Wire value `31`.
    PriceT,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumPriceType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Undefined => 0,
            Self::PriceA => 1,
            Self::PriceB => 2,
            Self::PriceC => 3,
            Self::PriceD => 4,
            Self::PriceIndividual => 5,
            Self::PricePurchase => 6,
            Self::PricePurchaseInCurrency => 7,
            Self::PriceBase => 8,
            Self::Other => 9,
            Self::Duty => 10,
            Self::Excise => 11,
            Self::QuantitativeDiscount => 12,
            Self::SaleCurrencyExchangeRate => 13,
            Self::PurchaseCurrencyExchangeRate => 14,
            Self::ContractorDiscount => 15,
            Self::PriceE => 16,
            Self::PriceF => 17,
            Self::PriceG => 18,
            Self::PriceH => 19,
            Self::PriceI => 20,
            Self::PriceJ => 21,
            Self::PriceK => 22,
            Self::PriceL => 23,
            Self::PriceM => 24,
            Self::PriceN => 25,
            Self::PriceO => 26,
            Self::PriceP => 27,
            Self::PriceQ => 28,
            Self::PriceR => 29,
            Self::PriceS => 30,
            Self::PriceT => 31,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumPriceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumPriceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Undefined,
            1 => Self::PriceA,
            2 => Self::PriceB,
            3 => Self::PriceC,
            4 => Self::PriceD,
            5 => Self::PriceIndividual,
            6 => Self::PricePurchase,
            7 => Self::PricePurchaseInCurrency,
            8 => Self::PriceBase,
            9 => Self::Other,
            10 => Self::Duty,
            11 => Self::Excise,
            12 => Self::QuantitativeDiscount,
            13 => Self::SaleCurrencyExchangeRate,
            14 => Self::PurchaseCurrencyExchangeRate,
            15 => Self::ContractorDiscount,
            16 => Self::PriceE,
            17 => Self::PriceF,
            18 => Self::PriceG,
            19 => Self::PriceH,
            20 => Self::PriceI,
            21 => Self::PriceJ,
            22 => Self::PriceK,
            23 => Self::PriceL,
            24 => Self::PriceM,
            25 => Self::PriceN,
            26 => Self::PriceO,
            27 => Self::PriceP,
            28 => Self::PriceQ,
            29 => Self::PriceR,
            30 => Self::PriceS,
            31 => Self::PriceT,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumPrintParameterDocumentStatus`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumPrintParameterDocumentStatus {
    /// Wire value `0`.
    Automatic,
    /// Wire value `1`.
    Original,
    /// Wire value `2`.
    Copy,
    /// Wire value `3`.
    OriginalCopy,
    /// Wire value `4`.
    Empty,
    /// Wire value `5`.
    Custom,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumPrintParameterDocumentStatus {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Automatic => 0,
            Self::Original => 1,
            Self::Copy => 2,
            Self::OriginalCopy => 3,
            Self::Empty => 4,
            Self::Custom => 5,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumPrintParameterDocumentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumPrintParameterDocumentStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Automatic,
            1 => Self::Original,
            2 => Self::Copy,
            3 => Self::OriginalCopy,
            4 => Self::Empty,
            5 => Self::Custom,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumPrintParameterNotSettledDocuments`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumPrintParameterNotSettledDocuments {
    /// Wire value `0`.
    DoNotPrint,
    /// Wire value `1`.
    PrintForIssueDate,
    /// Wire value `2`.
    PrintForPrintDate,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumPrintParameterNotSettledDocuments {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::DoNotPrint => 0,
            Self::PrintForIssueDate => 1,
            Self::PrintForPrintDate => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumPrintParameterNotSettledDocuments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumPrintParameterNotSettledDocuments {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::DoNotPrint,
            1 => Self::PrintForIssueDate,
            2 => Self::PrintForPrintDate,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumPrintParameterOperationDateFormat`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumPrintParameterOperationDateFormat {
    /// Wire value `0`.
    OperationDate,
    /// Wire value `1`.
    OperationPeriod,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumPrintParameterOperationDateFormat {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::OperationDate => 0,
            Self::OperationPeriod => 1,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumPrintParameterOperationDateFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumPrintParameterOperationDateFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::OperationDate,
            1 => Self::OperationPeriod,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumProductType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumProductType {
    /// Wire value `0`.
    Article,
    /// Wire value `1`.
    Service,
    /// Wire value `2`.
    Kit,
    /// Wire value `3`.
    Set,
    /// Wire value `4`.
    ServiceProfit,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumProductType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Article => 0,
            Self::Service => 1,
            Self::Kit => 2,
            Self::Set => 3,
            Self::ServiceProfit => 4,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumProductType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Article,
            1 => Self::Service,
            2 => Self::Kit,
            3 => Self::Set,
            4 => Self::ServiceProfit,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumRDFStatus`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumRDFStatus {
    /// Wire value `0`.
    NoInvoice,
    /// Wire value `1`.
    InvoiceNotPrinted,
    /// Wire value `2`.
    InvoicePrinted,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumRDFStatus {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::NoInvoice => 0,
            Self::InvoiceNotPrinted => 1,
            Self::InvoicePrinted => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumRDFStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumRDFStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::NoInvoice,
            1 => Self::InvoiceNotPrinted,
            2 => Self::InvoicePrinted,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumRecordDecsriptionKind`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumRecordDecsriptionKind {
    /// Wire value `0`.
    FromDocument,
    /// Wire value `1`.
    FromRecord,
    /// Wire value `2`.
    FromFirstRecord,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumRecordDecsriptionKind {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::FromDocument => 0,
            Self::FromRecord => 1,
            Self::FromFirstRecord => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumRecordDecsriptionKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumRecordDecsriptionKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::FromDocument,
            1 => Self::FromRecord,
            2 => Self::FromFirstRecord,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumRepXPrintLocation`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumRepXPrintLocation {
    /// Wire value `1`.
    Document,
    /// Wire value `2`.
    CardIndex,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumRepXPrintLocation {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Document => 1,
            Self::CardIndex => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumRepXPrintLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumRepXPrintLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            1 => Self::Document,
            2 => Self::CardIndex,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumRepXReportText`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumRepXReportText {
    /// Wire value `0`.
    Empty,
    /// Wire value `1`.
    Original,
    /// Wire value `2`.
    Copy,
    /// Wire value `3`.
    OriginalCopy,
    /// Wire value `4`.
    Duplicate,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumRepXReportText {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Empty => 0,
            Self::Original => 1,
            Self::Copy => 2,
            Self::OriginalCopy => 3,
            Self::Duplicate => 4,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumRepXReportText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumRepXReportText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Empty,
            1 => Self::Original,
            2 => Self::Copy,
            3 => Self::OriginalCopy,
            4 => Self::Duplicate,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumReservationDocumentType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumReservationDocumentType {
    /// Wire value `0`.
    Unknown,
    /// Wire value `16`.
    SaleDocument,
    /// Wire value `33`.
    WarehouseDocument,
    /// Wire value `45`.
    ForeignOrder,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumReservationDocumentType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Unknown => 0,
            Self::SaleDocument => 16,
            Self::WarehouseDocument => 33,
            Self::ForeignOrder => 45,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumReservationDocumentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumReservationDocumentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Unknown,
            16 => Self::SaleDocument,
            33 => Self::WarehouseDocument,
            45 => Self::ForeignOrder,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumReservationKind`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumReservationKind {
    /// Wire value `0`.
    FIFOIndicationDelivery,
    /// Wire value `1`.
    FIFOQuantity,
    /// Wire value `2`.
    LIFOIndicationDelivery,
    /// Wire value `3`.
    LIFOQuantity,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumReservationKind {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::FIFOIndicationDelivery => 0,
            Self::FIFOQuantity => 1,
            Self::LIFOIndicationDelivery => 2,
            Self::LIFOQuantity => 3,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumReservationKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumReservationKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::FIFOIndicationDelivery,
            1 => Self::FIFOQuantity,
            2 => Self::LIFOIndicationDelivery,
            3 => Self::LIFOQuantity,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumReservationMode`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumReservationMode {
    /// Wire value `1`.
    MadeByUser,
    /// Wire value `2`.
    MadeByProgram,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumReservationMode {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::MadeByUser => 1,
            Self::MadeByProgram => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumReservationMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumReservationMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            1 => Self::MadeByUser,
            2 => Self::MadeByProgram,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumReservationPositionType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumReservationPositionType {
    /// Wire value `0`.
    Unknown,
    /// Wire value `18`.
    SaleDocument,
    /// Wire value `37`.
    WarehouseDocument,
    /// Wire value `30`.
    ForeignOrder,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumReservationPositionType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Unknown => 0,
            Self::SaleDocument => 18,
            Self::WarehouseDocument => 37,
            Self::ForeignOrder => 30,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumReservationPositionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumReservationPositionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Unknown,
            18 => Self::SaleDocument,
            37 => Self::WarehouseDocument,
            30 => Self::ForeignOrder,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumReservationType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumReservationType {
    /// Wire value `1`.
    Manual,
    /// Wire value `2`.
    Automatic,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumReservationType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Manual => 1,
            Self::Automatic => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumReservationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumReservationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            1 => Self::Manual,
            2 => Self::Automatic,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumSalePriceType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumSalePriceType {
    /// Wire value `0`.
    Undefined,
    /// Wire value `1`.
    PriceA,
    /// Wire value `2`.
    PriceB,
    /// Wire value `3`.
    PriceC,
    /// Wire value `4`.
    PriceD,
    /// Wire value `16`.
    PriceE,
    /// Wire value `17`.
    PriceF,
    /// Wire value `18`.
    PriceG,
    /// Wire value `19`.
    PriceH,
    /// Wire value `20`.
    PriceI,
    /// Wire value `21`.
    PriceJ,
    /// Wire value `22`.
    PriceK,
    /// Wire value `23`.
    PriceL,
    /// Wire value `24`.
    PriceM,
    /// Wire value `25`.
    PriceN,
    /// Wire value `26`.
    PriceO,
    /// Wire value `27`.
    PriceP,
    /// Wire value `28`.
    PriceQ,
    /// Wire value `29`.
    PriceR,
    /// Wire value `30`.
    PriceS,
    /// Wire value `31`.
    PriceT,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumSalePriceType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Undefined => 0,
            Self::PriceA => 1,
            Self::PriceB => 2,
            Self::PriceC => 3,
            Self::PriceD => 4,
            Self::PriceE => 16,
            Self::PriceF => 17,
            Self::PriceG => 18,
            Self::PriceH => 19,
            Self::PriceI => 20,
            Self::PriceJ => 21,
            Self::PriceK => 22,
            Self::PriceL => 23,
            Self::PriceM => 24,
            Self::PriceN => 25,
            Self::PriceO => 26,
            Self::PriceP => 27,
            Self::PriceQ => 28,
            Self::PriceR => 29,
            Self::PriceS => 30,
            Self::PriceT => 31,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumSalePriceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumSalePriceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Undefined,
            1 => Self::PriceA,
            2 => Self::PriceB,
            3 => Self::PriceC,
            4 => Self::PriceD,
            16 => Self::PriceE,
            17 => Self::PriceF,
            18 => Self::PriceG,
            19 => Self::PriceH,
            20 => Self::PriceI,
            21 => Self::PriceJ,
            22 => Self::PriceK,
            23 => Self::PriceL,
            24 => Self::PriceM,
            25 => Self::PriceN,
            26 => Self::PriceO,
            27 => Self::PriceP,
            28 => Self::PriceQ,
            29 => Self::PriceR,
            30 => Self::PriceS,
            31 => Self::PriceT,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumSettlementMethod`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumSettlementMethod {
    /// Wire value `0`.
    FIFO,
    /// Wire value `1`.
    LIFO,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumSettlementMethod {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::FIFO => 0,
            Self::LIFO => 1,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumSettlementMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumSettlementMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::FIFO,
            1 => Self::LIFO,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumSettlementType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumSettlementType {
    /// Wire value `0`.
    Due,
    /// Wire value `1`.
    Obligation,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumSettlementType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Due => 0,
            Self::Obligation => 1,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumSettlementType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumSettlementType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Due,
            1 => Self::Obligation,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumSideType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumSideType {
    /// Wire value `0`.
    Debit,
    /// Wire value `1`.
    Credit,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumSideType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Debit => 0,
            Self::Credit => 1,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumSideType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumSideType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Debit,
            1 => Self::Credit,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumSplitOpposingAccountType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumSplitOpposingAccountType {
    /// Wire value `99`.
    DebitSplitInCreaditWithMultipleOpposingAccounts,
    /// Wire value `101`.
    DebitSplitInDebitWithSingleOpposingAccount,
    /// Wire value `103`.
    DebitWithoutSplitWithSingleOpposingAccount,
    /// Wire value `199`.
    CreditSplitInDebitWithMultipleOpposingAccounts,
    /// Wire value `201`.
    CreditSplitInCreditWithSingleOpposingAccount,
    /// Wire value `203`.
    CreditWithoutSplitWithSingleOpposingAccount,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumSplitOpposingAccountType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::DebitSplitInCreaditWithMultipleOpposingAccounts => 99,
            Self::DebitSplitInDebitWithSingleOpposingAccount => 101,
            Self::DebitWithoutSplitWithSingleOpposingAccount => 103,
            Self::CreditSplitInDebitWithMultipleOpposingAccounts => 199,
            Self::CreditSplitInCreditWithSingleOpposingAccount => 201,
            Self::CreditWithoutSplitWithSingleOpposingAccount => 203,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumSplitOpposingAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumSplitOpposingAccountType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            99 => Self::DebitSplitInCreaditWithMultipleOpposingAccounts,
            101 => Self::DebitSplitInDebitWithSingleOpposingAccount,
            103 => Self::DebitWithoutSplitWithSingleOpposingAccount,
            199 => Self::CreditSplitInDebitWithMultipleOpposingAccounts,
            201 => Self::CreditSplitInCreditWithSingleOpposingAccount,
            203 => Self::CreditWithoutSplitWithSingleOpposingAccount,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumSplitType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumSplitType {
    /// Wire value `0`.
    None,
    /// Wire value `1`.
    SplitInDebit,
    /// Wire value `2`.
    SplitInCredit,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumSplitType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::None => 0,
            Self::SplitInDebit => 1,
            Self::SplitInCredit => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumSplitType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumSplitType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::None,
            1 => Self::SplitInDebit,
            2 => Self::SplitInCredit,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumSplitValueType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumSplitValueType {
    /// Wire value `0`.
    Value,
    /// Wire value `1`.
    Percent,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumSplitValueType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Value => 0,
            Self::Percent => 1,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumSplitValueType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumSplitValueType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Value,
            1 => Self::Percent,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumSubjectType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumSubjectType {
    /// Wire value `-1`.
    EmptyNull,
    /// Wire value `0`.
    Other,
    /// Wire value `1`.
    Employees,
    /// Wire value `2`.
    Contractors,
    /// Wire value `3`.
    Accounts,
    /// Wire value `4`.
    Offices,
    /// Wire value `5`.
    Currencies,
    /// Wire value `8`.
    IncidentalContractors,
    /// Wire value `9`.
    Countries,
    /// Wire value `99`.
    Dictionaries,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumSubjectType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::EmptyNull => -1,
            Self::Other => 0,
            Self::Employees => 1,
            Self::Contractors => 2,
            Self::Accounts => 3,
            Self::Offices => 4,
            Self::Currencies => 5,
            Self::IncidentalContractors => 8,
            Self::Countries => 9,
            Self::Dictionaries => 99,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumSubjectType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumSubjectType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            -1 => Self::EmptyNull,
            0 => Self::Other,
            1 => Self::Employees,
            2 => Self::Contractors,
            3 => Self::Accounts,
            4 => Self::Offices,
            5 => Self::Currencies,
            8 => Self::IncidentalContractors,
            9 => Self::Countries,
            99 => Self::Dictionaries,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumTransactionInterestType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumTransactionInterestType {
    /// Wire value `0`.
    None,
    /// Wire value `1`.
    Statutory,
    /// Wire value `2`.
    FixedRate,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumTransactionInterestType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Statutory => 1,
            Self::FixedRate => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumTransactionInterestType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumTransactionInterestType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::None,
            1 => Self::Statutory,
            2 => Self::FixedRate,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumVatRateType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumVatRateType {
    /// Wire value `-1`.
    Unknown,
    /// Wire value `0`.
    Other,
    /// Wire value `3`.
    Zero,
    /// Wire value `4`.
    Exempt,
    /// Wire value `5`.
    NoSubjectTo,
    /// Wire value `10`.
    Basic,
    /// Wire value `11`.
    Reduced11,
    /// Wire value `12`.
    Reduced12,
    /// Wire value `13`.
    SuperReduced,
    /// Wire value `14`.
    Intermediate,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumVatRateType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Unknown => -1,
            Self::Other => 0,
            Self::Zero => 3,
            Self::Exempt => 4,
            Self::NoSubjectTo => 5,
            Self::Basic => 10,
            Self::Reduced11 => 11,
            Self::Reduced12 => 12,
            Self::SuperReduced => 13,
            Self::Intermediate => 14,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumVatRateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumVatRateType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            -1 => Self::Unknown,
            0 => Self::Other,
            3 => Self::Zero,
            4 => Self::Exempt,
            5 => Self::NoSubjectTo,
            10 => Self::Basic,
            11 => Self::Reduced11,
            12 => Self::Reduced12,
            13 => Self::SuperReduced,
            14 => Self::Intermediate,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumVatRegisterKind`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumVatRegisterKind {
    /// Wire value `100`.
    PurchaseStandard,
    /// Wire value `101`.
    PurchaseFixedAssets,
    /// Wire value `102`.
    PurchaseSpecial1,
    /// Wire value `103`.
    PurchaseSpecial2,
    /// Wire value `104`.
    PurchaseNonDeductibleVat,
    /// Wire value `106`.
    PurchaseLowerVat,
    /// Wire value `107`.
    PurchaseHigherVat,
    /// Wire value `200`.
    SaleStandard,
    /// Wire value `210`.
    SaleWithCustomizablePeriod,
    /// Wire value `203`.
    SaleProductDeliveryWhereBuyerIsTaxPayer,
    /// Wire value `204`.
    SaleNonDeductibleVat,
    /// Wire value `205`.
    SaleProductDeliveryAndServiceOutsideOfCountry,
    /// Wire value `206`.
    SaleLowerVat,
    /// Wire value `207`.
    SaleHigherVat,
    /// Wire value `300`.
    ImportStandard,
    /// Wire value `301`.
    ImportFixedAssets,
    /// Wire value `302`.
    ImportService,
    /// Wire value `303`.
    ImportProductDeliveryWhereBuyerIsTaxPayer,
    /// Wire value `304`.
    ImportProductDeliveryWithSimplifiedCustomProcedure,
    /// Wire value `400`.
    ExportStandard,
    /// Wire value `404`.
    ExportNonDeductibleVat,
    /// Wire value `405`.
    ExportProductDeliveryAndServiceOutsideOfCountry,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumVatRegisterKind {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::PurchaseStandard => 100,
            Self::PurchaseFixedAssets => 101,
            Self::PurchaseSpecial1 => 102,
            Self::PurchaseSpecial2 => 103,
            Self::PurchaseNonDeductibleVat => 104,
            Self::PurchaseLowerVat => 106,
            Self::PurchaseHigherVat => 107,
            Self::SaleStandard => 200,
            Self::SaleWithCustomizablePeriod => 210,
            Self::SaleProductDeliveryWhereBuyerIsTaxPayer => 203,
            Self::SaleNonDeductibleVat => 204,
            Self::SaleProductDeliveryAndServiceOutsideOfCountry => 205,
            Self::SaleLowerVat => 206,
            Self::SaleHigherVat => 207,
            Self::ImportStandard => 300,
            Self::ImportFixedAssets => 301,
            Self::ImportService => 302,
            Self::ImportProductDeliveryWhereBuyerIsTaxPayer => 303,
            Self::ImportProductDeliveryWithSimplifiedCustomProcedure => 304,
            Self::ExportStandard => 400,
            Self::ExportNonDeductibleVat => 404,
            Self::ExportProductDeliveryAndServiceOutsideOfCountry => 405,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumVatRegisterKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumVatRegisterKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            100 => Self::PurchaseStandard,
            101 => Self::PurchaseFixedAssets,
            102 => Self::PurchaseSpecial1,
            103 => Self::PurchaseSpecial2,
            104 => Self::PurchaseNonDeductibleVat,
            106 => Self::PurchaseLowerVat,
            107 => Self::PurchaseHigherVat,
            200 => Self::SaleStandard,
            210 => Self::SaleWithCustomizablePeriod,
            203 => Self::SaleProductDeliveryWhereBuyerIsTaxPayer,
            204 => Self::SaleNonDeductibleVat,
            205 => Self::SaleProductDeliveryAndServiceOutsideOfCountry,
            206 => Self::SaleLowerVat,
            207 => Self::SaleHigherVat,
            300 => Self::ImportStandard,
            301 => Self::ImportFixedAssets,
            302 => Self::ImportService,
            303 => Self::ImportProductDeliveryWhereBuyerIsTaxPayer,
            304 => Self::ImportProductDeliveryWithSimplifiedCustomProcedure,
            400 => Self::ExportStandard,
            404 => Self::ExportNonDeductibleVat,
            405 => Self::ExportProductDeliveryAndServiceOutsideOfCountry,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumVatRegisterPeriodType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumVatRegisterPeriodType {
    /// Wire value `0`.
    SpecificPeriod,
    /// Wire value `1`.
    Pending,
    /// Wire value `2`.
    PeriodSetByPayment,
    /// Wire value `3`.
    PeriodSetByMaturityTerm,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumVatRegisterPeriodType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::SpecificPeriod => 0,
            Self::Pending => 1,
            Self::PeriodSetByPayment => 2,
            Self::PeriodSetByMaturityTerm => 3,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumVatRegisterPeriodType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumVatRegisterPeriodType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::SpecificPeriod,
            1 => Self::Pending,
            2 => Self::PeriodSetByPayment,
            3 => Self::PeriodSetByMaturityTerm,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumVatRegisterType`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumVatRegisterType {
    /// Wire value `1`.
    Purchase,
    /// Wire value `2`.
    Sale,
    /// Wire value `3`.
    Import,
    /// Wire value `4`.
    Export,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumVatRegisterType {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Purchase => 1,
            Self::Sale => 2,
            Self::Import => 3,
            Self::Export => 4,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumVatRegisterType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumVatRegisterType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            1 => Self::Purchase,
            2 => Self::Sale,
            3 => Self::Import,
            4 => Self::Export,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumVatRegisterTypeABCD`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumVatRegisterTypeABCD {
    /// Wire value `1`.
    A,
    /// Wire value `2`.
    B,
    /// Wire value `3`.
    C,
    /// Wire value `4`.
    D,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumVatRegisterTypeABCD {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::A => 1,
            Self::B => 2,
            Self::C => 3,
            Self::D => 4,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumVatRegisterTypeABCD {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumVatRegisterTypeABCD {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            1 => Self::A,
            2 => Self::B,
            3 => Self::C,
            4 => Self::D,
            other => Self::UnknownValue(other),
        })
    }
}

/// WebAPI enum `enumYearClosingStatus`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum enumYearClosingStatus {
    /// Wire value `0`.
    Active,
    /// Wire value `1`.
    Closed,
    /// Wire value `2`.
    Aprroved,
    /// Value returned by the server that is unknown to this client version.
    UnknownValue(i32),
}

impl enumYearClosingStatus {
    pub fn to_wire_value(self) -> i32 {
        match self {
            Self::Active => 0,
            Self::Closed => 1,
            Self::Aprroved => 2,
            Self::UnknownValue(value) => value,
        }
    }
}

impl Serialize for enumYearClosingStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_wire_value())
    }
}

impl<'de> Deserialize<'de> for enumYearClosingStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Active,
            1 => Self::Closed,
            2 => Self::Aprroved,
            other => Self::UnknownValue(other),
        })
    }
}
