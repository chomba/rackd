pub mod views;
use std::marker::PhantomData;
use rusqlite::{Error, Transaction};
use crate::{net::shared::models::NetName, util::{db::{DbQuery, View}, query::GetByKey}};
use super::models::WanId;

pub struct GetWanByName<T> where T: View {
    pub name: NetName,
    pub view: PhantomData<T>
}

impl<T> DbQuery for GetWanByName<T> where T: View {
    type Ok = Option<T>;
    type Err = Error;

    fn run(self, db: &Transaction) -> Result<Self::Ok, Self::Err> {
        let query = GetByKey {
            key: "name",
            value: self.name,
            view: PhantomData::<T>
        };
        query.run(&db)
    }
}

pub struct GetWanById<T> where T: View {
    pub id: WanId,
    pub view: PhantomData<T>
}

impl<T> DbQuery for GetWanById<T> where T: View {
    type Ok = Option<T>;
    type Err = Error;

    fn run(self, db: &Transaction) -> Result<Self::Ok, Self::Err> {
        let query = GetByKey {
            key: "id",
            value: self.id,
            view: PhantomData::<T>
        };
        query.run(&db)
    }
}
