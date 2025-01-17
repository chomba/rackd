use std::{fmt::Display, net::{Ipv4Addr, Ipv6Addr}, str::FromStr};
use serde::{Deserialize, Serialize};
use crate::net::util::{tools::InternetUp, models::*};

#[derive(Debug, Default)]
pub struct Link {
    pub id: LinkId,
    pub name: LinkName,
    pub ipv6_addrs: Vec<Ipv6Addr>,
    pub ipv4_addrs: Vec<Ipv4Addr>,
    pub status: LinkStatus
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkName(String);

impl Display for LinkName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Copy, Clone)]
pub enum LinkStatus {
    Unknown,
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
        LinkStatus::Unknown
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone)]
pub struct LinkId(u32);

impl Display for LinkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
}

// models::casts::sql
// models::casts::netlink;

mod casts {
    use super::*;

    impl From<u32> for LinkId {
        fn from(value: u32) -> Self {
            LinkId(value)
        }
    }

    impl From<LinkId> for u32 {
        fn from(id: LinkId) -> Self {
            id.0
        }
    }

    #[derive(Debug)]
    pub enum LinkNameParseError {
        TooLarge
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

    mod sql {
        use rusqlite::{types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, Error, Result, ToSql};
        use super::*;

        impl ToSql for LinkStatus {
            fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
                let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
                Ok(json.into())
            }
        }
    
        impl FromSql for LinkStatus {
            fn column_result(status: ValueRef<'_>) -> FromSqlResult<Self> {
                let status = serde_json::from_str(status.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
                Ok(status)
            }
        }
        
        impl ToSql for LinkName {
            fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
                Ok(self.to_string().into())
            }
        }

        impl FromSql for LinkName {
            fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
                Ok(LinkName(String::from(value.as_str()?)))
            }
        }

        impl ToSql for LinkId {
            fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
                Ok(u32::from(*self).into())
            }
        }

        impl FromSql for LinkId {
            fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
                let value = u32::try_from(value.as_i64()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
                Ok(Self::from(value))
            }
        }
    }

    mod netlink {
        use futures::{TryStream, TryStreamExt};
        use netlink_packet_route::link::{LinkAttribute, LinkFlag, LinkMessage};
        use rtnetlink::Error;
        use crate::sys::util::netlink::FromNetlinkMessage;
        use super::*;

        // IFF_UP (Admin Up) If not set then Link is Admin Down
        // IFF_LOWER_UP (Link has Carrier/ Physical layer)
        //  https://www.kernel.org/doc/Documentation/networking/operstates.txt

        impl From<Vec<LinkFlag>> for LinkStatus {
            fn from(flags: Vec<LinkFlag>) -> Self {
                if let Ok(down) = LinkDown::try_from(&flags) {
                    return LinkStatus::Down(down);
                } else if let Ok(up) = LinkUp::try_from(&flags) {
                    return LinkStatus::Up(up);
                }
                LinkStatus::Unknown
            }
        }

        impl TryFrom<&Vec<LinkFlag>> for LinkDown {
            type Error = ();
            fn try_from(flags: &Vec<LinkFlag>) -> Result<Self, Self::Error> {
                if !flags.contains(&LinkFlag::Up) {
                    return Ok(LinkDown::AdminDown);
                } else if !flags.contains(&LinkFlag::LowerUp) {
                    return Ok(LinkDown::Disconnected);
                }
                Err(())
            }
        }

        impl TryFrom<&Vec<LinkFlag>> for LinkUp {
            type Error = ();
            fn try_from(flags: &Vec<LinkFlag>) -> Result<Self, Self::Error> {
                if flags.contains(&LinkFlag::Up) && flags.contains(&LinkFlag::LowerUp)  {
                    return Ok(LinkUp::Connected);
                }
                Err(())
            }
        }

        impl FromNetlinkMessage for Link {
            type Message = LinkMessage;
            async fn from_msg<T>(mut stream: T) -> Option<Self> where T: Unpin + TryStream<Ok = Self::Message, Error = Error> {
                let mut link = Link::default();
   
                let msg = match stream.try_next().await {
                    Ok(Some(msg)) => msg,
                    _ => return None
                };
            
                link.id = LinkId::from(msg.header.index);
                link.status = LinkStatus::from(msg.header.flags);

                for attribute in msg.attributes {
                    match attribute {
                        LinkAttribute::IfName(name) => link.name = LinkName(name),
                        _ => { }
                    }
                }
                Some(link)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::sys::{link::{domain::LinkName, query::GetLinkByName}, util::netlink::Netlink};

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