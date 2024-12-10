use std::{fmt::Display, str::FromStr};
use serde::{Deserialize, Serialize};
use crate::{app::shared::domain::Entity, util::net::types::IpPrefix};

pub trait Lan: Entity {
    type Prefix: IpPrefix;
    fn prefix(&self) -> Self::Prefix;
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct LanName(String);

impl LanName { 
    pub fn new(s: &str) -> Option<LanName>  {
        LanName::from_str(s).ok()
    }
}

impl Display for LanName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<LanName> for String {
    fn from(name: LanName) -> Self {
        name.0
    }
}


#[derive(Debug)]
pub struct LanNameParseError;

impl FromStr for LanName {
    type Err = LanNameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s.to_lowercase());
        let length = s.len();
        if length <= 20 {
            return Ok(LanName(s));
        }
        // let mut rules = Rules::new();
        // rules += StringRule::BoundedLength { min: 0, max: 10 };
        // rules.eval(s, |s| IspName(s), |value, violations| Error::IspName(IspNameError::Invalid { value, violations }))
        Err(LanNameParseError)
    }
}