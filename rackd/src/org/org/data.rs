use std::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::util::{domain::{Entity, Id, Metadata}, net::types::{IpPrefix, Ipv6Prefix}};

/// A struct that represents a **Company/Organization** that spans across *multiple geographical Sites*.
/// It has a one-to-many relationship with the **Site** struct and contains the following fields: 
/// - **name**: The organization's display name.
/// - **prefix**: IPv6 GUA Prefix used by the Organization. It needs to be /48 or shorter (e.g. /40, /44)
/// - **domain**: Root Domain used by the Organization.
/// - **asn**: The organization's AS Number.
// #[derive(Debug, Clone, Default)]
// pub struct Org {
//     pub meta: Metadata<Self>,
//     pub id: Id, 
//     pub name: OrgName,
//     pub domain: OrgDomain, 
//     pub prefix: Ipv6Prefix, // /48 GUA Prefix
//     pub asn: Option<Asn>    
// }

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrgName(String);

impl Display for OrgName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrgDomain {
    pub name: String,
    pub tld: String
}

impl Display for OrgDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.name, self.tld)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Asn(u32);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrgEvent {
    Created { id: Id, name: OrgName, domain: OrgDomain, prefix: Ipv6Prefix },
    Renamed { name: OrgName }
    // DomainUpdated
    // PrefixUpdated
}

impl Org {
    pub fn new(name: OrgName, domain: OrgDomain, prefix: Ipv6Prefix) -> Self {
        let e = OrgEvent::Created { id: Id::new(), name, domain, prefix };
        let mut org = Self::default();
        org.process(e);
        org
    }

    pub fn rename(&mut self, name: OrgName) {
        if self.name != name {
            let e = OrgEvent::Renamed { name };
            self.process(e)
        }
    }
}

impl Entity for Org {
    type Event = OrgEvent;

    fn apply(&mut self, e: &Self::Event) {
        match e {
            OrgEvent::Created { id, name, domain, prefix } => {
                self.id = *id;
                self.name = name.clone();
                self.domain = domain.clone();
                self.prefix = *prefix;
            },
            OrgEvent::Renamed { name } => {
                self.name = name.clone();
            }
        }
    }

    fn id(&self) -> Id { self.id }
    fn metadata(&mut self) -> &mut Metadata<Self> { &mut self.meta }
}

