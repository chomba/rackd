use crate::{app::{actor::AppActor, error::AppError, shared::domain::EventHandler}, sys::link::models::{LinkDown, LinkId, LinkUp}, util::{actor::{Payload, Process}, net::types::Ipv6Prefix}};
use super::query::*;

// pub struct IspNeedsTrackingEvent {
//     pub id: Id
// }

// impl Payload for IspNeedsTrackingEvent {
//     type Ok = ();
//     type Err = AppError;
// }

// // MOVE THIS EVENT TO THE SYSACTOR
// impl Process<IspNeedsTrackingEvent> for AppActor {
//     fn process(self, actor: &mut AppActor) -> Result<Self::Ok, AppError> {
//         let db = actor.db.begin()?;
//         let query = GetIspById { id: self.id };
//         let mut isp = match db.run(&query) {
//             Ok(isp) => isp,
//             Err(_) => return Ok(())
//         };
        
//         // TBD
//         Ok(())
//     }
// }

// IspLinkIpv6AddressChanged
// IspLinkIpv4AddressChanged

pub struct IspLinkWentUp {
    pub id: LinkId,
    pub up: LinkUp
}

impl Payload for IspLinkWentUp {
    type Ok = ();
    type Err = AppError;
}

impl Process for IspLinkWentUp {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        let query = GetIspByLink { link: self.id };
        let mut isp = match db.run(query) {
            Ok(isp) => isp,
            Err(_) => return Ok(())
        };
        isp.handle(&self);
        db.save(&mut isp);
        Ok(())
    }
}

pub struct IspLinkWentDown {
    pub id: LinkId,
    pub down: LinkDown
}

impl Payload for IspLinkWentDown {
    type Ok = ();
    type Err = AppError;
}

impl Process for IspLinkWentDown {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        let query = GetIspByLink { link: self.id };
        let mut isp = match db.run(query) {
            Ok(isp) => isp,
            Err(_) => return Ok(())
        };
        isp.handle(&self);
        db.save(&mut isp);
        Ok(())
    }
}

pub struct IspLinkPrefixChanged {
    pub id: LinkId,
    pub prefix: Ipv6Prefix
}

impl Payload for IspLinkPrefixChanged {
    type Ok = ();
    type Err = AppError;
}

impl Process for IspLinkPrefixChanged {
    type Actor = AppActor;
    
    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        Ok(())
    }
}
