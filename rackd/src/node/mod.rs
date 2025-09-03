pub mod model;


pub struct RackNode {
    pub id: RackId,
    pub trunks: HashMap<TrunkName, LinkId> // trunk1, trunk2, trunk3
    // Once a Node is configured as part of a rack cluster
    // its name is managed by rackd and changed to [RACKCODE]-N01, [RACKCODE]-N02, [RACKCODE]-N03
}