use super::super::{actor::AppActor, error::AppError};
use super::models::*;
use super::query::*;
use crate::sys::link::models::{LinkDown, LinkStatus};
use crate::sys::link::cmd::{DisableLink, EnableLink};
use crate::sys::link::{cmd::TrackIspLink, models::LinkId};
use crate::sys::link::query::*;
use crate::util::actor::Process;
use crate::util::domain::Id;
use crate::util::actor::Payload;
use crate::util::net::types::PrefixLen;

/// Command that creates a new ISP
pub struct CreateIsp {
    pub name: IspName,
    pub link: LinkId,
    pub prefix_len: PrefixLen
}

impl Payload for CreateIsp {
    type Ok = Id;
    type Err = AppError;
}

impl Process for CreateIsp {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.lock().begin()?;

        let query = GetIspByName { name: self.name.clone() };
        if let Ok(isp) = db.run(query) {
            Err(AppError::IspNameAlreadyInUse { name: isp.name })?;
        }
        let query = GetIspByLink { link: self.link };
        if let Ok(isp) = db.run(query) {
            Err(AppError::IspLinkAlreadyInUse { link: isp.link_name, used_by: isp.name })?;
        }
        let query = GetLinkById { id: self.link };
        let link = match actor.sys.blocking_send(query) {
            Ok(link) => link,
            Err(_) => Err(AppError::IspLinkNotFound { id: query.id })?
        };

        let prefix = match self.prefix_len {
            PrefixLen::V4(prefix_len) => {
                let prefix = match link.ipv4_prefix(prefix_len) {
                    Some(prefix) => prefix,
                    None => Err(AppError::IspLinkHasNoIpv4Prefix { id: link.id })?
                };
                IspPrefix::V4(prefix)
            },
            PrefixLen::V6(prefix_len) => {
                let prefix = match link.ipv6_prefix(prefix_len) {
                    Some(prefix) => prefix,
                    None => Err(AppError::IspLinkHasNoIpv6Prefix { id: link.id })?
                };
                IspPrefix::V6(prefix)
            },
            PrefixLen::DualStack((ipv4_prefix_len, ipv6_prefix_len)) => {
                let ipv4_prefix = match link.ipv4_prefix(ipv4_prefix_len) {
                    Some(prefix) => prefix,
                    None => Err(AppError::IspLinkHasNoIpv4Prefix { id: link.id })?
                };
                let ipv6_prefix = match link.ipv6_prefix(ipv6_prefix_len) {
                    Some(prefix) => prefix,
                    None => Err(AppError::IspLinkHasNoIpv6Prefix { id: link.id })?
                };
                IspPrefix::DualStack((ipv4_prefix, ipv6_prefix))
            }
        };
        
        let tracker = match actor.sys.blocking_send(TrackIspLink { tracker: None, link: link.id, status: link.status, prefix }) {
            Ok(tracker) => tracker,
            Err(_) => Err(AppError::IspLinkCantBeTracked { id: link.id })?
        };

        let rank = IspRank(db.run(GetIspCount)? + 1);
        let mut isp = Isp::new(self.name, rank, (self.link, link.name, link.status), prefix, tracker);
        db.save(&mut isp)?;
        Ok(isp.id)
    }
}

/// Command that enables an administratively
/// disabled ISP Link
pub struct EnableIspLink {
    pub id: Id
}

impl Payload for EnableIspLink {
    type Ok = ();
    type Err = AppError;
}

impl Process for EnableIspLink {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        let query = GetIspById { id: self.id };
        let isp = db.run(query)?;

        if isp.link_status != LinkStatus::Down(LinkDown::AdminDown) {
            Err(AppError::IspLinkIsAlreadyEnabled { id: isp.link })?;
        }
        if let Err(_) = actor.sys.blocking_send(EnableLink { id: isp.link }) {
            Err(AppError::IspLinkCantBeEnabled { id: isp.link })?
        }
        Ok(())
    }
}

/// Command that administratively disables an ISP Link
pub struct DisableIspLink {
    pub id: Id
}

impl Payload for DisableIspLink {
    type Ok = ();
    type Err = AppError;
}

impl Process for DisableIspLink {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        let query = GetIspById { id: self.id };
        let isp = db.run(query)?;

        if isp.link_status == LinkStatus::Down(LinkDown::AdminDown) {
            Err(AppError::IspLinkIsAlreadyDisabled { id: isp.link })?;
        }
        if let Err(_) = actor.sys.blocking_send(DisableLink { id: isp.link }) {
            Err(AppError::IspLinkCantBeDisabled { id: isp.link })?
        }
        Ok(())
    }
}

// pub struct SyncRoutingTables;

// impl AppRequest for SyncRoutingTables {
//     type Ok = ();

//     fn process(self, actor: &mut AppActor) -> Result<Self::Ok, AppError> {
//         // create border66.conf in /etc/iproute2/rt_tables.d/
//         let db = actor.db.begin()?;
//         let isps = db.run(&GetAllIsps)?;

//         let file = match OpenOptions::new().read(true).write(true).create(true).open("/etc/iproute2/rt_tables.d/") {
//             Ok(file) => file,
//             Err(_) => Isp
//         };

//         // store border66.conf content in 
//         // let mut contents = String::new();
//         let mut w = Vec::new();
//         for isp in isps {
//             writeln!(&mut w, "{} isp{}", isp.rank, isp.rank)?;
//         }
//         Ok(())
//     }
// }



#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{sys::link::models::LinkName, system::System, util::net::types::PrefixLen};
    use super::*;
    
    #[tokio::test]
    async fn can_create_isp() {
        let system = System::mock();
        let app = system.app;
        let sys = system.sys;

        let query = GetLinkByName { name: LinkName::from_str("dummy1").unwrap() };
        let link = sys.send(query).await.unwrap();
        let cmd = CreateIsp { 
            name: IspName::from_str("Starlink").unwrap(), 
            link: link.id,   
            prefix_len: PrefixLen::V4(32) 
        };
        let new_isp = app.send(cmd).await.unwrap();

        let query = GetIspById { id: new_isp };
        let isp = app.send(query).await.unwrap();
        assert_eq!(new_isp, isp.id);
    }


    async fn wan_is_created_for_isp() {
        let system = System::mock();
        let app = system.app;
        let sys = system.sys;

        let query = GetLinkByName { name: LinkName::from_str("dummy1").unwrap() };
        let link = sys.send(query).await.unwrap();
        let cmd = CreateIsp { 
            name: IspName::from_str("Starlink").unwrap(), 
            link: link.id,   
            prefix_len: PrefixLen::V4(32) 
        };
        let new_isp = app.send(cmd).await.unwrap();
    }
}