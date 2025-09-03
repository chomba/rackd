use std::collections::HashMap;
use rusqlite::{types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, Connection, ToSql, Transaction};
use serde::{Deserialize, Serialize};
use include_dir::Dir;
use log::{error, info};
use std::{collections::BTreeMap, ops::Bound::{Excluded, Included}};
use semver::Version;
use crate::{db::cmd::traits::*, util::models::Event};
use super::query::traits::DbView;

pub struct Projectors(pub HashMap<String, Projector>);

impl Projectors {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn register<T: DbView>(&mut self) {
        let projector = Projector {
            table: T::name(),
            apply: T::update,
            // sql_create: T::sql_create()
        };
        self.0.insert(String::from(projector.table), projector);
        // save 
    }

    pub fn exec(&self, tx: &Transaction, e: &Event) {
        for (_, projector) in &self.0 {
            (projector.apply)(tx, e)
        }
    }

    // pub fn rebuild<T: View>(&self, tx: &Transaction) {
    //     info!("[REBUILD_PROJECTION] Rebuilding Projection for View {}", T::name());
    //     if let Err(e) = tx.execute(&format!("DROP TABLE IF EXISTS {}", T::name()), ()) {
    //         error!("Failed to Drop Table {} when rebuilding projection: {}", T::name(), e);
    //         std::process::exit(-1)
    //     }
    //     info!("[REBUILD_PROJECTION] View Table Dropped");
    //     if let Err(e) = tx.execute(T::sql_create(), ()) {
    //         error!("Failed to Recreate {} when rebuilding projection: {}", T::name(), e);
    //         std::process::exit(-1)
    //     }
    //     info!("[REBUILD_PROJECTION] View Table Recreated");
    //     // TBD
    // }
}

pub struct Projector {
    pub table: &'static str,
    pub apply: fn(&Transaction, &Event)
    // pub sql_create: &'static str
}

pub type DbAction = fn(&Transaction) -> Result<(), rusqlite::Error>;
pub type DbHandler = fn(&Transaction, &Event) -> Result<(), rusqlite::Error>;

pub struct Migration {
    version: Version,
    up: Option<MigrationFn>,
    down: Option<MigrationFn>
}

pub type MigrationFn = fn(&Transaction) -> ();

impl Migration {
    pub fn to(version: &str) -> Self {
        let version = Version::parse(version).unwrap();
        Self { version, up: None, down: None }
    }

    pub fn up(self, action: MigrationFn) -> Self {
        Self { up: Some(action), ..self }
    }

    pub fn down(self, action: MigrationFn) -> Self {
        Self { down: Some(action), ..self }
    }
}

pub struct MigrationRunner<'a> {
    pub schemas: Dir<'a>, 
    pub migrations: BTreeMap<Version, Migration>  
}

impl<'a> MigrationRunner<'a> {
    pub fn new(schemas: Dir<'a>) -> Self {
        Self {
            schemas,
            migrations: BTreeMap::new()
        }
    }

    pub fn register<M>(mut self, migrations: M) -> Self where M: IntoIterator<Item = Migration> {
        for m in migrations {
            self.migrations.insert(m.version.clone(), m);
        }
        self
        // Self(BTreeMap::from_iter(migrations.into_iter().map(|m| (m.version.clone(), m))))
    }

    pub fn run(&self, mut conn: Connection) -> Connection {
        let tx = conn.transaction().map_err(|e| error!("Failed to open tx: {}", e)).unwrap();
        let table_count = match tx.query_row("SELECT COUNT(*) FROM sqlite_master WHERE type='table'", [], |row| row.get::<_, usize>(0)) {
            Ok(count) => count,
            Err(_) => 0
        };

        let metadata = match table_count { 
            0 => DbMetadata::default(),
            _ => {
                match tx.get::<DbMetadata>("metadata") {
                    Some(value) => value,
                    None => {
                        info!("Database is empty, using defaults");
                        DbMetadata::default()
                    }
                }
            }
        };
        let _ = tx.commit();
        let bin_version = version();
        let db_version = metadata.version.clone();
        info!("Database Version: {} - Binary Version: {}", db_version, bin_version);

        if bin_version < db_version {
            error!("Binary Version is Higher than Db Schema");
            panic!();
        }
        
        for (version, migration) in self.migrations.range((Excluded(db_version), Included(bin_version))) {
            let tx = conn.transaction().map_err(|e| error!("{}", e)).unwrap();
            Self::deploy(tx, migration, &metadata, &self.schemas);
            info!("FINISHED RUNNING MIGRATION TO {}", version)
        }
        conn
    }

    fn deploy(tx: Transaction, migration: &Migration, metadata: &DbMetadata, schemas: &Dir) {
        let target_version = migration.version.clone();
        let sql_path = format!("{}_up.sql", target_version.to_string());
        
        if let Some(file) = schemas.get_file(&sql_path) {
            let sql = match file.contents_utf8() {
                Some(sql) => sql,
                None => panic!("Failed to read migrations file")
            };
            if let Err(e) = tx.execute_batch(sql) {
                println!("Failed to execute migration script: {e:?}");
            } 
        }

        if let Some(up) = migration.up {
            up(&tx);
        }
        let mut metadata = metadata.clone();
        metadata.version = target_version;
        tx.set::<DbMetadata>("metadata", &metadata);
        tx.commit();
        info!("Finished Deploying Migration");
    }
}


#[derive(Debug)]
pub enum MigrationError {
    BinaryVersionHigherThanDbSchema,
    DbProblem
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbMetadata {
    pub version: Version
}

impl Default for DbMetadata {
    fn default() -> Self {
        Self { 
            version: Version::new(0, 0, 0)
        }
    }
}

pub fn version() -> semver::Version {
    let version = env!("CARGO_PKG_VERSION");
    semver::Version::parse(version).unwrap()
}

impl From<rusqlite::Error> for MigrationError {
    fn from(value: rusqlite::Error) -> Self {
        // TBD
        MigrationError::DbProblem
    }
}


impl FromSql for DbMetadata {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?)
    }
}

impl ToSql for DbMetadata {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(serde_json::to_string(self).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?.into())
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
