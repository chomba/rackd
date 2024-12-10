use rusqlite::{types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef}, Result, ToSql};
use super::models::LanName;

impl ToSql for LanName {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for LanName {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(LanName::from(String::from(value.as_str()?)))
    }
}
    
pub trait Foo {
    fn foo(&self);
}