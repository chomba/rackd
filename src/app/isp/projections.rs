use crate::{app::{data::DbSession, error::AppError, isp::models::IspEvent, shared::domain::*}, sys::link::models::LinkStatus};
use rusqlite::{named_params, params};

pub fn project_isp(db: &DbSession, event: &Event) -> Result<(), AppError> {
    return match &event.data {
        InnerEvent::Isp(e) => {
            match e {
                IspEvent::Created { id, rank, name, link, link_name, link_status, prefix, tracker } => {
                    db.tx().execute("INSERT INTO isp (version, id, rank, name, link, link_name, link_status, prefix, tracker) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                        params![event.version, id, rank, name, link, link_name, link_status, prefix, tracker])?;
                    Ok(())
                },
                IspEvent::Renamed { name } => {
                    let mut stmt = db.tx().prepare("UPDATE isp SET name = :name, version = :version WHERE id = :id")?;
                    stmt.execute(named_params! {
                        ":name": name,
                        ":version": event.version,
                        ":id": event.stream_id
                    })?;
                    Ok(())
                },
                IspEvent::LinkWentUp { up } => {
                    let mut stmt = db.tx().prepare("UPDATE isp SET link_status = :link_status, version = :version WHERE id = :id")?;
                    stmt.execute(named_params! {
                        ":link_status": LinkStatus::Up(*up),
                        ":version": event.version,
                        ":id": event.stream_id
                    })?;
                    Ok(())
                }, 
                IspEvent::LinkWentDown { down } => {
                    let mut stmt = db.tx().prepare("UPDATE isp SET link_status = :link_status, version = :version WHERE id = :id")?;
                    stmt.execute(named_params! {
                        ":link_status": LinkStatus::Down(*down),
                        ":version": event.version,
                        ":id": event.stream_id
                    })?;
                    Ok(())
                },
                IspEvent::Deleted => {
                    let mut stmt = db.tx().prepare("UPDATE isp SET deleted = :deleted, version = :version WHERE id = :id")?;
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
    }
}