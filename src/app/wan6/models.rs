use serde::{Deserialize, Serialize};
use crate::{app::{shared::domain::*, wan::models::*}, util::{domain::Id, net::types::{Ipv6Prefix, Ipv6PrefixExt}}};

#[derive(Debug, Default)]
pub struct Wan6 {
    pub meta: Metadata,
    pub id: Id,
    pub prefix: Wan6Prefix,
    pub iprefix: Ipv6Prefix,
    pub name: WanName
    // pub owners: BTreeSet<RouterNode>
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize, Hash)]
pub enum Wan6Prefix {
    Isp(Id),
    Extension(Ipv6PrefixExt)
}

impl Default for Wan6Prefix {
    fn default() -> Self {
        Wan6Prefix::Isp(Id::default())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Wan6Event {
    Created { id: Id, name: WanName, prefix: Wan6Prefix, iprefix: Ipv6Prefix },
    Renamed { name: WanName },
    PrefixUpdated { prefix: Wan6Prefix, iprefix: Ipv6Prefix },
    PrefixRecomputed { prefix: Ipv6Prefix },
    Deleted
}

impl Wan6 {
    pub fn new(id: Id, name: WanName, prefix: Wan6Prefix, iprefix: Ipv6Prefix) -> Self {
        let e = Wan6Event::Created { id, name, prefix, iprefix };
        let mut wan = Self::default();
        wan.process(e);
        wan
    }

    pub fn rename(&mut self, name: WanName) {
        if self.name != name {
            let e = Wan6Event::Renamed { name };
            self.process(e);
        }
    }

    pub fn set_prefix(&mut self, prefix: Wan6Prefix, iprefix: Ipv6Prefix) {
        if self.prefix != prefix {
            let e = Wan6Event::PrefixUpdated { prefix, iprefix };
            self.process(e);
        }
    }

    pub fn delete(&mut self) {
        let e = Wan6Event::Deleted;
        self.process(e);
    }
}

impl Entity for Wan6 {
    type Event = Wan6Event;

    fn apply(&mut self, e: &Self::Event) {
        match e {
            Wan6Event::Created { id, name, prefix, iprefix } => {
                self.id = *id;
                self.name = name.clone();
                self.prefix = *prefix;
                self.iprefix = *iprefix;
            },
            Wan6Event::Renamed { name } => {
                self.name = name.clone();
            },
            Wan6Event::PrefixUpdated { prefix, iprefix } => {
                self.prefix = *prefix;
                self.iprefix = *iprefix;
            },
            Wan6Event::PrefixRecomputed { prefix } => {
                self.iprefix = *prefix;
            },
            Wan6Event::Deleted => { }
        }
    }

    fn id(&self) -> Id { self.id }
    fn metadata(&mut self) -> &mut Metadata { &mut self.meta }
}

impl Wan for Wan6 {
    type Prefix = Ipv6Prefix;

    fn prefix(&self) -> Self::Prefix {
        self.iprefix
    }
}