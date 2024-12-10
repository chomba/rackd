use crate::{sys::link::models::{LinkId, LinkName}, util::{domain::Id, net::types::{Ipv4Prefix, Ipv6Prefix}}};
use super::{isp::models::IspName, lan::models::LanName, lan6::models::{Lan6, Lan6Prefix}, snat6::models::*, wan::models::WanName, wan4::models::{Wan4, Wan4Prefix}, wan6::models::{Wan6, Wan6Prefix}};

#[derive(Debug)]
pub enum AppError {
    // Queries
    NotFound,
    // Lan
    LanNameAlreadyInUse { name: LanName },
    // Lan4PrefixIsInvalid { prefix: Lan4Prefix },
    // Lan4PrefixOverlaps { prefix: Lan4Prefix, computed_prefix: Ipv4Prefix, overlaps: Vec<Lan4> },
    Lan6PrefixIsInvalid { prefix: Lan6Prefix },
    Lan6PrefixOverlaps { prefix: Lan6Prefix, iprefix: Ipv6Prefix, overlaps: Vec<Lan6> },
    // Wan
    WanNameAlreadyInUse { name: WanName },
    Wan4PrefixIsInvalid { prefix: Wan4Prefix },
    Wan4PrefixOverlaps { prefix: Wan4Prefix, iprefix: Ipv4Prefix, overlaps: Vec<Wan4> },
    Wan6PrefixIsInvalid { prefix: Wan6Prefix },
    Wan6PrefixOverlaps { prefix: Wan6Prefix, iprefix: Ipv6Prefix, overlaps: Vec<Wan6> },
    // Isp
    IspNameAlreadyInUse { name: IspName },
    IspLinkAlreadyInUse { link: LinkName, used_by: IspName },
    IspLinkNotFound { id: LinkId },
    IspLinkHasNoIpv4Prefix { id: LinkId },
    IspLinkHasNoIpv6Prefix { id: LinkId },
    IspLinkIsAlreadyEnabled { id: LinkId },
    IspLinkIsAlreadyDisabled { id: LinkId },
    IspLinkCantBeEnabled { id: LinkId },
    IspLinkCantBeDisabled { id: LinkId },
    IspLinkCantBeTracked { id: LinkId },
    // SNat
    SNat6PrefixIsInvalid { prefix: SNat6Prefix },
    SNat6PrefixOverlaps { prefix: SNat6Prefix, iprefix: Ipv6Prefix, overlaps: Vec<SNat6> },
    SNat6HasInvalidTargetPrefix { prefix: SNat6TargetPrefix },
    SNat6TargetPrefixOverlaps { prefix: SNat6TargetPrefix, iprefix: Ipv6Prefix, overlaps: Vec<SNat6Target> },
    SNat6TargetNotFound { id: Id },
    SNat6ModeHasUnknownTargets { mode: SNat6Mode, unknowns: Vec<Id> },
    SNat6ModeAlreadyInUse { mode: SNat6Mode },

    Db(rusqlite::Error),
    Netlink(rtnetlink::Error)
}

impl From<rusqlite::Error> for AppError {
    fn from(error: rusqlite::Error) -> Self {
        // map rusqlite::Error::QueryReturnedNoRows 
        // map otheer rusqlite errors
        AppError::Db(error)
    }
}

impl From<rtnetlink::Error> for AppError {
    fn from(error: rtnetlink::Error) -> Self {
        AppError::Netlink(error)
    }
}

