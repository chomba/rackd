use field_types::FieldName;
use serde::Deserialize;
use thiserror::Error;
use utoipa::ToSchema;
use crate::{actors::cmd::RackdCmdActor, db::{cmd::traits::EntityStore, query::traits::QueryRunner, Tx}, net::{query::{GetNetworkByName, GetNetworkByTrunkVlan}, views::NetworkView, NetName, VlanId}, rack::Rack, trunk::model::{Trunk, TrunkId}, util::{actor::{Payload, Process}, models::Entity, traits::OptionExt}, wan::model::{entity::{Wan, WanEvent}, values::{WanId, WanMode}}};

#[derive(Debug, Deserialize, ToSchema, FieldName)]
pub struct CreateWan {
    pub trunk: TrunkId,
    pub vlan: VlanId,
    pub name: NetName,
    pub mode: WanMode
}

#[derive(Debug, Error)]
pub enum CreateWanError {
    #[error("Db Error")]
    Db(#[from] rusqlite::Error),
    #[error("Rack hasn't been initialized")]
    RackNotFound,
    #[error("Trunk with ID not found")]
    TrunkNotFound,
    #[error("Wan Name already in use")]
    NameAlreadyInUse,
    #[error("VLAN/Trunk already in use")] // Add By 'NetworkKind' with ID
    TrunkVlanAlreadyInUse
}

impl Payload for CreateWan {
    type Ok = WanId;
    type Err = CreateWanError;
}

impl CreateWan {
    fn exec(&self, rack: Option<Rack>, trunk: Option<Trunk>, name_twin: Option<NetworkView>, trunk_vlan_twin: Option<NetworkView>) -> Result<Wan, CreateWanError> {
        let rack = rack.ok_or(CreateWanError::RackNotFound)?;
        let trunk = trunk.ok_or(CreateWanError::TrunkNotFound)?;
        name_twin.err_or(CreateWanError::NameAlreadyInUse)?;
        trunk_vlan_twin.err_or(CreateWanError::TrunkVlanAlreadyInUse)?;
        let mut wan = Wan::default();
        wan.process(WanEvent::Created {
            id: WanId::new(),
            rack, 
            trunk, 
            vlan: self.vlan, 
            name: self.name.clone(),
            mode: self.mode
        });
        Ok(wan)
    }
}

impl Process for CreateWan {
    type Actor = RackdCmdActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.tx()?;
        // get rack from KeyValueStore
        let rack = Some(Rack::default()); 
        let trunk = tx.load(self.trunk)?;
        let name_twin = tx.run(GetNetworkByName { name: self.name.clone() })?;
        let trunk_vlan_twin = tx.run(GetNetworkByTrunkVlan { trunk: self.trunk, vlan: self.vlan })?;
        self.exec(rack, trunk, name_twin, trunk_vlan_twin).map(|mut wan| {
            tx.save(&mut wan)?;
            Ok(wan.id)
        })?
    }
}

pub mod casts {
    use crate::{actors::cmd::RackdCmd, util::actor::Msg, wan::cmd::WanCmd};
    use super::*;

    // impl From<rusqlite::Error> for CreateWanError {
    //     fn from(error: rusqlite::Error) -> Self {
    //         Self::Db(format!("{error:?}"))
    //     }
    // }

    impl From<Msg<CreateWan>> for RackdCmd {
        fn from(cmd: Msg<CreateWan>) -> Self {
            Self::Wan(WanCmd::Create(cmd))
        }
    }
}

pub mod api {
    use std::collections::HashMap;
    use serde_json::Value;
    use axum::{extract::{OriginalUri, State}, response::IntoResponse};
    use crate::{actors::system::Rackd, net::{NetName, VlanId}, trunk::model::TrunkId, util::api::{Error, Json, JsonKeyError, Response, TryFromJson}, wan::model::values::WanMode};
    use super::{CreateWan, CreateWanError, CreateWanFieldName};

    #[utoipa::path(post, path = "/wan/create", tag = "wan",
        request_body = CreateWan,
        responses((status = OK, body = Response))
    )]
    #[axum::debug_handler]
    pub async fn create(State(rackd): State<Rackd>, OriginalUri(uri): OriginalUri, Json(cmd): Json<CreateWan>) -> impl IntoResponse {
        let path = uri.path();
        let response = rackd.exec(cmd).await
            .map(|wan_id| Response::ok(wan_id, path).to_axum_json())
            .unwrap_or_else(|error| Response::<()>::error(error, path).to_axum_json());
        (axum::http::StatusCode::OK, response).into_response()
    }

    impl TryFromJson for CreateWan {
        fn try_from(mut map: HashMap<String, Value>) -> Result<Self, Vec<Error>> {
            Self::check_keys(&map, CreateWan::as_field_name_array().map(|f| f.name()))?;
            let trunk = map.remove(CreateWanFieldName::Trunk.name()).unwrap_or_default();
            let vlan = map.remove(CreateWanFieldName::Vlan.name()).unwrap_or_default();
            let name = map.remove(CreateWanFieldName::Name.name()).unwrap_or_default();
            let mode = map.remove(CreateWanFieldName::Mode.name()).unwrap_or_default();

            match (TrunkId::try_from(trunk), VlanId::try_from(vlan), NetName::try_from(name), WanMode::try_from(mode)) {
                (Ok(trunk), Ok(vlan), Ok(name), Ok(mode)) => Ok(Self { trunk, vlan, name, mode }),
                (r1, r2, r3, r4) => {
                    let e1 = r1.map_err(|e| Error::from(e)).err();
                    let e2 = r2.map_err(|e| Error::from(e)).err();
                    let e3 = r3.map_err(|e| Error::from(e)).err();
                    let e4 = r4.map_err(|e| Error::from(e)).err();

                    let errors: Vec<Error> = [e1, e2, e3, e4].into_iter().filter_map(|e| e).collect();
                    Err(errors)
                }
            }
        }
    }

    impl From<CreateWanError> for Error {
        fn from(error: CreateWanError) -> Self {
            let msg = error.to_string();
            match error {
                CreateWanError::Db(_) => Error::new("CREATE_WAN_DB_ERROR", msg),
                CreateWanError::RackNotFound => Error::new("CREATE_WAN_RACK_NOT_FOUND", msg),
                CreateWanError::TrunkNotFound => Error::new("CREATE_WAN_TRUNK_NOT_FOUND", msg),
                CreateWanError::NameAlreadyInUse => Error::new("CREATE_WAN_NAME_ALREADY_IN_USE", msg),
                CreateWanError::TrunkVlanAlreadyInUse => Error::new("CREATE_WAN_TRUNK_VLAN_ALREADY_IN_USE", msg)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::{actors::system::Rackd, net::{NetName, VlanId}, trunk::{cmd::create::CreateTrunk, model::{TrunkId, TrunkName}}, wan::{cmd::create::{CreateWan, CreateWanError}, model::values::WanMode, *}};

    #[tokio::test]
    async fn cant_create_if_rack_doesnt_exist() {
        todo!()
    }

    #[tokio::test]
    async fn cant_create_if_trunk_doesnt_exist() {
        let rackd = Rackd::mock().unwrap();
        let cmd = CreateWan {
            trunk: TrunkId::new(),
            name: NetName::from_str("at&t").unwrap(),
            vlan: VlanId::try_from(1005).unwrap(),
            mode: WanMode::PPPoE
        };
        assert!(rackd.exec(cmd).await.is_err_and(|e| matches!(e, CreateWanError::TrunkNotFound)));
    }

    #[tokio::test]
    async fn cant_create_if_trunk_vlan_is_in_use() {
        let rackd = Rackd::mock().unwrap();
        let cmd = CreateTrunk {
            name: TrunkName::from_str("trunk1").unwrap()
        };
        let trunk_id = rackd.exec(cmd).await.unwrap();
        let cmd = CreateWan {
            trunk: trunk_id,
            vlan: VlanId::try_from(1005).unwrap(),
            name: NetName::from_str("at&t").unwrap(),
            mode: WanMode::IPoE
        };
        let _ = rackd.exec(cmd).await.unwrap();
        
        let cmd = CreateWan {
            trunk: trunk_id,
            vlan: VlanId::try_from(1005).unwrap(),
            name: NetName::from_str("verizon").unwrap(),
            mode: WanMode::IPoE
        };
        assert!(rackd.exec(cmd).await.is_err_and(|e| matches!(e, CreateWanError::TrunkVlanAlreadyInUse)));
        // TBD: 
        // CreateLan
    }

    #[tokio::test]
    async fn cant_create_if_name_is_in_use() {
        let rackd = Rackd::mock().unwrap();
        let cmd = CreateTrunk {
            name: TrunkName::from_str("trunk1").unwrap()
        };
        let trunk_id = rackd.exec(cmd).await.unwrap();
        let cmd = CreateWan { 
            trunk: trunk_id,
            vlan: VlanId::try_from(4002).unwrap(),
            name: NetName::from_str("Verizon").unwrap(),
            mode: WanMode::PPPoE
        };
        rackd.exec(cmd).await.unwrap();
        let cmd = CreateWan { 
            trunk: trunk_id,
            vlan: VlanId::try_from(4003).unwrap(),
            name: NetName::from_str("Verizon").unwrap(),
            mode: WanMode::PPPoE
        };

        assert!(rackd.exec(cmd).await.is_err_and(|e| matches!(e, CreateWanError::NameAlreadyInUse)));
    }
}