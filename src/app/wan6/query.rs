use std::marker::PhantomData;
use rusqlite::named_params;
use crate::{app::{actor::AppActor, data::{framework::traits::DbQuery, DbSession}, error::AppError, shared::query::{GetAll, GetByField}, wan::{models::WanName, query::GetWanOverlappings}}, util::{actor::{Payload, Process}, domain::Id, net::types::{IpPrefix, Ipv6Prefix, Ipv6PrefixExt, Ipv6PrefixOverlap}}};

use super::models::{Wan6, Wan6Prefix};

/// Query that returns the IPv6 WAN with the specified id
pub struct GetWan6ById {
    pub id: Id
}

impl Payload for GetWan6ById {
    type Err = AppError;
    type Ok = Wan6;
}

impl Process for GetWan6ById {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        db.run(self)        
    }
}

impl DbQuery for GetWan6ById {
    type Ok = Wan6;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetByField {
            entity: PhantomData::<Wan6>,
            field_name: "id",
            field_value: self.id
        };
        db.run(query)
    }
}

/// Query that returns the IPv6 WAN with the specified name
pub struct GetWan6ByName {
    pub name: WanName
}

impl Payload for GetWan6ByName {
    type Err = AppError;
    type Ok = Wan6;
}

impl Process for GetWan6ByName {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        db.run(self)
    }
}

impl DbQuery for GetWan6ByName {
    type Ok = Wan6;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetByField {
            entity: PhantomData::<Wan6>,
            field_name: "name",
            field_value: self.name
        };
        db.run(query)
    }
}

/// Query that returns all IPv6 WANs
pub struct GetAllWan6;

impl Payload for GetAllWan6 {
    type Ok = Vec<Wan6>;
    type Err = AppError;
}

impl Process for GetAllWan6 {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        db.run(self)
    }
}

impl DbQuery for GetAllWan6 {
    type Ok = Vec<Wan6>;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetAll {
            entity: PhantomData::<Wan6>
        };
        db.run(query)
    }
}

/// Query that computes the actual IPv6 prefix
/// of the specified IPV6 WAN prefix
pub struct ComputeWan6Prefix {
    pub prefix: Wan6Prefix
}

impl Payload for ComputeWan6Prefix {
    type Err = AppError;
    type Ok = Ipv6Prefix;
}

impl Process for ComputeWan6Prefix {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        db.run(self)
    }
}

impl DbQuery for ComputeWan6Prefix {
    type Ok = Ipv6Prefix;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let sql = "SELECT prefix FROM wan WHERE id = :id";
        match self.prefix {
            Wan6Prefix::Isp(id) => {
                let prefix = db.tx().query_row(sql, named_params! { ":id": id }, |row| Ok(row.get(0)?))?;
                Ok(prefix)
            },
            Wan6Prefix::Extension(Ipv6PrefixExt { id, ext }) => {
                let prefix: Ipv6Prefix = db.tx().query_row(sql, named_params! { ":id": id }, |row| Ok(row.get(0)?))?;
                if let Some(extended_prefix) = prefix.extend(ext) {
                    return Ok(extended_prefix);
                }
                Err(AppError::Wan6PrefixIsInvalid { prefix: self.prefix })?
            }
        }  
    }
}

/// Query that returns all IPv6 WANs that overlap 
/// with the specified IPv6 Prefix
pub struct GetWan6Overlappings {
    pub prefix: Ipv6Prefix
}

impl Payload for GetWan6Overlappings {
    type Err = AppError;
    type Ok = Vec<Wan6>;
}

impl Process for GetWan6Overlappings {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        db.run(self)
    }
}

impl DbQuery for GetWan6Overlappings {
    type Ok = Vec<Wan6>;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetWanOverlappings {
            entity: PhantomData::<Wan6>,
            prefix: self.prefix
        };
        db.run(query)
    }
}