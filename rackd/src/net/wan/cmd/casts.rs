use crate::{actors::cmd::RackdCmd, net::NetCmd, util::actor::Msg};
use super::*;

impl From<Msg<Create>> for RackdCmd {
    fn from(cmd: Msg<Create>) -> Self {
        Self::Net(NetCmd::Wan(WanCmd::Create(cmd)))
    }
}

impl From<Msg<Rename>> for RackdCmd {
    fn from(cmd: Msg<Rename>) -> Self {
        Self::Net(NetCmd::Wan(WanCmd::Rename(cmd)))
    }
}