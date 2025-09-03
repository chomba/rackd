use std::fmt::Display;
use utoipa::ToSchema;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::{trunk::model::TrunkEvent, wan::model::entity::WanEvent};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub version: u32,
    pub events: Vec<Event>
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            version: 0,
            events: vec![]
        }
    }
}

pub trait Entity where Self::E: Into<EventData> {
    type E;

    fn id(&self) -> Id; 
    fn metadata(&mut self) -> &mut Metadata;
    fn apply(&mut self, e: &Self::E);
    fn process(&mut self, e: Self::E) {
        self.apply(&e);
        let stream_id = self.id();
        let metadata = self.metadata();
        let e = Event::single(stream_id, e.into(), metadata.version);
        metadata.version += 1;
        metadata.events.push(e);
    }    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Id,
    // pub seq: u32,
    pub stream_id: Id,
    pub data: EventData,
    pub version: u32
}

impl Event {
    pub fn single(stream_id: Id, inner_event: EventData, current_version: u32) -> Self {
        Self {
            id: Id::new(),
            stream_id,
            data: inner_event,
            version: current_version + 1
        }
    }

    pub fn many<T>(stream_id: Id, inner_events: T, mut current_version: u32) -> Vec<Self>
        where T: IntoIterator<Item = EventData> {
        let mut events = Vec::new();
        for e in inner_events {
            let event = Self::single(stream_id, e, current_version);
            current_version = event.version;
            events.push(event);
        }
        events
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EventData {
    Wan(WanEvent),
    Trunk(TrunkEvent)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, ToSchema)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for Id {
    fn default() -> Self {
        Self(Uuid::nil())
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub mod casts {
    use std::str::FromStr;
    use serde_json::Value;
    use thiserror::Error;
    use unicode_segmentation::UnicodeSegmentation;
    use uuid::Uuid;
    use super::Id;  

    impl FromStr for Id {
        type Err = uuid::Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(Uuid::parse_str(s)?))
        }
    }

    #[derive(Debug, Error)]
    pub enum IdError {
        #[error("Value is not a String [{}]", .0)]
        InvalidType(Value),
        #[error("Value is not a UUID [{}]", .0)]
        InvalidFormat(String),
        #[error("No value provided")]
        MissingValue
    }

    impl TryFrom<Value> for Id {
        type Error = IdError;

        fn try_from(value: Value) -> Result<Self, Self::Error> {
            match value {
                Value::String(s) => match Uuid::parse_str(&s) {
                    Err(_) => Err(IdError::InvalidFormat(s)),
                    Ok(id) => Ok(Id(id))
                },
                Value::Null => Err(IdError::MissingValue),
                _ => Err(IdError::InvalidType(value))
            }
        }
    }  

    #[derive(Debug)]
    pub struct InvalidChars { 
        pub value: String, 
        pub chars: Vec<String> 
    }

    impl InvalidChars {
        // TBD: Add Parameter: allow_extended_unicode
        pub fn from<T>(value: T, forbidden_chars: &[char]) -> Option<InvalidChars> where T: Into<String> {
            let value = value.into();
            let mut bad_graphemes = vec![];
            for g in value.graphemes(true) {
                let chars: Vec<char> = g.chars().collect(); 
                match chars.len() {
                    1 => {
                        let c = chars[0];
                        if !char::is_ascii_alphanumeric(&c) || forbidden_chars.contains(&c) {
                            bad_graphemes.push(String::from(c));
                        }
                    },
                    _ => bad_graphemes.push(String::from(g))
                }
            }

            if bad_graphemes.is_empty() {
                None
            } else {
                Some(InvalidChars { value, chars: bad_graphemes })
            }
        }
    }

    // impl TryFrom<Option<Value>> for Id {
    //     type Error = IdError;

    //     fn try_from(value: Option<Value>) -> Result<Self, Self::Error> {
    //         match value {
    //             Some(value) => Self::try_from(value),
    //             None => Err(IdError::MissingKey)
    //         }
    //     }
    // }

    // impl Display for InvalidId {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         write!(f, "{} is not a valid UUID", self.0)
    //     }
    // }

    // impl TryFrom<Value> for Id {
    //     type Error = ;
    // } 


    pub trait FromManyStr where Self: Sized {
        type Error;
        fn from_many_str(values: &Vec<String>) -> Result<Vec<Self>, Self::Error>; 
    }
    
    // impl FromManyStr for Id {
    //     type Error = uuid::Error;
    //     fn from_many_str(values: &Vec<String>) -> Result<Vec<Self>, Self::Error> {
    //         let mut ids = Vec::<Id>::new();
    //         for s in values {
    //             match Uuid::from_str(s) {
    //                 Ok(id) => ids.push(Id(id)),
    //                 Err(e) => return Err(e)
    //             }
    //         }
    //         Ok(ids)
    //     }
    // }
}

pub mod sqlite {
    use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
    use rusqlite::{Error, ToSql};
    use rusqlite::Result;
    use uuid::Uuid;
    use crate::util::models::EventData;
    use super::Id;

    impl ToSql for Id {
        fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
            Ok(self.0.to_string().into())
        }
    }
    
    impl FromSql for Id {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            let id = Uuid::try_from(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
            Ok(Self(id))
        }
    }

    impl ToSql for EventData {
        fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
            let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
            Ok(json.into())
        }
    }

    impl FromSql for EventData {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            let value: Self = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
            Ok(value)
        }
    }
}

