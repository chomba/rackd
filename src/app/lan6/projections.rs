use super::models::*;
use crate::app::shared::domain::{Event, InnerEvent};
use crate::app::{data::DbSession, error::AppError};
use crate::util::domain::Id;
use crate::util::net::types::Ipv6PrefixExt;
use rusqlite::{named_params, params};

pub fn project_lan6(db: &DbSession, event: &Event) -> Result<(), AppError> {
    return match &event.data {
        InnerEvent::Lan6(e) => {
            match e {
                Lan6Event::Created { id, name, prefix, iprefix } => {
                    db.tx().execute("INSERT INTO lan6 (version, id, name, prefix, iprefix) VALUES (?1, ?2, ?3, ?4, ?5)", // Add ON CONFLICT?
                        params![event.version, id, name, prefix, iprefix])?;
                    Ok(())
                },
                Lan6Event::Renamed { name } => {
                    let mut stmt = db.tx().prepare("UPDATE lan6 SET name = :name, version = :version WHERE id = :id")?;
                    stmt.execute(named_params! {
                        ":name": name,
                        ":version": event.version,
                        ":id": event.stream_id
                    })?;
                    Ok(())
                },
                Lan6Event::PrefixUpdated { prefix, iprefix } => {
                    let mut stmt = db.tx().prepare("UPDATE lan6 SET prefix = :prefix, iprefix = :iprefix, version = :version WHERE id = :id")?;
                    stmt.execute(named_params! {
                        ":prefix": prefix,
                        ":iprefix": iprefix,
                        ":version": event.version,
                        ":id": event.stream_id
                    })?;
                    Ok(())
                },
                Lan6Event::PrefixRecomputed { prefix, .. } => {
                    let mut stmt = db.tx().prepare("UPDATE lan6 SET iprefix = :iprefix, version = :version WHERE id = :id")?;
                    stmt.execute(named_params! {
                        ":iprefix": prefix,
                        ":version": event.version,
                        ":id": event.stream_id
                    })?;
                    Ok(())
                },
                Lan6Event::Deleted => {
                    let mut stmt = db.tx().prepare("UPDATE lan6 SET deleted = :deleted, version = :version WHERE id = :id")?;
                    stmt.execute(named_params! {
                        ":deleted": true,
                        ":version": event.version,
                        ":id": event.stream_id
                    })?;
                    Ok(())
                }
            }
        },
        _ => Ok(())
    };
}

pub fn project_lan6_descendants(db: &DbSession, event: &Event) -> Result<(), AppError> {
    if let InnerEvent::Lan6(e) = &event.data {
        match e {
            Lan6Event::Created { id: child_id, prefix, .. } => {
                if let Lan6Prefix::Extension(Ipv6PrefixExt { id: parent_id, .. }) = prefix {
                    let tx = db.tx();
                    let mut stmt = tx.prepare("SELECT DISTINCT id FROM lan6_descendant WHERE descendant_id = :descendant_id")?;
                    let rows = stmt.query_map(named_params! { ":descendant_id": parent_id }, |row| {
                        Ok(row.get::<_, Id>(0)?)
                    })?;
                    let mut stmt = tx.prepare("INSERT INTO lan6_descendant (id, descendant_id) VALUES (?1, ?2)")?;
                    for row in rows {
                        let id = row?;
                        stmt.execute((id, child_id))?;
                    }
                    stmt.execute((parent_id, child_id))?;                       
                }
            },
            Lan6Event::Deleted => {

            }
            _ => { }
        }
    }
    Ok(())
}

// pub fn project_lan(db: &DbSession, event: &Event) -> Result<(), AppError> {
//     return match &event.data {
//         InnerEvent::Lan(e) => {
//             match e {
//                 LanEvent::Created { id, name, prefix, ipv6_prefix } => {
//                     db.tx().execute("INSERT INTO lan (version, id, name, prefix, ipv6_prefix, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", // Add ON CONFLICT?
//                         params![event.version, id, name, prefix, ipv6_prefix, LanStatus::Nominal])?;
//                     Ok(())
//                 },
//                 LanEvent::Renamed { name } => {
//                     let mut stmt = db.tx().prepare("UPDATE lan SET name = :name, version = :version WHERE id = :id")?;
//                     stmt.execute(named_params! {
//                         ":name": name,
//                         ":version": event.version,
//                         ":id": event.stream_id
//                     })?;
//                     Ok(())
//                 },
//                 LanEvent::PrefixChanged { prefix, ipv6_prefix } => {
//                     let mut stmt = db.tx().prepare("UPDATE lan SET prefix = :prefix, ipv6_prefix = :ipv6_prefix, version = :version WHERE id = :id")?;
//                     stmt.execute(named_params! {
//                         ":prefix": prefix,
//                         ":ipv6_prefix": ipv6_prefix,
//                         ":version": event.version,
//                         ":id": event.stream_id
//                     })?;
//                     // Update all other prefixes that depend on the changed prefix
//                     Ok(())
//                 },
//                 LanEvent::Deleted => {
//                     let mut stmt = db.tx().prepare("UPDATE lan SET status = :status")?;
//                     stmt.execute(&[(":status", &(LanStatus::Deleted as u32))])?;
//                     Ok(())
//                 }
//             }
//         },
//         _ => Ok(())
//     };
// }

// pub fn project_lan_descendants(db: &DbSession, event: &Event) -> Result<(), AppError> {
//     if let InnerEvent::Lan(e) = &event.data {
//         match e {
//             LanEvent::Created { id: child_id, prefix, .. } => {
//                 if let LanPrefix::Extension { id: parent_id, .. } = prefix {
//                     let tx = db.tx();
//                     let mut stmt = tx.prepare("SELECT DISTINCT id FROM lan_descendant WHERE descendant_id = :descendant_id")?;
//                     let rows = stmt.query_map(named_params! { ":descendant_id": parent_id }, |row| {
//                         Ok(row.get::<_, Id>(0)?)
//                     })?;
//                     let mut stmt = tx.prepare("INSERT INTO lan_descendant (id, descendant_id) VALUES (?1, ?2)")?;
//                     for row in rows {
//                         let id = row?;
//                         stmt.execute((id, child_id))?;
//                     }
//                     stmt.execute((parent_id, child_id))?;                       
//                 }
//             },
//             LanEvent::Deleted => {

//             }
//             _ => { }
//         }
//     }
//     Ok(())
// }