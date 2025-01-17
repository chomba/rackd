use std::{collections::HashMap, future::Future};
use tokio_util::sync::CancellationToken;
use crate::sys::link::domain::LinkId;

pub trait LinkTracker where Self: Send + 'static {
    fn link(&self) -> LinkId;
    fn work(&mut self) -> impl Future<Output = ()> + Send;
}

pub struct LinkTrackers(HashMap<LinkId, Vec<TrackerHandle>>);

impl LinkTrackers {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn spawn<T>(&mut self, tracker: T) where T: LinkTracker {
        let token = CancellationToken::new();
        let handle = TrackerHandle { token: token.clone() };
        let trackers = match self.0.get_mut(&tracker.link()) {
            Some(trackers) => trackers,
            None => {
                self.0.insert(tracker.link(), Vec::new());
                self.0.get_mut(&tracker.link()).unwrap()
            }
        };
        trackers.push(handle);
        tokio::spawn(Self::run(tracker, token));
    }

    pub fn untrack(&mut self, link: &LinkId) {
        if let Some(trackers) = self.0.get(link) {
            for tracker in trackers {
                tracker.cancel();
            }
        }
        self.0.remove(link);
    }

    async fn run<T>(mut tracker: T, cancellation_token: CancellationToken) where T: LinkTracker {
        tokio::select! {
            _ = cancellation_token.cancelled() => {
                // Log termination
            }
            _ = tracker.work() => {
                // Work Terminated
            }
        }
    }
}

#[derive(Clone)]
pub struct TrackerHandle {
    token: CancellationToken
    // channel sender?
}

impl TrackerHandle {
    pub fn cancel(&self) {
        self.token.cancel();
    }
}
