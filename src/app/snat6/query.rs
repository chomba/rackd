use rusqlite::{named_params, Error, Result};
use super::models::*;
use crate::app::data::framework::traits::{DbQuery, MapRow};
use crate::app::data::DbSession;
use crate::app::error::AppError;
use crate::util::domain::Id;
use crate::util::net::types::{IpPrefix, Ipv6Prefix, Ipv6PrefixExt};

pub struct GetSNat6ById {
    pub id: Id
}

impl DbQuery for GetSNat6ById {
    type Ok = SNat6;
    type Err = AppError;
    
    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let sql = format!("{} WHERE id = :id", SNat6::select());
        match db.tx().query_row(&sql, named_params! { ":id": self.id }, SNat6::map) {
            Ok(nat) => Ok(nat),
            Err(Error::QueryReturnedNoRows) => Err(AppError::SNat6TargetNotFound { id: self.id })?,
            Err(e) => Err(e)?
        }
    }
}

pub struct ComputeSNat6Prefix {
    pub prefix: SNat6Prefix
}

impl DbQuery for ComputeSNat6Prefix {
    type Ok = Ipv6Prefix;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let sql = "SELECT ipv6_prefix FROM lan WHERE id = :id";
        match self.prefix {
            SNat6Prefix::Literal(value) => Ok(value),
            SNat6Prefix::Lan(id) => {
                match db.tx().query_row(sql, named_params! { ":id": id }, |row| Ok(row.get(1)?)) {
                    Ok(prefix) => Ok(prefix),
                    Err(_) => Err(AppError::SNat6PrefixIsInvalid { prefix: self.prefix })?
                }
            },
            SNat6Prefix::LanExtension(Ipv6PrefixExt { id, ext }) => {
                let prefix: Ipv6Prefix = match db.tx().query_row(sql, named_params! { ":id": id }, |row| Ok(row.get(1)?)) {
                    Ok(prefix) => prefix,
                    Err(_) => Err(AppError::SNat6PrefixIsInvalid { prefix: self.prefix })?
                };

                if let Some(extended_prefix) = prefix.extend(ext) {
                    return Ok(extended_prefix);
                }
                Err(AppError::SNat6PrefixIsInvalid { prefix: self.prefix })?
            }
        }
    }
}

pub struct ComputeSNat6TargetPrefix {
    pub prefix: SNat6TargetPrefix
}

impl DbQuery for ComputeSNat6TargetPrefix {
    type Ok = Ipv6Prefix;
    type Err = AppError;
    
    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let sql = "SELECT ipv6_prefix FROM wan WHERE id := id";
        match self.prefix {
            SNat6TargetPrefix::Wan(id) => {
                match db.tx().query_row(sql, named_params! { ":id": id }, |row| Ok(row.get(0)?)) {
                    Ok(prefix) => Ok(prefix),
                    Err(_) => Err(AppError::SNat6HasInvalidTargetPrefix { prefix: self.prefix })?
                }
            },
            SNat6TargetPrefix::WanExtension(Ipv6PrefixExt { id, ext }) => {
                let prefix: Ipv6Prefix = match db.tx().query_row(sql, named_params! { ":id": id }, |row| Ok(row.get(0)?)) {
                    Ok(prefix) => prefix,
                    Err(_) => Err(AppError::SNat6HasInvalidTargetPrefix { prefix: self.prefix })?
                };

                if let Some(extended_prefix) = prefix.extend(ext) {
                    return Ok(extended_prefix);
                }
                Err(AppError::SNat6HasInvalidTargetPrefix { prefix: self.prefix })?
            }
        }
    }
}

pub struct GetAllSNats;

impl DbQuery for GetAllSNats {
    type Ok = Vec<SNat6>;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let mut nats = vec![];
        let sql = format!("{}", SNat6::select());
        let mut stmt = db.tx().prepare(&sql)?;
        let rows = stmt.query_map((), SNat6::map)?;
        for row in rows {
            nats.push(row?);
        }
        Ok(nats)
    }
}

pub struct GetSNat6Overlappings {
    pub prefix: Ipv6Prefix
}

impl DbQuery for GetSNat6Overlappings {
    type Ok = Vec<SNat6>;
    type Err = AppError;
    
    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let snats =  GetAllSNats.run(db).unwrap();
        let overlaps = snats.into_iter().filter(|nat| self.prefix.overlaps(nat.iprefix).is_some()).collect();
        Ok(overlaps)
    }
}

pub struct GetAllSNat6Targets;

impl DbQuery for GetAllSNat6Targets {
    type Ok = Vec<SNat6Target>;
    type Err = AppError;
    
    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let mut targets = vec![];
        let mut stmt = db.tx().prepare(SNat6Target::select())?;
        let rows = stmt.query_map((), SNat6Target::map)?;
        for row in rows {
            targets.push(row?);
        }
        Ok(targets)
    }
}

pub struct GetSNat6TargetOverlappings {
    pub prefix: Ipv6Prefix
}

impl DbQuery for GetSNat6TargetOverlappings {
    type Ok = Vec<SNat6Target>;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let targets = GetAllSNat6Targets.run(db)?;
        let overlaps = targets.into_iter().filter(|target| self.prefix.overlaps(target.iprefix).is_some()).collect();
        Ok(overlaps)
    }
}

pub struct GetSNatTargetById {
    pub id: Id,
    pub snat_id: Id
}

impl DbQuery for GetSNatTargetById {
    type Ok = SNat6Target;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let sql = format!("{} WHERE id = :id AND snat_id = :snat_id", SNat6Target::select());
        match db.tx().query_row(&sql, named_params! { ":id": self.id, ":snat_id": self.snat_id }, SNat6Target::map) {
            Ok(target) => Ok(target),
            Err(Error::QueryReturnedNoRows) => Err(AppError::SNat6TargetNotFound { id: self.id })?,
            Err(e) => Err(e)? 
        }
    }
}

pub struct ComputeSNatStatusForMode {
    pub mode: SNat6Mode
}

impl DbQuery for ComputeSNatStatusForMode {
    type Ok = SNat6Status;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        // TBD
        Ok(SNat6Status::default())
        // get targets from mode
        // DbQuery snat_targets and return a status
    }
}