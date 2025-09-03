use serde::{Deserialize, Serialize};
use crate::org::model::Asn;
use super::RackId;

// Racks will implement ANYCAST DNS 
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rack {
    // pub meta: Metadata<Self>,
    // PE-LIM-1 (Country-City-Sequential Number)
    // pe-lim-1.chomba.org
    // pub seq: u32,
    pub id: RackId,
    pub asn: Asn // ZIP Code

    // pub nodes: HashMap<Id, RackNode>,
    // pub trunks: BTreeSet<TrunkId>,
    // pub trunk_count: u8, // Number of trunks used by nodes in the cluster
    // pub status: RackStatus
}

impl Default for Rack {
    fn default() -> Self {
        Self {
            id: RackId::new(),
            asn: Asn::try_from(4001).unwrap()
        }
    }
}

// During Setup
// 1) List all Nodes running rackd (list hostname, host links along with their MAC Addresses)
// Have the user select all nodes that will become part of the rackd cluster
// 2) List all Nodes added to the cluster and now have the user add TRUNK Links (e.g [RACK-CODE]TRUNK01)
// Then the user will go through each node and select which LINK should be used for which RACK trunk
