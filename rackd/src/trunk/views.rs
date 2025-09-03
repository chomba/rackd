use log::error;
use rusqlite::{named_params, params, Row, Transaction};
use serde::{Deserialize, Serialize};
use crate::{db::query::traits::DbView, util::models::{Event, EventData}};
use super::model::{TrunkEvent, TrunkId, TrunkName};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrunkView {
    pub id: TrunkId,
    pub name: TrunkName
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TrunkIdView {
    pub id: TrunkId,
    pub name: TrunkName
}

impl DbView for TrunkView {
    fn name() -> &'static str {
        "trunk_view"
    }

    fn update(tx: &Transaction, e: &Event) {
        match &e.data {
            EventData::Trunk(data) => match data {
                TrunkEvent::Created { rack, id, name } => {
                    let sql = format!("INSERT INTO {} (id, name) VALUES (?1, ?2)", Self::name());
                    tx.execute(&sql, params![e.stream_id, name]).map_err(|e| error!("{e}")).unwrap();
                },
                TrunkEvent::Renamed { to, .. } => {
                    let sql = format!("UPDATE {} SET name = :name WHERE id = :id", Self::name());
                    tx.execute(&sql, named_params! { ":id": e.stream_id, ":name": to }).map_err(|e| error!("{e}")).unwrap();
                }
            },
            _ => {}
        }
    }

    fn select_fields() -> &'static str {
        "id, name"
    }

    
    fn try_from(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: TrunkId(row.get(0)?),
            name: row.get(1)?
        })
    }
}