use futures::TryStreamExt;
use netlink_packet_route::address::AddressAttribute;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use crate::{sys::{actor::SysActor, error::SysError, util::netlink::{FromNetlinkMessage, Netlink, NlQuery}}, util::actor::{AsyncProcess, Payload}};
use super::models::*;

#[derive(Copy, Clone)]
pub struct GetLinkById {
    pub id: LinkId
}

impl Payload for GetLinkById {
    type Ok = Link;
    type Err = SysError;
}

impl AsyncProcess for GetLinkById {
    type Actor = SysActor;
    
    async fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        actor.netlink.run(self).await
    }
}

impl NlQuery for GetLinkById {
    type Ok = Link;
    type Err = SysError;

    async fn run(self, netlink: &Netlink) -> Result<Self::Ok, Self::Err> {
        // read: https://www.infradead.org/~tgr/libnl/doc/route.html
        let stream = netlink.route().link().get().match_index(self.id.0).execute();
        let mut link = match Link::from_msg(stream).await {
            Some(link) => link,
            None => Err(SysError::NotFound)?
        };
        (link.ipv6_addrs, link.ipv4_addrs) = netlink.run(GetLinkAddressesById { id: self.id }).await.unwrap();
        Ok(link) 
    }   
}

/// Query that returns the Link with the specified name
pub struct GetLinkByName {
    pub name: LinkName
}

impl Payload for GetLinkByName {
    type Ok = Link;
    type Err = SysError;
}

impl AsyncProcess for GetLinkByName {
    type Actor = SysActor;

    async fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        actor.netlink.run(self).await
    }
}

impl NlQuery for GetLinkByName {
    type Ok = Link;
    type Err = SysError;

    async fn run(self, netlink: &Netlink) -> Result<Self::Ok, Self::Err> {
        let stream = netlink.route().link().get().match_name(self.name.to_string()).execute();
        let mut link = match Link::from_msg(stream).await {
            Some(link) => link,
            None => Err(SysError::NotFound)?
        };
        (link.ipv6_addrs, link.ipv4_addrs) = netlink.run(GetLinkAddressesById { id: link.id }).await.unwrap();
        Ok(link)
    }
}

pub struct GetLinkAddressesById {
    pub id: LinkId
}

impl NlQuery for GetLinkAddressesById {
    type Ok = (Vec<Ipv6Addr>, Vec<Ipv4Addr>);
    type Err = ();

    async fn run(self, netlink: &Netlink) -> Result<Self::Ok, Self::Err> {
        let mut stream = netlink.route().address().get().set_link_index_filter(self.id.0).execute();
        let mut ipv6_addrs = Vec::new();
        let mut ipv4_addrs = Vec::new();
        while let Ok(Some(packet)) = stream.try_next().await {
            for attr in packet.attributes {
                match attr {
                    AddressAttribute::Address(IpAddr::V6(addr)) => ipv6_addrs.push(addr),
                    AddressAttribute::Address(IpAddr::V4(addr)) => ipv4_addrs.push(addr),
                    _ => continue
                }
            }
        }
        Ok((ipv6_addrs, ipv4_addrs))
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::sys::{link::{models::LinkName, query::GetLinkByName}, util::netlink::Netlink};

    #[tokio::test]
    async fn get_link_by_name() {
        let netlink = Netlink::connect().unwrap();
        let query = GetLinkByName { name: LinkName::from_str("dummy1").unwrap() };
        let link = netlink.run(query).await;
        assert!(link.is_ok());
        println!("{link:?}");
    }
}