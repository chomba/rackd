use serde::{Deserialize, Serialize};
use crate::{app::{shared::domain::*, wan::models::*}, util::{domain::Id, net::types::{Ipv4Prefix, Ipv4PrefixExt}}};

#[derive(Debug, Default)]
pub struct Wan4 {
    pub meta: Metadata,
    pub id: Id,
    pub prefix: Wan4Prefix,
    pub iprefix: Ipv4Prefix,
    pub name: WanName
    // pub owners: BTreeSet<RouterNode>
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize, Hash)]
pub enum Wan4Prefix {
    Isp(Id),
    Extension(Ipv4PrefixExt)
}

impl Default for Wan4Prefix {
    fn default() -> Self {
        Wan4Prefix::Isp(Id::default())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Wan4Event {
    Created { id: Id, name: WanName, prefix: Wan4Prefix, iprefix: Ipv4Prefix },
    Renamed { name: WanName },
    PrefixUpdated { prefix: Wan4Prefix, iprefix: Ipv4Prefix },
    PrefixRecomputed { new_prefix: Ipv4Prefix, triggered_by: Id },
    Deleted
}

impl Wan4 {
    pub fn new(id: Id, name: WanName, prefix: Wan4Prefix, iprefix: Ipv4Prefix) -> Self {
        let e = Wan4Event::Created { id, name, prefix, iprefix };
        let mut wan = Self::default();
        wan.process(e);
        wan
    }

    pub fn rename(&mut self, name: WanName) {
        if self.name != name {
            let e = Wan4Event::Renamed { name };
            self.process(e);
        }
    }

    pub fn set_prefix(&mut self, prefix: Wan4Prefix, iprefix: Ipv4Prefix) {
        if self.prefix != prefix {
            let e = Wan4Event::PrefixUpdated { prefix, iprefix };
            self.process(e);
        }
    }

    pub fn delete(&mut self) {
        let e = Wan4Event::Deleted;
        self.process(e);
    }
}

impl Entity for Wan4 {
    type Event = Wan4Event;

    fn apply(&mut self, e: &Self::Event) {
        match e {
            Wan4Event::Created { id, name, prefix, iprefix } => {
                self.id = *id;
                self.name = name.clone();
                self.prefix = *prefix;
                self.iprefix = *iprefix;
            },
            Wan4Event::Renamed { name } => {
                self.name = name.clone();
            },
            Wan4Event::PrefixUpdated { prefix, iprefix } => {
                self.prefix = *prefix;
                self.iprefix = *iprefix;
            },
            Wan4Event::PrefixRecomputed { new_prefix, .. } => {
                self.iprefix = *new_prefix;
            },
            Wan4Event::Deleted => { }
        }
    }

    fn id(&self) -> Id { self.id }
    fn metadata(&mut self) -> &mut Metadata { &mut self.meta }
}

impl Wan for Wan4 {
    type Prefix = Ipv4Prefix;
    
    fn prefix(&self) -> Self::Prefix {
        self.iprefix    
    }
}
