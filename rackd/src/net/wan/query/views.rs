use rusqlite::Row;
use crate::{net::{shared::models::*, wan::models::*}, org::rack::models::RackId, util::db::View};
use crate::util::models::Event;
use rusqlite::Transaction;
// use crate::data::framework::traits::ProjectionMode;

/// Wan Configuration View
#[derive(Debug, Default)]
pub struct Wan {
    pub id: WanId,
    pub rack: RackId,
    pub trunk: TrunkId,
    pub vlan: VlanId,
    pub conn: WanConnection,
    pub name: NetName,
    // pub layer2: WanL2Conf,
}

impl View for Wan {
    fn name() -> &'static str { "view.wan" }
    fn update(tx: &Transaction, e: &Event) -> Result<(), rusqlite::Error> {
        Ok(())
    }

    fn sql_create() -> &'static str {
        ""
    }

    fn sql_select() -> &'static str {
        "SELECT id, rack, trunk, vlan, conn, name FROM view.name"
    }
    
    fn try_from(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(Wan {
            // meta: Metadata {
            //     seq: row.get(0)?,
            //     version: row.get(1)?,
            //     ..Default::default()
            // },
            id: row.get(0)?,
            rack: row.get(1)?,
            trunk: row.get(2)?,
            vlan: row.get(3)?,
            conn: row.get(4)?,
            name: row.get(5)?
        })
    }
}


// impl View for WanView {
//     fn apply(&mut self, e: &Event) {
//         match &e.data {
//             EventData::Wan(WanEvent::Created(e)) => {
//                 self.id = e.id;
//                 self.rack = e.rack;
//                 self.trunk = e.trunk;
//                 self.vlan = e.vlan;
//                 self.conn = e.conn.clone();
//                 self.name = e.name.clone();
//             },
//             _ => { }
//         }
//     }
// }


