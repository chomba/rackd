use rusqlite::{params, types::FromSql, ToSql, Transaction};
use crate::util::models::Event;

pub trait EventStore {
    // fn load_all(&self) -> Result<
    fn save(&self, e: &Event) -> Result<(), rusqlite::Error>;
    fn save_many<'b>(&self, events: impl IntoIterator<Item = &'b Event>) -> Result<(), rusqlite::Error>;
}

impl<'a> EventStore for Transaction<'a> {
    fn save(&self, e: &Event) -> Result<(), rusqlite::Error> {
        // SET DATETIME AND TRANSACION ID
        // GET/SET Version of each Stream

        // Don't prepare the SQL Statement every time ()
        let mut stmt = self.prepare("INSERT INTO event (id, stream_id, version, data) VALUES (?1, ?2, ?3, ?4)")?;
        stmt.execute(params! { e.id, e.stream_id, e.version, e.data })?;

        // let mut stmt = self.prepare("INSERT INTO event (id, stream_id, version, data) VALUES (?1, ?2, ?3, ?4) RETURNING seq, id, stream_id, version, data")?;
        // let stored_event = stmt.query_row(params! { e.id, e.stream_id, e.version, e.data }, |row| {
        //     Ok(Event {
        //         // seq: row.get(0)?,
        //         id: row.get(0)?,
        //         stream_id: row.get(1)?,
        //         version: row.get(2)?,
        //         data: row.get(3)?
        //     })
        // })?;
        super::projectors().exec(self, &e);
        // super::reactors::reactors().exec(self, &e);
        return Ok(());
    }    

    fn save_many<'b>(&self, events: impl IntoIterator<Item = &'b Event>) -> Result<(), rusqlite::Error> {
        for e in events {
            self.save(e)?;
        }
        Ok(())
    }
}

pub trait KeyValueStore {
    fn get<T>(&self, key: &str) -> Result<Option<T>, rusqlite::Error> where T: FromSql;
    fn set<T>(&self, key: &str, value: &T) -> Result<(), rusqlite::Error> where T: ToSql;
}

impl<'a> KeyValueStore for Transaction<'a> {
    fn get<T>(&self, key: &str) -> Result<Option<T>, rusqlite::Error> where T: FromSql {
        let mut stmt = self.prepare("SELECT value FROM key_value WHERE key = :key")?;
        match stmt.query_row(&[(":key", key)], |row| row.get::<_, T>(0)) {
            Ok(value) => return Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
            Err(e) => Err(e)?
        }
    }

    fn set<T>(&self, key: &str, value: &T) -> Result<(), rusqlite::Error> where T: ToSql {
        let mut stmt = self.prepare("INSERT INTO key_value (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value")?;
        let _ = stmt.execute(params! { key, value })?;
        Ok(())
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

