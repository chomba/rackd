use std::{fmt::Display, str::FromStr};
use crate::app::lan::models::*;
use serde::{Deserialize, Serialize};
use crate::{app::shared::domain::{Entity, Metadata}, util::{domain::Id, net::types::{Ipv6Prefix, Ipv6PrefixExt, Prefix}}};

#[derive(Debug, Default, Clone)]
pub struct Lan6 {
    pub meta: Metadata,
    pub id: Id,
    pub name: LanName,
    pub prefix: Lan6Prefix,
    pub iprefix: Ipv6Prefix,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
pub enum Lan6Prefix {
    Literal(Ipv6Prefix),
    Extension(Ipv6PrefixExt)
}

impl Default for Lan6Prefix {
    fn default() -> Self {
        Lan6Prefix::Literal(Ipv6Prefix::default())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Lan6Event {
    Created { id: Id, name: LanName, prefix: Lan6Prefix, iprefix: Ipv6Prefix }, 
    Renamed { name: LanName },
    PrefixUpdated { prefix: Lan6Prefix, iprefix: Ipv6Prefix },
    PrefixRecomputed { prefix: Ipv6Prefix, triggered_by: Id },
    Deleted
}

// #[derive(Debug, Serialize, Deserialize, Copy, Clone)]
// pub enum LanKind {
//     V4(Lan4Prefix),
//     V6(Lan6Prefix),
//     DualStack(Lan4Prefix, Lan6Prefix)
// }

// impl Default for LanKind {
//     fn default() -> Self {
//         LanKind::V4(Lan4Prefix::Literal(Ipv4Prefix::default()))
//     }
// }

// #[derive(Debug)]
// pub struct LanPrefixParseError;

// impl FromStr for LanPrefix {
//     type Err = LanPrefixParseError;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//             // TBD
//             Err(LanPrefixParseError)
//     }
// }

impl Entity for Lan6 {
    type Event = Lan6Event;
    
    fn apply(&mut self, e: &Self::Event) {
        match e {
            Lan6Event::Created { id, name, prefix, iprefix } => {
                self.id = *id;
                self.name = name.clone();
                self.prefix = *prefix;
                self.iprefix = *iprefix;
            },
            Lan6Event::Renamed { name } => {
                self.name = name.clone();
            },
            Lan6Event::PrefixUpdated { prefix, iprefix} => {
                self.prefix = *prefix;
                self.iprefix = *iprefix;
            },
            Lan6Event::PrefixRecomputed { prefix, .. } => {
                self.iprefix = *prefix;
            },
            Lan6Event::Deleted => { }
        }
    }
    fn id(&self) -> Id { self.id }
    fn metadata(&mut self) -> &mut Metadata { &mut self.meta }
}

impl Lan6 {
    pub fn new(name: LanName, prefix: Lan6Prefix, iprefix: Ipv6Prefix) -> Self  {
        let e = Lan6Event::Created { id: Id::new(), name, prefix, iprefix };
        let mut lan = Lan6::default();
        lan.process(e);
        lan
    }

    pub fn rename(&mut self, name: LanName) {
        if self.name != name {
            let e = Lan6Event::Renamed { name };
            self.process(e);
        }
    }

    pub fn set_prefix(&mut self, prefix: Lan6Prefix, iprefix: Ipv6Prefix) {
        let e = Lan6Event::PrefixUpdated { prefix, iprefix };
        self.process(e);
    }

    pub fn delete(&mut self) {
        let e = Lan6Event::Deleted;
        self.process(e);
    }
}

impl Lan for Lan6 {
    type Prefix = Ipv6Prefix;

    fn prefix(&self) -> Self::Prefix {
        self.iprefix
    }
}