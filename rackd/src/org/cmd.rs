
// use thiserror::Error;

// pub struct Create {
//     pub name: OrgName,
//     pub domain: OrgDomain,
//     pub prefix: Ipv6Prefix
// }

// #[derive(Error, Debug)]
// pub enum CreateOrgError {
//     #[error("Prefix Length {0} is too large (expected <= {max})", max = 48)]
//     PrefixTooLarge(u8),
//     #[error("Unknown")]
//     Unknown(#[from] rusqlite::Error)
// }

// impl Payload for CreateOrg {
//     type Ok = Id;
//     type Err = CreateOrgError;
// }

// impl Process for CreateOrg {
//     type Actor = AppActor;  

//     fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
//         let db = actor.db.lock().begin()?;

//         if self.prefix.len > 48 {
//             Err(CreateOrgError::PrefixTooLarge(self.prefix.len))?
//         }

//         let mut org = Org::new(self.name, self.domain, self.prefix);
//         db.save_commit(&mut org)?;
//         Ok(org.id)
//     }  
// }

// pub struct RenameOrg {
//     pub name: OrgName
// }

// #[derive(Debug, Error)]
// pub enum RenameOrgError {
//     #[error("Name {0} is already in use")]
//     NameAlreadyInUse(OrgName),
//     #[error("Unknown")]
//     Unknown(#[from] rusqlite::Error)
// }

// impl Payload for RenameOrg {
//     type Ok = ();
//     type Err = RenameOrgError;
// }

// impl Process for RenameOrg {
//     type Actor = AppActor;

//     fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
//         let db = actor.db.lock().begin()?;

//         // TBD
//         Ok(())    
//     }
// }