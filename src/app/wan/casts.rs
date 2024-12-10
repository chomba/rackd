use rusqlite::{types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef}, Result, ToSql};
use super::models::WanName;

impl ToSql for WanName {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(self.0.clone().into())
    }
}

impl FromSql for WanName {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(WanName(String::from(value.as_str()?)))
    }
}