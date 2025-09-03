pub mod entity;
pub mod values;

pub use entity::*;
pub use values::*;

// Rack Commands (RackCmd) and Org Commands (OrgCmd) as well as Virtual/Overlay Network Commands (VNetCmd)
// need to be propagated using the Raft Consensus Algorithm
// On the key value store each rackd instance should only store:
// Local Rack Id and Local Org Id

// For example:
// Local Rack wants to change its ASN then the following command needs to be sent to that rack
// rack::cmd::SetAsn { asn: 4009 }
// Such command needs to be first propagate to most other racks before it can be processed
// Meaning all (majority) of racks will process the command in a lockstep
// Which also means, that every rack will know the identity (Id, ASN, Common Name, Location, etc)
// of every other rack in the organization

// Another example:
// The admin wants to change the name of the Organization, so it sends the following command to
// its local rackd instance: org::cmd::SetName { name: "McDonalds" }
// Such command is first forwarded to the Raft Leader and then propagated to the majority of racks
// ensuring that Organization Identity Information is always the same (consistent) across all racks in the organization

