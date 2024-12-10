use crate::app::{data::DbSession, error::AppError, lan6::models::LanEvent, util::domain::{Event, InnerEvent}};

use super::query::ComputeLanPrefix;

pub fn lan_react_to_lan_events(db: &DbSession, event: &Event) -> Result<(), AppError> {
    match &event.data {
        InnerEvent::Lan(LanEvent::KindUpdated { kind, prefix }) => {
            // STORE EVENT 
        },
        _ => { }
    }

    Ok(())
}