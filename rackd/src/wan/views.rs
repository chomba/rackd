use log::error;
use rusqlite::{named_params, params, Row};
use serde::{Deserialize, Serialize};
use crate::{db::query::traits::DbView, net::{NetName, VlanId}, org::model::Asn, rack::RackId, trunk::{model::{TrunkEvent, TrunkId, TrunkName}, views::TrunkIdView}, util::models::{Event, EventData}};
use rusqlite::Transaction;
use super::model::{entity::WanEvent, values::{WanId, WanMode}};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WanView {
    pub id: WanId,
    pub rack: RackIdView,
    pub trunk: TrunkIdView,
    pub vlan: VlanId,
    pub name: NetName,
    pub mode: WanMode,
    pub telemetry: Option<WanTelemetry>
    // pub prefixes: Vec<DelegatedPrefix>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WanTelemetry {
    pub status: WanStatus
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RackIdView {
    pub id: RackId,
    pub asn: Asn
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WanStatus {
    Up, Down
}

impl Default for WanStatus {
    fn default() -> Self {
        Self::Down
    }
}

impl DbView for WanView {
    fn name() -> &'static str {
        "wan_view"
    }

    fn update(tx: &Transaction, e: &Event) {
        match &e.data {
            EventData::Wan(data) => match data {
                WanEvent::Created { id, rack, trunk, vlan, name, mode } => {
                    let sql = format!("INSERT INTO {} (id, rack_id, rack_asn, trunk_id, trunk_name, vlan, name, mode) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", Self::name());
                    tx.execute(&sql, params![e.stream_id, rack.id, rack.asn, trunk.id, trunk.name, vlan, name, mode]).map_err(|e| error!("{e}")).unwrap();
                },
                WanEvent::Renamed { to, .. } => {
                    let sql = format!("UPDATE {} SET name = :name WHERE id = :id", Self::name());
                    tx.execute(&sql, named_params! { ":id": WanId(e.stream_id), ":name": to }).map_err(|e| error!("{e}")).unwrap(); 
                },
                WanEvent::MacAddrSet { to, .. } => {
                    let sql = format!("UPDATE {} SET mac = :mac WHERE id = :id", Self::name());
                    tx.execute(&sql, named_params! { ":id": WanId(e.stream_id), ":mac": to }).map_err(|e| error!("{e}")).unwrap();
                },
                // TBD
                _ => {}
            },
            EventData::Trunk(data) => match data {
                TrunkEvent::Renamed { to, .. } => {
                    let sql = format!("UPDATE {} SET trunk_name := trunk_name WHERE trunk_id = :trunk_id", Self::name());
                    tx.execute(&sql, named_params! { ":trunk_id": TrunkId(e.stream_id), ":trunk_name": to  }).map_err(|e| error!("{e}")).unwrap();
                },
                _ => {}
            }
        }
    }

    fn select_fields() -> &'static str {
        "id, rack_id, rack_asn, trunk_id, trunk_name, vlan, name, mode"
    }

    fn try_from(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            rack: RackIdView { 
                id: row.get(1)?,
                asn: row.get(2)?
            },
            trunk: TrunkIdView {
                id: TrunkId(row.get(3)?),
                name: row.get(4)?
            },
            vlan: row.get(5)?,
            name: row.get(6)?,
            mode: row.get(7)?,
            ..Default::default()
        })
    }
}