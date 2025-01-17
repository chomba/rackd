

// During Setup
// 1) List all Nodes running rackd (list hostname, host links along with their MAC Addresses)
// Have the user select all nodes that will become part of the rackd cluster
// 2) List all Nodes added to the cluster and now have the user add TRUNK Links (e.g [RACK-CODE]TRUNK01)
// Then the user will go through each node and select which LINK should be used for which RACK trunk

use std::collections::{BTreeSet, HashMap};
use crate::{net::shared::models::LinkId, util::models::Id};

// Racks will implement ANYCAST DNS 
pub struct Rack {
    // pub meta: Metadata<Self>,
    // PE-LIM-1 (Country-City-Sequential Number)
    // pe-lim-1.chomba.org
    // pub seq: u32,
    pub id: RackId,
    // pub org: Id,
    pub code: RackCode, // ZIP Code
    pub nodes: HashMap<Id, RackNode>,
    pub trunks: BTreeSet<TrunkId>,
    // pub trunk_count: u8, // Number of trunks used by nodes in the cluster
    pub status: RackStatus
}

pub type RackId = Id;

pub struct TrunkId(u8);

pub struct RackCode(String);

pub struct RackNode {
    pub id: Id,
    pub trunks: HashMap<TrunkId, LinkId> // trunk1, trunk2, trunk3
    // Once a Node is configured as part of a rack cluster
    // its name is managed by rackd and changed to [RACKCODE]-N01, [RACKCODE]-N02, [RACKCODE]-N03
}

pub enum RackStatus {
    Operational,
    Degraded,
    Offline
}


