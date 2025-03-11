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

impl From<Msg<SetMacToSpoof>> for RackdCmd {
    fn from(cmd: Msg<SetMacToSpoof>) -> Self {
        Self::Net(NetCmd::Wan(WanCmd::SetMacToSpoof(cmd)))
    }
}

impl From<Msg<SetMacToAuto>> for RackdCmd {
    fn from(cmd: Msg<SetMacToAuto>) -> Self {
        Self::Net(NetCmd::Wan(WanCmd::SetMacToAuto(cmd)))
    }
}

impl From<Msg<SetIpv6ToStatic>> for RackdCmd {
    fn from(cmd: Msg<SetIpv6ToStatic>) -> Self {
        Self::Net(NetCmd::Wan(WanCmd::SetIpv6ToStatic(cmd)))
    }
}

impl From<Msg<SetIpv6ToRA>> for RackdCmd {
    fn from(cmd: Msg<SetIpv6ToRA>) -> Self {
        Self::Net(NetCmd::Wan(WanCmd::SetIpv6ToRA(cmd)))
    }
}

impl From<Msg<SetIpv4ToStatic>> for RackdCmd {
    fn from(cmd: Msg<SetIpv4ToStatic>) -> Self {
        Self::Net(NetCmd::Wan(WanCmd::SetIpv4ToStatic(cmd)))
    }
}

impl From<Msg<SetIpv4ToDHCP>> for RackdCmd {
    fn from(cmd: Msg<SetIpv4ToDHCP>) -> Self {
        Self::Net(NetCmd::Wan(WanCmd::SetIpv4ToDHCP(cmd)))
    }
}