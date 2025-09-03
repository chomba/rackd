use std::marker::PhantomData;
use rusqlite::Transaction;
use crate::{actors::cmd::RackdCmdActor, db::{query::traits::{DbQuery, GetByKey}, Tx}, trunk::{model::{TrunkId, TrunkName}, views::TrunkView}, util::actor::{Payload, Process}};

pub struct GetTrunkById {
    pub id: TrunkId
}

impl DbQuery for GetTrunkById {
    type Ok = Option<TrunkView>;

    fn run(&self, tx: &Transaction) -> Result<Self::Ok, rusqlite::Error> {
        let query = GetByKey {
            key: "id",
            value: &self.id.0,
            view: PhantomData::<TrunkView>
        };
        query.run(&tx)
    }
}

impl Payload for GetTrunkById {
    type Ok = Option<TrunkView>;
    type Err = rusqlite::Error;
}

impl Process for GetTrunkById {
    type Actor = RackdCmdActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.tx()?;
        self.run(&tx)
    }
}

pub struct GetTrunkByName {
    pub name: TrunkName
}

impl DbQuery for GetTrunkByName {
    type Ok = Option<TrunkView>;

    fn run(&self, tx: &Transaction) -> Result<Self::Ok, rusqlite::Error> {
        let query = GetByKey {
            key: "name",
            value: &self.name,
            view: PhantomData::<TrunkView>
        };
        query.run(&tx)
    }
}

impl Payload for GetTrunkByName {
    type Ok = Option<TrunkView>;
    type Err = rusqlite::Error;
}

impl Process for GetTrunkByName {
    type Actor = RackdCmdActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let tx = actor.conn.tx()?;
        self.run(&tx)
    }
}