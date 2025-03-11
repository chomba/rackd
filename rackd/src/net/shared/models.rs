use std::{fmt::Display, net::{Ipv4Addr, Ipv6Addr}, str::FromStr};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone)]
pub struct LinkId(u32);


#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub struct VlanId(u16);

impl Display for VlanId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct NetName(String);

impl Display for NetName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl NetName { 
    pub fn new(s: &str) -> Option<NetName>  {
        NetName::from_str(&s.to_lowercase()).ok()
    }
}

/// Logical Trunks added to the rack
/// Each TrunkId maps to one physical Link on each node
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub struct TrunkId(u8);

impl Display for TrunkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub enum PrefixLen {
    V4(u8),
    V6(u8),
    DualStack((u8, u8))
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
pub struct Ipv6HostAddr {
    pub addr: Ipv6Addr,
    pub prefix_len: Ipv6PrefixLen
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
pub struct Ipv4HostAddr {
    pub addr: Ipv4Addr,
    pub mask_len: Ipv4PrefixLen
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Ipv4PrefixLen(u8);

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Ipv6PrefixLen(u8);

impl Ipv6PrefixLen {
    pub fn new(value: u8) -> Option<Self> {
        // TBD: call try_from and map result to option
        Some(Self(value))
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl Ipv4PrefixLen {
    pub fn new(value: u8) -> Option<Self> {
        // TBD: call try_from and map result to option
        Some(Self(value))
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

//  ipv6-address/prefix-length (RFC2373)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct Ipv4Prefix {
    pub addr: Ipv4Addr,
    pub len: u8
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct Ipv6Prefix {
    pub addr: Ipv6Addr,
    pub len: u8
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Prefix {
    V4(Ipv4Prefix),
    V6(Ipv6Prefix),
    DualStack(Ipv4Prefix, Ipv6Prefix)
}

impl Default for Prefix {
    fn default() -> Self {
        Prefix::V4(Ipv4Prefix::default())
    }
}

pub trait IpPrefix where Self: Default + Copy + Sized + PartialEq + Eq {
    type Addr;
    fn new(addr: Self::Addr, len: u8) -> Self;
    fn extend(&self, ext: Self) -> Option<Self>;
    fn truncate(&self, len: u8) -> Option<Self>; 
    fn overlaps(&self, other: Self) -> Option<IpPrefixOverlap<Self>>; 
    fn first(&self) -> Self::Addr;
    fn last(&self) -> Self::Addr;
    fn endpoints(&self) -> (Self::Addr, Self::Addr) {
        (self.first(), self.last())
    }
}

impl Display for Ipv4Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.addr, self.len)
    }
}

impl Display for Ipv6Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.addr, self.len)
    }
}

impl Default for Ipv4Prefix {
    fn default() -> Self {
        Self { addr: Ipv4Addr::new(0, 0, 0, 0), len: u8::default() }
    }
}

impl Default for Ipv6Prefix {
    fn default() -> Self {
        Self { addr: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0), len: u8::default() }
    }
}

// pub enum PrefixExt {
//     V4(Ipv4PrefixExt),
//     V6(Ipv6PrefixExt)
// }

// #[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
// pub struct IpPrefixExt<P> where P: IpPrefix {
//     pub id: Id,
//     pub ext: P
// }

// pub type Ipv4PrefixExt = IpPrefixExt<Ipv4Prefix>;
// pub type Ipv6PrefixExt = IpPrefixExt<Ipv6Prefix>;









// #[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
// pub struct Ipv4PrefixExt {
//     pub id: Id,
//     pub ext: Ipv4Prefix
// }

// #[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
// pub struct Ipv6PrefixExt {
//     pub id: Id,
//     pub ext: Ipv6Prefix
// }

// pub enum PrefixOverlap {
//     V4(Ipv4PrefixOverlap),
//     V6(Ipv6PrefixOverlap),
//     DualStack(Ipv4PrefixOverlap, Ipv6PrefixOverlap)
// }

pub type Ipv4PrefixOverlap = IpPrefixOverlap<Ipv4Prefix>;
pub type Ipv6PrefixOverlap = IpPrefixOverlap<Ipv6Prefix>;

#[derive(Debug)]
pub enum IpPrefixOverlap<T> where T: IpPrefix {
    Equal,
    Subset((T::Addr, T::Addr)),
    Partial((T::Addr, T::Addr))
}

impl IpPrefix for Ipv6Prefix {
    type Addr = Ipv6Addr;

    fn new(addr: Ipv6Addr, len: u8) -> Self {
        match len {
            0 => Self { addr: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0), len },
            len @ 1..=128 => {
                let addr = addr.to_bits();  
                let mask = u128::MAX << (128 - len);
                let addr = Ipv6Addr::from_bits(addr & mask);
                Self { addr, len }
            },
            len @ 129.. => {
                let len = len % 128;
                Self::new(addr, len)
            }
        }
    }

    fn truncate(&self, len: u8) -> Option<Ipv6Prefix> {
        if len >= self.len {
            return None;
        }
        let bits = self.addr.to_bits() & (u128::MAX << 128 - len);
        Some(Ipv6Prefix::new(Ipv6Addr::from_bits(bits), len))
    }

    fn extend(&self, mut ext: Ipv6Prefix) -> Option<Ipv6Prefix>  {
        if ext.len <= self.len {
            return None;
        }
        // Prepare Address for Extension (bitwise OR)
        // Align offset to Hex Quartet boundary
        let offset = match self.len % 16 {
            0 => self.len,
            _ => (self.len / 16) * 16
        };
        ext.addr = Ipv6Addr::from_bits(ext.addr.to_bits() >> offset);

        // IF (BASE.addr/BASE.prefix_len |== EXT.addr/BASE.prefix_len) 
        // THEN (EXT.addr | BASE.addr)/EXT.prefix_len IS A VALID EXTENSION
        let truncated_ext = match ext.truncate(self.len) {
            Some(value) => value,
            None => return None
        };
        if (truncated_ext.addr.to_bits() | self.addr.to_bits()) == self.addr.to_bits()  {
            let new_prefix = Ipv6Prefix {
                addr: Ipv6Addr::from_bits(self.addr.to_bits() | ext.addr.to_bits()),
                len: ext.len
            };
            return Some(new_prefix);
        }
        None
    }
    
    fn first(&self) -> Ipv6Addr {
        self.addr
    }

    fn last(&self) -> Ipv6Addr {
        match self.len {
            0 | 128 => self.addr,
            _ => {
                let last = u128::MAX >> self.len;
                Ipv6Addr::from_bits(self.addr.to_bits() | last)
            }
        }
    }

    fn overlaps(&self, other: Ipv6Prefix) -> Option<Ipv6PrefixOverlap> {
        // GIVEN Ranges A, B WHERE A.0 <= B.0
        let a = self.endpoints().min(other.endpoints());
        let b = self.endpoints().max(other.endpoints());
        
        if a == b {
            Some(Ipv6PrefixOverlap::Equal)
        } else if a.1 >= b.1 {
            Some(Ipv6PrefixOverlap::Subset((b.0, b.1)))    // A ∩ B ≠ ∅ AND B ⊂ A (AAAAXXAAAA)
        } else if a.1 >= b.0 {
            Some(Ipv6PrefixOverlap::Partial((b.0, a.1)))    // A ∩ B ≠ ∅ AND B ⊈ A (AAAAXXBB)  
        } else {
            None                                        // A ∩ B = ∅ (AAAA BBBB)
        }
    }
}

impl IpPrefix for Ipv4Prefix {
    type Addr = Ipv4Addr;

    fn new(addr: Ipv4Addr, len: u8) -> Self {
        match len {
            0 => Self { addr: Ipv4Addr::new(0, 0, 0, 0), len },
            len @ 1..=32 => {
                let addr = addr.to_bits();  
                let mask = u32::MAX << (32 - len);
                let addr = Ipv4Addr::from_bits(addr & mask);
                Self { addr, len }
            },
            len @ 33.. => {
                let len = len % 32;
                Self::new(addr, len)
            }
        }
    }

    fn truncate(&self, len: u8) -> Option<Ipv4Prefix> {
        if len >= self.len {
            return None;
        }
        let bits = self.addr.to_bits() & (u32::MAX << 32 - len);
        Some(Ipv4Prefix::new(Ipv4Addr::from_bits(bits), len))
    }

    fn extend(&self, mut ext: Ipv4Prefix) -> Option<Ipv4Prefix>  {
        if ext.len <= self.len {
            return None;
        }
        // Prepare Address for Extension (bitwise OR)
        // Align offset to the byte boundary
        let offset = match self.len % 8 {
            0 => self.len,
            _ => (self.len / 8) * 8
        };
        ext.addr = Ipv4Addr::from_bits(ext.addr.to_bits() >> offset);

        // IF (BASE.addr/BASE.prefix_len |== EXT.addr/BASE.prefix_len) 
        // THEN (EXT.addr | BASE.addr)/EXT.prefix_len IS A VALID EXTENSION
        let truncated_ext = match ext.truncate(self.len) {
            Some(value) => value,
            None => return None
        };
        if (truncated_ext.addr.to_bits() | self.addr.to_bits()) == self.addr.to_bits()  {
            let new_prefix = Ipv4Prefix {
                addr: Ipv4Addr::from_bits(self.addr.to_bits() | ext.addr.to_bits()),
                len: ext.len
            };
            return Some(new_prefix);
        }
        None
    }

    fn first(&self) -> Self::Addr {
        self.addr
    }

    fn last(&self) -> Ipv4Addr {
        match self.len {
            0 | 32 => self.addr,
            _ => {
                let last = u32::MAX >> self.len;
                Ipv4Addr::from_bits(self.addr.to_bits() | last)
            }
        }
    }

    fn overlaps(&self, other: Ipv4Prefix) -> Option<Ipv4PrefixOverlap> {
        // GIVEN Ranges A, B WHERE A.0 <= B.0
        let a = self.endpoints().min(other.endpoints());
        let b = self.endpoints().max(other.endpoints());
        
        if a == b {
            Some(Ipv4PrefixOverlap::Equal)
        } else if a.1 >= b.1 {
            Some(Ipv4PrefixOverlap::Subset((b.0, b.1)))    // A ∩ B ≠ ∅ AND B ⊂ A (AAAAXXAAAA)
        } else if a.1 >= b.0 {
            Some(Ipv4PrefixOverlap::Partial((b.0, a.1)))    // A ∩ B ≠ ∅ AND B ⊈ A (AAAAXXBB)  
        } else {
            None                                        // A ∩ B = ∅ (AAAA BBBB)
        }
    }
}

#[derive(Debug, Error)]
pub enum PrefixParseError {
    // V4()
    // V6()
    #[error("Invalid Format, expected format: <addresss>/<prefix>")]
    InvalidFormat,
    #[error("Invalid Ipv6 Address Format")]
    InvalidIpv6Address,
    #[error("Invalid Prefix Length Format")]
    InvalidPrefixLength
}

// impl FromStr for Prefix<T> where T: FromStr {
    
// }

impl FromStr for Ipv6Prefix {
    type Err = PrefixParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (addr, prefix_len) = match s.split_once("/") {
            Some(value) => value,
            None => return Err(PrefixParseError::InvalidFormat)
        };

        let addr = match Ipv6Addr::from_str(addr) {
            Ok(value) => value,
            Err(_) => return Err(PrefixParseError::InvalidIpv6Address)
        };

        let len = match u8::from_str(prefix_len) {
            Ok(len) if len > 0 && len <= 128 => len,
            _ => Err(PrefixParseError::InvalidPrefixLength)?
        };

        Ok(Ipv6Prefix::new(addr, len))
    }
}


impl FromStr for Ipv4Prefix {
    type Err = PrefixParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (addr, prefix_len) = match s.split_once("/") {
            Some(value) => value,
            None => return Err(PrefixParseError::InvalidFormat)
        };

        let addr = match Ipv4Addr::from_str(addr) {
            Ok(value) => value,
            Err(_) => return Err(PrefixParseError::InvalidIpv6Address)
        };

        let len = match u8::from_str(prefix_len) {
            Ok(len) if len > 0 && len <= 32 => len,
            _ => Err(PrefixParseError::InvalidPrefixLength)?
        };

        Ok(Ipv4Prefix::new(addr, len))
    }
}

// #[derive(Error, Debug)]
// #[error("TBD")]
// pub enum PrefixLenParseError {
//     ValueOutsideBoundsForIpv4,
//     ValueOutsideBoundsForIpv6
// }

// #[derive(Debug, Default, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub struct Ipv4PrefixLen(u8);

// impl TryFrom<u8> for Ipv4PrefixLen {
//     type Error = PrefixLenParseError;
//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         if value < 1 || value > 32 {
//             return Err(PrefixLenParseError::ValueOutsideBoundsForIpv4);
//         }
//         Ok(Ipv4PrefixLen(value))
//     }
// }

// impl Display for Ipv4PrefixLen {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.value())
//     }
// }

// impl Ipv4PrefixLen {
//     pub fn value(&self) -> u8 {
//         self.0
//     }
// }


// #[derive(Debug, Default, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub struct Ipv6PrefixLen(u8);

// impl Ipv6PrefixLen {
//     pub fn value(&self) -> u8 {
//         self.0
//     }
// }

// impl Display for Ipv6PrefixLen {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

// impl TryFrom<i64> for Ipv6PrefixLen {
//     type Error = PrefixLenParseError;
//     fn try_from(value: i64) -> Result<Ipv6PrefixLen, Self::Error> {
//         match u8::try_from(value) {
//             Ok(len) if len > 0 && len <= 128 => Ok(Ipv6PrefixLen(len)),
//             _ => Err(PrefixLenParseError::ValueOutsideBoundsForIpv6)
//         }
//     }
// }


pub mod casts {
    use std::str::FromStr;
    use super::NetName;

    impl From<NetName> for String {
        fn from(value: NetName) -> Self {
            value.0
        }
    }

    pub enum NetNameParseError {
        InvalidChar
    }
    
    impl FromStr for NetName {
        type Err = NetNameParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(NetName(String::from(s)))
        }
    }

    pub mod sql {
        use rusqlite::{types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, Error, Result, ToSql};
        use crate::net::shared::models::{NetName, TrunkId, VlanId};
    
        impl ToSql for NetName {
            fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
                Ok(self.0.as_str().into())
            }
        }

        impl FromSql for NetName {
            fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
                Ok(NetName(String::from(value.as_str()?)))
            }
        }

        impl ToSql for VlanId {
            fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
                Ok(self.0.into())
            }
        }

        impl FromSql for VlanId {
            fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
                Ok(VlanId(value.as_i64()? as u16))
            }
        }

        impl ToSql for TrunkId {
            fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
                Ok(self.0.into())
            }
        }

        impl FromSql for TrunkId {
            fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
                Ok(TrunkId(value.as_i64()? as u8))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn can_create_ipv4_prefix() {
        let addr = Ipv4Addr::new(172, 24, 16, 255);
        let prefix = Ipv4Prefix::new(addr, 24);
        assert_eq!(prefix.to_string(), "172.24.16.0/24");

        let addr = Ipv4Addr::new(192, 168, 1, 1);
        let prefix = Ipv4Prefix::new(addr, 32);
        assert_eq!(prefix.to_string(), "192.168.1.1/32");
    }

    #[test]
    fn can_get_ipv4_prefix_endpoints() {
        // /24 Prefix
        let addr = Ipv4Addr::new(172, 24, 16, 255);
        let prefix = Ipv4Prefix::new(addr, 24);
        assert_eq!(prefix.first().to_string(), "172.24.16.0");
        assert_eq!(prefix.last().to_string(), "172.24.16.255");

        // /32 Prefix
        let addr = Ipv4Addr::new(172, 24, 16, 255);
        let prefix = Ipv4Prefix::new(addr, 32);
        assert_eq!(prefix.first().to_string(), "172.24.16.255");
        assert_eq!(prefix.last().to_string(), "172.24.16.255");
    }

    #[test]
    fn ipv6_prefix_type() {

    }

    #[test]
    fn parse_ipv6_prefix() {
        let s1 = "2001:1388:1640:77::/56";
        let s2 = "2001:1388:1640:00::/56";
        let prefix1 = Ipv6Prefix::from_str(s1).unwrap();
        let prefix2 = Ipv6Prefix::from_str(s2).unwrap();
        assert_eq!(prefix1, prefix2);
    }

    #[test]
    fn truncate_ipv6_prefix() {
        let prefix = Ipv6Prefix::from_str("2001:1388:1640:1277::/64").unwrap();
        let expected = Ipv6Prefix::from_str("2001:1388:1640:1200::/56").unwrap();
        let truncated = prefix.truncate(56).unwrap();
        assert_eq!(expected, truncated);

        let prefix = Ipv6Prefix::from_str("2001:1388:1640:1277:ffff::/64").unwrap();
        let expected = Ipv6Prefix::from_str("2001:1388::/32").unwrap();
        let truncated = prefix.truncate(32).unwrap();
        assert_eq!(expected, truncated);
    }

    #[test]
    fn ipv6_prefix_overlap() {
        // TBD
        let prefix1 = Ipv6Prefix::from_str("2001:1388:1640:1277::/64").unwrap();
        let prefix2 = prefix1.clone();
        let overlap = prefix1.overlaps(prefix2);
        println!("OVERLAP: {overlap:?}");
        assert!(overlap.is_some());
    }

    #[test]
    fn get_ipv6_prefix_endpoints() {
        let prefix = Ipv6Prefix::from_str("2001:1388:1640:1277::/64").unwrap();
        let endpoints = prefix.endpoints();
        let expected = (Ipv6Addr::from_str("2001:1388:1640:1277::").unwrap(), Ipv6Addr::from_str("2001:1388:1640:1277:ffff:ffff:ffff:ffff").unwrap());
        assert_eq!(endpoints, expected);
    }

    #[test]
    fn extend_ipv6_prefix() {
        let prefix = Ipv6Prefix::from_str("2001:1388:1640:1277::/64").unwrap();    
        let extension = Ipv6Prefix::from_str("1200::/80").unwrap();
        let expected = Ipv6Prefix::from_str("2001:1388:1640:1277:1200::/80").unwrap();
        let extended = prefix.extend(extension).unwrap();
        assert_eq!(extended, expected);

        let prefix = Ipv6Prefix::from_str("2001:1388:1640::/48").unwrap();    
        let extension = Ipv6Prefix::from_str("ac00:1200::/80").unwrap();
        let expected = Ipv6Prefix::from_str("2001:1388:1640:ac00:1200::/80").unwrap();
        let extended = prefix.extend(extension).unwrap();
        assert_eq!(extended, expected);

        let prefix = Ipv6Prefix::from_str("2001:1388:1640:c000::/52").unwrap();    
        let extension = Ipv6Prefix::from_str("aa:1200::/80").unwrap();
        let expected = Ipv6Prefix::from_str("2001:1388:1640:c0aa:1200::/80").unwrap();
        let extended = prefix.extend(extension).unwrap();
        assert_eq!(extended, expected);

        let prefix = Ipv6Prefix::from_str("2001:1388:1640:8000::/49").unwrap();    
        let extension = Ipv6Prefix::from_str("7fff:1200::/80").unwrap();
        let expected = Ipv6Prefix::from_str("2001:1388:1640:ffff:1200::/80").unwrap();
        let extended = prefix.extend(extension).unwrap();
        assert_eq!(extended, expected);
    }

}
