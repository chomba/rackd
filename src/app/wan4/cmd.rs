use crate::app::wan::models::*;
use crate::{app::{actor::AppActor, error::AppError}, util::{actor::{Payload, Process}, domain::Id, net::types::{Ipv4PrefixExt, Ipv6PrefixExt, PrefixExt}}};
use super::query::*;
use super::models::*;

pub struct CreateWan4Cmd {
    pub name: WanName,
    pub ext: Ipv4PrefixExt
}

impl Payload for CreateWan4Cmd {
    type Ok = Id;
    type Err = AppError;
}

impl Process for CreateWan4Cmd {
    type Actor = AppActor;
    
    fn process(self, actor: &mut AppActor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.lock().begin()?;
        let query = GetWan4ByName { name: self.name.clone() };
        if let Ok(wan) = db.run(query) {
            Err(AppError::WanNameAlreadyInUse { name: wan.name })?
        }
        let prefix = Wan4Prefix::Extension(self.ext);
        let iprefix = db.run(ComputeWan4Prefix { prefix })?;
        let overlaps = db.run(GetWan4Overlappings { prefix: iprefix })?;
        if !overlaps.is_empty() {
            Err(AppError::Wan4PrefixOverlaps { prefix, iprefix, overlaps })?;
        }

        let mut wan = Wan4::new(Id::new(), self.name, prefix, iprefix);
        db.save(&mut wan)?;
        Ok(wan.id)
    }
}

pub struct RenameWan4Cmd {
    pub id: Id,
    pub name: WanName
}

impl Payload for RenameWan4Cmd {
    type Ok = ();    
    type Err = AppError;
}

impl Process for RenameWan4Cmd {
    type Actor = AppActor;

    fn process(self, actor: &mut AppActor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.lock().begin()?;
        let mut wan = db.run(GetWan4ById { id: self.id })?;

        let query = GetWan4ByName { name: self.name.clone() };
        if let Ok(wan) = db.run(query) {
            Err(AppError::WanNameAlreadyInUse { name: wan.name })?;
        }
        wan.rename(self.name);
        db.save(&mut wan)?;
        Ok(())
    }
}

pub struct DeleteWan4Cmd {
    pub id: Id
}

impl Payload for DeleteWan4Cmd {
    type Ok = ();
    type Err = AppError;
}

impl Process for DeleteWan4Cmd {
    type Actor = AppActor;
    
    fn process(self, actor: &mut AppActor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.lock().begin()?;
        let mut wan = db.run(GetWan4ById { id: self.id })?;
        wan.delete();
        db.save(&mut wan)?;
        Ok(())
    }
}