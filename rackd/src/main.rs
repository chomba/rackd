// Conect the rack's first node to your exisiting network
// Access your node's console and install rackd
// Then run: 
// rack new lim15109 
// yo'll be asked for:
// Select trunk interface, enter a domain name, and a prefix
// Optionally you can just pass the following CLI arguments
// --trunk bond1 --domain chomba.org  --prefix 2001:4860:4860::/48
// You'll then be presented with a success screen and will be asked
// Note: The VLAN used for the rack's node network will always be 1919
// to configure a host on that ipv6 network and then
// access https://lim15109.chomba.org (which will resolve to )
// the success screen will also include a token
// then open rackd electron app click on add rack

// on the initialization wizard you'll be asked whether:
// you want to CREATE a new rack or JOIN an existing rack
// Select CREATE a new rack and you'll be asked to provide the following details:
// Interface, Nodes's IPv6 prefix, Public Key, Rack's name then hit CREATE
// Upon creation SSH access to the rack's node is set up


// Now move to the VLAN you entered (you'll neeed to connect to a port on your ToR switch connected to that VLAN)
// Now open rackd-desktop client (electron app) and click on add rack
// Then enter the rack's IPV6 Prefix and your Private key.
// the GUI will take you to the rack's edit page

// To discover the IP Addresses of another rack in the organization
// the rack must first try to directly contact that rack through an ip address
// on the GUA /48 block (if there's still a path to the node then it will work)
// if there's no available path the node then use the node's name A record 
// hopefully DynDNS updates wont take longer than (5 minutes)

// Netfilter vs eBPF:https://www.reddit.com/r/networking/comments/1dxcrrx/difference_between_netfilter_and_ebpf/ 
// SlackHQ Nebula uses github.com/vishvananda/netlink 
// Read: https://github.com/slackhq/nebula/discussions/986

// authentication: https://webauthn.guide/
// rackd will orchestrate
// kind (To run kubernetes nodes as containers) - Kubers
// containerd (container runtime) with runc (shim) and runwasm
// quemu (for VM virtualization)
// Routing Suite (https://github.com/holo-routing/holo) +  Systemd
// Firewall rules built with EBPF(https://github.com/aya-rs/aya + bpfman)
// WASM (wastime + runwasm)

// Should each Rack know about all networks on each other rack within the Organization?
// No, that'd require all racks to be in the same RAFT consensus domain which would require
// at least 3 racks to be deployed within the organization
// Here's how it should work:
// Each rack is the source of truth for the networks within it.
// Each rack should only know the address and name of each other rack in the same organization
// Each rack will have its own Private ASN and its own IPv6 Address Space (e.g /56)
// After installation you can either create a new organization with rack new
// Or you can join an existing rack

// P2P Connections over QUIC
// https://github.com/n0-computer/iroh
// https://developers.yubico.com/Passkeys/

// Traffic tunneling over QUIC
// https://www.f5.com/company/blog/quic-will-eat-the-internet
// https://news.ycombinator.com/item?id=26838840
//https://tailscale.com/blog/quic-udp-throughput

// OOB Management network:
// https://wiki.archlinux.org/title/USB/IP

// Install BFPTools:
// install linux-tools-common linux-tools-generic linux-tools-$(uname -r)

// rackd subsystems:
// networking
// storage
// containers
// system-containers
// vms
// functions
// storage

// You start with a rack, configure its WAN, LANs and DNS
// In order for 2 racks to form an organization they must each be using the same TLD (e.g chomba.org)
// For example, let's image you have these 2 independent racks:

// Rack #1: (LIM15109 - AS4200000001) 
//  - *.lim15109.org.chomba.org => (CNAME) *.4200000001.org.chomba.org 
//  - 4200000001.org.chomba.org => (A) 2a0f:85c1:083f:0100:: (This /128 GUA-Anycast is configured on all nodes in the rack)
//  - wan1.4200000001.org.chomba.org => (A) ISP1 Public IP Address
//  - wan2.4200000001.org.chomba.org => (A) ISP2 Public IP Address

// Rack #2: (CRZ0302 - AS4200000002)
//  - *.crz0302.org.chomba.org => (CNAME) *.4200000002.org.chomba.org
//  - 4200000002.org.chomba.org => 2a0f:85c1:083f:0200:: (This /128 GUA-Anycast is configured on all nodes in the rack)
//  - wan1.4200000002.org.chomba.org => ISP1 Public IP Address

// From Rack #2 (any node on rack #2), we will join Rack #1 to create our 2-rack organization:
// $ rack join 4200000001.org.chomba.org --token RACK1-TOKEN --org 2a0f:85c1:083f:ffff::/64
// # Note how --org is used to specify the organization's network ID
// # the --org option only needs to be specified when creating the org.

// After rack2 joins rack1 the following will happen:
// - org.chomba.org => 2a0f:85c1:083f:ffff::
// both rack#1 and rack#2 will share the same ORG-TOKEN
// rack1 will become the LEADER and rack2 will become the FOLLOWER
// each rack will configure their wireguard links
// Rack #1 will configure:
// - wg1 over wan1 (2a0f:85c1:083f:ffff:FA56:EA01::1)
// - wg2 over wan2 (2a0f:85c1:083f:ffff:FA56:EA01::2)
// Rack #2 will configure:
// - wg1 over wan1 (2a0f:85c1:083f:ffff:FA56:EA02::1)
//   - peers: 


// rack (ASN 4200000001 with prefix 2a0f:85c1:083f:0100/56 and named lim15109)
//  *.lim15109.org.chomba.org / CNAME / *.asn4200000001.org.chomba.org 
//  @.asn4200000001.org.chomba.org / A /  2a0f:85c1:083f:0100::0

// wan1.asn4200000001.org.chomba.org / A / xxxx:...:xxxx
// wan2.asn4200000001.org.chomba.org / A / xxxx:...:xxxx


// On the first node that will be part of the local rack do:

// # Create a 1-node rack with ASN 4200000001 prefix 2a0f:85c1:083f:0100/56 and t1 as the trunk 
// $ rack new as4200000001 2a0f:85c1:083f:0100/56 t1 [--name lim15109] [--trunk t2 | --trunk t3]

// The cmd will set the node as the master node in the rack
// and will make sure lim15109.local and as4200000001.local resolve 
// to the master node Link-Local Address via trunk t1
// node1.lim15109.local and node1.as4200000001.local will also resolve to the node's LLA
// It will also configure @rack..01::1/64 as the GUA on the node's link via Trunk1

// To join another node to the rack, run the following cmd from the node to be added:
// $ rack node join as4200000001 trunk1
// Nodes use mDNS to resolve the the hostname to the rack's master node link-local ip address
// Once the node joins the rack it will be assigned a node number (e.g node2)
// then it will configure @rack..01::2/64 as the GUA on the node's link via trunk1
// at this point both nodes will become part of the cluster and raft's replication logic
// will be applied so that both nodes move in a lockstep
// On this Mode the first node will always be the LEADER and the 2nd node will always be the FOLLOWER
// And there will be no LEADER ELECTION Process running on the nodes, meaning that if the leader fails
// then the system will become read-only and the follower will only reply to READ requests
// You can issue a command to switch the LEADER/FOLLOWER Roles on a 2-node cluster
// For instance, if you need to take down the current LEADER for maintanence then the rack
// will become read-only until the LEADER is back up, but you can manually switch Roles before
// taking down the current leader, so that the system is WRITABLE while one of the 2 nodes in the rack
// is being serviced. Raft's Leader Election algorithm will only be engaged when there are more than 2
// nodes in the cluster.
// 

// The "Elected" REST Gateway LEADER receives a command on one of its endpoints
// It passes it onto the RAFT Actor which stores it as an UNCOMMITED Command in its LOG
// and broadcasts the command to all FOLLOWERS which in turn add it as UNCOMMITTED commands 
// to their own LOGS, followers reply back to the LEADER acknowledging the receipt of those COMMANDs
// Once the leader receives enough replies back from the majority of FOLLOWERs it marks the COMMAND
// as COMMITTED and APPLIES it to the STATE MACHINE (our own code), meaning, the CMD will be processed
// by the appropiate actor generate some domain events, persist them to SQLite and run INLINE-PROJECTIONS
// The LEADER will include the Id of the LAST COMMITTED COMMAND on future HEART-BEATS sent to followers 
// so they can also commit any pending (not yet committed) UNCOMMITED EVENT

// Queries sent to the GATEWAY can be load balanced between all Members of the CLUSTER

// If the watchdog detects a monitored WAN Link has come down then it will produce an internal Event
// that event will be emitted to the IN-MEMORY Bus which will in turn deliver it to the
// API Gateway which will transform then into one or more commands and pass them to the RAFT Actor
// so that they can be propagated

// WanLinkDown Watchdog Event => 
//  RemoveIspFromSNATCommand (Processed by SNAT ACTOR), 
//  UpdateISPStatusCommand (Processed by ISP Actor) 

// https://www.codecentric.de/wissens-hub/blog/cqrs-es-akka


// The Watchdog is the book of record for liveness of a Link
// The APP receives events from the watchdog and processes them (downstream event processor)

// If the watchog detect a change in the address of a tracked link
// then it will emit a LinkAddressChanged Event
// that will be sent to the RAFT frontend and once replicated
// will be passed to all actors so they can act upon it.

// Internal Events are events that originite from a command, thus thye don't need to be replicated
// because it's enough for the command itself to be replicated

// External Events are events that don't originate from  a command, thus they need to be processed
// as 

// Node1 is in charge of wan1
// Node2 is in chargo of wan2
// Node3 is in charge of wan3
// WatchDog monitor wan1 on Node1
// monitor wan2 on Node2

// EMITS CHANGES ABOUT ALL NAMED NETWORKS
// RECEIVES CHANGES ABOUT DYNAMIC NETWORKS FROM THE WATCHDOG

use std::{net::{Ipv4Addr, Ipv6Addr, SocketAddr}, time::Duration};
use aya_log_ebpf::info;
use aya::{maps::Array, programs::{Xdp, XdpFlags}};
use aya_log::EbpfLogger;
use log::{debug, warn};
use rackd::api;
// use crate::{actors::{self, system::ActorSystem}, net::{shared::models::NetName, wan::cmd::Create}};
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, signal};
use dotenv::dotenv;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;
// Authentication:
// https://fly.io/blog/api-tokens-a-tedious-survey/
// Use Autehnticated requests?

// Read more about SD-WAN:
// https://packetpushers.net/blog/implementing-zero-trust-for-a-borderless-world/

// WebMesh
// https://webmeshproj.github.io/documentation/getting-started/

// Renamed rackd to rackd-api



#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();    
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Error)
        .format_target(false)
        .format_timestamp(None)
        .try_init();

    #[derive(OpenApi)]
    #[openapi(info(description = "API DESCRIPTION HERE"))]
    struct ApiDoc;

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/v1", api::router())
        .split_for_parts();

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()));
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    let listener = TcpListener::bind(&address).await?;
    axum::serve(listener, router.into_make_service()).await
    
    // env_logger::init();
    // ActorSystem::run();
    
    // let app = Router::new()
    //     .route("/wan", post())

    // Pass configuration file as argument border66 -conf=/etc/border66/config
    // Run ApiActor
    // match signal::ctrl_c().await {
    //     Ok(()) => { 
    //         println!("Program Terminated");
    //     },
    //     Err(err) => {
    //         eprintln!("Unable to listen for shutdown signal: {}", err);
    //     }
    // }
    
    // let conn = Connection::open("/data/lab/rust/quickreplace/events.db").expect("Failed to get connection");
    // es::create(&conn).expect("Failed to create db");
    // let id = Uuid::new_v4();
    // let lan = Lan::new(id, LanName::new("lab"), Ipv6Net::new(Ipv6Addr::new(0xfd, 0, 0, 0, 0, 0, 0, 0), 24).unwrap());
    // es::save(&conn, lan).expect("Failed to Save Lan");
    // match es::load::<Lan>(&conn, id) {
    //     Ok(events) => { 
    //         println!("{events:?}");
    //         println!("");
    //     },
    //     Err(e) => {
    //         println!("{e:?}");
    //     }
    // }



    // let config = Config::get().await;
    // let system = ActorSystem::new(&config.datadir);
    // let handles = system.run();
    // let cmd = CreateIsp { 
    //     name: IspName::new("MOVISTAR"), 
    //     link: NetLink::new("@wan1"),
    //     prefix: IspPrefix::new("@link1/64")
    // };

    // let response = handles.isp.send(cmd).await;

    // println!("CREATE CMD: {response:?}");
    // let cmd = RenameIsp {
    //     id: Uuid::parse_str("71cfcf9a-0c0b-4b46-b1f7-82aa95c83855").unwrap(),
    //     new_name: IspName::new("wow")
    // };
    // let response = handles.isp.send(cmd).await;
    // println!("RENAME CMD: {response:?}")
}

// async fn init_database() {
//     let db_url = Config::get().await.db_url.as_str();
//     if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
//         println!("Creating DB: {}", db_url);
//         match Sqlite::create_database(db_url)
//     }
// }



// hactl isp add movistar link=wan1 prefix=@wan1.pd/64
// hactl isp rename movistar claro
// hactl isp set claro link=wan2 prefix=@wan2.pd/64
// hactl isp set claro link=wan2 
// hactl isp enable claro
// hactl isp disable claro

// hactl nat default policy=failover isp=isp1,isp2,isp3
// hactl nat default policy=loadshare isp=@all
// hactl nat default policy=single  isp=isp2
// hactl nat add home-failover network=172.24.0.0/16 policy=failover
// hactl nat add lab-loadshare network=172.24.10.0/32 policy=loadshare
// hactl nat disable lab-loadshare
// hactl nat add lab-failover network=172.14.10.0/32  policy=failover@isp1,isp2
// hactl nat priority lab-failover claro=10 movistar=15
// hactl isp priority set 

// hactl nat set lab-failover priority=isp1,isp2
// hactl nat rename lab-failover lab-failover@isp1,isp2
// hactl nat set lab-failover policy=passthrough
// hactl nat set lab-failover network=172.24.11.0/32

// hactl ddns set  provide=cloudflare




// Person can implement Deref<Output=str> so you can coerce from &Person to &str (its name)
// Person can implement AsRef<str> so you can borrow a &str from &Person


// // SPDX-License-Identifier: MIT

// use futures::stream::TryStreamExt;
// use rtnetlink::{new_connection, Error, Handle};

// #[tokio::main]
// async fn main() -> Result<(), ()> {
//     let (connection, handle, _) = new_connection().unwrap();
//     tokio::spawn(connection);

//     let link = "dummy0".to_string();
//     println!("dumping address for link \"{link}\"");

//     if let Err(e) = dump_addresses(handle, link).await {
//         eprintln!("{e}");
//     }

//     Ok(())
// }

// async fn dump_addresses(handle: Handle, link: String) -> Result<(), Error> {
//     let mut links = handle.link().get().match_name(link.clone()).execute();
//     if let Some(link) = links.try_next().await? {
//         let mut addresses = handle
//             .address()
//             .get()
//             .set_link_index_filter(link.header.index)
//             .execute();
//         while let Some(msg) = addresses.try_next().await? {
//             println!("{msg:?}");
//         }
//         Ok(())
//     } else {
//         eprintln!("link {link} not found");
//         Ok(())
//     }
// }


// // use std::ops::{Add, Mul};

// // // Generic Dot product of vectors
// // fn dot<T>(v1: &[T], v2: &[T]) -> T  
// //     where T: Copy + Default + Mul<Output=T> + Add<Output=T> {
// //     let mut sum = T::default();
// //     for i in 0..v1.len() {
// //         sum = sum + v1[i] * v2[i];
// //     }
// //     sum
// // }
 
// // fn main() {
// //     let v1 = [1, 2, 3];
// //     let v2 = [4, 5, 6];
// //     let dot_product = dot(&v1, &v2);
// //     println!("{:?} dot {:?} = {}", v1, v2, dot_product);
// // }