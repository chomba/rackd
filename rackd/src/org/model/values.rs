use std::fmt::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct OrgId(pub Uuid);

impl OrgId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrgName(String);

impl Display for OrgName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrgDomain {
    pub name: String,
    pub tld: String
}

impl Display for OrgDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.name, self.tld)
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Default)]
pub struct Asn(u16);

pub mod casts {
    use super::Asn;

    #[derive(Debug)]
    pub enum AsnError {
        InvalidRange
    }

    impl TryFrom<u32> for Asn {
        type Error = AsnError;
        fn try_from(value: u32) -> Result<Self, Self::Error> {
            // TBD
            Ok(Asn(value as u16))
        }
    }
}

pub mod sqlite {
    use rusqlite::{Result, types::{FromSql, FromSqlError, FromSqlResult, ValueRef, ToSqlOutput}, ToSql};
    use super::*;

    impl FromSql for Asn {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            let asn = u16::try_from(value.as_i64()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
            Ok(Asn(asn))
        }
    }

    impl ToSql for Asn {
        fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
            Ok(self.0.into())
        }
    }
}
