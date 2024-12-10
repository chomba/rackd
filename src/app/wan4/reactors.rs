use crate::app::error::AppError;
use crate::app::isp::models::{IspEvent, IspPrefix};
use crate::app::shared::domain::{Event, InnerEvent};
use crate::app::wan::models::WanName;
use crate::app::data::DbSession;
use crate::app::wan6::models::{Wan6, Wan6Prefix};
use crate::util::domain::Id;
use super::models::*;

pub fn wan_react_to_isp_events(db: &DbSession, event: &Event) -> Result<(), AppError> {
    match &event.data {
        InnerEvent::Isp(e) => {
            match e {
                IspEvent::Created { id, rank, name, link, link_name, link_status, prefix, tracker } => {                    

                    match prefix {
                        IspPrefix::DualStack((ipv4_prefix, ipv6_prefix)) => {
                            let mut wan4 = Wan4::new(Id::new(), WanName(name.to_string()), Wan4Prefix::Isp(*id), *ipv4_prefix);
                            let mut wan6 = Wan6::new(Id::new(), WanName(name.to_string()), Wan6Prefix::Isp(*id), *ipv6_prefix);
                            db.presave(&mut wan4)?;
                            db.presave(&mut wan6)?;
                        },
                        IspPrefix::V4(prefix) => {
                            let mut wan = Wan4::new(Id::new(), WanName(name.to_string()), Wan4Prefix::Isp(*id), *prefix);
                            db.presave(&mut wan)?;
                        },
                        IspPrefix::V6(prefix) => {
                            let mut wan = Wan6::new(Id::new(), WanName(name.to_string()), Wan6Prefix::Isp(*id), *prefix);
                            db.presave(&mut wan)?;
                        }
                    }
                    
                },
                _ => { }
            }
        },
        _ => { }
    }
    Ok(())
}