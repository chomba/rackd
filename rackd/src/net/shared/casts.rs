use rusqlite::{types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, Error, Result, ToSql};
use super::models::*;

impl ToSql for Prefix {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
        Ok(json.into())
    }
}

impl FromSql for Prefix {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let prefix = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(prefix)
    }
}

impl ToSql for Ipv4Prefix {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
        Ok(json.into())
    }
}

impl FromSql for Ipv4Prefix {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let prefix = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(prefix)
    }
}

impl ToSql for Ipv6Prefix {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
        Ok(json.into())
    }
}

impl FromSql for Ipv6Prefix {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let prefix = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(prefix)
    }
}
// impl ToSql for Ipv6PrefixLength {
//     fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
//         Ok(self.value().into())
//     }
// }

// impl FromSql for Ipv6PrefixLength {
//     fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
//         let prefix_len = Ipv6PrefixLength::try_from(value.as_i64()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
//         Ok(prefix_len)
//     }
// }