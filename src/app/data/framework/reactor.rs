use crate::app::shared::domain::Event;
use super::{db::DbSession, traits::DbHandler};

pub struct Reactors(Vec<DbHandler>);

impl Reactors {
    pub fn new(reactors: Vec<DbHandler>) -> Self {
        Self(reactors)
    }

    pub fn run(&self, db: &DbSession, e: &Event) {
        for reactor in &self.0 {
            reactor(db, e);
        }
    }
}
