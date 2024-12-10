use std::{collections::HashMap, net::{Ipv4Addr, Ipv6Addr}, time::Duration};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::{process::Command, task::JoinSet, time::{self, timeout}};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PublicDNS {
    Google, Cloudflare, Quad9, OpenDNS, AdGuard
}

lazy_static! {
    pub static ref PublicDNSList: HashMap<PublicDNS, (Ipv4Addr, Ipv6Addr)> = HashMap::from_iter([
        (PublicDNS::Google, (Ipv4Addr::new(8, 8, 8, 8), Ipv6Addr::new(0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8888))),
        (PublicDNS::Cloudflare, (Ipv4Addr::new(1, 1, 1, 1), Ipv6Addr::new(0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x1111))),
        (PublicDNS::Quad9, (Ipv4Addr::new(9, 9, 9, 9), Ipv6Addr::new(0x2620, 0x00fe, 0, 0, 0, 0, 0, 0x0009))),
        (PublicDNS::OpenDNS, (Ipv4Addr::new(208, 67, 222, 222), Ipv6Addr::new(0x2620, 0x0119, 0x0035, 0x0035, 0, 0, 0, 0x0035))),
        (PublicDNS::AdGuard, (Ipv4Addr::new(94, 140, 14, 14), Ipv6Addr::new(0x2a10, 0x50c0, 0, 0, 0, 0, 0x0ad1, 0x00ff)))
    ]);
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Copy, Clone)]
pub enum InternetUp {
    V4, V6, DualStack
}

pub struct InternetTester {
    pub ipv4_addr: Ipv4Addr,
    pub ipv6_addr: Ipv6Addr
}

impl Default for InternetTester {
    fn default() -> Self {
        Self {
            ipv4_addr: Ipv4Addr::new(127, 0, 0, 1),
            ipv6_addr: Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1)
        }
    }
}

impl InternetTester {
    pub async fn connectivity(&self) -> Option<InternetUp> {
        const PROBE_COUNT: usize = 5;
        const QUORUM_COUNT: usize = PROBE_COUNT / 2 + 1; 
        let mut ping4_probes = JoinSet::new();
        let mut ping6_probes = JoinSet::new();

        let mut i: usize = usize::default();
        while i < PROBE_COUNT {
            let mut it = PublicDNSList.iter();
            while let Some((dns, (dns_ipv4, dns_ipv6))) = it.next() {
                if i >= PROBE_COUNT {
                    break;
                }
                ping4_probes.spawn(ping4(self.ipv4_addr, *dns_ipv4, 1));
                ping6_probes.spawn(ping6(self.ipv6_addr, *dns_ipv6, 1));
                i += 1;
            }
        }

        let mut ping4_ok_count = usize::default();
        while let Some(res) = ping4_probes.join_next().await {
            if res.unwrap() == PingStatus::Success {
                ping4_ok_count += 1;
            }
        }

        let mut ping6_ok_count = usize::default();
        while let Some(res) = ping6_probes.join_next().await {
            if res.unwrap() == PingStatus::Success {
                ping6_ok_count += 1;
            }
        }

        let ping4_ok = ping4_ok_count >= QUORUM_COUNT;
        let ping6_ok = ping6_ok_count >= QUORUM_COUNT;
        if ping4_ok && ping6_ok {
            return Some(InternetUp::DualStack)
        } else if ping6_ok {
            return Some(InternetUp::V6)
        } else if ping4_ok {
            return Some(InternetUp::V4)
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PingStatus {
    Success,
    Error // NoDefaultGateway, etc parse ICMP Error responses
}

pub async fn ping4(from: Ipv4Addr, to: Ipv4Addr, count: u8) -> PingStatus {
    match tokio::spawn(timeout(Duration::from_secs(1), ping4_unbounded(from, to, count))).await {
        Ok(Ok(status)) => status,
        _ => PingStatus::Error // PingStatus::TimedOut?
    }
}

async fn ping4_unbounded(from: Ipv4Addr, to: Ipv4Addr, count: u8) -> PingStatus {
    let output = Command::new("ping")
        .args([&to.to_string(), "-I", &from.to_string(), "-c", &count.to_string()]).output().await;
    match output {
        Ok(output) if output.status.success() => PingStatus::Success,
        _ => PingStatus::Error
    }
}

pub async fn ping6(from: Ipv6Addr, to: Ipv6Addr, count: u8) -> PingStatus {
    match tokio::spawn(timeout(Duration::from_secs(1), ping6_unbounded(from, to, count))).await {
        Ok(Ok(status)) => status,
        _ => PingStatus::Error // PingStatus::TimedOut?
    }

    // tokio::select! {
    //     ping_status = ping6_unbounded(from, to, count) => { ping_status }
    //     _ = time::sleep(Duration::from_secs(1)) => { PingStatus::Error }
    // }
}

pub async fn ping6_unbounded(from: Ipv6Addr, to: Ipv6Addr, count: u8) -> PingStatus {
    let output = Command::new("ping")
        .args(["-6", &to.to_string(), "-I", &from.to_string(), "-c", &count.to_string()]).output().await;
    match output {
        Ok(output) if output.status.success() => PingStatus::Success,
        _ => PingStatus::Error
    }
}


#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};
    use crate::util::net::tools::{InternetTester, InternetUp, PingStatus, PublicDNS, PublicDNSList};
    use super::{ping4, ping6};

    static LOCAL_IPV4_ADDR: Ipv4Addr = Ipv4Addr::new(172, 24, 20, 100);
    static LOCAL_IPV6_ADDR: Ipv6Addr = Ipv6Addr::new(0x2800, 0x0200, 0xfb80, 0x00ef, 0xffff, 0xffff, 0xffff, 0x0001);

    #[tokio::test]
    async fn test_ping4() {
        let (google, _) = PublicDNSList.get(&PublicDNS::Google).unwrap();
        let (cloudflare, _) = PublicDNSList.get(&PublicDNS::Cloudflare).unwrap();
        let (opendns, _) = PublicDNSList.get(&PublicDNS::OpenDNS).unwrap();
        let (quad9, _) = PublicDNSList.get(&PublicDNS::Quad9).unwrap();
        assert_eq!(ping4(LOCAL_IPV4_ADDR, *google, 1).await, PingStatus::Success);
        assert_eq!(ping4(LOCAL_IPV4_ADDR, *cloudflare, 1).await, PingStatus::Success);
        assert_eq!(ping4(LOCAL_IPV4_ADDR, *opendns, 1).await, PingStatus::Success);
        assert_eq!(ping4(LOCAL_IPV4_ADDR, *quad9, 1).await, PingStatus::Success);
    }

    #[tokio::test]
    async fn test_ping6() {
        let (_, google) = PublicDNSList.get(&PublicDNS::Google).unwrap();
        let (_, cloudflare) = PublicDNSList.get(&PublicDNS::Cloudflare).unwrap();
        let (_, opendns) = PublicDNSList.get(&PublicDNS::OpenDNS).unwrap();
        let (_, quad9) = PublicDNSList.get(&PublicDNS::Quad9).unwrap();
        assert_eq!(ping6(LOCAL_IPV6_ADDR, *google, 1).await, PingStatus::Error);
        assert_eq!(ping6(LOCAL_IPV6_ADDR, *cloudflare, 1).await, PingStatus::Error);
        assert_eq!(ping6(LOCAL_IPV6_ADDR, *opendns, 1).await, PingStatus::Error);
        assert_eq!(ping6(LOCAL_IPV6_ADDR, *quad9, 1).await, PingStatus::Error);
    }

    #[tokio::test]
    async fn test_internet_connectivity() {
        let tester = InternetTester {
            ipv4_addr: LOCAL_IPV4_ADDR,
            ipv6_addr: LOCAL_IPV6_ADDR
        };
        assert_eq!(tester.connectivity().await, Some(InternetUp::V4));
    }
}