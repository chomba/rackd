use std::marker::PhantomData;
use rusqlite::{named_params, types::FromSql, ToSql, Transaction};
use super::db::{DbQuery, View};

pub struct GetByKey<'a, T, F> where T: View, F: ToSql + FromSql {
    pub key: &'a str,
    pub value: F,
    pub view: PhantomData<T>
}

impl<'a, T, F> DbQuery for GetByKey<'a, T, F> where T: View, F: ToSql + FromSql {
    type Ok = Option<T>;
    type Err = rusqlite::Error;

    fn run(self, tx: &Transaction) -> Result<Self::Ok, Self::Err> {
        let sql = format!("{} WHERE {} = :field AND deleted = :deleted", T::sql_select(), self.key);
        match tx.query_row(&sql, named_params! { ":field": self.value, ":deleted": false }, T::try_from) {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e)
        }
    }
}

pub struct GetAll<T> {
    pub view: PhantomData<T>
}

impl<T> DbQuery for GetAll<T> where T: View {
    type Ok = Vec<T>;
    type Err = rusqlite::Error;

    fn run(self, tx: &Transaction) -> Result<Self::Ok, Self::Err> {
        let mut items = vec![];
        let sql = format!("{} WHERE deleted = :deleted", T::sql_select());
        let mut stmt = tx.prepare(&sql)?;
        let rows = stmt.query_map(named_params! { ":deleted": false }, T::try_from)?;
        for row in rows {
            items.push(row?);
        }
        Ok(items)
    }
}

pub struct GetCount<T> {
    pub view: PhantomData<T>
}

impl<T> DbQuery for GetCount<T> where T: View {
    type Err = rusqlite::Error;
    type Ok = usize; 

    fn run(self, tx: &Transaction) -> Result<Self::Ok, Self::Err> {
        let sql = format!("SELECT COUNT(*) FROM {} WHERE deleted = :deleted", T::name());
        let mut stmt = tx.prepare(&sql)?;
        match stmt.query_row(named_params! { ":deleted": false }, |row| row.get::<_, usize>(0)) {
            Ok(count) => Ok(count),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(0),
            Err(e) => Err(e)?
        }
    }
}