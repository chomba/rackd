use std::marker::PhantomData;
use rusqlite::types::FromSql;
use rusqlite::{named_params, Error, ToSql};
use crate::app::data::framework::traits::MapRow;
use crate::app::data::DbSession;
use crate::{app::{data::framework::traits::DbQuery, error::AppError, shared::domain::Entity}, util::domain::Id};

// pub struct GetById<T> {
//     pub id: Id,
//     pub entity: PhantomData<T>
// }

// impl<T> DbQuery for GetById<T> where T: Entity + MapRow {
//     type Err = AppError;
//     type Ok = T;

//     fn run(&self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
//         let query = GetByField {
//             field_name: "id",
//             field_value: self.id,
//             entity: PhantomData::<T>
//         };
//         query.run(&db)
//     }
// }

// pub struct GetByName<T, N> {
//     pub name: N,
//     pub entity: PhantomData<T>
// }

// impl<T, N> DbQuery for GetByName<T, N> where T: Entity + MapRow {
//     type Err = AppError;
//     type Ok = T;

//     fn run(&self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
//         let query = GetByField {
//             field_name: "name",
//             field_value: self.name,
//             entity: PhantomData::<T>
//         };
//         query.run(&db)
//     }
// }

pub struct GetByField<'a, T, F> where T: Entity + MapRow, F: ToSql + FromSql {
    pub field_name: &'a str,
    pub field_value: F,
    pub entity: PhantomData<T>
}

impl<'a, T, F> DbQuery for GetByField<'a, T, F> where T: Entity + MapRow, F: ToSql + FromSql {
    type Ok = T;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let sql = format!("{} WHERE {} = :field AND deleted = :deleted", T::select(), self.field_name);
        match db.tx().query_row(&sql, named_params! { ":field": self.field_value, ":deleted": false }, T::map) {
            Ok(isp) => Ok(isp),
            Err(Error::QueryReturnedNoRows) => Err(AppError::NotFound)?,
            Err(e) => Err(e)?
        }
    }
}

pub struct GetAll<T> {
    pub entity: PhantomData<T>
}

impl<T> DbQuery for GetAll<T> where T: Entity + MapRow {
    type Ok = Vec<T>;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let mut isps = vec![];
        let sql = format!("{} WHERE deleted = :deleted", T::select());
        let mut stmt = db.tx().prepare(&sql)?;
        let rows = stmt.query_map(named_params! { ":deleted": false }, T::map)?;
        for row in rows {
            isps.push(row?);
        }
        Ok(isps)
    }
}

pub struct GetCount<T> {
    pub entity: PhantomData<T>
}

impl<T> DbQuery for GetCount<T> where T: Entity + MapRow {
    type Err = AppError;
    type Ok = usize; 

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let sql = format!("SELECT COUNT(*) FROM {} WHERE deleted = :deleted", T::table());
        let mut stmt = db.tx().prepare(&sql)?;
        match stmt.query_row(named_params! { ":deleted": false }, |row| row.get::<_, usize>(0)) {
            Ok(count) => Ok(count),
            Err(Error::QueryReturnedNoRows) => Ok(0),
            Err(e) => Err(e)?
        }
    }
}