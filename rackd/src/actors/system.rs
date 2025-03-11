use log::error;
use rusqlite::Connection;
use semver::Version;
use crate::{conf, util::actor::{Actor, AsyncActor, Handle}};
use super::cmd::{RackdCmd, RackdCmdActor};
use crate::db;

// The API Actor (Handling REST/gRPC request) is going to front the
// CmdActor and the QueryActor, meaning the API Actor needs to hold both the Cmd Actor
// and the QueryActor and is going to publicly expose their behavior to end-users
// 

// Our Error Kernel (Components of the program that should NEVER fail) includes:
// - Internal EventBus
// - Supervision (Restart, Resume, Reinitialize Actors)

// pub struct ActorSystem {
//     pub app: Handle<AppMessage>,
//     pub sys: Handle<SysMessage>
//     // pub watchdog: ActorHandle<WatchdogActor>
// }

// struct ActorInstance<A> where A: Actor {
//     id: Id,
//     node: Id,
//     address: Ipv6Addr,
//     port: u32,
//     status: ActorStatus,
//     actor: A,
//     receiver: mpsc::Receiver<A::Message>,
//     sender: mpsc::Sender<A::Message>
// }

pub enum ActorStatus {
    Learner, // Initial State
    Follower,
    Leader
}

// pub async fn run_jobs(sender: tokio::sync::mpsc::Sender<AppMessage>) {
//     loop {
//         time::sleep(Duration::from_secs(3)).await;
//         sender.send(AppMessage::RunJobs).await;
//     }
// }



pub struct ActorSystem {
    pub cmd: Handle<RackdCmd>
}

impl ActorSystem {
    pub fn mock() -> Self {
        let conn = Connection::open_in_memory()
            .map_err(|e| error!("Failed to open in-memory db: {}", e))
            .unwrap();
        Self::new(conn)
    }

    pub fn new(cmd_db: Connection) -> Self {
        // Run Db Migrations
        // let settings = conf::settings();
        let conn = db::cmd::migrations::runner().run(cmd_db);
        // Spawn Actors
        let cmd_handle = RackdCmdActor::spawn(RackdCmdActor::new(conn));
        Self { cmd: cmd_handle }


        // let (tx_app, rx_app) = mpsc::channel::<AppMessage>(10);
        // let (tx_sys, rx_sys) = mpsc::channel::<SysMessage>(10);
        
        // // let app_token = CancellationToken::new();
        // let app_handle = Handle { sender: tx_app };
        // let sys_token = CancellationToken::new();
        // let sys_handle = Handle { sender: tx_sys };

        // let db = Db::new(path).expect("Failed to Create DB");
        // let db = match MIGRATIONS.run(db) {
        //     Ok(db) => db,
        //     Err(e) => { 
        //         println!("{e:?}");
        //         panic!("Failed to run Migrations");
        //     }
        // };
        // // log
        // let netlink = Netlink::connect().expect("Failed to Connect to Netlink socket");
        // let app = AppActor { db, sys: sys_handle.clone() };

        // // Ebpf
        // let sys = SysActor { 
        //     netlink, 
        //     trackers: LinkTrackers::new(),
        //     app: app_handle.clone() 
        // };
        // tokio::task::spawn_blocking(AppActor::run(app, rx_app));
        // tokio::spawn(SysActor::run(sys, rx_sys, sys_token));
        // Self { app: app_handle, sys: sys_handle }
    }

    // pub fn version() -> Version {
    //     let version = env!("CARGO_PKG_VERSION");
    //     Version::parse(version).unwrap()
    // }
}

// #[cfg(test)]
// mod tests {
//     use crate::app::data::{registry::MIGRATIONS, Db};

//     #[test]
//     pub fn run_migrations() {
//         let db = Db::new(Option::<&str>::None).unwrap();
//         assert!(MIGRATIONS.run(db).is_ok());
//     }
// }
