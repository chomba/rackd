use std::{net::{Ipv4Addr, Ipv6Addr}, sync::Arc, time::Duration};
use crate::{app::{actor::AppMessage, isp::{events::{IspLinkWentDown, IspLinkWentUp}, domain::IspPrefix}}, util::{actor::Handle, domain::Id, net::{tools::InternetTester, types::IpPrefix}}};
use crate::sys::{link::domain::{LinkDown, LinkId, LinkStatus, LinkUp}, util::{netlink::Netlink, trackers::LinkTracker}};
use super::query::GetLinkById;
use aya::{maps::Array, Ebpf};

pub struct LinkStatusTracker {
    pub link: LinkId,
    pub status: LinkStatus,
    pub prefix: IspPrefix,
    pub netlink: Netlink,
    pub app: Handle<AppMessage>,
}

impl LinkStatusTracker {
    async fn set_status(&mut self, status: LinkStatus) -> () {
        if self.status != status {
            self.status = status;
            match self.status {
                LinkStatus::Up(up) => self.app.emit(IspLinkWentUp { id: self.link, up }).await,
                LinkStatus::Down(down) => self.app.emit(IspLinkWentDown { id: self.link, down }).await,
                _ => { }
            }
        }
    }
}

impl LinkTracker for LinkStatusTracker {
    fn link(&self) -> LinkId { self.link }
    async fn work(&mut self) -> () {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let link = match self.netlink.run(GetLinkById { id: self.link }).await {
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

pub struct LinkGatewayTracker {
    pub link: LinkId,
    pub ebpf: Arc<Ebpf>,
    pub app: Handle<AppMessage>,
}

impl LinkTracker for LinkGatewayTracker {
    fn link(&self) -> LinkId { self.link }
    async fn work(&mut self) -> () {
        loop {
            tokio::time::sleep(Duration::from_secs(2)).await;

            let gateway6_map: Array::<_, u128> = Array::try_from(self.ebpf.map("IPV6_GATEWAY").unwrap()).unwrap();
            let gateway4_map: Array::<_, u32> = Array::try_from(self.ebpf.map("IPV4_GATEWAY").unwrap()).unwrap();
            let gateway6 = Ipv6Addr::from_bits(gateway6_map.get(&0, 0).unwrap());
            let gateway4 = Ipv4Addr::from_bits(gateway4_map.get(&0, 0).unwrap());
    
            println!("IPv6 Gateway: {gateway6:?} - IPv4 Gateway: {gateway4:?}");
            println!("ok 2 sec elapsed");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use aya::programs::{Xdp, XdpFlags};

    #[tokio::test]
    async fn test_ebpf_gateway_tracker() {
        // let mut ebpf = aya::Ebpf::load(aya::include_bytes_aligned!(concat!(
        //     env!("OUT_DIR"),
        //     "/rackd"
        // ))).unwrap();
        
        // let program: &mut Xdp = ebpf.program_mut("rackd").unwrap().try_into().expect("Failed to cast program to XDP program");
        // program.load().expect("failed to load ebpf program");
        // program.attach("wan2",XdpFlags::default()).expect("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE");

        // loop {
        //     tokio::time::sleep(Duration::from_secs(2)).await;
        //     println!("ok 2 sec elapsed");
        // }
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