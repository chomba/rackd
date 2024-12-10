use crate::app::{data::DbSession, error::AppError, shared::domain::Event};

pub fn snat_react_to_isp_events(db: &DbSession, event: &Event) -> Result<(), AppError> {
    // go through all SNATs and update their Status
    // match &event.data {
    //     InnerEvent::Isp(IspEvent::)
    // }
    Ok(())
}