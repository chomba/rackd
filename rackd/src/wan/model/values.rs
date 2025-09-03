use std::{fmt::Display, net::Ipv6Addr};
use macaddr::MacAddr6;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::{net::{Ipv6HostAddr, Ipv6Prefix}, util::models::Id};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, ToSchema)]
pub struct WanId(pub Id);

impl WanId {
    pub fn new() -> Self {
        Self(Id::new())
    }
}

impl Display for WanId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "wan with id: {}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, ToSchema)]
pub enum WanMode {
    IPoE,
    PPPoE
}

impl Default for WanMode {
    fn default() -> Self {
        Self::IPoE
    }
}

#[derive(Debug)]
pub struct ISPL2Conf {
    pub mac: Option<MacAddr6>,
    // ONTConf
    // 802.1x Auth
}

// #[derive(Debug)]
// pub enum WanStatus {
//     EnabledOnNode(Id),
//     Disabled
// }

// impl Default for WanStatus {
//     fn default() -> Self {
//         Self::Disabled
//     }
// }

// /// ISP with Static IP Address Assigment
// /// - **ipv4**: Most ISPs will only provide a single IPv4 address for configuration on the CPE device (address, subnet mask, gateway)
// /// - **ipv6**: According to RFCXXX CPEs using static configuration for IPv6 should use SLACC
// ///             to configure the CPE's WAN interface and the Delegated Prefix should be statically configured on the PD daemon. 


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WanPPPoE {
    pub username: String,
    pub password: String
}

// #[derive(Debug, Serialize, Deserialize, Clone, Default)]
// pub struct WanIp {
//     pub ipv6: WanIpv6,
//     pub ipv4: WanIpv4, //optional
//     // pub dhcp4: Dhcp4Conf,
//     // ipv4_mtu
//     // ipv6_mtu
// }

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "mode", content = "host")]
#[serde(rename_all = "snake_case")] 
pub enum WanIpv6 {
    Auto,   // DHCPv6 client Enabled +  Follow RA
    Static(Ipv6Host)
}

impl Default for WanIpv6 {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
pub struct Ipv6Host {
    pub addr: Ipv6HostAddr,
    pub gateway: Ipv6Addr
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub struct WanDhcp6 {
    pub duid: Dhcp6Duid,
    pub iana: Dhcp6Iana,
    pub iapd: Dhcp6Iapd
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Dhcp6Iana {
    pub iaid: u32,
    pub valid_lt: u32,
    pub preferred_lt: u32
}

impl Default for Dhcp6Iana {
    fn default() -> Self {
        Self {
            iaid: 0,
            valid_lt: 2000,
            preferred_lt: 1500 
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Dhcp6Iapd {
    pub iaid: u32,
    pub prefix_hint: Ipv6Prefix,
    pub valid_lt: u32,
    pub preferred_lt: u32
}

impl Default for Dhcp6Iapd {
    fn default() -> Self {
        Self {
            iaid: 0,
            prefix_hint: Ipv6Prefix::default(),
            valid_lt: 0,
            preferred_lt: 0
        }
    }
}

/// Based on https://datatracker.ietf.org/doc/html/rfc3315#section-9
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Dhcp6Duid {
    LLT(DuidLLT),   // DUID-LLT (Type 1: Link-Layer Address + Time)
    AutoLLT,        // Use L2's MAC Address as the LLA + rackd installation time as the Time
    EN(DuidEN),     // DUID-EN (Type 2: Vendor Enterprise Number + Vendor Assigned ID)
    AutoEN,         // Use 43793 as the Vendor and the Rack's ID as the Vendor Assigned ID
    LL(DuidLL),     // DUID-LL (Type 3: Link-Layer Address)     
    AutoLL,         // Use L2's MAC Address as the LLA
    Raw(u128)
}

impl Default for Dhcp6Duid {
    fn default() -> Self {
        Self::AutoEN
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct DuidLLT {
    hw_type: u16, // Set it to 1 for ethernet: https://www.iana.org/assignments/arp-parameters/arp-parameters.xhtml
    time: u32,
    address: u128
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct DuidEN {
    pub pen: u16, // Private Enterprise Number
    pub id: u128 // Vendor Assigned ID
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct DuidLL {
    hw_type: u16, // Set it to 1 for ethernet
    address: u128
}

// #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
// pub struct Ipv6Host {
//     pub address: Ipv6Addr,
//     pub prefix_len: u8
// }

pub mod casts {
    use serde_json::Value;
    use thiserror::Error;
    use crate::util::models::{casts::IdError, Id};
    use super::{WanId, WanMode};

    impl From<WanId> for Id {
        fn from(value: WanId) -> Self {
            value.0
        }
    }
    
    #[derive(Debug, Error)]
    #[error("WanIdError: {:?}", .0)]
    pub struct WanIdError(#[from]IdError);

    impl TryFrom<Value> for WanId {
        type Error = WanIdError;

        fn try_from(value: Value) -> Result<Self, Self::Error> {
            Ok(Self(Id::try_from(value)?))
        }
    }       
    
    #[derive(Debug, Error)]
    pub enum WanModeError {
        #[error("Value is not a String [{}]", .0)]
        InvalidType(Value),
        #[error("Option is not valid [{}]", .0)]
        InvalidOption(String),
        #[error("No value provided")]
        MissingValue
    }

    impl TryFrom<Value> for WanMode {
        type Error = WanModeError;

        fn try_from(value: Value) -> Result<Self, Self::Error> {
            match value {
                Value::String(s) => match s.to_lowercase().as_str() {
                    "ipoe" => Ok(WanMode::IPoE),
                    "pppoe" => Ok(WanMode::PPPoE),
                    _ => Err(WanModeError::InvalidOption(s))
                },
                Value::Null => Err(WanModeError::MissingValue),
                _ => Err(WanModeError::InvalidType(value))
            }
        }
    }
}

pub mod api {
    use crate::util::api::Error;
    use super::casts::{WanIdError, WanModeError};

    impl From<WanIdError> for Error {
        fn from(error: WanIdError) -> Self {
            Error::new("WAN_ID_ERROR", error.to_string())
        }
    }

    impl From<WanModeError> for Error {
        fn from(error: WanModeError) -> Self {
            Error::new("WAN_MODE_ERROR", error.to_string())
        }
    }
}

pub mod sqlite {
    use rusqlite::{types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, Error, Result, ToSql};
    use super::*;

    impl ToSql for WanId {
        fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
            self.0.to_sql()
        }
    }
    
    impl FromSql for WanId {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            Ok(Self(Id::column_result(value)?))
        }
    }

    impl ToSql for WanMode {
        fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
            let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
            Ok(json.into())
        }
    }

    impl FromSql for WanMode {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            let value: Self = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
            Ok(value)
        }
    }
}