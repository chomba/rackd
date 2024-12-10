use crate::app::{data::DbSession, error::AppError, isp::models::IspEvent, shared::domain::*};

pub fn isp_react_to_isp_events(db: &DbSession, event: &Event) -> Result<(), AppError> {
    match &event.data {
        InnerEvent::Isp(IspEvent::Created { id, rank, name, link, link_name, link_status, prefix, tracker }) => {
            // SEND EVENT TO BUS
            // db.loopback.emit(IspNeedsTrackingEvent { id: *id });
        }
        _ => { }
    }

    Ok(())
}