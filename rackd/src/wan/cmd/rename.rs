use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::{actors::cmd::RackdCmdActor, db::{cmd::traits::EntityStore, query::traits::QueryRunner, Tx}, net::{query::GetNetworkByName, views::NetworkView, NetName}, util::{actor::{Payload, Process}, models::Entity, traits::OptionExt}, wan::model::{entity::{Wan, WanEvent}, values::WanId}};

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameWan {
    pub id: WanId,
    pub name: NetName
}

#[derive(Debug, Error)]
pub enum RenameWanError {
    #[error("Db Error")]
    Db(#[from] rusqlite::Error),
    #[error("Rack not found")]
    RackNotFound,
    #[error("Wan with id x Not Found")]
    WanNotFound,
    #[error("Wan Name already in use")]
    NameAlreadyInUse
}

impl Payload for RenameWan {
    type Ok = ();
    type Err = RenameWanError;
}

impl RenameWan {
    fn exec(&self, wan: Option<Wan>, name_twin: Option<NetworkView>) -> Result<Wan, RenameWanError> {
        let mut wan = wan.ok_or(RenameWanError::WanNotFound)?;
        name_twin.err_or(RenameWanError::NameAlreadyInUse)?;
        wan.process(WanEvent::Renamed { from: wan.name.clone(), to: self.name.clone() });
        Ok(wan)
    }
}

impl Process for RenameWan {
    type Actor = RackdCmdActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.tx()?;
        let wan = tx.load(self.id)?;
        let name_twin = tx.run(GetNetworkByName { name: self.name.clone() })?;
        self.exec(wan, name_twin).map(|mut wan| {
            tx.save(&mut wan)?;
            Ok(())
        })?
    }
}

pub mod casts {
    use crate::{actors::cmd::RackdCmd, util::actor::Msg, wan::cmd::WanCmd};
    use super::RenameWan;

    impl From<Msg<RenameWan>> for RackdCmd {
        fn from(cmd: Msg<RenameWan>) -> Self {
            Self::Wan(WanCmd::Rename(cmd))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::{actors::system::Rackd, net::{NetName, VlanId}, trunk::{cmd::create::CreateTrunk, model::TrunkName}, wan::{cmd::{create::CreateWan, rename::{RenameWan, RenameWanError}}, model::values::{WanId, WanMode}}};

    #[tokio::test]
    async fn cant_rename_if_wan_doesnt_exists() {
        let rackd = Rackd::mock().unwrap();
        let cmd = RenameWan {
            id: WanId::new(),
            name: NetName::from_str("at&t").unwrap()
        };
        assert!(rackd.exec(cmd).await.is_err_and(|e| matches!(e, RenameWanError::WanNotFound)));
    }

    #[tokio::test]
    async fn cant_rename_if_new_name_is_already_in_use() {
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
        let att_id = rackd.exec(cmd).await.unwrap();
        let cmd = CreateWan {
            trunk: trunk_id,
            vlan: VlanId::try_from(4001).unwrap(),
            name: NetName::from_str("verizon").unwrap(),
            mode: WanMode::PPPoE
        };
        let _ = rackd.exec(cmd).await.unwrap();
        
        // Trying to rename with the exact same name
        let cmd = RenameWan {
            id: att_id,
            name: NetName::from_str("at&t").unwrap()
        };
        assert!(rackd.exec(cmd).await.is_err_and(|e| matches!(e, RenameWanError::NameAlreadyInUse)));
        
        // Trying to rename with the same name but different case
        let cmd = RenameWan {
            id: att_id,
            name: NetName::from_str("AT&T").unwrap()
        };
        assert!(rackd.exec(cmd).await.is_err_and(|e| matches!(e, RenameWanError::NameAlreadyInUse)));
        
        // Trying to rename with a name used by another WAN
        let cmd = RenameWan {
            id: att_id,
            name: NetName::from_str("verizon").unwrap()
        };
        assert!(rackd.exec(cmd).await.is_err_and(|e| matches!(e, RenameWanError::NameAlreadyInUse)));
    }
}