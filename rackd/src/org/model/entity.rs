use serde::{Deserialize, Serialize};
use crate::{net::Ipv6Prefix, rack::RackId, util::models::{Entity, Metadata}};
use super::{Asn, OrgDomain, OrgId, OrgName};

/// A struct that represents a **Company/Organization** that spans across *multiple geographical Sites*.
/// It has a one-to-many relationship with the **Site** struct and contains the following fields: 
/// - **name**: The organization's display name.
/// - **prefix**: IPv6 GUA Prefix used by the Organization. It needs to be /48 or shorter (e.g. /40, /44)
/// - **domain**: Root Domain used by the Organization.
/// - **asn**: The organization's AS Number.
#[derive(Debug, Default)]
pub struct Org {
    pub meta: Metadata,
    pub id: OrgId, 
    pub asn: Asn,
    pub name: OrgName,
    pub domain: OrgDomain, 
    pub prefix: Ipv6Prefix, // /48 GUA Prefix
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrgEvent {
    Created { id: RackId, name: OrgName, domain: OrgDomain, prefix: Ipv6Prefix },
    Renamed { name: OrgName }
    // DomainUpdated
    // PrefixUpdated
}

// impl Entity for Org {
//     type E = OrgEvent;

//     fn apply(&mut self, e: &Self::Event) {
//         match e {
//             OrgEvent::Created { id, name, domain, prefix } => {
//                 self.id = *id;
//                 self.name = name.clone();
//                 self.domain = domain.clone();
//                 self.prefix = *prefix;
//             },
//             OrgEvent::Renamed { name } => {
//                 self.name = name.clone();
//             }
//         }
//     }

//     fn id(&self) -> Id { self.id }
//     fn metadata(&mut self) -> &mut Metadata { &mut self.meta }
// }
