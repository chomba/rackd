use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use crate::{actors::query::RackdQueryActor, db::{query::traits::{DbQuery, GetByKey, QueryRunner}, Tx}, net::{query::GetByName, NetName}, util::{actor::{Payload, Process}, query::GetByKeyError}, wan::{model::values::WanId, views::WanView}};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWanById {
    pub id: WanId
}

impl Payload for GetWanById {
    type Ok = WanView;
    type Err = GetByKeyError<WanId>;
}

impl DbQuery for GetWanById {
    type Ok = Option<WanView>;

    fn run(&self, tx: &rusqlite::Transaction) -> Result<Self::Ok, rusqlite::Error> {
        let query = GetByKey {
            key: "id",
            value: &self.id,
            view: PhantomData::<WanView>
        };
        query.run(&tx)
    }
}

impl Process for GetWanById {
    type Actor = RackdQueryActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.tx()?;
        let wan = self.run(&tx)?;
        let wan = match wan {
            Some(wan) => wan,
            None => Err(GetByKeyError::NotFound(self.id))?
        };
        // hydrate with data from NetLink
        Ok(wan)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWanByName {
    pub name: NetName
}

impl Payload for GetWanByName {
    type Ok = WanView;
    type Err = GetByKeyError<NetName>;
}

impl DbQuery for GetWanByName {
    type Ok = Option<WanView>;

    fn run(&self, tx: &rusqlite::Transaction) -> Result<Self::Ok, rusqlite::Error> {
        tx.run(GetByName { name: &self.name, view: PhantomData::<WanView> })
    }
}

impl Process for GetWanByName {
    type Actor = RackdQueryActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.tx()?;
        let wan = self.run(&tx)?;
        let wan = match wan {
            Some(wan) => wan,
            None => Err(GetByKeyError::NotFound(self.name))?
        };
        // hydrate with data from NetLink
        Ok(wan)
    }
}

pub mod casts {
    use crate::{actors::query::RackdQuery, util::actor::Msg, wan::query::WanQuery};
    use super::{GetWanById, GetWanByName};

    impl From<Msg<GetWanById>> for RackdQuery {
        fn from(query: Msg<GetWanById>) -> Self {
            Self::Wan(WanQuery::GetWanById(query))
        }
    }

    impl From<Msg<GetWanByName>> for RackdQuery {
        fn from(query: Msg<GetWanByName>) -> Self {
            Self::Wan(WanQuery::GetWanByName(query))
        }
    }
}

pub mod api {
    use axum::{extract::{OriginalUri, Path, State}, response::IntoResponse};
    use crate::{actors::system::Rackd, util::api::Response, wan::model::values::WanId};

    #[utoipa::path(get, path = "/wan/{wan_id}", tag = "wan",
        params(("wan_id" = WanId, Path, description = "Wan UUID")),
        responses((status = OK, body = Response))
    )]
    #[axum::debug_handler]
    pub async fn get_wan_by_id(State(rackd): State<Rackd>, OriginalUri(uri): OriginalUri, Path(wan_id): Path<WanId>) -> impl IntoResponse {
        let path = uri.path();
        let response = rackd.query(super::GetWanById { id: wan_id }).await
            .map(|wan_id| Response::ok(wan_id, path).to_axum_json())
            .unwrap_or_else(|error| Response::<()>::error(error, path).to_axum_json());
        (axum::http::StatusCode::OK, response).into_response()    
    }
}

// GetWanByTrunkVlan