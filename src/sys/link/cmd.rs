use crate::{app::isp::models::IspPrefix, sys::{actor::SysActor, error::SysError, link::models::{LinkId, LinkStatus}, util::netlink::{Netlink, NlCommand}}, util::{actor::{AsyncProcess, Payload, Process}, domain::Id}};
use super::tracker::IspLinkTracker;

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
        handle.link().set(self.id.0).up().execute().await?;
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
        handle.link().set(self.id.0).down().execute().await?;
        Ok(())
    }
}


pub struct TrackIspLink {
    pub tracker: Option<Id>,
    pub link: LinkId,
    pub status: LinkStatus,
    pub prefix: IspPrefix
}

impl Payload for TrackIspLink {
    type Ok = Id;
    type Err = ();    
}

impl Process for TrackIspLink {
    type Actor = SysActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        if let Some(ref tracker) = self.tracker {
            actor.trackers.cancel(tracker);
        }
        let tracker = IspLinkTracker {
            link: self.link, status: self.status, prefix: self.prefix, sender: actor.app.clone()
        };
        let handle = actor.trackers.spawn(tracker, actor.netlink.clone());
        Ok(handle.tracker_id)
    }
}

#[cfg(test)]
mod tests {
    use std::{net::Ipv4Addr, str::FromStr};
    use crate::{app::isp::models::IspPrefix, sys::{link::{cmd::*, models::*, query::*}, util::netlink::Netlink}, system::System, util::net::types::Ipv4Prefix};
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
        
        let cmd = TrackIspLink { tracker: None, link: link.id, status: LinkStatus::Unknwon, prefix };
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
