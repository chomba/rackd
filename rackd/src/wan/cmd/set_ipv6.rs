// use serde::{Deserialize, Serialize};
// use thiserror::Error;
// use crate::{actors::cmd::RackdCmdActor, db::{cmd::traits::EntityStore, Tx}, wan::model::{values::{WanId, WanIpv6}, Wan, WanEvent}, util::{actor::{Payload, Process}, models::Entity}};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct SetIpv6 {
//     pub id: WanId,
//     pub ip: WanIpv6
// }

// #[derive(Debug, Error)]
// pub enum SetIpv6Error {
//     #[error("Db Error")]
//     Db(#[from] rusqlite::Error),
//     #[error("Wan with id x can't be found")]
//     WanNotFound,
//     #[error("Wan IPv6 settings not available for PPPoE Connections")]
//     ConnectionIsPPPoE,
//     #[error("Wan IPv6 Host already set")]
//     AlreadySet,

//     // #[error("Prefix Length needs to be greated than 64")]
//     // InvalidPrefixLength(Ipv6PrefixLen),
//     // #[error("IPv6 Address {0} isn't a valid GUA Address")]
//     // InvalidIpv6Address(Ipv6Addr),
//     // #[error("Gateway {0} isn't a valid LL Address")]
//     // InvalidIpv6Gateway(Ipv6Addr)
// }

// impl Payload for SetIpv6 {
//     type Ok = ();
//     type Err = SetIpv6Error;
// }

// impl SetIpv6 {
//     fn exec(&self, wan: Option<Wan>) -> Result<Wan, SetIpv6Error> {
//         let mut wan = wan.ok_or(SetIpv6Error::WanNotFound)?;

//         if wan.ip.ipv6 == self.ip {
//             Err(SetIpv6Error::AlreadySet)?
//         } else {
//             wan.process(WanEvent::Ipv6Set { from: wan.ip.ipv6.clone(), to: self.ip.clone() });
//             Ok(wan)
//         }

//         // TBD: Create struct: WanIpv6Host(Ipv6Host) so that WanIpv6Host::new(Ipv6Host)? returns an 
//         // error that is mapped to SetIpv6::InvalidHost
//         // Perhaps this should just be warnings displayed on the client side
//         // if self.host.addr.prefix_len.value() < 64 {
//         //     Err(SetIpv6ToStaticError::InvalidPrefixLength(self.host.addr.prefix_len))?;
//         // } else if !Ipv6Addr::is_global(&self.host.addr.addr) {
//         //     Err(SetIpv6ToStaticError::InvalidIpv6Address(self.host.addr.addr))?;
//         // } else if !Ipv6Addr::is_unicast_link_local(&self.host.gateway) {
//         //     Err(SetIpv6ToStaticError::InvalidIpv6Gateway(self.host.gateway))?;
//         // }
//     }
// }

// impl Process for SetIpv6 {
//     type Actor = RackdCmdActor;

//     fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
//         let tx = actor.conn.tx()?;
//         let wan = tx.load::<Wan>(self.id)?;
//         self.exec(wan).map(|mut wan| {
//             tx.save(&mut wan)?;
//             Ok(())
//         })?
//     }
// }