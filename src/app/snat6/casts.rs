use rusqlite::{Error, Row, ToSql};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::Result;
use crate::app::data::framework::traits::MapRow;
use crate::app::shared::domain::Metadata;
use crate::app::snat6::models::*;

impl MapRow for SNat6 {
    fn table() -> &'static str { 
        "snat" 
    }
    
    fn select() -> &'static str {
        "SELECT seq, version, id, prefix, iprefix, targets, mode, status FROM snat"
    }

    fn map(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(SNat6 {
            meta: Metadata {
                seq: row.get(0)?,
                version: row.get(1)?,
                ..Default::default()
            },
            id: row.get(2)?,
            prefix: row.get(3)?,
            iprefix: row.get(4)?,
            targets: row.get(5)?,
            mode: row.get(6)?,
            status: row.get(7)?
        })
    }
}

impl MapRow for SNat6Target {
    fn table() -> &'static str {
        "snat_target"
    }

    fn select() -> &'static str {
        "SELECT id, snat_id, prefix, iprefix FROM snat_target"
    }

    fn map(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(SNat6Target {
            id: row.get(0)?,
            snat_id: row.get(1)?,
            prefix: row.get(2)?,
            iprefix: row.get(3)?,
        })
    }
}

impl ToSql for SNat6Targets {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
        Ok(json.into())
    }
}

impl FromSql for SNat6Targets {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let targets: SNat6Targets = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(targets)
    }
}

impl ToSql for SNat6Prefix {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
        Ok(json.into())
    }
}

impl FromSql for SNat6Prefix {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let source: SNat6Prefix = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(source)
    }
}

impl ToSql for SNat6Mode {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
        Ok(json.into())
    }
}

impl FromSql for SNat6Mode {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let mode: SNat6Mode = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(mode)
    }
}

impl ToSql for SNat6Status {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
        Ok(json.into())
    }
}

impl FromSql for SNat6Status {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let status = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(status)
    }
}

impl ToSql for SNat6TargetPrefix {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
        Ok(json.into())
    }
}

impl FromSql for SNat6TargetPrefix {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let target_prefix: SNat6TargetPrefix = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(target_prefix)
    }
}