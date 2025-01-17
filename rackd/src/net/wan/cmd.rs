pub mod casts;
use std::{marker::PhantomData, net::Ipv6Addr};
use macaddr::MacAddr6;
use thiserror::Error;
use crate::{actors::cmd::RackdCmdActor, db::{cmd::traits::*, query::traits::*}, net::shared::models::*, org::rack::models::RackId, util::{actor::*, models::Event, traits::*}};
use super::{models::*, query::{views::Wan, GetWanById, GetWanByName}};

pub enum WanCmd {
    Create(Msg<Create>),
    Rename(Msg<Rename>)
}

pub struct Create {
    pub rack: RackId, 
    pub trunk: TrunkId, 
    pub vlan: VlanId, 
    pub conn: WanConnection, 
    pub name: NetName
}

#[derive(Debug, Error)]
pub enum CreateError {
    #[error("Db Error")]
    Db(#[from] rusqlite::Error),
    #[error("Wan Name already in use")]
    NameAlreadyInUse,
    #[error("VLAN/Trunk already in use")]
    VLANTrunkAlreadyInUse
}

impl Execute for Create {
    type In = Option<Wan>;
    type Out = (WanId, Event);
    
    fn exec(self, namesake_isp: Self::In) -> Result<Self::Out, Self::Err> {
        namesake_isp.err_or(CreateError::NameAlreadyInUse)?;
        let e = Created {
            id: WanId::new(),
            rack: self.rack,
            trunk: self.trunk,
            vlan: self.vlan,
            conn: self.conn,
            name: self.name
        };
        Ok((e.id, Event::new(e.id, e.into())))
    }    
}

impl Payload for Create {
    type Ok = WanId;
    type Err = CreateError;
}

impl Process for Create {
    type Actor = RackdCmdActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.transaction()?;
        let namesake_isp = tx.run(GetWanByName { name: self.name.clone(), view: PhantomData::<Wan> })?;
        self.exec(namesake_isp).map(|(id, e)| {
            let _ = tx.save(&e)?;
            Ok(id)
        })?
    }
}

pub struct Rename {
    pub id: WanId,
    pub name: NetName
}

impl Execute for Rename {
    type In = (Option<Wan>, Option<Wan>);
    type Out = Event;

    fn exec(self, (isp, namesake_isp): Self::In) -> Result<Self::Out, Self::Err> {
        isp.ok_or(RenameError::WanNotFound)?;
        namesake_isp.err_or(RenameError::NameAlreadyInUse)?;
        let e = Renamed { name: self.name.clone() };
        Ok(Event::new(self.id, e.into()))
    }
}

#[derive(Debug, Error)]
pub enum RenameError {
    #[error("Db Error")]
    Db(#[from] rusqlite::Error),
    #[error("Wan with id x Not Found")]
    WanNotFound,
    #[error("Wan Name already in use")]
    NameAlreadyInUse
}

impl Payload for Rename {
    type Ok = ();
    type Err = RenameError;
}

impl Process for Rename {
    type Actor = RackdCmdActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.transaction()?;
        let isp = tx.run(GetWanById { id: self.id, view: PhantomData::<Wan> })?;
        let namesake_isp = tx.run(GetWanByName { name: self.name.clone(), view: PhantomData::<Wan> })?;
        self.exec((isp, namesake_isp)).map(|e| {
            let _ = tx.save(&e)?;
            Ok(())
        })?
    }
}

pub struct SpoofMac {
    id: WanId,
    mac: MacAddr6
}

#[derive(Debug, Error)]
pub enum SpoofMacError {
    #[error("Db Error")]
    Db(#[from] rusqlite::Error),
    #[error("Wan with id x can't be found")]
    WanNotFound,
    #[error("Reserved MAC Addresses can't be used")]
    UnusableMACAddress
}

impl Execute for SpoofMac {
    type In = Option<Wan>;
    type Out = Event;

    fn exec(self, isp: Self::In) -> Result<Self::Out, Self::Err> {
        let _ = isp.ok_or(SpoofMacError::WanNotFound)?;
        let e = MacSpoofed { mac: self.mac };
        Ok(Event::new(self.id, e.into()))
    }
}

impl Payload for SpoofMac {
    type Err = SpoofMacError;
    type Ok = ();
}

impl Process for SpoofMac {
    type Actor = RackdCmdActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.transaction()?;
        let isp = tx.run(GetWanById { id: self.id, view: PhantomData::<Wan> })?;
        self.exec(isp).map(|e| {
            let _ = tx.save(&e)?;
            Ok(())
        })?
    }
}

pub struct UnspoofMac {
    pub id: WanId
}

#[derive(Debug, Error)]
pub enum UnspoofMacError {
    #[error("Db Error")]
    Db(#[from] rusqlite::Error),
    #[error("Wan with id x can't be found")]
    WanNotFound
}

impl Execute for UnspoofMac {
    type In = Option<Wan>;
    type Out = Event;

    fn exec(self, isp: Self::In) -> Result<Self::Out, Self::Err> {
        let _ = isp.ok_or(UnspoofMacError::WanNotFound)?;
        Ok(Event::new(self.id, MacUnspoofed.into()))
    }
}

impl Payload for UnspoofMac {
    type Ok = ();
    type Err = UnspoofMacError;
}

impl Process for UnspoofMac {
    type Actor = RackdCmdActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.transaction()?;
        let isp = tx.run(GetWanById { id: self.id, view: PhantomData::<Wan> })?;
        self.exec(isp).map(|e| {
            let _ = tx.save(&e)?;
            Ok(())
        })?
    }
}

pub struct SetIpv6ToStatic {
    pub id: WanId,
    pub host: Ipv6Host
}

#[derive(Debug, Error)]
pub enum SetIpv6ToStaticError {
    #[error("Db Error")]
    Db(#[from] rusqlite::Error),
    #[error("Wan with id x can't be found")]
    WanNotFound,
    #[error("Prefix Length needs to be greated than 64")]
    InvalidPrefixLength(Ipv6PrefixLen),
    #[error("IPv6 Address {0} isn't a valid GUA Address")]
    InvalidIpv6Address(Ipv6Addr),
    #[error("Gateway {0} isn't a valid LL Address")]
    InvalidIpv6Gateway(Ipv6Addr)
}

impl Execute for SetIpv6ToStatic {
    type In = Option<Wan>;
    type Out = Event;

    fn exec(self, wan: Self::In) -> Result<Self::Out, Self::Err> {
        wan.ok_or(SetIpv6ToStaticError::WanNotFound)?;
        if self.host.addr.prefix_len.value() < 64 {
            Err(SetIpv6ToStaticError::InvalidPrefixLength(self.host.addr.prefix_len))?;
        } else if !Ipv6Addr::is_global(&self.host.addr.addr) {
            Err(SetIpv6ToStaticError::InvalidIpv6Address(self.host.addr.addr))?;
        } else if !Ipv6Addr::is_unicast_link_local(&self.host.gateway) {
            Err(SetIpv6ToStaticError::InvalidIpv6Gateway(self.host.gateway))?;
        }
        let e = Ipv6SetToStatic { id: self.id, host: self.host };
        Ok(Event::new(e.id, e.into()))
    }
}

impl Payload for SetIpv6ToStatic {
    type Ok = ();
    type Err = SetIpv6ToStaticError;
}

impl Process for SetIpv6ToStatic {
    type Actor = RackdCmdActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.transaction()?;
        let isp = tx.run(GetWanById { id: self.id, view: PhantomData::<Wan> })?;
        // let link = sys.send_blocking(GetLinkByName { name: "trunk1.[VLAN-ID]" })
        self.exec(isp).map(|e: Event| {
            let _ = tx.save(&e)?;
            Ok(())
        })?
    }
}
