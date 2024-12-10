use std::collections::HashMap;
use crate::app::shared::domain::Event;
use super::{db::DbSession, traits::DbHandler};

pub struct Projection {
    table: String,
    update: DbHandler
}

impl Projection {
    pub fn new(table: &str, update: DbHandler) -> Self {
        Self {
            table: String::from(table), 
            update
        }
    }
}

pub struct Projections(HashMap<String, Projection>);

impl Projections {
    pub fn new<P>(projections: P) -> Self where P: IntoIterator<Item = Projection> {
        Self(HashMap::from_iter(projections.into_iter().map(|p| (p.table.clone(), p))))
    }

    pub fn run(&self, db: &DbSession, e: &Event) {
        for (table, projection) in &self.0 {
            if let Err(error) = (projection.update)(db, e) {
                // Refactor Introduce Logging
                println!("Failed to run projection: {error:?}");
                panic!("Failed to run projection");
            }
        }
    }

    //     fn rebuild(db: &DbSession, events: &Vec<Event>) -> Result<(), AppError> {
//         let sql = format!("DROP TABLE {}", Self::table());
//         db.tx().execute(&sql, ())?;
//         for e in events {
//             Self::update(db, e);
//         }
//         Ok(())
//     }

    // rebuild given table?
}