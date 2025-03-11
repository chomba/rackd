use std::{any::type_name, marker::PhantomData};
use log::error;
use rusqlite::{named_params, types::FromSql, ToSql, Transaction};
use serde::{Deserialize, Serialize};
use crate::util::models::Event;

pub trait View: Sized {
    // fn meta(&mut self) -> &mut Metadata;
    fn name() -> &'static str;
    fn update(tx: &Transaction, e: &Event);
    // /// Convert SqliteRow to View 
    fn try_from(row: &rusqlite::Row) -> Result<Self, rusqlite::Error>;
    fn sql_select() -> &'static str;
    // fn sql_create() -> &'static str;
}

// #[derive(Debug, Serialize, Deserialize, Default)]
// pub struct Metadata {
//     pub version: u32,
//     // last_updated_on
// }

pub trait DbQuery {
    type Result;
    fn run(self, tx: &Transaction) -> Self::Result; 
}

pub trait QueryRunner {
    fn run<Q: DbQuery>(&self, query: Q) -> Q::Result;
}

impl<'a> QueryRunner for Transaction<'a> {
    fn run<Q: DbQuery>(&self, query: Q) -> Q::Result {
        query.run(self)
    }
}

pub struct GetByKey<'a, T, F> where T: View, F: ToSql + FromSql {
    pub key: &'a str,
    pub value: F,
    pub view: PhantomData<T>
}

impl<'a, T, F> DbQuery for GetByKey<'a, T, F> where T: View, F: ToSql + FromSql {
    type Result = Option<T>;

    fn run(self, tx: &Transaction) -> Self::Result {
        let sql = format!("{} WHERE {} = :field AND deleted = :deleted", T::sql_select(), self.key);
        match tx.query_row(&sql, named_params! { ":field": self.value, ":deleted": false }, T::try_from) {
            Ok(value) => Some(value),
            Err(rusqlite::Error::QueryReturnedNoRows) => None,
            Err(e) => Err(e).map_err(|e| error!("query_row() in GetByKey<{}> failed: {}", type_name::<T>(), e)).unwrap()
        }
    }
}

pub struct GetAll<T> {
    pub view: PhantomData<T>
}

impl<T> DbQuery for GetAll<T> where T: View {
    type Result = Vec<T>;

    fn run(self, tx: &Transaction) -> Self::Result {
        let mut items = vec![];
        let sql = format!("{} WHERE deleted = :deleted", T::sql_select());
        let mut stmt = tx.prepare(&sql)
            .map_err(|e| error!("prepare() in GetAll<{}> failed: {}", type_name::<T>(), e))
            .unwrap();
        let rows = stmt.query_map(named_params! { ":deleted": false }, T::try_from)
            .map_err(|e| error!("query_map() in GetAll<{}> failed: {}", type_name::<T>(), e)).unwrap();
        for row in rows {
            items.push(row.map_err(|e| error!("Failed to map query results to view {}: {}", std::any::type_name::<T>(), e)).unwrap());
        }
        items
    }
}

pub struct GetCount<T> {
    pub view: PhantomData<T>
}

impl<T> DbQuery for GetCount<T> where T: View {
    type Result = usize; 

    fn run(self, tx: &Transaction) -> Self::Result {
        let sql = format!("SELECT COUNT(*) FROM {} WHERE deleted = :deleted", T::name());
        let mut stmt = tx.prepare(&sql)
            .map_err(|e| error!("prepare() in GetCount<{}> failed: {}", type_name::<T>(), e))
            .unwrap();
        match stmt.query_row(named_params! { ":deleted": false }, |row| row.get::<_, usize>(0)) {
            Ok(count) => count,
            Err(rusqlite::Error::QueryReturnedNoRows) => 0,
            Err(e) => Err(e).map_err(|e| error!("query_row() in GetCount<{}> failed: {}", type_name::<T>(), e)).unwrap()
        }
    }
}

