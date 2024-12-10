use std::str::FromStr;
use core::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::{app::shared::domain::*, sys::link::models::*, util::{domain::Id, net::types::{Ipv4Prefix, Ipv6Prefix, PrefixLen}}};
use super::events::*;

#[derive(Debug, Default)]
pub struct Isp {
    pub meta: Metadata,
    pub id: Id,
    pub rank: IspRank,
    pub name: IspName,
    pub link: LinkId,
    pub link_name: LinkName,
    pub link_status: LinkStatus,
    pub prefix: IspPrefix,
    pub tracker: Id
    // asn: Option<u8>
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum IspPrefix {
    V4(Ipv4Prefix),
    V6(Ipv6Prefix),
    DualStack((Ipv4Prefix, Ipv6Prefix))
}

impl IspPrefix {
    pub fn v4(&self) -> Option<Ipv4Prefix> {
        if let IspPrefix::DualStack((prefix, _)) = self {
            return Some(*prefix);
        } else if let IspPrefix::V4(prefix) = self {
            return Some(*prefix);
        }
        None
    }

    pub fn v6(&self) -> Option<Ipv6Prefix> {
        if let IspPrefix::DualStack((_, prefix)) = self {
            return Some(*prefix);
        } else if let IspPrefix::V6(prefix) = self {
            return Some(*prefix);
        }
        None
    }

    pub fn len(&self) -> PrefixLen {
        match self {
            IspPrefix::V4(prefix) => PrefixLen::V4(prefix.len),
            IspPrefix::V6(prefix) => PrefixLen::V6(prefix.len),
            IspPrefix::DualStack(prefix) => PrefixLen::DualStack((prefix.0.len, prefix.1.len))
        }
    }
}

impl Default for IspPrefix {
    fn default() -> Self {
        IspPrefix::V4(Ipv4Prefix::default())
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct IspRank(pub usize);

impl Display for IspRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct IspName(String);

impl IspName {
    pub fn new(s: &str) -> Option<Self> {
        Self::from_str(s).ok()
    }
}

#[derive(Debug)]
pub enum IspNameParseError {
    InvalidLength
}

impl FromStr for IspName {
    type Err = IspNameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TBD
        Ok(IspName(String::from(s)))
    }
}

impl Display for IspName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IspEvent {
    Created { id: Id, rank: IspRank, name: IspName, link: LinkId, link_name: LinkName, link_status: LinkStatus, prefix: IspPrefix, tracker: Id },
    Renamed { name: IspName },
    LinkWentUp { up: LinkUp }, 
    LinkWentDown { down: LinkDown },
    Deleted,
}

impl Entity for Isp {
    type Event = IspEvent;

    fn apply(&mut self, e: &Self::Event) {
        match e {
            IspEvent::Created { id, rank, name, link, link_name, link_status, prefix, tracker } => {
                self.id = *id;
                self.rank = *rank;
                self.name = name.clone();
                self.link = *link;
                self.link_name = link_name.clone();
                self.link_status = *link_status;
                self.prefix = *prefix;
                self.tracker = *tracker;
            },
            IspEvent::Renamed { name } => {
                self.name = name.clone();
            },
            IspEvent::LinkWentUp { up } => {
                self.link_status = LinkStatus::Up(*up);
            },
            IspEvent::LinkWentDown { down } => {
                self.link_status = LinkStatus::Down(*down);
            },
            IspEvent::Deleted => { }
        }    
    }

    fn metadata(&mut self) -> &mut Metadata { &mut self.meta }
    fn id(&self) -> Id { self.id }
}

impl Isp {
    pub fn new(name: IspName, rank: IspRank, link: (LinkId, LinkName, LinkStatus), prefix: IspPrefix, tracker: Id) -> Self {
        let e = IspEvent::Created { 
            id: Id::new(), name, rank, link: link.0, link_name: link.1, link_status: link.2, prefix, tracker 
        };
        let mut isp = Isp::default();
        isp.process(e);
        isp
    }

    pub fn rename(&mut self, name: IspName) {
        if self.name != name {
            let e = IspEvent::Renamed { name };
            self.process(e);
        }
    }
}

impl EventHandler<IspLinkWentUp> for Isp {
    fn handle(&mut self, e: &IspLinkWentUp) -> () {
        if self.link_status != LinkStatus::Up(e.up) {
            let e = IspEvent::LinkWentUp { up: e.up };
            self.process(e);
        }

        // match self.status {
        //     LinkStatus::Enabled if self.link_status != LinkStatus::Up(e.up) => {
        //         let e = IspEvent::LinkWentUp { up: e.up };
        //         self.process(e);
        //     }
        //     _ => {}
        // }
    }
}

impl EventHandler<IspLinkWentDown> for Isp {
    fn handle(&mut self, e: &IspLinkWentDown) -> () {
        if self.link_status != LinkStatus::Down(e.down) {
            let e = IspEvent::LinkWentDown { down: e.down };
            self.process(e);
        }

        // match self.status {
        //     IspStatus::Enabled if self.link_status != LinkStatus::Down(e.down) => {
        //         let e = IspEvent::LinkWentDown { down: e.down };
        //         self.process(e);
        //     },
        //     _ => {}
        // }
    }
}