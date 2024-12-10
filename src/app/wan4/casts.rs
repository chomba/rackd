use rusqlite::{types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, Error, Result, Row, ToSql};
use crate::app::{data::framework::traits::MapRow, shared::domain::Metadata, wan4::models::*};

impl MapRow for Wan4 {
    fn table() -> &'static str { "wan4" }
    fn select() -> &'static str { 
        "SELECT seq, version, id, name, prefix, iprefix FROM wan4" 
    }
    fn map(row: &Row) -> Result<Self> {
        Ok(Wan4 {
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

impl ToSql for Wan4Prefix {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
        Ok(json.into())
    }
}

impl FromSql for Wan4Prefix  {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let prefix = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(prefix)
    }
}

// impl ToSql for WanStatus {
//     fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
//         Ok((*self as u32).into())
//     }
// }

// impl FromSql for WanStatus {
//     fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
//         let value = u8::try_from(value.as_i64()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
//         Ok(WanStatus::try_from(value).map_err(|e| FromSqlError::Other(Box::new(e)))?)
//     }
// }