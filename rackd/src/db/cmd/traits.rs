use log::error;
use rusqlite::{params, types::FromSql, ToSql, Transaction};
use serde::{de::DeserializeOwned, Serialize};
use crate::util::models::{Entity, Event, Id};

pub trait EntityStore {
    fn save<T>(&self, entity: &mut T) -> Result<(), rusqlite::Error> where T: Entity + Serialize;
    fn load<T, K>(&self, id: K) -> Result<Option<T>, rusqlite::Error> where T: Entity + DeserializeOwned, K: Into<Id>;
}

impl<'a> EntityStore for Transaction<'a> {
    fn save<T>(&self, entity: &mut T) -> Result<(), rusqlite::Error> where T: Entity + Serialize {
        let mut stmt = self.prepare("INSERT INTO entity (id, value) VALUES (?1, ?2)")
            .map_err(|e| { error!("prepare() in EntityStore::save() failed: {}", e); e })?;
        
        stmt.execute(params! { entity.id(), serde_json::to_string(&entity).unwrap() })
            .map_err(|e| { error!("execute() in EntityStore::save() failed: {}", e); e })?;

        let events = std::mem::take(&mut entity.metadata().events);
        EventStore::save_many(self, &events)?;
        Ok(())
    }

    fn load<T, K>(&self, id: K) -> Result<Option<T>, rusqlite::Error> where T: Entity + DeserializeOwned, K: Into<Id> {
        let id: Id = id.into();
        let mut stmt = self.prepare("SELECT value FROM entity WHERE id = :id")
            .map_err(|e| { error!("prepare() in EntityStore::load() failed: {}", e); e })?;
        match stmt.query_row(&[(":id", &id.to_string())], |row| row.get::<_, String>(0)) {
            Ok(value) => Ok(Some(serde_json::from_str(value.as_str()).unwrap())),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e).map_err(|e| { error!("query_row() in EntityStore::load() failed: {}", e); e })?
        }
    }
}


pub trait EventStore {
    fn save(&self, e: &Event) -> Result<(), rusqlite::Error>;
    fn save_many<'b>(&self, events: impl IntoIterator<Item = &'b Event>) -> Result<(), rusqlite::Error>;
}

impl<'a> EventStore for Transaction<'a> {
    fn save(&self, e: &Event) -> Result<(), rusqlite::Error> {
        // SET DATETIME AND TRANSACION ID
        // Don't prepare the SQL Statement every time ()
        let mut stmt = self.prepare("INSERT INTO event (id, stream_id, version, data) VALUES (?1, ?2, ?3, ?4)")
            .map_err(|e| { error!("prepare() in EventStore::save() failed: {}", e); e })?;
        stmt.execute(params! { e.id, e.stream_id, e.version, e.data })
            .map_err(|e| { error!("execute() in EventStore::save() failed: {}", e); e })?;

        super::projectors().exec(self, &e);
        // super::reactors::reactors().exec(self, &e);
        Ok(())
    }    

    fn save_many<'b>(&self, events: impl IntoIterator<Item = &'b Event>) -> Result<(), rusqlite::Error> {
        for e in events {
            EventStore::save(self, e)?;
        }
        Ok(())
    }
}

pub trait KeyValueStore {
    fn get<T: FromSql>(&self, key: &str) -> Option<T>;
    fn set<T: ToSql>(&self, key: &str, value: &T);
}

impl<'a> KeyValueStore for Transaction<'a> {
    fn get<T: FromSql>(&self, key: &str) -> Option<T> {
        let mut stmt = self.prepare("SELECT value FROM key_value WHERE key = :key")
            .map_err(|e| error!("prepare() in KeyValueStore::get() failed: {}", e)).unwrap();
        match stmt.query_row(&[(":key", key)], |row| row.get::<_, T>(0)) {
            Ok(value) => Some(value),
            Err(rusqlite::Error::QueryReturnedNoRows) => None,
            Err(e) => Err(e).map_err(|e| error!("query_row() in KeyValueStore::get() failed: {}", e)).unwrap()
        }
    }

    fn set<T: ToSql>(&self, key: &str, value: &T) {
        let mut stmt = self.prepare("INSERT INTO key_value (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value")
            .map_err(|e| error!("prepare() in KeyValueStore::set() failed: {}", e)).unwrap();
        let _ = stmt.execute(params! { key, value })
            .map_err(|e| error!("execute() in KeyValueStore::set() failed: {}", e)).unwrap();
    }
}

// impl<'a> Tx<'a> {
//     pub fn save(&self, e: &Event) -> Result<(), rusqlite::Error> {
//         // SET DATETIME AND TRANSACION ID
//         // GET/SET Version
//         let mut stmt = self.0.prepare("INSERT INTO event (id, stream_id, version, data) VALUES (?1, ?2, ?3, ?4) RETURNING seq, id, stream_id, version, data")?;
//         let stored_event = stmt.query_row(params! { e.id, e.stream_id, e.version, e.data }, |row| {
//             Ok(Event {
//                 // seq: row.get(0)?,
//                 id: row.get(0)?,
//                 stream_id: row.get(1)?,
//                 version: row.get(2)?,
//                 data: row.get(3)?
//             })
//         })?;
//         self.run_on_save(&stored_event)?;
//         Ok(())
//     }

//     pub fn save_many<'b>(&self, events: impl IntoIterator<Item = &'b Event>) -> Result<(), rusqlite::Error> {
//         for e in events {
//             self.save(e)?;
//         }
//         Ok(())
//     }

//     // fn exec_projectors(&self, e: &Event) {
//     //     for (_, projector) in &CmdDb::projectors().0 {
//     //         if let Err(error) = (projector.updater)(&self.0, e) {
//     //             error!("[BUG] Projector failed: {}", error);
//     //             std::process::exit(-1);
//     //         }
//     //     }
//     // }

//     fn run_on_save(&self, e: &Event) -> Result<(), rusqlite::Error> {
//         self.exec_projectors(e);
//     //  self.exec_reactors(e);
//         Ok(())
//     }

//     pub fn get<T>(&self, key: &str) -> Result<T, rusqlite::Error> where T: FromSql {
//         let mut stmt = self.0.prepare("SELECT value FROM key_value WHERE key = :key")?;
//         match stmt.query_row(&[(":key", key)], |row| row.get::<_, T>(0)) {
//             Ok(value) => return Ok(value),
//             // Err(Error::QueryReturnedNoRows) => return Ok(None),
//             Err(e) => Err(e)?
//         }
//     }

//     pub fn set<T>(&self, key: &str, value: &T) -> Result<(), rusqlite::Error> where T: ToSql {
//         let mut stmt = self.0.prepare("INSERT INTO key_value (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value")?;
//         let _ = stmt.execute(params! { key, value })?;
//         Ok(())
//     }

//     // pub fn get_version(&self) -> Result<Version, rusqlite::Error> {
//     //     Ok(self.get::<DbMetadata>("metadata").unwrap_or_default().version)
//     // }
// }

