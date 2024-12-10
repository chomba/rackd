use rusqlite::{types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, Error, Result, Row, ToSql};
use crate::app::{data::framework::traits::MapRow, shared::domain::Metadata, wan6::models::{Wan6, Wan6Prefix}};

impl MapRow for Wan6 {
    fn table() -> &'static str { 
        "wan6" 
    }

    fn select() -> &'static str { 
        "SELECT seq, version, id, name, prefix, iprefix FROM wan6" 
    }

    fn map(row: &Row) -> Result<Self> {
        Ok(Wan6 {
            meta: Metadata {
                seq: row.get(0)?,
                version: row.get(1)?,
                ..Default::default()
            },
            id: row.get(2)?,
            name: row.get(3)?,
            prefix: row.get(4)?,
            iprefix: row.get(5)?,
        })
    }
}

impl ToSql for Wan6Prefix {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
        Ok(json.into())
    }
}

impl FromSql for Wan6Prefix  {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let prefix = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(prefix)
    }
}

