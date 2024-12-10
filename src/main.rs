
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

use tokio::signal;

// Authentication:
// https://fly.io/blog/api-tokens-a-tedious-survey/
// Use Autehnticated requests?

#[tokio::main]
async fn main() {    
    // Pass configuration file as argument border66 -conf=/etc/border66/config
    // Run ApiActor
    match signal::ctrl_c().await {
        Ok(()) => { 
            println!("Program Terminated");
        },
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        }
    }
    
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