use std::{collections::BTreeMap, ops::Bound::{Excluded, Included}};
use semver::Version;
use crate::app::{data::registry::SCHEMA_MIGRATIONS, shared::version};
use super::db::{Db, DbMetadata, DbSession};

pub struct Migration {
    version: Version,
    up: Option<MigrationFn>,
    down: Option<MigrationFn>
}

pub type MigrationFn = fn(&DbSession) -> Result<(), MigrationError>;

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

pub struct Migrations(BTreeMap<Version, Migration>);

impl Migrations {
    pub fn new<M>(migrations: M) -> Self where M: IntoIterator<Item = Migration> {
        Self(BTreeMap::from_iter(migrations.into_iter().map(|m| (m.version.clone(), m))))
    }

    pub fn run(&self, mut db: Db) -> Result<Db, MigrationError> {
        let metadata = db.begin()?.get::<DbMetadata>("metadata").unwrap_or_default();
        let db_version = metadata.version.clone();
        let bin_version = version();
        if bin_version < db_version {
            Err(MigrationError::BinaryVersionHigherThanDbSchema)?;
        }
        
        for (version, migration) in self.0.range((Excluded(db_version), Included(bin_version))) {
            let db = db.begin()?;
            Self::deploy(db, migration, &metadata)?;
            // println!("FINISHED RUNNING MIGRATION TO {}", version);
            // Log: Finished migrating to version
        }
        Ok(db)
    }

    fn deploy(db: DbSession, migration: &Migration, metadata: &DbMetadata) -> Result<(), MigrationError> {
        let target_version = migration.version.clone();
        let sql_path = format!("{}_up.sql", target_version.to_string());
        
        
        if let Some(file) = SCHEMA_MIGRATIONS.get_file(&sql_path) {
            let sql = match file.contents_utf8() {
                Some(sql) => sql,
                None => panic!("Failed to read migrations file")
            };
            if let Err(e) = db.tx().execute_batch(sql) {
                println!("Failed to execute migration script: {e:?}");
            } 
        }

        if let Some(up) = migration.up {
            up(&db)?;
        }
        let mut metadata = metadata.clone();
        metadata.version = target_version;
        db.set::<DbMetadata>("metadata", &metadata)?;
        db.commit()?;
        // println!("DEPLOY DOES COMMIT THE CHANGES");
        Ok(())
    }
}


#[derive(Debug)]
pub enum MigrationError {
    BinaryVersionHigherThanDbSchema,
    DbProblem
}

impl From<rusqlite::Error> for MigrationError {
    fn from(value: rusqlite::Error) -> Self {
        // TBD
        MigrationError::DbProblem
    }
}