use std::marker::PhantomData;
use rusqlite::named_params;
use crate::{app::{actor::AppActor, data::{framework::traits::DbQuery, DbSession}, error::AppError, shared::query::*, wan::{models::*, query::GetWanOverlappings}}, util::{actor::{Payload, Process}, domain::Id, net::types::*}};
use super::models::*;

/// Query that returns the IPv4 WAN with the specified id
pub struct GetWan4ById {
    pub id: Id
}

impl Payload for GetWan4ById {
    type Ok = Wan4;
    type Err = AppError;
}

impl Process for GetWan4ById {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        self.run(&db)
    }
}

impl DbQuery for GetWan4ById {
    type Ok = Wan4;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetByField {
            entity: PhantomData::<Wan4>,
            field_name: "id",
            field_value: self.id
        };
        db.run(query)
    }
}

/// Query that returns the IPv4 WAN with the specified WanName
pub struct GetWan4ByName {
    pub name: WanName
}

impl Payload for GetWan4ByName {
    type Ok = Wan4;
    type Err = AppError;
}

impl Process for GetWan4ByName {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        self.run(&db)
    }
}

impl DbQuery for GetWan4ByName {
    type Err = AppError;
    type Ok = Wan4;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetByField {
            entity: PhantomData::<Wan4>,
            field_name: "name",
            field_value: self.name
        };
        db.run(query)
    }
}

/// Query that returns all IPv4 WANs
pub struct GetAllWan4;

impl Payload for GetAllWan4 {
    type Err = AppError;
    type Ok = Vec<Wan4>;
}

impl Process for GetAllWan4 {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        self.run(&db)
    }
}

impl DbQuery for GetAllWan4 {
    type Err = AppError;
    type Ok = Vec<Wan4>;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetAll { entity: PhantomData::<Wan4> };
        db.run(query)
    }
}

/// Query that computes the IPv4 Prefix of the specified IPv4 WAN Prefix
pub struct ComputeWan4Prefix {
    pub prefix: Wan4Prefix
}

impl Payload for ComputeWan4Prefix {
    type Ok = Ipv4Prefix;
    type Err = AppError;
}

impl Process for ComputeWan4Prefix {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        self.run(&db)
    }
}

impl DbQuery for ComputeWan4Prefix {
    type Ok = Ipv4Prefix;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let sql = "SELECT prefix FROM wan WHERE id = :id";
        match self.prefix {
            Wan4Prefix::Isp(id) => {
                let prefix = db.tx().query_row(sql, named_params! { ":id": id }, |row| Ok(row.get(0)?))?;
                Ok(prefix)
            },
            Wan4Prefix::Extension(Ipv4PrefixExt { id, ext }) => {
                let prefix: Ipv4Prefix = db.tx().query_row(sql, named_params! { ":id": id }, |row| Ok(row.get(0)?))?;
                if let Some(extended_prefix) = prefix.extend(ext) {
                    return Ok(extended_prefix);
                }
                Err(AppError::Wan4PrefixIsInvalid { prefix: self.prefix })?
            }
        }  
    }
}

/// Query that returns all Wan4 entities
/// that overlap with the specified Ipv4 Prefix
pub struct GetWan4Overlappings {
    pub prefix: Ipv4Prefix
}

impl Payload for GetWan4Overlappings {
    type Ok = Vec<Wan4>;
    type Err = AppError;
}

impl Process for GetWan4Overlappings {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        self.run(&db)
    }
}

impl DbQuery for GetWan4Overlappings {
    type Ok = Vec<Wan4>;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetWanOverlappings {
            entity: PhantomData::<Wan4>,
            prefix: self.prefix
        };
        db.run(query)
    }
}
