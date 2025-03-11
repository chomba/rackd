use std::str::FromStr;
use crate::{actors::{cmd::RackdCmd, system::ActorSystem}, net::wan::cmd::*};

fn new_api() -> Handle<RackdCmd> {
    let _ = env_logger::builder()
    .filter_level(log::LevelFilter::Error)
    .format_target(false)
    .format_timestamp(None)
    .try_init();
    let actors = ActorSystem::mock();
    // let actors = ActorSystem::new(Connection::open("/data/lab/rust/rackd/rackd/tmp/test.db").unwrap());
    actors.cmd
}

#[tokio::test]
async fn can_create_if_namesake_wan_doesnt_exists() {
    let api = new_api();
    let cmd = Create { 
        name: NetName::new("at&t").unwrap(), 
        ..Default::default()
    };
    assert!(api.send(cmd).await.is_ok());
    let cmd = Create { 
        name: NetName::new("Verizon").unwrap(), 
        ..Default::default()
    };
    assert!(api.send(cmd).await.is_ok());
}

#[tokio::test]
async fn cant_create_if_namesake_wan_exists() {
    let api = new_api();
    let cmd = Create { 
        name: NetName::new("at&t").unwrap(), 
        ..Default::default()
    };
    assert!(api.send(cmd).await.is_ok());
    let cmd = Create {
        name: NetName::new("AT&T").unwrap(),
        ..Default::default()
    };
    assert!(api.send(cmd).await.is_err());
    let cmd = Create {
        name: NetName::new("at&t").unwrap(),
        ..Default::default()
    };
    assert!(api.send(cmd).await.is_err())
}

#[tokio::test]
async fn can_rename_if_name_is_available() {
    let api = new_api();
    let cmd = Create {
        name: NetName::new("Verizon").unwrap(),
        ..Default::default()
    };
    let id = api.send(cmd).await.unwrap();
    let cmd = Rename {
        id,
        name: NetName::new("Movistar").unwrap(),
    };
    assert!(api.send(cmd).await.is_ok());
    let cmd = Rename {
        id,
        name: NetName::new("Americatel").unwrap(),
    };
    assert!(api.send(cmd).await.is_ok());
}

#[tokio::test]
async fn cant_rename_if_name_isnt_available() {
    let api = new_api();
    let cmd = Create { 
        name: NetName::new("at&t").unwrap(), 
        ..Default::default()
    };
    let id = api.send(cmd).await.unwrap();
    let cmd = Rename {
        id, name: NetName::new("AT&T").unwrap()
    };
    assert!(api.send(cmd).await.is_err());
    let cmd = Rename {
        id, name: NetName::new("at&t").unwrap()
    };
    assert!(api.send(cmd).await.is_err());
}

#[tokio::test]
async fn can_spoof_and_unspoof_mac() {
    let api = new_api();
    let cmd = Create { 
        name: NetName::new("at&t").unwrap(), 
        ..Default::default()
    };
    let id = api.send(cmd).await.unwrap();
    let cmd = SetMacToSpoof { 
        id,  mac: MacAddr6::new(0x76, 0xdc, 0x3a, 0x78, 0xaf, 0xd0) 
    };
    assert!(api.send(cmd).await.is_ok());
    let cmd = SetMacToAuto { id };
    assert!(api.send(cmd).await.is_ok());
    let cmd = SetMacToSpoof { 
        id,  mac: MacAddr6::new(0x76, 0xdc, 0x3a, 0x78, 0xaf, 0xd0) 
    };
    assert!(api.send(cmd).await.is_ok());
}

#[tokio::test]
async fn can_switch_between_ipv6_modes() {
    let api = new_api();
    let cmd = Create { 
        name: NetName::new("at&t").unwrap(), 
        ..Default::default()
    };
    let id = api.send(cmd).await.unwrap();
    let cmd = SetIpv6ToStatic {
        id,
        host: Ipv6Host { 
            addr: Ipv6HostAddr {
                addr: Ipv6Addr::from_str("2800:200:44:8814:216:3eff:fe17:bb6f").unwrap(),
                prefix_len: Ipv6PrefixLen::new(64).unwrap()
            },
            gateway: Ipv6Addr::from_str("fe80::216:3eff:fe17:bb6f").unwrap()
        }
    };
    assert!(api.send(cmd).await.is_ok());
    let cmd = SetIpv6ToRA { id };
    assert!(api.send(cmd).await.is_ok());
}

#[tokio::test]
async fn can_switch_between_ipv4_modes() {
    let api = new_api();
    let cmd = Create { 
        name: NetName::new("at&t").unwrap(), 
        ..Default::default()
    };
    let id = api.send(cmd).await.unwrap();
    let cmd = SetIpv4ToStatic {
        id,
        host: Ipv4Host {
            addr: Ipv4HostAddr {
                addr: Ipv4Addr::new(10, 10, 100, 10),
                mask_len: Ipv4PrefixLen::new(24).unwrap()
            },
            geteway: Ipv4Addr::new(10, 10, 100, 1)
        }
    };
    assert!(api.send(cmd).await.is_ok());
    let cmd = SetIpv4ToDHCP { id };
    assert!(api.send(cmd).await.is_ok()); 
}