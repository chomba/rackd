pub mod util;
pub mod cmd;
pub mod query;
// mod raft;
use rusqlite::{Connection, Transaction};

pub trait Tx {
    fn tx(&mut self) -> Result<Transaction, rusqlite::Error>;
}

impl Tx for Connection {
    fn tx(&mut self) -> Result<Transaction, rusqlite::Error> {
        let mut tx = self.transaction()?;
        tx.set_drop_behavior(rusqlite::DropBehavior::Commit);
        Ok(tx)
    }
}