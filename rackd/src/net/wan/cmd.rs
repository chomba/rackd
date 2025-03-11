pub mod casts;
#[cfg(test)]
mod tests;
use std::{marker::PhantomData, net::{Ipv4Addr, Ipv6Addr}};
use macaddr::MacAddr6;
use thiserror::Error;
use crate::{actors::cmd::RackdCmdActor, db::{cmd::traits::*, query::traits::*, Tx}, net::shared::models::*, org::rack::models::RackId, util::{actor::*, models::Event, traits::*}};
use super::{models::*, query::{views::Wan, GetWanById, GetWanByName}};

pub enum WanCmd {
    Create(Msg<Create>),
    Rename(Msg<Rename>),
    SetMacToAuto(Msg<SetMacToAuto>),
    SetMacToSpoof(Msg<SetMacToSpoof>),
    SetIpv6ToStatic(Msg<SetIpv6ToStatic>),
    SetIpv6ToRA(Msg<SetIpv6ToRA>),
    SetIpv4ToDHCP(Msg<SetIpv4ToDHCP>),
    SetIpv4ToStatic(Msg<SetIpv4ToStatic>)
}

#[derive(Debug, Default)]
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

impl Payload for Create {
    type Ok = WanId;
    type Err = CreateError;
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
        Ok((e.id, Event::single(e.id, e.into(), 0)))
    }    
}

impl Process for Create {
    type Actor = RackdCmdActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.tx()?;
        // TBD: Load RackId, TrunkId, VlanId
        let namesake_wan = tx.run(GetWanByName { name: self.name.clone(), view: PhantomData::<Wan> });
        self.exec(namesake_wan).map(|(id, e)| {
            tx.save(&e);
            Ok(id)
        })?
    }
}

pub struct Rename {
    pub id: WanId,
    pub name: NetName
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

impl Execute for Rename {
    type In = (Option<Wan>, Option<Wan>);
    type Out = Event;

    fn exec(self, (wan, namesake_wan): Self::In) -> Result<Self::Out, Self::Err> {
        let wan = wan.ok_or(RenameError::WanNotFound)?;
        namesake_wan.err_or(RenameError::NameAlreadyInUse)?;
        let e = Renamed { name: self.name.clone() };
        Ok(Event::single(self.id, e.into(), wan.version))
    }
}

impl Process for Rename {
    type Actor = RackdCmdActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.tx()?;
        let wan = tx.run(GetWanById { id: self.id, view: PhantomData::<Wan> });
        let namesake_wan = tx.run(GetWanByName { name: self.name.clone(), view: PhantomData::<Wan> });
        self.exec((wan, namesake_wan)).map(|e| {
            tx.save(&e);
            Ok(())
        })?
    }
}

pub struct SetMacToSpoof {
    id: WanId,
    mac: MacAddr6
}

#[derive(Debug, Error)]
pub enum SetMacToSpoofError {
    #[error("Db Error")]
    Db(#[from] rusqlite::Error),
    #[error("Wan's MAC is already spoofed")]
    AlreadySet,
    #[error("Wan with id x can't be found")]
    WanNotFound,
    #[error("Reserved MAC Addresses can't be used")]
    UnusableMACAddress
}

impl Payload for SetMacToSpoof {
    type Err = SetMacToSpoofError;
    type Ok = ();
}

impl Execute for SetMacToSpoof {
    type In = Option<Wan>;
    type Out = Event;

    fn exec(self, wan: Self::In) -> Result<Self::Out, Self::Err> {
        let wan = wan.ok_or(SetMacToSpoofError::WanNotFound)?;
        if wan.mac == WanMac::Spoof(self.mac) {
            Err(SetMacToSpoofError::AlreadySet)?;
        }
        let e = MacSetToSpoof { mac: self.mac };
        Ok(Event::single(self.id, e.into(), wan.version))
    }
}

impl Process for SetMacToSpoof {
    type Actor = RackdCmdActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.tx()?;
        let wan = tx.run(GetWanById { id: self.id, view: PhantomData::<Wan> });
        self.exec(wan).map(|e| {
            tx.save(&e);
            Ok(())
        })?
    }
}

pub struct SetMacToAuto {
    pub id: WanId
}

#[derive(Debug, Error)]
pub enum SetMacToAutoError {
    #[error("Db Error")]
    Db(#[from] rusqlite::Error),
    #[error("Already Set")]
    AlreadySet,
    #[error("Wan with id x can't be found")]
    WanNotFound
}

impl Payload for SetMacToAuto {
    type Ok = ();
    type Err = SetMacToAutoError;
}

impl Execute for SetMacToAuto {
    type In = Option<Wan>;
    type Out = Event;

    fn exec(self, wan: Self::In) -> Result<Self::Out, Self::Err> {
        let wan = wan.ok_or(SetMacToAutoError::WanNotFound)?;
        if WanMac::Auto == wan.mac {
            Err(SetMacToAutoError::AlreadySet)?;
        }
        Ok(Event::single(self.id, MacSetToAuto.into(), wan.version))
    }
}

impl Process for SetMacToAuto {
    type Actor = RackdCmdActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.tx()?;
        let wan = tx.run(GetWanById { id: self.id, view: PhantomData::<Wan> });
        self.exec(wan).map(|e| {
            tx.save(&e);
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
    #[error("Wan IPv6 settings not available for PPPoE Connections")]
    ConnectionIsPPPoE,
    #[error("Wan IPv6 Host already set")]
    AlreadySet,
    #[error("Prefix Length needs to be greated than 64")]
    InvalidPrefixLength(Ipv6PrefixLen),
    #[error("IPv6 Address {0} isn't a valid GUA Address")]
    InvalidIpv6Address(Ipv6Addr),
    #[error("Gateway {0} isn't a valid LL Address")]
    InvalidIpv6Gateway(Ipv6Addr)
}

impl Payload for SetIpv6ToStatic {
    type Ok = ();
    type Err = SetIpv6ToStaticError;
}

impl Execute for SetIpv6ToStatic {
    type In = Option<Wan>;
    type Out = Event;

    fn exec(self, wan: Self::In) -> Result<Self::Out, Self::Err> {
        let wan = wan.ok_or(SetIpv6ToStaticError::WanNotFound)?;
        match wan.conn {
            WanConnection::PPPoE(_) => Err(SetIpv6ToStaticError::ConnectionIsPPPoE)?,
            WanConnection::IPoE(ip) if ip.ipv6 == Ipv6Conf::Static(self.host) => Err(SetIpv6ToStaticError::AlreadySet)?,
            _ => { }
        }
        if self.host.addr.prefix_len.value() < 64 {
            Err(SetIpv6ToStaticError::InvalidPrefixLength(self.host.addr.prefix_len))?;
        } else if !Ipv6Addr::is_global(&self.host.addr.addr) {
            Err(SetIpv6ToStaticError::InvalidIpv6Address(self.host.addr.addr))?;
        } else if !Ipv6Addr::is_unicast_link_local(&self.host.gateway) {
            Err(SetIpv6ToStaticError::InvalidIpv6Gateway(self.host.gateway))?;
        }
        let e = Ipv6SetToStatic { id: self.id, host: self.host };
        Ok(Event::single(e.id, e.into(), wan.version))
    }
}

impl Process for SetIpv6ToStatic {
    type Actor = RackdCmdActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.tx()?;
        let wan = tx.run(GetWanById { id: self.id, view: PhantomData::<Wan> });
        // let link = sys.send_blocking(GetLinkByName { name: "trunk1.[VLAN-ID]" })
        self.exec(wan).map(|e: Event| {
            tx.save(&e);
            Ok(())
        })?
    }
}

pub struct SetIpv6ToRA {
    pub id: WanId
}

#[derive(Debug, Error)]
pub enum SetIpv6ToRAError {
    #[error("Db Error")]
    Db(#[from] rusqlite::Error),
    #[error("Wan with id x can't be found")]
    WanNotFound,
    #[error("Already Set")]
    AlreadySet,
    #[error("Wan IPv6 settings not available for PPPoE Connections")]
    ConnectionIsPPPoE
}

impl Payload for SetIpv6ToRA {
    type Ok = ();
    type Err = SetIpv6ToRAError;
}

impl Execute for SetIpv6ToRA {
    type In = Option<Wan>;
    type Out = Event;

    fn exec(self, wan: Self::In) -> Result<Self::Out, Self::Err> {
        let wan = wan.ok_or(SetIpv6ToRAError::WanNotFound)?;
        match wan.conn {
            WanConnection::PPPoE(_) => Err(SetIpv6ToRAError::ConnectionIsPPPoE)?,
            WanConnection::IPoE(ip) if ip.ipv6 == Ipv6Conf::FromRA => Err(SetIpv6ToRAError::AlreadySet)?,
            _ => { }
        }
        Ok(Event::single(self.id, Ipv6SetToRA.into(), wan.version))
    }
}

impl Process for SetIpv6ToRA {
    type Actor = RackdCmdActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.tx()?;
        let wan = tx.run(GetWanById { id: self.id, view: PhantomData::<Wan> });
        self.exec(wan).map(|e| {
            tx.save(&e);
            Ok(())
        })?
    }
}

pub struct SetIpv4ToDHCP {
    pub id: WanId
}

#[derive(Debug, Error)]
pub enum SetIpv4ToDHCPError {
    #[error("Db Error")]
    Db(#[from] rusqlite::Error),
    #[error("Wan with id x can't be found")]
    WanNotFound,
    #[error("Already Set")]
    AlreadySet,
    #[error("Wan IPv6 settings not available for PPPoE Connections")]
    ConnectionIsPPPoE
}

impl Payload for SetIpv4ToDHCP {
    type Ok = ();
    type Err = SetIpv4ToDHCPError;
}

impl Execute for SetIpv4ToDHCP {
    type In = Option<Wan>;
    type Out = Event;

    fn exec(self, wan: Self::In) -> Result<Self::Out, Self::Err> {
        let wan = wan.ok_or(SetIpv4ToDHCPError::WanNotFound)?;
        match wan.conn {
            WanConnection::PPPoE(_) => Err(SetIpv4ToDHCPError::ConnectionIsPPPoE)?,
            WanConnection::IPoE(ip) if ip.ipv4 == Ipv4Conf::DHCP => Err(SetIpv4ToDHCPError::AlreadySet)?,
            _ => { }
        }
        Ok(Event::single(self.id, Ipv4SetToDHCP.into(), wan.version))
    }
}

impl Process for SetIpv4ToDHCP {
    type Actor = RackdCmdActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.tx()?;
        let wan = tx.run(GetWanById { id: self.id, view: PhantomData::<Wan> });
        self.exec(wan).map(|e| {
            tx.save(&e);
            Ok(())
        })?
    }
}

pub struct SetIpv4ToStatic {
    pub id: WanId,
    pub host: Ipv4Host
}

#[derive(Debug, Error)]
pub enum SetIpv4ToStaticError {
    #[error("Db Error")]
    Db(#[from] rusqlite::Error),
    #[error("Wan with id x can't be found")]
    WanNotFound,
    #[error("Already Set")]
    AlreadySet,
    #[error("Wan IPv6 settings not available for PPPoE Connections")]
    ConnectionIsPPPoE,
    #[error("Prefix Length needs to be greated than 8")]
    InvalidMaskLength(Ipv4PrefixLen),    
    #[error("IPv4 Address {0} isn't a valid host ip address")]
    InvalidIpv4Address(Ipv4Addr),
    #[error("Gateway {0} isn't a valid Address for use as the gateway")]
    InvalidIpv4Gateway(Ipv4Addr)
}

impl Payload for SetIpv4ToStatic {
    type Err = SetIpv4ToStaticError;
    type Ok = ();
}

impl Execute for SetIpv4ToStatic {
    type In = Option<Wan>;
    type Out = Event;

    fn exec(self, wan: Self::In) -> Result<Self::Out, Self::Err> {
        let wan = wan.ok_or(SetIpv4ToStaticError::WanNotFound)?;
        match wan.conn {
            WanConnection::PPPoE(_) => Err(SetIpv4ToStaticError::ConnectionIsPPPoE)?,
            WanConnection::IPoE(ip) if ip.ipv4 == Ipv4Conf::Static(self.host) => Err(SetIpv4ToStaticError::AlreadySet)?,
            _ => { }
        }
        if self.host.addr.mask_len.value() < 8 {
            Err(SetIpv4ToStaticError::InvalidMaskLength(self.host.addr.mask_len))?;
        } else if is_invalid_unicast_ipv4_addr(&self.host.addr.addr) {
            Err(SetIpv4ToStaticError::InvalidIpv4Address(self.host.addr.addr))?;
        } else if is_invalid_unicast_ipv4_addr(&self.host.addr.addr) {
            Err(SetIpv4ToStaticError::InvalidIpv4Gateway(self.host.geteway))?;
        }
        let e = Ipv4SetToStatic { host: self.host };
        return Ok(Event::single(self.id, e.into(), wan.version));

        fn is_invalid_unicast_ipv4_addr(addr: &Ipv4Addr) -> bool {
            Ipv4Addr::is_documentation(addr) || Ipv4Addr::is_broadcast(&addr) || 
            Ipv4Addr::is_link_local(&addr) || Ipv4Addr::is_loopback(&addr) ||
            Ipv4Addr::is_multicast(&addr) || Ipv4Addr::is_reserved(&addr )
        }
    }
}

impl Process for SetIpv4ToStatic {
    type Actor = RackdCmdActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.tx()?;
        let wan = tx.run(GetWanById { id: self.id, view: PhantomData::<Wan> });
        self.exec(wan).map(|e| {
            tx.save(&e);
            Ok(())
        })?
    }
}