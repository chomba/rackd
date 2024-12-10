use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Error, ToSql};
use rusqlite::Result;
use uuid::Uuid;
use crate::util::domain::Id;
use super::domain::*;

impl ToSql for Id {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(self.0.to_string().into())
    }
}

impl FromSql for Id {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let id = Uuid::try_from(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(Id(id))
    }
}

impl ToSql for InnerEvent {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
        Ok(json.into())
    }
}

impl FromSql for InnerEvent {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let e = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(e)
    }
}