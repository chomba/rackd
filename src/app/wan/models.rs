use serde::{Deserialize, Serialize};
use crate::{app::shared::domain::Entity, util::net::types::IpPrefix};

pub trait Wan: Entity {
    type Prefix: IpPrefix;
    fn prefix(&self) -> Self::Prefix;
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WanName(pub String);