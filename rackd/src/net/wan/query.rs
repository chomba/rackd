pub mod views;
use std::marker::PhantomData;
use rusqlite::Transaction;
use crate::{db::query::traits::{DbQuery, GetByKey, View}, net::shared::models::NetName};
use super::models::WanId;

pub struct GetWanByName<T> where T: View {
    pub name: NetName,
    pub view: PhantomData<T>
}

impl<T> DbQuery for GetWanByName<T> where T: View {
    type Result = Option<T>;

    fn run(self, db: &Transaction) -> Self::Result {
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
    type Result = Option<T>;

    fn run(self, db: &Transaction) -> Self::Result {
        let query = GetByKey {
            key: "id",
            value: self.id,
            view: PhantomData::<T>
        };
        query.run(&db)
    }
}
