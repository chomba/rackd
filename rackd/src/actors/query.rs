use rusqlite::Connection;
use crate::{util::actor::{Actor, Process}, wan::query::WanQuery};

#[derive(Debug)]
pub struct RackdQueryActor {
    pub conn: Connection
}

#[derive(Debug)]
pub enum RackdQuery {
    Wan(WanQuery)
}

impl Actor for RackdQueryActor {
    type Message = RackdQuery;
    
    fn receive(&mut self, query: RackdQuery) {
        match query {
            RackdQuery::Wan(query) => match query {
                WanQuery::GetWanById(query) => {
                    let response = query.payload.process(self);
                    let _ = query.respond_to.send(response);
                },
                WanQuery::GetWanByName(query) => {
                    let response = query.payload.process(self);
                    let _ = query.respond_to.send(response);
                }
            }
        }
    }
}

impl RackdQueryActor {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }
}