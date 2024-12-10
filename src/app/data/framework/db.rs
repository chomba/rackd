use std::{path::Path, time::Duration};
use rusqlite::{params, types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, Connection, Error, ToSql, Transaction, TransactionBehavior};
use semver::{BuildMetadata, Prerelease, Version};
use serde::{Deserialize, Serialize};
use crate::app::{data::registry::*, error::AppError, shared::domain::*};
use super::traits::DbQuery;

pub struct Db {
    pub conn: Connection,
    pub config: DbSessionConfig,
}

impl Db {
    pub fn new<P>(path: Option<P>) -> Result<Db, AppError> where P: AsRef<Path> {
        let conn = match path {
            Some(path) => Connection::open(path)?,
            None => Connection::open_in_memory()?
        };
        // let metadata = Db::read_metadata(&conn)?;
        Ok(Db { conn, config: DbSessionConfig::default() }) 
    }

    pub fn lock(&mut self) -> &mut Self {
        self.config.behavior = SessionBehavior(TransactionBehavior::Immediate);
        self
    }

    pub fn retry(&mut self, count: u8, interval: Duration) -> &mut Self {
        self.config.retry = DbSessionRetry { count, interval };
        self
    }

    // RENAME TO Begin
    pub fn begin(&mut self) -> Result<DbSession, rusqlite::Error> {
        let config = std::mem::take(&mut self.config);
        let mut retries = config.retry.count;
        loop {
            match Transaction::new_unchecked(&self.conn, config.behavior.0) {
                Ok(tx) => return Ok(DbSession { tx }),
                Err(_) if retries > 1 => {
                    retries -= 1;
                    std::thread::sleep(config.retry.interval);
                    continue;
                },
                Err(error) => Err(error)?
            }
        }
    }
}

pub struct DbSession<'a> {
    pub tx: Transaction<'a>
}

impl<'a> DbSession<'a> {
    pub fn new(tx: Transaction<'a>) -> Self {
        Self { tx }
    }

    pub fn tx(&self) -> &Transaction {
        &self.tx
    }

    pub fn run<Q>(&self, query: Q) -> Result<Q::Ok, Q::Err> where Q: DbQuery  {
        query.run(&self)
        // Ok((query, result))
    }

    pub fn commit(self) -> Result<(), rusqlite::Error> {
        Ok(self.tx.commit()?)
    }

    pub fn presave<T>(&self, entity: &mut T) -> Result<(), AppError> where T: Entity, T::Event: Into<InnerEvent> {
        let events = entity.events();
        // let mut stored_events = Vec::with_capacity(events.len());
        for e in events {
            let mut stmt = self.tx.prepare("INSERT INTO event (id, stream_id, version, data) VALUES (?1, ?2, ?3, ?4) RETURNING seq, id, stream_id, version, data")?;
            let stored_event = stmt.query_row(params! { e.id, e.stream_id, e.version, e.data }, |row| {
                Ok(Event {
                    seq: row.get(0)?,
                    id: row.get(1)?,
                    stream_id: row.get(2)?,
                    version: row.get(3)?,
                    data: row.get(4)?
                })
            })?;
            self.run_on_save(&stored_event)?;
            // stored_events.push(stored_event);
        }
        // for e in stored_events {
        //     self.run_on_save(&e)?;
        // }
        Ok(())
    }

    pub fn save<T>(self, entity: &mut T) -> Result<(), AppError> where T: Entity, T::Event: Into<InnerEvent> {
        self.presave(entity)?;
        self.tx.commit()?;
        Ok(())
    }

    pub fn run_on_save(&self, event: &Event) -> Result<(), AppError> {
        PROJECTIONS.run(&self, event);
        REACTORS.run(&self, event);
        Ok(())
    }

    pub fn get<T>(&self, key: &str) -> Result<T, rusqlite::Error> where T: FromSql {
        let mut stmt = self.tx.prepare("SELECT value FROM key_value WHERE key = :key")?;
        match stmt.query_row(&[(":key", key)], |row| row.get::<_, T>(0)) {
            Ok(value) => return Ok(value),
            // Err(Error::QueryReturnedNoRows) => return Ok(None),
            Err(e) => Err(e)?
        }
    }

    pub fn set<T>(&self, key: &str, value: &T) -> Result<(), rusqlite::Error> where T: ToSql {
        let mut stmt = self.tx.prepare("INSERT INTO key_value (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value")?;
        let _ = stmt.execute(params! { key, value })?;
        Ok(())
    }

    pub fn get_version(&self) -> Result<Version, AppError> {
        Ok(self.get::<DbMetadata>("metadata").unwrap_or_default().version)
    }

    // pub fn set_version
}

pub struct DbSessionConfig {
    pub retry: DbSessionRetry,
    pub behavior: SessionBehavior
}

pub struct SessionBehavior(pub TransactionBehavior);

impl std::fmt::Debug for SessionBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tx_behavior = match self.0 {
            TransactionBehavior::Immediate => "Immediate",
            TransactionBehavior::Deferred => "Deferred",
            TransactionBehavior::Exclusive => "Exclusive",
            _ => ""
        };
        f.debug_struct("SessionBehavior").field("0", &tx_behavior).finish()
    }
}

#[derive(Debug)]
pub struct DbSessionRetry {
    pub count: u8,
    pub interval: Duration
}

impl Default for DbSessionConfig {
    fn default() -> Self {
        Self {
            retry: DbSessionRetry { count: 1, interval: Duration::from_millis(0) },
            behavior: SessionBehavior(TransactionBehavior::Deferred)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbMetadata {
    pub version: Version,
}

impl Default for DbMetadata {
    fn default() -> Self {
        Self { 
            version: Version {
                major: u64::default(),
                minor: u64::default(),
                patch: u64::default(),
                build: BuildMetadata::EMPTY,
                pre: Prerelease::EMPTY 
            }
        }
    }
}

impl FromSql for DbMetadata {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?)
    }
}

impl ToSql for DbMetadata {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?.into())
    }
}

// Sqlite has no support for table-level blocks
// Meaning it will block the entire database to guarantee isolation
// https://www.sqlite.org/isolation.html
// https://www.sqlite.org/rescode.html#busy
// https://www.sqlite.org/lang_transaction.html

// We need to atomically to update the events table and all projections
// used by our write (CMD) model, so we need to immediately place a WRITE
// lock when a Write Command is executed as such we need to use BEGIN_IMMEDIATE
// Read more here: https://www.sqlite.org/lang_transaction.html#immediate
// Only One Transaction can be run at a time with a single SQLITE Connection
// The CMDActor is only going to process one message at a time
// so it makes sense for a single CMDActor instance to hold a single connection
// and not create a new connection each time a message is processed
// It's possible to spin up multiple CMDActor instances but given that 
// SQLite only allows at most once WRITER to proceed concurrently, it would
// not actually increase performance on the WRITER Side (writer-writer concurrency still not possible)
//, and it may be much better to just increase the channel size, but it might let the API server
// serve a READ and WRITE operation at the same time (read+write concurrency)
