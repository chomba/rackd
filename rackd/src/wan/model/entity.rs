use serde::{Serialize, Deserialize};
use crate::{net::{Ipv4Params, MacAddr, NetName, VlanId}, rack::{Rack, RackId}, trunk::model::{Trunk, TrunkId}, util::models::{Entity, Id, Metadata}};
use super::values::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Wan {
    pub meta: Metadata,
    pub id: WanId,
    pub rack: RackId,
    pub trunk: TrunkId,
    pub vlan: VlanId,
    pub name: NetName,
    pub mode: WanMode,
    pub mac: MacAddr,
    pub ipv4: Ipv4Params,
    // pub ipv6: Ipv6Address,
    pub pppoe: WanPPPoE,
    pub dhcp6: WanDhcp6
}

impl Entity for Wan {
    type E = WanEvent;

    fn id(&self) -> Id {
        self.id.into()
    }

    fn metadata(&mut self) -> &mut Metadata {
        &mut self.meta
    }

    fn apply(&mut self, event: &Self::E) {
        match event {
            WanEvent::Created { id, rack, trunk, vlan, name, mode } => {
                self.id = *id;
                self.rack = rack.id;
                self.trunk = trunk.id;
                self.vlan = *vlan;
                self.name = name.clone();
                self.mode = *mode;
            },
            WanEvent::Renamed { to, .. } => {
                self.name = to.clone();
            },
            WanEvent::MacAddrSet { to, .. } => {
                self.mac = *to;
            },
            WanEvent::Ipv4ParamsSet { to, .. } => {
                self.ipv4 = *to;
            },
            // WanEvent::Ipv6Set { to, .. } => {
            //     self.ip.ipv6 = to.clone();
            // }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WanEvent {
    Created { id: WanId, rack: Rack, trunk: Trunk, vlan: VlanId, name: NetName, mode: WanMode },
    Renamed { from: NetName, to: NetName },
    MacAddrSet { from: MacAddr, to: MacAddr },
    Ipv4ParamsSet { from: Ipv4Params, to: Ipv4Params },
    // Ipv6AddrSet { from: WanIpv6, to: WanIpv6 }
    // Ipv6SetToRA(Ipv6SetToRA),
    // Ipv6SetToStatic(Ipv6SetToStatic),

    // Ipv4SetToDHCP(Ipv4SetToDHCP),
    // Ipv4SetToStatic(Ipv4SetToStatic)

    // Ipv4Disabled(Ipv4Disabled),

    // DHCPv6 IA for Prefix Delegation
    // Dhcp6IAPDAdded { iaid: u32, prefix_hint: Ipv6Prefix, preferred_lt: u32, valid_lt: u32 },
    // Dhcp6IAPDPrefixHintUpdated { iaid: u32, prefix_hint: Ipv6Prefix },
    // Dhcp6IAPDPreferredLTUpdated { iaid: u32, preferred_lt: u32 },
    // Dhcp6IAPDValidLTUpdated { iaid: u32, valid_lt: u32 },
    // DHCPv6 IA for Non-temporary Address
    // Dhcp6IANAAdded { iaid: u32, preferred_lt: u32, valid_lt: u32 },
    // DHCPv6 DUID
    // Dhcp6DuidSwitchedToAutoEN,
    // Dhcp6DuidSwitchedToEN { pen: u16, id: u128 },
    // Dhcp6DuidSwitchedToAutoLL,
    // Dhcp6DuidSwitchedToLL { hw_type: u16, mac: MacAddr6 },
    // Dhcp6DuidSwitchedToAutoLLT,
    // Dhcp6DuidSwitchedToLLT { hw_type: u16, mac: MacAddr6, time: u32 },
    // Dhcp6DuidSwitchedToRaw { value: u128 },



    // These 2 events should be processed by the Stats Collector and displayed by Graphana
//     LinkWentUp { up: LinkUp }, 
//     LinkWentDown { down: LinkDown },
}

pub mod casts {
    use crate::{trunk::model::TrunkEvent, util::models::EventData};
    use super::WanEvent;

    impl From<WanEvent> for EventData {
        fn from(e: WanEvent) -> Self {
            Self::Wan(e)
        }
    }

    impl From<TrunkEvent> for EventData {
        fn from(e: TrunkEvent) -> Self {
            Self::Trunk(e)
        }
    }
}