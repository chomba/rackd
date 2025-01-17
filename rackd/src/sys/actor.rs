use std::sync::Arc;
use aya::Ebpf;
use crate::{app::actor::AppMessage, util::actor::*};
use super::{link::{cmd::*, query::*}, util::{netlink::Netlink, trackers::LinkTrackers}};

pub struct SysActor {
    pub netlink: Netlink,
    pub trackers: LinkTrackers,
    pub app: Handle<AppMessage>
}

impl AsyncActor for SysActor {
    type Message = SysMessage;

    async fn receive(&mut self, message: Self::Message) {
        match message {
            SysMessage::GetLinkById(msg) => {
                let response = msg.payload.process(self).await;
                let _ = msg.respond_to.send(response);
            },
            SysMessage::GetLinkByName(msg) => {
                let response = msg.payload.process(self).await;
                let _ = msg.respond_to.send(response);
            },
            SysMessage::TrackIspLink(msg) => {
                let response = msg.payload.process(self);
                let _ = msg.respond_to.send(response);
            },
            SysMessage::EnableLink(msg) => {
                let response = msg.payload.process(self).await;
                let _ = msg.respond_to.send(response);
            },
            SysMessage::DisableLink(msg) => {
                let response = msg.payload.process(self).await;
                let _ = msg.respond_to.send(response);
            }
        }
    }
}

pub type EnableLinkCmd = Msg<EnableLink>;
pub type DisableLinkCmd = Msg<DisableLink>;
pub type TrackIspLinkCmd = Msg<TrackIspLink>;
pub type GetLinkByIdQuery = Msg<GetLinkById>;
pub type GetLinkByNameQuery = Msg<GetLinkByName>;

pub enum SysMessage {
    EnableLink(EnableLinkCmd),
    DisableLink(DisableLinkCmd),
    TrackIspLink(TrackIspLinkCmd),
    GetLinkById(GetLinkByIdQuery),
    GetLinkByName(GetLinkByNameQuery),
}

impl From<GetLinkByIdQuery> for SysMessage {
    fn from(value: GetLinkByIdQuery) -> Self {
        SysMessage::GetLinkById(value)
    }
}

impl From<GetLinkByNameQuery> for SysMessage {
    fn from(value: GetLinkByNameQuery) -> Self {
        SysMessage::GetLinkByName(value)
    }
}

impl From<TrackIspLinkCmd> for SysMessage {
    fn from(value: TrackIspLinkCmd) -> Self {
        SysMessage::TrackIspLink(value)
    }
}

impl From<EnableLinkCmd> for SysMessage {
    fn from(value: EnableLinkCmd) -> Self {
        SysMessage::EnableLink(value)
    }
}

impl From<DisableLinkCmd> for SysMessage {
    fn from(value: DisableLinkCmd) -> Self {
        SysMessage::DisableLink(value)
    }
}