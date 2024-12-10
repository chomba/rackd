use std::str::FromStr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id(pub uuid::Uuid);

impl Id {
    pub fn new() -> Self {
        Id(Uuid::new_v4())
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