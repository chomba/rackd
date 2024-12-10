use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Error, Row, ToSql};
use rusqlite::Result;
use crate::app::shared::domain::Metadata;
use crate::app::data::framework::traits::MapRow;
use super::models::*;

impl MapRow for Lan6 {
    fn table() -> &'static str { "lan6" }

    fn select() -> &'static str {
        "SELECT seq, version, id, name, prefix, iprefix FROM lan6"
    }

    fn map(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(Lan6 {
            meta: Metadata {
                seq: row.get(0)?,
                version: row.get(1)?,
                ..Default::default()
            },
            id: row.get(2)?,
            name: row.get(3)?,
            prefix: row.get(4)?,
            iprefix: row.get(5)?
        })
    }
}

impl ToSql for Lan6Prefix {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
        Ok(json.into())
    }
}

impl FromSql for Lan6Prefix {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let prefix: Lan6Prefix = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(prefix)
    }
}

// impl ToSql for LanPrefix {
//     fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
//         let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
//         Ok(json.into())
//     }
// }

// impl FromSql for LanPrefix {
//     fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
//         let prefix: Self = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
//         Ok(prefix)
//     }
// }

// impl ToSql for LanStatus {
//     fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
//         Ok((*self as u32).into())
//     }
// }

// impl FromSql for LanStatus {
//     fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
//         let value = u8::try_from(value.as_i64()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
//         Ok(LanStatus::try_from(value).map_err(|e| FromSqlError::Other(Box::new(e)))?)
//     }
// }