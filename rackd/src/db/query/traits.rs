use rusqlite::Transaction;
use crate::util::db::DbQuery;

pub trait QueryRunner {
    fn run<Q: DbQuery>(&self, query: Q) -> Result<Q::Ok, Q::Err>;
}

impl<'a> QueryRunner for Transaction<'a> {
    fn run<Q: DbQuery>(&self, query: Q) -> Result<Q::Ok, Q::Err> {
        query.run(self)
    }
}