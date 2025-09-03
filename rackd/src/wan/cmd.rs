use crate::util::actor::Msg;
pub mod create;
pub mod rename;
pub mod set_mac;
pub mod set_ipv6;
pub mod set_ipv4;

#[derive(Debug)]
pub enum WanCmd {
    Create(Msg<create::CreateWan>),
    Rename(Msg<rename::RenameWan>),
    SetMacAddr(Msg<set_mac::SetMacAddr>),
    SetIpv4Params(Msg<set_ipv4::SetIpv4Params>),
    // SetIpv6(Msg<set_ipv6::SetIpv6>)
}