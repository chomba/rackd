use std::sync::Arc;
use aya::programs::{Xdp, XdpFlags};
use log::warn;
use crate::{isp::models::IspPrefix, sys::{actor::SysActor, error::SysError, link::domain::{LinkId, LinkStatus}, util::netlink::{Netlink, NlCommand}}, util::{actor::{AsyncProcess, Payload, Process}, domain::Id}};
use super::trackers::{LinkGatewayTracker, LinkStatusTracker};

pub struct EnableLink {
    pub id: LinkId
}

impl Payload for EnableLink {
    type Ok = ();
    type Err = SysError;
}

impl AsyncProcess for EnableLink {
    type Actor = SysActor;

    async fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        Ok(actor.netlink.exec(self).await?)
    }
}

impl NlCommand for EnableLink {
    type Ok = ();
    type Err = SysError;

    async fn exec(self, netlink: &Netlink) -> Result<Self::Ok, Self::Err> {
        let handle = netlink.route();
        handle.link().set(self.id.into()).up().execute().await?;
        Ok(())
    }
}

pub struct DisableLink {
    pub id: LinkId
}

impl Payload for DisableLink {
    type Ok = ();
    type Err = SysError;
}

impl AsyncProcess for DisableLink {
    type Actor = SysActor;

    async fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        Ok(actor.netlink.exec(self).await?)
    }
}

impl NlCommand for DisableLink {
    type Ok = ();
    type Err = SysError;

    async fn exec(self, netlink: &Netlink) -> Result<Self::Ok, Self::Err> {
        let handle = netlink.route();
        handle.link().set(self.id.into()).down().execute().await?;
        Ok(())
    }
}


pub struct TrackIspLink {
    // pass link: Link instead of status, id, prefix?
    pub link: LinkId,
    pub status: LinkStatus,
    pub prefix: IspPrefix
}

impl Payload for TrackIspLink {
    type Ok = ();
    type Err = ();    
}

impl Process for TrackIspLink {
    type Actor = SysActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        actor.trackers.untrack(&self.link);

        // Load Trackers
        let status_tracker = LinkStatusTracker {
            link: self.link, status: self.status, prefix: self.prefix, netlink: actor.netlink.clone(), app: actor.app.clone()
        };

        // Link Gateway Tracker
        let mut ebpf = aya::Ebpf::load(aya::include_bytes_aligned!(concat!(
            env!("OUT_DIR"),
            "/rackd"
        ))).unwrap();

        if let Err(e) = aya_log::EbpfLogger::init(&mut ebpf) {
            // This can happen if you remove all log statements from your eBPF program.
            warn!("failed to initialize eBPF logger: {}", e);
        }
        
        // Load EBPF Program
        let program: &mut Xdp = ebpf.program_mut("program").unwrap().try_into().expect("Failed to cast program to XDP program");
        program.load().expect("failed to load ebpf program");
        program.attach_to_if_index(self.link.into(), XdpFlags::default()).expect("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE");

        let gateway_tracker = LinkGatewayTracker {
            link: self.link, app: actor.app.clone(), ebpf: Arc::new(ebpf)
        };
        actor.trackers.spawn(status_tracker);
        actor.trackers.spawn(gateway_tracker);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{net::Ipv4Addr, str::FromStr};
    use crate::{network::isp::data::IspPrefix, sys::{link::{cmd::*, domain::*, query::*}, util::netlink::Netlink}, system::System, util::net::types::Ipv4Prefix};
    use crate::sys::util::netlink::NlQuery;
    use super::TrackIspLink;
    use crate::util::net::types::IpPrefix;

    #[tokio::test]
    async fn can_track_isp_link() {
        let system = System::mock();
        let sys = system.sys;
        let query = GetLinkByName { name: LinkName::from_str("dummy1").unwrap() };
        let link = sys.send(query).await.unwrap();
        let prefix = IspPrefix::V4(Ipv4Prefix::new(Ipv4Addr::new(127, 0, 0, 1), 24));
        
        let cmd = TrackIspLink { link: link.id, status: LinkStatus::Unknown, prefix };
        assert!(sys.send(cmd).await.is_ok());
    }

    #[tokio::test]
    async fn enable_link() {
        let netlink = Netlink::connect().unwrap();
        let query = GetLinkByName { name: LinkName::from_str("dummy1").unwrap() };
        let link = netlink.run(query).await.unwrap();
        
        let cmd = EnableLink { id: link.id };
        assert!(netlink.exec(cmd).await.is_ok());
        let link = netlink.run(GetLinkById { id: link.id }).await.unwrap();
        assert_ne!(link.status, LinkStatus::Down(LinkDown::AdminDown));
    }

    #[tokio::test]
    async fn disable_link() {
        let netlink = Netlink::connect().unwrap();
        let query = GetLinkByName { name: LinkName::from_str("dummy1").unwrap() };
        let link = query.run(&netlink).await.unwrap();
        
        let cmd = DisableLink { id: link.id };
        assert!(cmd.exec(&netlink).await.is_ok());
    }
}
