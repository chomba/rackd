use rusqlite::Row;
use crate::app::{error::AppError, shared::domain::Event};
use super::db::DbSession;

pub trait MapRow where Self: Sized {
    fn table() -> &'static str;
    fn select() -> &'static str;
    fn map(row: &Row) -> Result<Self, rusqlite::Error>;
}

pub trait DbQuery {
    type Ok;
    type Err;
    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err>; 
}

pub trait DbCommand {
    type Ok;
    type Err;
    fn exec(self, db: &DbSession) -> Result<Self::Ok, Self::Err>;
}

pub type DbAction = fn(&DbSession) -> Result<(), AppError>;
pub type DbHandler = fn(&DbSession, &Event) -> Result<(), AppError>;