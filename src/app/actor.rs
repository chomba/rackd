use super::isp::events::*;
use super::isp::query::*;
use super::lan6::query::GetAllLan6;
use super::lan6::query::GetLan6ById;
use super::lan6::query::GetLan6Overlappings;
use crate::app::error::AppError;
use crate::sys::actor::SysMessage;
use crate::sys::link::cmd::TrackIspLink;
use crate::util::actor::{Actor, Msg, Payload, Process};
use crate::util::actor::Handle;
use super::data::Db;
use super::lan6::cmd::*;
use super::snat6::cmd::*;
use super::isp::cmd::*;

pub struct AppActor {
    pub db: Db,
    pub sys: Handle<SysMessage>
}

impl Actor for AppActor {
    type Message = AppMessage;
    fn receive(&mut self, message: AppMessage) {
        match message {
            AppMessage::Initialize(msg) => {
                let response = msg.payload.process(self);
                let _ = msg.respond_to.send(response);
            },
            // Isps
            AppMessage::CreateIsp(msg) => {
                let response = msg.payload.process(self);
                let _ = msg.respond_to.send(response);
            },
            AppMessage::GetIspById(msg) => {
                let response = msg.payload.process(self);
                let _ = msg.respond_to.send(response);
            },
            AppMessage::GetIspByName(msg) => {
                let response = msg.payload.process(self);
                let _ = msg.respond_to.send(response);
            },
            AppMessage::IspLinkPrefixChanged(msg) => {
                let _ = msg.payload.process(self);
            }
            // Process HeartBeats
            AppMessage::IspLinkWentDown(msg) => {
                let _ = msg.payload.process(self);
            },
            AppMessage::IspLinkWentUp(msg) => {
                let _ = msg.payload.process(self);
            },
            // Lans
            AppMessage::CreateLan6(msg) => {
                let response = msg.payload.process(self);
                let _ = msg.respond_to.send(response);
            },
            AppMessage::RenameLan6(msg) => {
                let response = msg.payload.process(self);
                let _ = msg.respond_to.send(response);
            },
            AppMessage::SetLan6Prefix(msg) => {
                let response = msg.payload.process(self);
                let _ = msg.respond_to.send(response);
            },
            AppMessage::DeleteLan6(msg) => {
                let response = msg.payload.process(self);
                let _ = msg.respond_to.send(response);
            },
            AppMessage::GetLan6Overlappings(msg) => {
                let response = msg.payload.process(self);
                let _ = msg.respond_to.send(response);
            },
            AppMessage::GetAllLan6(msg) => {
                let response = msg.payload.process(self);
                let _ = msg.respond_to.send(response);
            },
            AppMessage::GetLan6ById(msg) => {
                let response = msg.payload.process(self);
                let _ = msg.respond_to.send(response);
            },
            AppMessage::CreateSNat6(msg) => {
                let response = msg.payload.process(self);
                let _ = msg.respond_to.send(response);
            },
            AppMessage::AddSNat6Target(msg) => {
                let response = msg.payload.process(self);
                let _ = msg.respond_to.send(response);
            },
            // AppMessage::UpdateSNat6Target(msg) => {
            //     let response = msg.payload.process(self);
            //     let _ = msg.respond_to.send(response);
            // },
            // AppMessage::EnableSNatSingleEgress(cmd) => {
            //     let response = cmd.payload.process(self);
            //     let _ = cmd.respond_to.send(response);
            // },
            // AppMessage::EnableSNatFailoverEgress(cmd) => {
            //     let response = cmd.payload.process(self);
            //     let _ = cmd.respond_to.send(response);
            // },
            // AppMessage::EnableSNatLoadsharedEgress(cmd) => {
            //     let response = cmd.payload.process(self);
            //     let _ = cmd.respond_to.send(response);
            // },
            // AppMessage::DisableSNatEgress(cmd) => {
            //     let response = cmd.payload.process(self);
            //     let _ = cmd.respond_to.send(response);
            // },
            // AppMessage::Event(e) => self.handle(e).await
        }
    }
}

pub struct Initialize;

impl Payload for Initialize {
    type Ok = ();
    type Err = AppError;
}

impl Process for Initialize {
    type Actor = AppActor;
    
    fn process(self, actor: &mut AppActor) -> Result<Self::Ok, AppError> {
        let db = actor.db.begin()?;
        let isps = db.run(GetAllIsp)?;
        for isp in isps {
            let cmd = TrackIspLink { tracker: Some(isp.tracker), link: isp.link, status: isp.link_status, prefix: isp.prefix };
            let _ = actor.sys.blocking_send(cmd);
        }
        Ok(())
    }
}

pub type InitializeCmd = Msg<Initialize>;
// Lan
pub type CreateLan6Cmd = Msg<CreateLan6>;
pub type RenameLan6Cmd = Msg<RenameLan6>;
pub type SetLan6PrefixCmd = Msg<SetLan6Prefix>;
pub type DeleteLan6Cmd = Msg<DeleteLan6>;
pub type GetLan6OverlappingsQuery = Msg<GetLan6Overlappings>;
pub type GetAllLan6Query = Msg<GetAllLan6>;
pub type GetLan6ByIdQuery = Msg<GetLan6ById>;
// SNat
pub type CreateSNat6Cmd = Msg<CreateSNat6>;
pub type AddSNat6TargetCmd = Msg<AddSNat6Target>;
// pub type UpdateSNat6TargetCmd = Msg<UpdateSNat6Target>;
// Isp
pub type CreateIspCmd = Msg<CreateIsp>;
pub type GetIspByIdQuery = Msg<GetIspById>;
pub type GetIspByNameQuery = Msg<GetIspByName>;
pub type IspLinkWentUpEvent = Msg<IspLinkWentUp>;
pub type IspLinkWentDownEvent = Msg<IspLinkWentDown>;
pub type IspLinkPrefixChangedEvent = Msg<IspLinkPrefixChanged>;

pub enum AppMessage {
    Initialize(InitializeCmd),
    // Lan
    CreateLan6(CreateLan6Cmd),
    RenameLan6(RenameLan6Cmd),
    SetLan6Prefix(SetLan6PrefixCmd),
    DeleteLan6(DeleteLan6Cmd),
    GetAllLan6(GetAllLan6Query),
    GetLan6ById(GetLan6ByIdQuery),
    GetLan6Overlappings(GetLan6OverlappingsQuery),
    // SNat
    CreateSNat6(CreateSNat6Cmd),
    AddSNat6Target(AddSNat6TargetCmd),
    // UpdateSNat6Target(UpdateSNat6TargetCmd),
    // Isp
    CreateIsp(CreateIspCmd),
    GetIspByName(GetIspByNameQuery),
    GetIspById(GetIspByIdQuery),
    IspLinkWentDown(IspLinkWentDownEvent),
    IspLinkWentUp(IspLinkWentUpEvent),
    IspLinkPrefixChanged(IspLinkPrefixChangedEvent),
}

// SNat

impl From<CreateSNat6Cmd> for AppMessage {
    fn from(value: CreateSNat6Cmd) -> Self {
        AppMessage::CreateSNat6(value)
    }
}

impl From<AddSNat6TargetCmd> for AppMessage {
    fn from(value: AddSNat6TargetCmd) -> Self {
        AppMessage::AddSNat6Target(value)
    }
}

// impl From<UpdateSNat6TargetCmd> for AppMessage {
//     fn from(value: UpdateSNat6TargetCmd) -> Self {
//         AppMessage::UpdateSNat6Target(value)
//     }
// }

// Lan

impl From<CreateLan6Cmd> for AppMessage {
    fn from(value: CreateLan6Cmd) -> Self {
        AppMessage::CreateLan6(value)
    }
}

impl From<RenameLan6Cmd> for AppMessage {
    fn from(value: RenameLan6Cmd) -> Self {
        AppMessage::RenameLan6(value)
    }
}

impl From<SetLan6PrefixCmd> for AppMessage {
    fn from(value: SetLan6PrefixCmd) -> Self {
        AppMessage::SetLan6Prefix(value)
    }
}

impl From<DeleteLan6Cmd> for AppMessage {
    fn from(value: DeleteLan6Cmd) -> Self {
        AppMessage::DeleteLan6(value)
    }
}

impl From<GetLan6OverlappingsQuery> for AppMessage {
    fn from(value: GetLan6OverlappingsQuery) -> Self {
        AppMessage::GetLan6Overlappings(value)
    }
}

impl From<GetAllLan6Query> for AppMessage {
    fn from(value: GetAllLan6Query) -> Self {
        AppMessage::GetAllLan6(value)
    }
}

impl From<GetLan6ByIdQuery> for AppMessage {
    fn from(value: GetLan6ByIdQuery) -> Self {
        AppMessage::GetLan6ById(value)
    }
}

// ISP

impl From<InitializeCmd> for AppMessage {
    fn from(value: InitializeCmd) -> Self {
        AppMessage::Initialize(value)
    }
}

impl From<IspLinkWentUpEvent> for AppMessage {
    fn from(e: IspLinkWentUpEvent) -> Self {
        AppMessage::IspLinkWentUp(e)
    }
}

impl From<IspLinkWentDownEvent> for AppMessage {
    fn from(e: IspLinkWentDownEvent) -> Self {
        AppMessage::IspLinkWentDown(e)
    }
}

// Isp

impl From<GetIspByNameQuery> for AppMessage {
    fn from(query: GetIspByNameQuery) -> Self {
        AppMessage::GetIspByName(query)
    }
}

impl From<GetIspByIdQuery> for AppMessage {
    fn from(query: GetIspByIdQuery) -> Self {
        AppMessage::GetIspById(query)
    }
}

impl From<CreateIspCmd> for AppMessage {
    fn from(cmd: CreateIspCmd) -> Self {
        AppMessage::CreateIsp(cmd)
    }
}