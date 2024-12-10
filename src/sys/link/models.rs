use std::{fmt::Display, net::{Ipv4Addr, Ipv6Addr}, str::FromStr};
use serde::{Deserialize, Serialize};
use crate::util::net::{tools::InternetUp, types::*};

#[derive(Debug, Default)]
pub struct Link {
    pub id: LinkId,
    pub name: LinkName,
    pub ipv6_addrs: Vec<Ipv6Addr>,
    pub ipv4_addrs: Vec<Ipv4Addr>,
    pub status: LinkStatus
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Copy, Clone)]
pub enum LinkStatus {
    Unknwon,
    Up(LinkUp),
    Down(LinkDown)
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Copy, Clone)]
pub enum LinkDown {
    WentMissing,     // LinkIndex was deleted Missing (i.e. PPPoE Connection failed)
    AdminDown,      // IFF_UP is not set
    Disconnected,   // IFF_UP is set but IFF_LOWERUP is not set (No Carrier)
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Copy, Clone)]
pub enum LinkUp {
    Connected,          // IFF_UP & IFF_RUNNING (or IFF_LOWERUP?) are set (Cable is connected)
    InternetUp(InternetUp)     // L3 is working correctly
}

impl Default for LinkStatus {
    fn default() -> Self {
        LinkStatus::Unknwon
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone)]
pub struct LinkId(pub u32);

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkName(pub String);

#[derive(Debug)]
pub enum LinkNameParseError {
    TooLarge
}

impl Display for LinkName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


impl FromStr for LinkName {
    type Err = LinkNameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s.to_lowercase());
        let length = s.len();
        if length >= 1 && length <= 40 {
            return Ok(LinkName(s));
        }
        // let mut rules = Rules::new();
        // rules += StringRule::BoundedLength { min: 0, max: 40 }; 
        // rules.eval(s, |s| NetLink(s), |value, violations| Error::NetLink(NetLinkError::Invalid { value, violations }))
        Err(LinkNameParseError::TooLarge)
    }
}

impl Link {
    pub fn ipv6_prefix(&self, prefix_len: u8) -> Option<Ipv6Prefix> {
        for addr in &self.ipv6_addrs {
            let prefix = Ipv6Prefix::new(*addr, prefix_len);
            if prefix.last() == *addr {
                return Some(prefix);
            }
        }
        None
    }

    pub fn ipv4_prefix(&self, prefix_len: u8) -> Option<Ipv4Prefix> {
        for addr in &self.ipv4_addrs {
            let prefix = Ipv4Prefix::new(*addr, prefix_len);
            if prefix.last() == *addr {
                return Some(prefix);
            }
        }
        None
    }

    // pub fn ipv6_addr(&self, prefix_len: PrefixLength) -> Option<Ipv6Addr> {
    //     if let Some(prefix) = self.get_prefix(prefix_len) {
    //         return Some(prefix.last());
    //     }
    //     None
    // }

    // pub fn ipv4_addr(&self) -> Option<Ipv4Addr> {

    // }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::sys::{link::{models::LinkName, query::GetLinkByName}, util::netlink::Netlink};

    #[tokio::test] 
    async fn get_link_ipv4_prefix() {
        // TBD: create interface
        let netlink = Netlink::connect().unwrap();
        let query = GetLinkByName { name: LinkName::from_str("dummy1").unwrap() };
        let link = netlink.run(query).await.unwrap();
        let prefix = link.ipv4_prefix(24);
        assert!(prefix.is_some());        
    }
}