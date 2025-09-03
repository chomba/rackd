use std::{any::type_name, marker::PhantomData};
use log::error;
use rusqlite::{named_params, types::FromSql, ToSql, Transaction};
use crate::util::models::Event;

pub trait DbView: Sized {
    // fn meta(&mut self) -> &mut Metadata;
    fn name() -> &'static str;
    fn update(tx: &Transaction, e: &Event);
    // /// Convert SqliteRow to View 
    fn try_from(row: &rusqlite::Row) -> Result<Self, rusqlite::Error>;
    fn select_fields() -> &'static str;
    fn sql_select() -> String {
        format!("SELECT {} FROM {}", Self::select_fields(), Self::name())
    }
    // fn sql_create() -> &'static str;
}

// #[derive(Debug, Serialize, Deserialize, Default)]
// pub struct Metadata {
//     pub version: u32,
//     // last_updated_on
// }

pub trait DbQuery {
    type Ok;
    fn run(&self, tx: &Transaction) -> Result<Self::Ok, rusqlite::Error>; 
}

pub trait QueryRunner {
    fn run<Q: DbQuery>(&self, query: Q) -> Result<Q::Ok, rusqlite::Error>;
}

impl<'a> QueryRunner for Transaction<'a> {
    fn run<Q: DbQuery>(&self, query: Q) -> Result<Q::Ok, rusqlite::Error> {
        query.run(self)
    }
}

pub struct GetByKey<'a, T, F> where T: DbView, F: ToSql + FromSql {
    pub key: &'a str,
    pub value: &'a F,
    pub view: PhantomData<T>
}

impl<'a, T, F> DbQuery for GetByKey<'a, T, F> where T: DbView, F: ToSql + FromSql {
    type Ok = Option<T>;

    fn run(&self, tx: &Transaction) -> Result<Self::Ok, rusqlite::Error> {
        let sql = format!("{} WHERE {} = :field AND deleted = :deleted", T::sql_select(), self.key);
        match tx.query_row(&sql, named_params! { ":field": self.value, ":deleted": false }, T::try_from) {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e).map_err(|e| { 
                error!("query_row() in GetByKey<{}> failed: {}", type_name::<T>(), e); 
                e 
            })?
        }
    }
}

pub struct GetAll<T> {
    pub view: PhantomData<T>
}

impl<T> DbQuery for GetAll<T> where T: DbView {
    type Ok = Vec<T>;

    fn run(&self, tx: &Transaction) -> Result<Self::Ok, rusqlite::Error> {
        let mut items = vec![];
        let sql = format!("{} WHERE deleted = :deleted", T::sql_select());
        let mut stmt = tx.prepare(&sql)
            .map_err(|e| { 
                error!("prepare() in GetAll<{}> failed: {}", type_name::<T>(), e);
                e
            })?;
        let rows = stmt.query_map(named_params! { ":deleted": false }, T::try_from)
            .map_err(|e| {
                error!("query_map() in GetAll<{}> failed: {}", type_name::<T>(), e);
                e
            })?;
        for row in rows {
            items.push(row.map_err(|e| { 
                error!("Failed to map query results to view {}: {}", std::any::type_name::<T>(), e);
                e
            })?);
        }
        Ok(items)
    }
}

pub struct GetCount<T> {
    pub view: PhantomData<T>
}

impl<T> DbQuery for GetCount<T> where T: DbView {
    type Ok = usize;

    fn run(&self, tx: &Transaction) -> Result<Self::Ok, rusqlite::Error>  {
        let sql = format!("SELECT COUNT(*) FROM {} WHERE deleted = :deleted", T::name());
        let mut stmt = tx.prepare(&sql)
            .map_err(|e| { 
                error!("prepare() in GetCount<{}> failed: {}", type_name::<T>(), e);
                e
            })?;
            
        match stmt.query_row(named_params! { ":deleted": false }, |row| row.get::<_, usize>(0)) {
            Ok(count) => Ok(count),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(0),
            Err(e) => Err(e).map_err(|e| { 
                error!("query_row() in GetCount<{}> failed: {}", type_name::<T>(), e);
                e
            })?
        }
    }
}

