use std::str::FromStr;
use rusqlite::{types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, Error, Result, Row, ToSql};
use crate::{app::{data::framework::traits::MapRow, shared::domain::Metadata}, sys::link::models::*};
use super::models::*;

impl MapRow for Isp {
    fn table() -> &'static str {  "isp" }

    fn select() -> &'static str { 
        "SELECT seq, version, id, rank, name, link, link_name, link_status, prefix, tracker FROM isp" 
    }

    fn map(row: &Row) -> Result<Self, Error> {
        Ok(Isp {
            meta: Metadata {
                seq: row.get(0)?,
                version: row.get(1)?,
                ..Default::default()
            },
            id: row.get(2)?,
            rank: row.get(3)?,
            name: row.get(4)?,
            link: row.get(5)?,
            link_name: row.get(6)?,
            link_status: row.get(7)?,
            prefix: row.get(8)?,
            tracker: row.get(9)?
        })
    }
}

impl ToSql for IspName {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for IspName {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(IspName::from_str(value.as_str()?).unwrap())
    }
}

impl ToSql for IspRank {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok((self.0 as i64).into())
    }
}

impl FromSql for IspRank {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let rank = usize::try_from(value.as_i64()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(IspRank(rank))
    }
}

// impl ToSql for IspStatus {
//     fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
//         let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
//         Ok(json.into())
//     }
// }

// impl FromSql for IspStatus {
//     fn column_result(status: ValueRef<'_>) -> FromSqlResult<Self> {
//         let status = serde_json::from_str(status.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
//         Ok(status)
//     }
// }

impl ToSql for LinkStatus {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
        Ok(json.into())
    }
}

impl FromSql for LinkStatus {
    fn column_result(status: ValueRef<'_>) -> FromSqlResult<Self> {
        let status = serde_json::from_str(status.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(status)
    }
}

impl ToSql for IspPrefix {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
        Ok(json.into())
    }
}

impl FromSql for IspPrefix {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let prefix = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(prefix)
    }
}

impl ToSql for LinkName {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(self.0.clone().into())
    }
}

impl FromSql for LinkName {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(LinkName(String::from(value.as_str()?)))
    }
}

impl ToSql for LinkId {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(self.0.into())
    }
}

impl FromSql for LinkId {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let value = u32::try_from(value.as_i64()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(Self(value))
    }
}
