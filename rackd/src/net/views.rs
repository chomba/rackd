use crate::wan::model::entity::WanEvent;
use crate::{db::query::traits::DbView, trunk::views::TrunkIdView, util::models::EventData};
use crate::util::models::{Event, Id};
use rusqlite::{named_params, params, Error, Row, Transaction};
use serde::{Deserialize, Serialize};
use super::model::values::{NetName, VlanId};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct NetworkId(pub Uuid);

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkView {
    pub id: Id,
    pub name: NetName,
    pub trunk: TrunkIdView,
    pub vlan: VlanId,
    pub kind: NetworkKind
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkKind {
    Lan, Wan
}

impl DbView for NetworkView {
    fn name() -> &'static str { "network_view" }
    fn update(tx: &Transaction, e: &Event) {
        match &e.data {
            EventData::Wan(inner) => match inner {
                WanEvent::Created { id, trunk, vlan, name, .. } => {
                    let sql = format!("INSERT INTO {} (id, trunk_id, trunk_name, vlan, name, kind) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", Self::name());
                    tx.execute(&sql, params![id.0, trunk.id, trunk.name, vlan, name, NetworkKind::Wan]).unwrap();                    
                },
                WanEvent::Renamed { to, .. } => {
                    let sql = format!("UPDATE {} set name = :name WHERE id = :id", Self::name());
                    tx.execute(&sql, named_params! { ":id": e.stream_id, ":name": to }).unwrap();
                },
                _ => { }
            },
            // TBD
            _ => {}
        }
    }

    fn select_fields() -> &'static str {
        "id, trunk_id, trunk_name, vlan, name, kind"
    }

    fn try_from(row: &Row) -> Result<Self, Error> {
        Ok(NetworkView {
            id: row.get(0)?,
            trunk: TrunkIdView {
                id: row.get(1)?,
                name: row.get(2)?
            },
            vlan: row.get(3)?,
            name: row.get(4)?,
            kind: row.get(5)?
        })
    }
}

pub mod sql {
    use rusqlite::{Result, types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, ToSql};
    use super::*;

    // impl ToSql for NetworkId {
    //     fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
    //         Ok(self.0.to_string().into())
    //     }
    // }
    
    // impl FromSql for NetworkId {
    //     fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
    //         let id = Uuid::try_from(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
    //         Ok(Self(id))
    //     }
    // }

    impl ToSql for NetworkKind {
        fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
            let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
            Ok(json.into())
        }
    }
    
    impl FromSql for NetworkKind {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            let kind = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
            Ok(kind)
        }
    }
}
