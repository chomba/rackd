use log::error;
use rusqlite::{named_params, params, Row};
use crate::{db::query::traits::View, net::{shared::models::*, wan::models::*}, org::rack::models::RackId, util::models::EventData};
use crate::util::models::Event;
use rusqlite::Transaction;

/// Wan Configuration View
#[derive(Debug, Default)]
pub struct Wan {
    pub version: u32,
    pub id: WanId,
    pub rack: RackId,
    pub trunk: TrunkId,
    pub vlan: VlanId,
    pub conn: WanConnection,
    pub name: NetName,
    pub mac: WanMac
}

impl View for Wan {
    fn name() -> &'static str { "wan" }
    fn update(tx: &Transaction, e: &Event) {
        return match &e.data {
            EventData::Wan(inner) => match inner {
                WanEvent::Created(data) => {
                    tx.execute("INSERT INTO wan (version, id, rack, trunk, vlan, conn, name, mac) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                        params![e.version, e.stream_id, data.rack, data.trunk, data.vlan, data.conn, data.name, WanMac::default()]).map_err(|e| error!("{e}")).unwrap();
                },
                WanEvent::Renamed(data) => {
                    tx.execute("UPDATE wan SET name = :name, version = :version WHERE id = :id",
                        named_params! { ":id": e.stream_id, ":name": data.name, ":version": e.version }).map_err(|e| error!("{e}")).unwrap();                    
                },
                WanEvent::MacSetToAuto(_) => {
                    tx.execute("UPDATE wan SET mac = :mac, version = :version WHERE id = :id",
                        named_params! { ":id": e.stream_id, ":mac": WanMac::Auto, ":version": e.version }).map_err(|e| error!("{e}")).unwrap();
                },
                WanEvent::MacSetToSpoof(data) => {
                    tx.execute("UPDATE wan SET mac = :mac, version = :version WHERE id = :id", 
                        named_params! { ":id": e.stream_id, ":mac": WanMac::Spoof(data.mac), ":version": e.version }).map_err(|e| error!("{e}")).unwrap();
                },
                WanEvent::Ipv6SetToRA(_) => {
                    let mut ip = get_ipoe_conf(tx, e.stream_id);
                    ip.ipv6 = Ipv6Conf::FromRA;
                    set_conn(tx, e.stream_id, WanConnection::IPoE(ip), e.version);
                },
                WanEvent::Ipv6SetToStatic(data) => {
                    let mut ip = get_ipoe_conf(tx, e.stream_id);
                    ip.ipv6 = Ipv6Conf::Static(data.host);
                    set_conn(tx, e.stream_id, WanConnection::IPoE(ip), e.version);
                },
                WanEvent::Ipv4SetToDHCP(_) => {
                    let mut ip = get_ipoe_conf(tx, e.stream_id);
                    ip.ipv4 = Ipv4Conf::DHCP;
                    set_conn(tx, e.stream_id, WanConnection::IPoE(ip), e.version);
                },
                WanEvent::Ipv4SetToStatic(data) => {
                    let mut ip = get_ipoe_conf(tx, e.stream_id);
                    ip.ipv4 = Ipv4Conf::Static(data.host);
                    set_conn(tx, e.stream_id, WanConnection::IPoE(ip), e.version);
                }
            }
        };

        fn get_ipoe_conf(tx: &Transaction, id: WanId) -> IPoEConf {
            let conn = tx.query_row("SELECT conn FROM wan WHERE id = :id",  named_params! { ":id": id },
                |row| Ok(row.get::<_, WanConnection>(0)?)).map_err(|e| error!("{e}")).unwrap();
            match conn {
                WanConnection::IPoE(ip) => ip,
                WanConnection::PPPoE(_) => { 
                    let msg = "[BUG] Can't project because conn is set to PPPoE";
                    error!("{msg}");
                    panic!("{msg}");
                }
            }
        }

        fn set_conn(tx: &Transaction, id: WanId, conn: WanConnection, version: u32) {
            tx.execute("UPDATE wan SET conn = :conn, version = :version WHERE id = :id", 
            named_params! { ":id": id, ":conn": conn, ":version": version}).map_err(|e| error!("{e}")).unwrap();
        }

            // WanEvent::Isp(e) => {
            //     match e {
            //         IspEvent::Created { id, rank, name, link, link_name, link_status, prefix } => {
            //             db.tx().execute("INSERT INTO isp (version, id, rank, name, link, link_name, link_status, prefix) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            //                 params![event.version, id, rank, name, link, link_name, link_status, prefix])?;
            //             Ok(())
            //         },
            //         IspEvent::Renamed { name } => {
            //             let mut stmt = db.tx().prepare("UPDATE isp SET name = :name, version = :version WHERE id = :id")?;
            //             stmt.execute(named_params! {
            //                 ":name": name,
            //                 ":version": event.version,
            //                 ":id": event.stream_id
            //             })?;
            //             Ok(())
            //         },
            //         IspEvent::LinkWentUp { up } => {
            //             let mut stmt = db.tx().prepare("UPDATE isp SET link_status = :link_status, version = :version WHERE id = :id")?;
            //             stmt.execute(named_params! {
            //                 ":link_status": LinkStatus::Up(*up),
            //                 ":version": event.version,
            //                 ":id": event.stream_id
            //             })?;
            //             Ok(())
            //         },
            //         IspEvent::LinkWentDown { down } => {
            //             let mut stmt = db.tx().prepare("UPDATE isp SET link_status = :link_status, version = :version WHERE id = :id")?;
            //             stmt.execute(named_params! {
            //                 ":link_status": LinkStatus::Down(*down),
            //                 ":version": event.version,
            //                 ":id": event.stream_id
            //             })?;
            //             Ok(())
            //         },
            //         IspEvent::Deleted => {
            //             let mut stmt = db.tx().prepare("UPDATE isp SET deleted = :deleted, version = :version WHERE id = :id")?;
            //             stmt.execute(named_params! {
            //                 ":deleted": true,
            //                 ":version": event.version,
            //                 ":id": event.stream_id
            //             })?;
            //             Ok(())
            //         }
            //     }
            // },
            // _ => Ok(())
    }

    fn sql_select() -> &'static str {
        "SELECT version, id, rack, trunk, vlan, conn, name, mac FROM wan"
    }

    fn try_from(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(Wan {
            version: row.get(0)?,
            id: row.get(1)?,
            rack: row.get(2)?,
            trunk: row.get(3)?,
            vlan: row.get(4)?,
            conn: row.get(5)?,
            name: row.get(6)?,
            mac: row.get(7)?
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


