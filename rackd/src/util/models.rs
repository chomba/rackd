use std::fmt::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::net::wan::models::WanEvent;

#[derive(Debug, Clone)]
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
    Wan(WanEvent)
}
// #[derive(Debug, Clone)]
// pub struct Metadata<T> where T: Entity {
//     // Date: CreatedOn
//     // Date: LastModified
//     pub seq: u32,       // Sequence Number
//     pub version: u32,   // Current Entity Version
//     pub events: Option<Vec<T::Event>>
// }

// impl<T> Default for Metadata<T> where T: Entity {
//     fn default() -> Self {
//         Self {
//             seq: u32::default(),
//             version: u32::default(),
//             events: None
//         }
//     }
// }

// pub trait EventHandler<E> {
//     fn handle(&mut self, event: &E) -> ();
// }

// pub trait Entity where Self: Sized {
//     type Event; // EtityEvent becomes AppEvent (EventData) which later becomes Event
//     fn id(&self) -> Id;
//     fn apply(&mut self, e: &Self::Event);
//     fn metadata(&mut self) -> &mut Metadata<Self>;
    
//     fn events(&mut self) -> Vec<Self::Event> {
//         match self.metadata().events.take() {
//             Some(events) => events,
//             None => vec![]
//         }
//     }

//     fn process(&mut self, e: Self::Event) {
//         self.apply(&e);
//         let metadata = self.metadata();
//         match metadata.events {
//             Some(ref mut events) => events.push(e),
//             None => metadata.events = Some(vec![e])
//         }
//         metadata.version += 1;
//     }
// }

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

mod casts {
    use std::str::FromStr;
    use uuid::Uuid;
    use super::Id;

    impl From<Uuid> for Id {
        fn from(id: Uuid) -> Self {
            Self(id)
        }
    }
    
    impl From<Id> for Uuid {
        fn from(id: Id) -> Self {
            id.0
        }
    }

    pub trait FromManyStr where Self: Sized {
        type Error;
        fn from_many_str(values: &Vec<String>) -> Result<Vec<Self>, Self::Error>; 
    }
    
    impl FromManyStr for Id {
        type Error = uuid::Error;
        fn from_many_str(values: &Vec<String>) -> Result<Vec<Self>, Self::Error> {
            let mut ids = Vec::<Id>::new();
            for s in values {
                match Uuid::from_str(s) {
                    Ok(id) => ids.push(Id(id)),
                    Err(e) => return Err(e)
                }
            }
            Ok(ids)
        }
    }

    mod sqlite {
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
                Ok(Id(id))
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
}

