use std::marker::PhantomData;
use crate::{app::{actor::AppActor, error::AppError, lan::{models::LanName, query::GetLanOverlappings}, shared::query::{GetAll, GetByField}}, util::actor::{Payload, Process}};
use super::models::*;
use rusqlite::{named_params, Error, Result};
use types::{IpPrefix, Ipv6Prefix, Ipv6PrefixExt, Ipv6PrefixOverlap, Prefix};
use crate::app::data::DbSession;
use crate::app::data::framework::traits::DbQuery;
use crate::util::domain::Id;
use crate::util::net::*;

/// Query that returns all IPv6 LANs
pub struct GetAllLan6;

impl Payload for GetAllLan6 {
    type Err = AppError;
    type Ok = Vec<Lan6>;
}

impl Process for GetAllLan6 {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> std::result::Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        db.run(self)
    }
}

impl DbQuery for GetAllLan6 {
    type Ok = Vec<Lan6>;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetAll {
            entity: PhantomData::<Lan6>
        };
        db.run(query)
    }
}

/// Query that returns the IPv6 LAN with the specified id
pub struct GetLan6ById {
    pub id: Id
}

impl Payload for GetLan6ById {
    type Err = AppError;
    type Ok = Lan6;
}

impl Process for GetLan6ById {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> std::result::Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        db.run(self)
    }
}

impl DbQuery for GetLan6ById {
    type Ok = Lan6;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetByField {
            entity: PhantomData::<Lan6>,
            field_name: "id",
            field_value: self.id
        };
        db.run(query)
    }
}

/// Query that returns the IPv6 LAN with the specified name
pub struct GetLan6ByName {
    pub name: LanName
}

impl Payload for GetLan6ByName {
    type Err = AppError;
    type Ok = Lan6;
}

impl Process for GetLan6ByName {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> std::result::Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        db.run(self)
    }
}

impl DbQuery for GetLan6ByName {
    type Ok = Lan6;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetByField {
            entity: PhantomData::<Lan6>,
            field_name: "name",
            field_value: self.name
        };
        db.run(query)
    }
}

/// Query that computes the IPv6 Prefix of the specified IPV6 LAN Prefix
pub struct ComputeLan6Prefix {
    pub prefix: Lan6Prefix
}

impl Payload for ComputeLan6Prefix {
    type Err = AppError;
    type Ok = Ipv6Prefix;
}

impl Process for ComputeLan6Prefix {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        db.run(self)
    }
}

impl DbQuery for ComputeLan6Prefix {
    type Ok = Ipv6Prefix;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let sql = "SELECT ipv6_prefix FROM lan WHERE id = :id";
        match self.prefix {
            Lan6Prefix::Literal(value) => Ok(value),
            Lan6Prefix::Extension(Ipv6PrefixExt { id, ext }) => {
                let prefix: Ipv6Prefix = db.tx().query_row(sql, named_params! { ":id": id }, |row| Ok(row.get(0)?))?;
                if let Some(extended_prefix) = prefix.extend(ext) {
                    return Ok(extended_prefix);
                }
                Err(AppError::Lan6PrefixIsInvalid { prefix: self.prefix })?
            }
        }
    }
}

/// Query that returns all IPv6 LANs that overlap with the specified IPv6 Prefix
pub struct GetLan6Overlappings {
    pub prefix: Ipv6Prefix
}

impl Payload for GetLan6Overlappings {
    type Err = AppError;
    type Ok = Vec<Lan6>;
}

impl Process for GetLan6Overlappings {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        db.run(self)
    }
}

impl DbQuery for GetLan6Overlappings {
    type Ok = Vec<Lan6>;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetLanOverlappings {
            entity: PhantomData::<Lan6>,
            prefix: self.prefix
        };
        db.run(query)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::{app::lan6::{cmd::*, query::*}, system::System, util::net::types::Ipv6Prefix};

    fn sample_lan() -> (LanName, Lan6Prefix) {
        let name = LanName::new("HomeLab");
        let prefix = Lan6Prefix::Literal(Ipv6Prefix::from_str("2001:1388:1640:77ee::/64").unwrap());
        (name, prefix)
    }

    #[tokio::test]
    async fn can_get_overlapping_lans() {
        let app = System::mock().app;
        let prefix = Ipv6Prefix::from_str("2001:1388:1640:77ee::/64").unwrap();
        let cmd = CreateLan6 { 
            name: LanName::new("Homelab"), 
            prefix: Lan6Prefix::Literal(prefix) 
        };
        let _ = app.send(cmd).await.unwrap();

        let cmd = GetLan6Overlappings { prefix };
        let lans = app.send(cmd).await.unwrap();
        assert_eq!(lans.len(), 1);
    }

    #[tokio::test]
    async fn can_get_all_lans() {
        let app = System::mock().app;
        let (name, prefix) = sample_lan();
        let cmd = CreateLan6 { name: name.clone(), prefix };
        let _ = app.send(cmd).await.unwrap();

        let lans = app.send(GetAllLan6).await.unwrap();
        assert_eq!(lans.len(), 1);
    }
}
