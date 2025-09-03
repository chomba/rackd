use field_types::FieldName;
use serde::Deserialize;
use thiserror::Error;
use utoipa::ToSchema;
use crate::{actors::cmd::RackdCmdActor, db::{cmd::traits::EntityStore, query::traits::QueryRunner, Tx}, rack::Rack, trunk::{model::{Trunk, TrunkEvent, TrunkId, TrunkName}, query::{GetTrunkById, GetTrunkByName}, views::TrunkView}, util::{actor::{Payload, Process}, models::Entity, traits::OptionExt}};

#[derive(Debug, Deserialize, ToSchema, FieldName)]
pub struct CreateTrunk {
    pub name: TrunkName
}

#[derive(Debug, Error)]
pub enum CreateTrunkError {
    #[error("Db Error")]
    Db(#[from] rusqlite::Error),
    #[error("Rack hasn't been initialized")]
    RackUninitialized,
    #[error("Wan Name already in use")]
    NameAlreadyInUse
}

impl Payload for CreateTrunk {
    type Ok = TrunkId;
    type Err = CreateTrunkError;
}

impl CreateTrunk {
    fn exec(&self, rack: Option<Rack>, name_twin_trunk: Option<TrunkView>) -> Result<Trunk, CreateTrunkError> {
        let rack = rack.ok_or(CreateTrunkError::RackUninitialized)?;
        name_twin_trunk.err_or(CreateTrunkError::NameAlreadyInUse)?;
        let mut trunk = Trunk::default();
        trunk.process(TrunkEvent::Created {
            rack,
            id: TrunkId::new(),
            name: self.name.clone()
        });
        Ok(trunk)
    }
}

impl Process for CreateTrunk {
    type Actor = RackdCmdActor;
    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.tx()?;
        // get rack from KeyValueStore
        let rack = Some(Rack::default()); 
        let name_twin = tx.run(GetTrunkByName { name: self.name.clone() })?;
        self.exec(rack, name_twin).map(|mut trunk| {
            tx.save(&mut trunk)?;
            Ok(trunk.id)
        })?
    }
}

pub mod casts {
    use crate::{actors::cmd::RackdCmd, trunk::cmd::TrunkCmd, util::actor::Msg};
    use super::*;

    impl From<Msg<CreateTrunk>> for RackdCmd {
        fn from(cmd: Msg<CreateTrunk>) -> Self {
            Self::Trunk(TrunkCmd::Create(cmd))
        }
    }
}

pub mod api {
    use axum::{extract::{OriginalUri, State}, response::IntoResponse};
    use crate::{actors::system::Rackd, trunk::model::TrunkName, util::api::{Error, Json, Response, TryFromJson}};
    use super::{CreateTrunk, CreateTrunkError, CreateTrunkFieldName};
    use std::collections::HashMap;

    #[utoipa::path(post, path = "/trunk/create", tag = "trunk",
        request_body = CreateTrunk,
        responses((status = OK, body = Response))
    )]
    #[axum::debug_handler]
    pub async fn create(State(rackd): State<Rackd>, OriginalUri(uri): OriginalUri, Json(cmd): Json<CreateTrunk>) -> impl IntoResponse {
        let path = uri.path();
        let response = rackd.exec(cmd).await
            .map(|trunk_id| Response::ok(trunk_id, path).to_axum_json())
            .unwrap_or_else(|error| Response::<()>::error(error, path).to_axum_json());
        (axum::http::StatusCode::OK, response).into_response()
    }

    impl TryFromJson for CreateTrunk {
        fn try_from(mut map: HashMap<String, serde_json::Value>) -> Result<Self, Vec<Error>> {
            Self::check_keys(&map, Self::as_field_name_array().map(|f| f.name()))?;
            let name = map.remove(CreateTrunkFieldName::Name.name()).unwrap_or_default();
            match TrunkName::try_from(name) {
                Ok(name) => Ok(Self { name }),
                Err(e) => Err(vec![Error::from(e)])
            }
        }
    }

    impl From<CreateTrunkError> for Error {
        fn from(error: CreateTrunkError) -> Self {
            let msg = error.to_string();
            match error {
                CreateTrunkError::Db(_) => Self::new("CREATE_TRUNK_DB_ERROR", msg),
                CreateTrunkError::RackUninitialized => Self::new("CREATE_TRUNK_RACK_NOT_FOUND", msg),
                CreateTrunkError::NameAlreadyInUse => Self::new("CREATE_TRUNK_NAME_ALREADY_IN_USE", msg)
            }
        }
    }
}