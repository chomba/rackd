use crate::{app::{actor::AppActor, error::AppError, lan::models::LanName}, util::{actor::{Payload, Process}, domain::Id, net::types::Ipv6Prefix}};
use super::{models::*, query::*};

/// Command that creates an IPv6 LAN
pub struct CreateLan6 {
    pub name: LanName,
    pub prefix: Lan6Prefix
}

impl Payload for CreateLan6 {
    type Ok = Id;
    type Err = AppError;
}

impl Process for CreateLan6 {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.lock().begin()?;
        let query = GetLan6ByName { name: self.name.clone() };
        if let Ok(lan) = db.run(query) {
            Err(AppError::LanNameAlreadyInUse { name: lan.name })?;
        }

        let iprefix = db.run(ComputeLan6Prefix { prefix: self.prefix })?;
        let query = GetLan6Overlappings { prefix: iprefix };
        let overlaps = db.run(query)?;
        if !overlaps.is_empty() {
            Err(AppError::Lan6PrefixOverlaps { prefix: self.prefix, iprefix, overlaps })?;
        }

        let mut lan = Lan6::new(self.name, self.prefix, iprefix);
        db.save(&mut lan)?;
        Ok(lan.id)
        // Update This and Other Prefixes in reactor
    }
}

/// Command that renames an IPv6 LAN
pub struct RenameLan6 {
    pub id: Id,
    pub name: LanName
}

impl Payload for RenameLan6 {
    type Ok = ();    
    type Err = AppError;
}

impl Process for RenameLan6 {
    type Actor = AppActor;

    fn process(self, actor: &mut AppActor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.lock().begin()?;
        let mut lan = db.run(GetLan6ById { id: self.id })?;

        let query = GetLan6ByName { name: self.name.clone() };
        if let Ok(lan) = db.run(query) {
            Err(AppError::LanNameAlreadyInUse { name: lan.name })?;
        }
        lan.rename(self.name);
        db.save(&mut lan)?;
        Ok(())
    }
}

/// Command that updates the prefix of an IPv6 LAN
pub struct SetLan6Prefix {
    pub id: Id,
    pub prefix: Lan6Prefix
}

impl Payload for SetLan6Prefix {
    type Ok = ();
    type Err = AppError;
}

impl Process for SetLan6Prefix {
    type Actor = AppActor;

    fn process(self, actor: &mut AppActor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.lock().begin()?;
        let mut lan = db.run(GetLan6ById { id: self.id })?;
        // TBD
        // lan.set_kind((self.prefix, Ipv6Prefix::default()));
        db.save(&mut lan)?;
        Ok(())
    }
}

/// Command that deletes the IPv6 LAN with the specified id 
pub struct DeleteLan6 {
    pub id: Id
}

impl Payload for DeleteLan6 {
    type Ok = ();
    type Err = AppError;
}

impl Process for DeleteLan6 {
    type Actor = AppActor;

    fn process(self, actor: &mut AppActor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.lock().begin()?;
        let mut lan = db.run(GetLan6ById { id: self.id })?;
        lan.delete();
        db.save(&mut lan)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::{app::lan6::cmd::*, system::System};

    fn sample_lan() -> (LanName, Lan6Prefix) {
        let name = LanName::new("HomeLab");
        let prefix = Ipv6Prefix::from_str("2001:1388:1640:77ee::/64").unwrap();
        (name, Lan6Prefix::Literal(prefix))
    }

    #[tokio::test]
    async fn can_create_lan_using_available_data() {
        let app = System::mock().app;
        let (name, prefix) = sample_lan();
        let cmd = CreateLan6 { name: name.clone(), prefix };
        let new_lan = app.send(cmd).await.unwrap();

        let query = GetLan6ById { id: new_lan };
        let lan = app.send(query).await.unwrap();
        assert_eq!(new_lan, lan.id);
    }

    #[tokio::test]
    async fn cant_create_lan_using_overlapping_prefix() {
        let app = System::mock().app;
        let (name, prefix) = sample_lan();
        let cmd = CreateLan6 { name, prefix };
        let _ = app.send(cmd).await.unwrap();

        let new_prefix = prefix.clone();
        let cmd = CreateLan6 { name: LanName::new("Bob"), prefix: new_prefix };
        assert!(app.send(cmd).await.is_err());
    }

    #[tokio::test]
    async fn can_rename_lan_using_available_name() {
        let app = System::mock().app;
        let (original_name, prefix) = sample_lan();
        let cmd = CreateLan6 { name: original_name.clone(), prefix };
        let new_lan = app.send(cmd).await.unwrap();

        let new_name = LanName::new("Guests");
        assert!(new_name != original_name, "Test with an available name");
        let cmd = RenameLan6 { id: new_lan, name: new_name.clone() };
        let _ = app.send(cmd).await.unwrap();

        let query = GetLan6ById { id: new_lan };
        let renamed_lan = app.send(query).await.unwrap();
        assert_eq!(renamed_lan.name, new_name);
    }

    #[tokio::test]
    async fn cant_rename_lan_using_existing_name() {
        let app = System::mock().app;
        let (original_name, prefix) = sample_lan();
        let cmd = CreateLan6 { name: original_name.clone(), prefix };
        let new_lan = app.send(cmd).await.unwrap();

        let new_name = original_name.clone();
        assert!(new_name == original_name, "Test with an existing name");
        let cmd = RenameLan6 { id: new_lan, name: new_name.clone() };
        assert!(app.send(cmd).await.is_err());
    }

    #[tokio::test]
    async fn can_delete_lan() {
        let app = System::mock().app;
        let (name, original_prefix) = sample_lan();
        let cmd = CreateLan6 { name, prefix: original_prefix };
        let new_lan = app.send(cmd).await.unwrap();

        let cmd = DeleteLan6 { id: new_lan };
        let _ = app.send(cmd).await.unwrap();

        let query = GetLan6ById { id: new_lan };
        assert!(app.send(query).await.is_err());
    }
}