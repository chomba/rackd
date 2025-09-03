use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::{actors::cmd::RackdCmdActor, db::{cmd::traits::EntityStore, Tx}, net::MacAddr, util::{actor::{Payload, Process}, models::Entity}, wan::model::{entity::{Wan, WanEvent}, values::WanId}};

#[derive(Debug, Serialize, Deserialize)]
pub struct SetMacAddr {
    id: WanId,
    mac: MacAddr
}

#[derive(Debug, Error)]
pub enum SetMacAddrError {
    #[error("Db Error")]
    Db(#[from] rusqlite::Error),
    #[error("Wan's MAC is already set")]
    AlreadySet,
    #[error("Wan with id x can't be found")]
    WanNotFound
}

impl Payload for SetMacAddr {
    type Err = SetMacAddrError;
    type Ok = ();
}

impl SetMacAddr {
    fn exec(&self, wan: Option<Wan>) -> Result<Wan, SetMacAddrError> {
        let mut wan = wan.ok_or(SetMacAddrError::WanNotFound)?;
        if wan.mac == self.mac {
            Err(SetMacAddrError::AlreadySet)?
        } else {
            wan.process(WanEvent::MacAddrSet { from: wan.mac, to: MacAddr::Auto });
            Ok(wan)
        }
    }
}

impl Process for SetMacAddr {
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
    use super::SetMacAddr;

    impl From<Msg<SetMacAddr>> for RackdCmd {
        fn from(cmd: Msg<SetMacAddr>) -> Self {
            Self::Wan(WanCmd::SetMacAddr(cmd))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::{actors::system::Rackd, net::{MacAddr, NetName, VlanId}, trunk::{cmd::create::CreateTrunk, model::TrunkName}, wan::{cmd::{create::CreateWan, set_mac::SetMacAddrError}, model::values::{WanId, WanMode}}};
    use super::SetMacAddr;

    #[tokio::test]
    async fn cant_set_mac_addr_if_wan_doesnt_exist() {
        let rackd = Rackd::mock().unwrap();
        let cmd = SetMacAddr {
            id: WanId::new(),
            mac: MacAddr::Auto
        };
        assert!(rackd.exec(cmd).await.is_err_and(|e| matches!(e, SetMacAddrError::WanNotFound)));
    }

    #[tokio::test]
    async fn cant_set_mac_addr_if_already_set() {
        let rackd = Rackd::mock().unwrap();
        let cmd = CreateTrunk {
            name: TrunkName::from_str("trunk1").unwrap()
        };
        let trunk_id = rackd.exec(cmd).await.unwrap();
        let cmd = CreateWan {
            trunk: trunk_id,
            vlan: VlanId::try_from(4001).unwrap(),
            name: NetName::from_str("verizon").unwrap(),
            mode: WanMode::IPoE
        };
        let wan_id = rackd.exec(cmd).await.unwrap();
        let cmd = SetMacAddr {
            id: wan_id,
            mac: MacAddr::Auto
        };
        assert!(rackd.exec(cmd).await.is_err_and(|e| matches!(e, SetMacAddrError::AlreadySet)));
    }
}