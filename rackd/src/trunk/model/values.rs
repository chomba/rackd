use std::fmt::Display;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::util::models::Id;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct TrunkId(pub Id);

impl TrunkId {
    pub fn new() -> Self {
        Self(Id::new())
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone, ToSchema)]
pub struct TrunkName(String);

impl Display for TrunkName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub mod casts {
    use std::str::FromStr;
    use serde_json::Value;
    use thiserror::Error;
    use crate::util::models::casts::IdError;
    use super::*;

    impl From<TrunkId> for Id {
        fn from(value: TrunkId) -> Self {
            value.0
        }
    }

    #[derive(Debug, Error)]
    #[error("{}", .0)]
    pub struct TrunkIdError(#[from]IdError);
    
    impl TryFrom<Value> for TrunkId {
        type Error = TrunkIdError;

        fn try_from(value: Value) -> Result<Self, Self::Error> {
            Ok(Self(Id::try_from(value)?))
        }
    }

    #[derive(Debug, Error)]
    pub enum TrunkNameError {
        #[error("Value is not a string [{}]", .0)]
        InvalidType(Value),
        #[error("Value contains the following invalid characters: {} [{}]", .chars.0.join(","), .value)]
        InvalidChars { value: String, chars: InvalidChars },
        #[error("No value provided")]
        MissingValue,
    }

    #[derive(Debug)]
    pub struct InvalidChars(Vec<String>);

    impl FromStr for TrunkName {
        type Err = InvalidChars;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
                        // TBD
            // if value.contains(char::is_whitespace) {
            //     Err(TrunkNameError::InvalidCharacter)?
            // }
            Ok(Self(String::from(s)))
        }
    }

    impl TryFrom<Value> for TrunkName {
        type Error = TrunkNameError;

        fn try_from(value: Value) -> Result<Self, Self::Error> {
            match value {
                Value::Null => Err(TrunkNameError::MissingValue),
                Value::String(s) => match Self::from_str(&s) {
                    Ok(name) => Ok(name),
                    Err(chars) => Err(TrunkNameError::InvalidChars { value: s, chars })
                },
                _ => Err(TrunkNameError::InvalidType(value))
            }
        }
    }
}

pub mod api {
    use crate::util::api::Error;
    use super::casts::{TrunkIdError, TrunkNameError};

    impl From<TrunkIdError> for Error {
        fn from(error: TrunkIdError) -> Self {
            Error::new("TRUNK_ID_ERROR", error.to_string())
        }
    }

    impl From<TrunkNameError> for Error {
        fn from(error: TrunkNameError) -> Self {
            Error::new("TRUNK_NAME_ERROR", error.to_string())
        }
    }
}

pub mod sqlite {
    use rusqlite::{types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef}, Result, ToSql};
    use super::*;

    impl ToSql for TrunkId {
        fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
            self.0.to_sql()
        }
    }
    
    impl FromSql for TrunkId {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            Ok(Self(Id::column_result(value)?))
        }
    }

    impl FromSql for TrunkName {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            let name = String::from(value.as_str()?);
            Ok(TrunkName(name.into()))
        }
    }

    impl ToSql for TrunkName {
        fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
            Ok(self.0.as_str().into())
        }
    }
}