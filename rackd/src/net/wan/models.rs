pub mod casts;
use std::{fmt::Display, net::{Ipv4Addr, Ipv6Addr}};
use macaddr::MacAddr6;
use serde::{Serialize, Deserialize};
use crate::{net::shared::models::*, org::rack::models::RackId, util::models::Id};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WanEvent {
    Created(Created),
    Renamed(Renamed),
    MacSpoofed(MacSpoofed),
    MacUnspoofed(MacUnspoofed),
    Ipv6SetToRA(Ipv6SetToRA),
    Ipv6SetToStatic(Ipv6SetToStatic),
    Ipv6Disabled(Ipv6Disabled),
    Ipv4SetToDHCP(Ipv4SetToDHCP),
    Ipv4SetToStatic(Ipv4SetToStatic),
    Ipv4Disabled(Ipv4Disabled),

    // DHCPv6 IA for Prefix Delegation
    Dhcp6IAPDAdded { iaid: u32, prefix_hint: Ipv6Prefix, preferred_lt: u32, valid_lt: u32 },
    Dhcp6IAPDPrefixHintUpdated { iaid: u32, prefix_hint: Ipv6Prefix },
    Dhcp6IAPDPreferredLTUpdated { iaid: u32, preferred_lt: u32 },
    Dhcp6IAPDValidLTUpdated { iaid: u32, valid_lt: u32 },
    // DHCPv6 IA for Non-temporary Address
    Dhcp6IANAAdded { iaid: u32, preferred_lt: u32, valid_lt: u32 },
    // DHCPv6 DUID
    Dhcp6DuidSwitchedToAutoEN,
    Dhcp6DuidSwitchedToEN { pen: u16, id: u128 },
    Dhcp6DuidSwitchedToAutoLL,
    Dhcp6DuidSwitchedToLL { hw_type: u16, mac: MacAddr6 },
    Dhcp6DuidSwitchedToAutoLLT,
    Dhcp6DuidSwitchedToLLT { hw_type: u16, mac: MacAddr6, time: u32 },
    Dhcp6DuidSwitchedToRaw { value: u128 },
    // These 2 events should be processed by the Stats Collector and displayed by Graphana
//     LinkWentUp { up: LinkUp }, 
//     LinkWentDown { down: LinkDown },
}

// Requests must implement EventSource

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Created {
    pub id: WanId, 
    pub rack: RackId, 
    pub trunk: TrunkId, 
    pub vlan: VlanId, 
    pub conn: WanConnection, 
    pub name: NetName
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Renamed {
    pub name: NetName
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MacSpoofed {
    pub mac: MacAddr6
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct MacUnspoofed;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Ipv6SetToRA {
    pub id: WanId
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Ipv6SetToStatic {
    pub id: WanId,
    pub host: Ipv6Host
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Ipv6Disabled {
    pub id: WanId
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Ipv4SetToDHCP {
    pub id: WanId
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Ipv4SetToStatic {
    pub id: WanId,
    pub host: Ipv4Host
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Ipv4Disabled {
    pub id: WanId
}

pub type WanId = Id;

// #[derive(Debug, Default, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub struct WanId(uuid::Uuid);

// impl WanId {
//     pub fn new() -> Self {
//         Id(Uuid::new_v4())
//     }
// }

// impl Display for Id {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WanConnection {
    IPoE(IPoEConf),
    PPPoE(PPPoEConf)
}

impl Default for WanConnection {
    fn default() -> Self {
        Self::IPoE(IPoEConf::default())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PPPoEConf {
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct IPoEConf {
    pub ipv6: Ipv6Conf,
    pub dhcp6: Dhcp6Conf,
    pub ipv4: Ipv4Conf, //optional
    // pub dhcp4: Dhcp4Conf,
    // ipv4_mtu
    // ipv6_mtu
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Ipv6Conf {
    FromRA,
    Static(Ipv6Host),
    Disabled
}

impl Default for Ipv6Conf {
    fn default() -> Self {
        Self::FromRA
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Ipv6Host {
    pub addr: Ipv6HostAddr,
    pub gateway: Ipv6Addr
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub struct Dhcp6Conf {
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Ipv4Conf { 
    DHCP,
    Static(Ipv4Host),
    Disabled
}

impl Default for Ipv4Conf {
    fn default() -> Self {
        Self::DHCP
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Ipv4Host {
    pub addr: Ipv4HostAddr,
    pub geteway: Ipv4Addr
}

// #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
// pub struct Ipv6Host {
//     pub address: Ipv6Addr,
//     pub prefix_len: u8
// }


