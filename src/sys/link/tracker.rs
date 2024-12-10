use std::time::Duration;
use crate::{app::{actor::AppMessage, isp::{events::{IspLinkWentDown, IspLinkWentUp}, models::IspPrefix}}, util::{actor::Handle, domain::Id, net::{tools::InternetTester, types::IpPrefix}}};
use crate::sys::{link::models::{LinkDown, LinkId, LinkStatus, LinkUp}, util::{netlink::Netlink, tracking::Tracker}};

use super::query::GetLinkById;

pub enum TrackingError {
    FailedToSpawnTracker
}

pub struct IspLinkTracker {
    pub link: LinkId,
    pub status: LinkStatus,
    pub prefix: IspPrefix,
    pub sender: Handle<AppMessage>
}

impl IspLinkTracker {
    async fn set_status(&mut self, status: LinkStatus) -> () {
        if self.status != status {
            self.status = status;
            match self.status {
                LinkStatus::Up(up) => self.sender.emit(IspLinkWentUp { id: self.link, up }).await,
                LinkStatus::Down(down) => self.sender.emit(IspLinkWentDown { id: self.link, down }).await,
                _ => { }
            }
        }
    }
}

impl Tracker for IspLinkTracker {
    async fn work(&mut self, netlink: Netlink) -> () {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let link = match netlink.run(GetLinkById { id: self.link }).await {
                Ok(link) => link,
                Err(_) =>  {
                    self.set_status(LinkStatus::Down(LinkDown::WentMissing)).await;
                    continue;
                }
            };

            if let LinkStatus::Up(up) = link.status {
                let tester = InternetTester {
                    ipv4_addr: self.prefix.v4().unwrap_or_default().last(),
                    ipv6_addr: self.prefix.v6().unwrap_or_default().last()
                };
                match up {
                    LinkUp::Connected => { 
                        if let Some(up) = tester.connectivity().await {
                            self.set_status(LinkStatus::Up(LinkUp::InternetUp(up))).await
                        }
                    },
                    LinkUp::InternetUp(up) => {
                        // perhaps we should try again at least twice 
                        // in case connectivity is flapping more a few ms
                        if tester.connectivity().await.is_none() {
                            self.set_status(LinkStatus::Up(LinkUp::Connected)).await
                        }
                    }
                }
            }
        }
    }
}



// pub enum TrackerType {
//     LinkTracker {  },
//     PrefixTracker { link: LinkId, prefix: Ipv6Prefix },
//     // TrafficTracker
//     // PacketLossTracker
//     // ThrouhputTracker
//     // BGPDataTracker
//     // ODISFPModuleTracker
// }