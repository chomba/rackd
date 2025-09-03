use log::error;
use rusqlite::Connection;
use crate::util::actor::{Actor, Handle, Msg, Payload};
use super::{cmd::{RackdCmd, RackdCmdActor}, query::{RackdQuery, RackdQueryActor}};
use crate::db;
use thiserror::Error;

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

#[derive(Clone)]
pub struct Rackd {
    pub cmd: Handle<RackdCmd>,
    pub query: Handle<RackdQuery>
}

#[derive(Debug, Error)]
pub enum RackdError {
    #[error("{}", .0)]
    Db(#[from] rusqlite::Error)
}

impl Rackd {
    pub async fn exec<P>(&self, cmd: P) -> Result<P::Ok, P::Err> where P: Payload, RackdCmd: From<Msg<P>> {
        self.cmd.send(cmd).await
    }

    pub async fn query<P>(&self, query: P) -> Result<P::Ok, P::Err> where P: Payload, RackdQuery: From<Msg<P>> {
        // TBD: Schedule query to pool of QueryActors (only if stress tests show a single actor won't be enough)
        self.query.send(query).await
    }

    pub fn mock() -> Result<Self, RackdError> {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Error)
            .format_target(false)
            .format_timestamp(None)
            .try_init();
        db::cmd::migrations::runner().run(Connection::open_in_memory()?);
        let cmd = RackdCmdActor::spawn(RackdCmdActor::new(Connection::open_in_memory()?));
        let query = RackdQueryActor::spawn(RackdQueryActor::new(Connection::open_in_memory()?));
        Ok(Self { cmd, query })
    }

    pub fn new(path: &str) -> Result<Self, RackdError> {
        db::cmd::migrations::runner().run(Connection::open(path).unwrap());
        let cmd = RackdCmdActor::spawn(RackdCmdActor::new(Connection::open(path)?)); 
        let query = RackdQueryActor::spawn(RackdQueryActor::new(Connection::open(path)?)); 
        Ok(Self { cmd, query })

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
