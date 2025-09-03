use serde::{Deserialize, Serialize};
use crate::{rack::Rack, util::models::{Entity, Id, Metadata}};
use super::{TrunkId, TrunkName};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Trunk {
    pub meta: Metadata,
    pub id: TrunkId,
    pub name: TrunkName,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TrunkEvent {
    Created { rack: Rack, id: TrunkId, name: TrunkName },
    Renamed { from: TrunkName, to: TrunkName }
}

impl Entity for Trunk {
    type E = TrunkEvent;

    fn id(&self) -> Id {
        self.id.0
    }

    fn metadata(&mut self) -> &mut Metadata {
        &mut self.meta
    }

    fn apply(&mut self, e: &Self::E) {
        match e {
            TrunkEvent::Created { id, name, .. } => {
                self.id = *id;
                self.name = name.clone();
            },
            TrunkEvent::Renamed { to, .. } => {
                self.name = to.clone();
            }
        }
    }
}