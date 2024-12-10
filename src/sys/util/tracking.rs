use std::{collections::HashMap, future::Future};
use tokio_util::sync::CancellationToken;
use crate::util::domain::Id;
use super::netlink::Netlink;

pub trait Tracker where Self: Send + 'static {
    fn work(&mut self, netlink: Netlink) -> impl Future<Output = ()> + Send;
}

pub struct Trackers(HashMap<Id, TrackerHandle>);

impl Trackers {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn spawn<T>(&mut self, tracker: T, netlink: Netlink) -> TrackerHandle where T: Tracker {
        let token = CancellationToken::new();
        let id = Id::new();
        let handle = TrackerHandle { tracker_id: id, token: token.clone() };
        self.0.insert(id, handle.clone());
        tokio::spawn(Self::run(tracker, netlink, token));
        handle
    }

    pub fn cancel(&mut self, id: &Id) {
        if let Some(tracker) = self.0.get(id) {
            tracker.cancel();
        }
        self.0.remove(id);
    }

    async fn run<T>(mut tracker: T, netlink: Netlink, cancel: CancellationToken) where T: Tracker {
        tokio::select! {
            _ = cancel.cancelled() => {
                // Log termination
            }
            _ = tracker.work(netlink) => {
                // Work Terminated
            }
        }
    }
}

#[derive(Clone)]
pub struct TrackerHandle {
    pub tracker_id: Id,
    token: CancellationToken
    // channel sender?
}

impl TrackerHandle {
    pub fn cancel(&self) {
        self.token.cancel();
    }
}
