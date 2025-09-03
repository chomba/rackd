use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::{actors::cmd::RackdCmdActor, db::{cmd::traits::EntityStore, Tx}, net::Ipv4Params, util::{actor::{Payload, Process}, models::Entity}, wan::model::{entity::{Wan, WanEvent}, values::WanId}};

#[derive(Debug, Serialize, Deserialize)]
pub struct SetIpv4Params {
    pub id: WanId,
    pub ip: Ipv4Params
}

#[derive(Debug, Error)]
pub enum SetIpv4ParamsError {
    #[error("Db Error")]
    Db(#[from] rusqlite::Error),
    #[error("Rack not found")]
    RackNotFound,
    #[error("Wan with id x can't be found")]
    WanNotFound,
    #[error("Already Set")]
    AlreadySet
}

impl Payload for SetIpv4Params {
    type Ok = ();
    type Err = SetIpv4ParamsError;
}

impl SetIpv4Params {
    fn exec(&self, wan: Option<Wan>) -> Result<Wan, SetIpv4ParamsError> {
        let mut wan = wan.ok_or(SetIpv4ParamsError::WanNotFound)?;
        if wan.ipv4 == self.ip {
            Err(SetIpv4ParamsError::AlreadySet)?
        } else {
            wan.process(WanEvent::Ipv4ParamsSet { from: wan.ipv4.clone(), to: self.ip.clone() });
            Ok(wan)
        }
    }
}

impl Process for SetIpv4Params {
    type Actor = RackdCmdActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.tx()?;
        let wan = tx.load(self.id)?;
        self.exec(wan).map(|mut wan| {
            tx.save(&mut wan)?;
            Ok(())
        })?
    }
}

pub mod casts {
    use crate::{actors::cmd::RackdCmd, util::actor::Msg, wan::cmd::WanCmd};
    use super::SetIpv4Params;

    impl From<Msg<SetIpv4Params>> for RackdCmd {
        fn from(cmd: Msg<SetIpv4Params>) -> Self {
            Self::Wan(WanCmd::SetIpv4Params(cmd))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::{actors::system::Rackd, net::{Ipv4Params, NetName, VlanId}, trunk::{cmd::create::CreateTrunk, model::TrunkName}, wan::{cmd::{create::CreateWan, set_ipv4::{SetIpv4Params, SetIpv4ParamsError}}, model::values::{WanId, WanMode}}};

    #[tokio::test]
    async fn cant_set_ipv4_if_wan_doesnt_exist() {
        let rackd = Rackd::mock().unwrap();
        let cmd = SetIpv4Params {
            id: WanId::new(),
            ip: Ipv4Params::DHCP
        };
        assert!(rackd.exec(cmd).await.is_err_and(|e| matches!(e, SetIpv4ParamsError::WanNotFound)));
    }

    #[tokio::test]
    async fn cant_set_ipv4_if_already_set() {
        let rackd = Rackd::mock().unwrap();
        let cmd = CreateTrunk {
            name: TrunkName::from_str("trunk1").unwrap()
        };
        let trunk_id = rackd.exec(cmd).await.unwrap();
        let cmd = CreateWan {
            trunk: trunk_id,
            vlan: VlanId::try_from(4000).unwrap(),
            name: NetName::from_str("at&t").unwrap(),
            mode: WanMode::IPoE
        };
        let wan_id = rackd.exec(cmd).await.unwrap();
        assert!(matches!(Ipv4Params::default(), Ipv4Params::DHCP));
        let cmd = SetIpv4Params {
            id: wan_id, 
            ip: Ipv4Params::DHCP
        };
        assert!(rackd.exec(cmd).await.is_err_and(|e| matches!(e, SetIpv4ParamsError::AlreadySet)));
    }

    // #[tokio::test]
// async fn can_spoof_and_unspoof_mac() {
//     let api = new_api();
//     let cmd = Create { 
//         name: NetName::new("at&t").unwrap(), 
//         ..Default::default()
//     };
//     let id = api.send(cmd).await.unwrap();
//     let cmd = SetMacToSpoof { 
//         id,  mac: MacAddr6::new(0x76, 0xdc, 0x3a, 0x78, 0xaf, 0xd0) 
//     };
//     assert!(api.send(cmd).await.is_ok());
//     let cmd = SetMacToAuto { id };
//     assert!(api.send(cmd).await.is_ok());
//     let cmd = SetMacToSpoof { 
//         id,  mac: MacAddr6::new(0x76, 0xdc, 0x3a, 0x78, 0xaf, 0xd0) 
//     };
//     assert!(api.send(cmd).await.is_ok());
// }

// #[tokio::test]
// async fn can_switch_between_ipv6_modes() {
//     let api = new_api();
//     let cmd = Create { 
//         name: NetName::new("at&t").unwrap(), 
//         ..Default::default()
//     };
//     let id = api.send(cmd).await.unwrap();
//     let cmd = SetIpv6ToStatic {
//         id,
//         host: Ipv6Host { 
//             addr: Ipv6HostAddr {
//                 addr: Ipv6Addr::from_str("2800:200:44:8814:216:3eff:fe17:bb6f").unwrap(),
//                 prefix_len: Ipv6PrefixLen::new(64).unwrap()
//             },
//             gateway: Ipv6Addr::from_str("fe80::216:3eff:fe17:bb6f").unwrap()
//         }
//     };
//     assert!(api.send(cmd).await.is_ok());
//     let cmd = SetIpv6ToRA { id };
//     assert!(api.send(cmd).await.is_ok());
// }
}