use rusqlite::named_params;
use rusqlite::params;
use rusqlite::Result;
use uuid::Uuid;
use crate::app::data::DbSession;
use crate::app::error::AppError;
use crate::app::shared::domain::Event;
use crate::app::shared::domain::InnerEvent;
use crate::app::snat6::models::*;

pub fn project_snat6(db: &DbSession, event: &Event) -> Result<(), AppError> {
    match &event.data {
        InnerEvent::SNat6(e) => {
            match e {
                SNat6Event::Created { id, prefix, iprefix } => {
                    db.tx().execute("INSERT INTO snat (version, id, prefix, iprefix, targets, mode, status) VALUES (?1, ?2, ?3, ?4)", 
                        params![event.version, id, prefix, iprefix, SNat6Targets::default(), Option::<SNat6Mode>::None, SNat6Status::default()])?;
                    return Ok(());
                },
                SNat6Event::PrefixUpdated { prefix, iprefix } => {
                    let mut stmt = db.tx().prepare("UPDATE snat SET prefix = :prefix, iprefix = :iprefix, version = :version WHERE id = :id")?;
                    stmt.execute(named_params! {
                        ":prefix": prefix, 
                        ":iprefix": iprefix,
                        ":version": event.version, 
                        ":id": event.stream_id
                    })?;
                    return Ok(());
                },
                SNat6Event::TargetAdded { id, prefix, iprefix } |
                SNat6Event::TargetUpdated { id, prefix, iprefix } => {
                    let tx = db.tx();
                    let mut stmt = tx.prepare("SELECT targets FROM snat WHERE id = :id")?;
                    let mut targets: SNat6Targets = stmt.query_row(named_params! { ":id": event.stream_id }, |row| Ok(row.get(0)?))?;
                    targets.insert(*id, SNat6Target {
                        snat_id: event.stream_id, id: *id, prefix: *prefix, iprefix: *iprefix
                    });
                    let mut stmt = tx.prepare("UPDATE snat SET targets = :targets, version = :version WHERE id = :id")?;
                    stmt.execute( named_params! { ":id": event.stream_id, ":targets": targets, ":version": event.version })?;
                    return Ok(());
                },
                SNat6Event::TargetRemoved { id } => {
                    let tx = db.tx();
                    let mut stmt = tx.prepare("SELECT targets FROM snat WHERE id = :id")?;
                    let mut targets: SNat6Targets = stmt.query_row(named_params! { ":id": event.stream_id }, |row| Ok(row.get(0)?))?;
                    targets.remove(&id);
                    let mut stmt = tx.prepare("UPDATE snat SET targets = :targets, version = :version WHERE id = :id")?;
                    stmt.execute( named_params! { ":id": event.stream_id, ":mappings": targets, ":version": event.version })?;
                    return Ok(());
                },
                SNat6Event::ModeUpdated { mode } => {
                    let mut stmt = db.tx().prepare("UPDATE snat SET mode = :mode, version = :version WHERE id = :id")?;
                    stmt.execute(named_params! {
                        ":mode": mode, 
                        ":version": event.version, 
                        ":id": event.stream_id
                    })?;
                    return Ok(());
                },
                SNat6Event::StatusUpdated { status } => {
                    let mut stmt = db.tx().prepare("UPDATE snat SET mode = :mode, version = :version WHERE id = :id")?;
                    stmt.execute(named_params! {
                        ":status": status, 
                        ":version": event.version, 
                        ":id": event.stream_id
                    })?;
                    return Ok(());
                }
            }
        },
        _ => return Ok(())
    }

    fn update_version(db: &DbSession, version: u32, id: Uuid) -> Result<()> {
        let mut stmt = db.tx().prepare("UPDATE snat SET version = :version WHERE id = :id")?;
        stmt.execute(&[(":version", &version.to_string()), (":id", &id.to_string())])?;
        Ok(())
    }
}

pub fn project_snat6_target(db: &DbSession, event: &Event) -> Result<(), AppError> {
    if let InnerEvent::SNat6(e) = &event.data {
        match e {
            SNat6Event::TargetAdded { id, prefix, iprefix } => {
                db.tx().execute("INSERT INTO snat_target (id, prefix, iprefix, snat_id) VALUES (?1, ?2, ?3, ?4)",
                    params![id, prefix, iprefix, event.stream_id])?;
            },
            SNat6Event::TargetUpdated { id, prefix, iprefix } => {
                db.tx().execute("UPDATE snat_target SET prefix = :prefix, iprefix = :iprefix WHERE id = :id", 
                    named_params! { ":id": id, ":prefix": prefix, ":iprefix": iprefix })?;
            },
            SNat6Event::TargetRemoved { id } => {
                db.tx().execute("DELETE FROM snat_target WHERE id = :id", named_params! { ":id": id })?;
            },
            _ => { }
        }
    }
    Ok(())
}

// pub fn project_operational_snat


// pub struct SNat6SourcePrefixView {
//     snat_id: Uuid,
//     prefix: Ipv6Prefix
// }

// Target Prefixes must be unique within each SNAT
// It's not required for target prefixes to be globally unique
// target prefixes in ACTIVE MAPPINGS (EGRESS ENABLED) MUST be globally unique
// pub struct SNat6TargetPrefixView {
//     snat_id: Uuid,
//     snat_mapping_id: Uuid,
//     prefix: Ipv6Prefix
// }