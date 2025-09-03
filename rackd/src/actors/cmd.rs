use rusqlite::Connection;
use crate::trunk::cmd::TrunkCmd;
use crate::wan::cmd::WanCmd;
use crate::util::actor::{Actor, Process};

#[derive(Debug)]
pub struct RackdCmdActor {
    pub conn: Connection
}

#[derive(Debug)]
pub enum RackdCmd {
    Trunk(TrunkCmd),
    Wan(WanCmd)
}

impl Actor for RackdCmdActor {
    type Message = RackdCmd;

    fn receive(&mut self, cmd: RackdCmd) {
        match cmd {
            RackdCmd::Wan(cmd) => match cmd {
                WanCmd::Create(cmd) => {
                    let response = cmd.payload.process(self);
                    let _ = cmd.respond_to.send(response);
                },
                WanCmd::Rename(cmd) => {
                    let response = cmd.payload.process(self);
                    let _ = cmd.respond_to.send(response);
                },
                WanCmd::SetMacAddr(cmd) => {
                    let response = cmd.payload.process(self);
                    let _ = cmd.respond_to.send(response);
                },
                WanCmd::SetIpv4Params(cmd) => {
                    let response = cmd.payload.process(self);
                    let _ = cmd.respond_to.send(response);
                },
                // WanCmd::SetIpv6(cmd) => {
                //     let response = cmd.payload.process(self);
                //     let _ = cmd.respond_to.send(response);
                // }
            },
            RackdCmd::Trunk(cmd) => match cmd {
                TrunkCmd::Create(cmd) => {
                    let response = cmd.payload.process(self);
                    let _ = cmd.respond_to.send(response);
                }
            }
        }
        
        // match message {
        //     AppMessage::Initialize(msg) => {
        //         let response = msg.payload.process(self);
        //         let _ = msg.respond_to.send(response);
        //     },
        //     // Isps
        //     AppMessage::CreateIsp(msg) => {
        //         let response = msg.payload.process(self);
        //         let _ = msg.respond_to.send(response);
        //     },
        //     AppMessage::GetIspById(msg) => {
        //         let response = msg.payload.process(self);
        //         let _ = msg.respond_to.send(response);
        //     },
        //     AppMessage::GetIspByName(msg) => {
        //         let response = msg.payload.process(self);
        //         let _ = msg.respond_to.send(response);
        //     },
        //     AppMessage::IspLinkPrefixChanged(msg) => {
        //         let _ = msg.payload.process(self);
        //     }
        //     // Process HeartBeats
        //     AppMessage::IspLinkWentDown(msg) => {
        //         let _ = msg.payload.process(self);
        //     },
        //     AppMessage::IspLinkWentUp(msg) => {
        //         let _ = msg.payload.process(self);
        //     },
        //     // Lans
        //     AppMessage::CreateLan6(msg) => {
        //         let response = msg.payload.process(self);
        //         let _ = msg.respond_to.send(response);
        //     },
        //     AppMessage::RenameLan6(msg) => {
        //         let response = msg.payload.process(self);
        //         let _ = msg.respond_to.send(response);
        //     },
        //     AppMessage::SetLan6Prefix(msg) => {
        //         let response = msg.payload.process(self);
        //         let _ = msg.respond_to.send(response);
        //     },
        //     AppMessage::DeleteLan6(msg) => {
        //         let response = msg.payload.process(self);
        //         let _ = msg.respond_to.send(response);
        //     },
        //     AppMessage::GetLan6Overlappings(msg) => {
        //         let response = msg.payload.process(self);
        //         let _ = msg.respond_to.send(response);
        //     },
        //     AppMessage::GetAllLan6(msg) => {
        //         let response = msg.payload.process(self);
        //         let _ = msg.respond_to.send(response);
        //     },
        //     AppMessage::GetLan6ById(msg) => {
        //         let response = msg.payload.process(self);
        //         let _ = msg.respond_to.send(response);
        //     },
        //     AppMessage::CreateSNat6(msg) => {
        //         let response = msg.payload.process(self);
        //         let _ = msg.respond_to.send(response);
        //     },
        //     AppMessage::AddSNat6Target(msg) => {
        //         let response = msg.payload.process(self);
        //         let _ = msg.respond_to.send(response);
        //     }
        // }
    }
}

impl RackdCmdActor {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }
}

// NetCmdActor:
// Shares a single DB and DBConnection across all Network entities (LAN, WAN, etc)
// It's primarily used to handle Commands but can also reply to queries based on INLINE-PROJECTIONS
// There can only be a single instance of this actor
// If multiple instances are run at once, the DB will guarantee consistency
// at the cost of rejecting conflicting commands, in other words
// If WriteCommand A is sent to NetCmdActor1
// And WriteCommand B is sent to NetCmdActor2
// are both processed at the same time
// Then only one of them will go through an the other will be rejected
// THUS, THERE MUST BE A SINGLE INSTANCE OF THE NetAtomicActor to avoid failing commands.
// NOTE: There should probably be a command to place a LOCK on a certain Entity ID
// in order to let users guarantee no changes are made to an entity while they are editing it
// and that the view of the entity hasn't changed since they starting editing it.

// NetQueryActor
// There can be multiple instances of this actor
// Queries sent to this actor may query data from the DB used by the NetCmdActor singleton
// in order to get more strongly consitent data
// In general, this actor should have its own database and there should be multiple
// Background Processors running projections on data read from the DB used by the NetCmdActor
// Data from this Actor is evetually consistent 


// pub struct Initialize;

// impl Payload for Initialize {
//     type Ok = ();
//     type Err = AppError;
// }

// impl Process for Initialize {
//     type Actor = RackdActor;
    
//     fn process(self, actor: &mut RackdActor) -> Result<Self::Ok, AppError> {
//         let db = actor.db.begin()?;
//         let isps = db.run(GetAllIsp)?;
//         for isp in isps {
//             let cmd = TrackIspLink { link: isp.link, status: isp.link_status, prefix: isp.prefix };
//             let _ = actor.sys.blocking_send(cmd);
//         }
//         Ok(())
//     }
// }

// // Net
// pub type CreateNet6Cmd = Msg<CreateNet6>;
// pub type CreateNet4Cmd = Msg<CreateNet4>;
// pub type ExtendNet6Cmd = Msg<ExtendNet6>;
// pub type ExtendNet4Cmd = Msg<ExtendNet4>;
// pub type GetNet6Cmd = Msg<GetNet6>;
// pub type GetNet4Cmd = Msg<GetNet4>;


// pub type InitializeCmd = Msg<Initialize>;
// // Lan
// pub type CreateLan6Cmd = Msg<CreateLan6>;
// pub type RenameLan6Cmd = Msg<RenameLan6>;
// pub type SetLan6PrefixCmd = Msg<SetLan6Prefix>;
// pub type DeleteLan6Cmd = Msg<DeleteLan6>;
// pub type GetLan6OverlappingsQuery = Msg<GetLan6Overlappings>;
// pub type GetAllLan6Query = Msg<GetAllLan6>;
// pub type GetLan6ByIdQuery = Msg<GetLan6ById>;
// // SNat
// pub type CreateSNat6Cmd = Msg<CreateSNat6>;
// pub type AddSNat6TargetCmd = Msg<AddSNat6Target>;
// // pub type UpdateSNat6TargetCmd = Msg<UpdateSNat6Target>;
// // Isp
// pub type CreateIspCmd = Msg<CreateIsp>;
// pub type GetIspByIdQuery = Msg<GetIspById>;
// pub type GetIspByNameQuery = Msg<GetIspByName>;
// pub type IspLinkWentUpEvent = Msg<IspLinkWentUp>;
// pub type IspLinkWentDownEvent = Msg<IspLinkWentDown>;
// pub type IspLinkPrefixChangedEvent = Msg<IspLinkPrefixChanged>;

// pub enum AppMessage {
//     Initialize(InitializeCmd),
//     // Lan
//     CreateLan6(CreateLan6Cmd),
//     RenameLan6(RenameLan6Cmd),
//     SetLan6Prefix(SetLan6PrefixCmd),
//     DeleteLan6(DeleteLan6Cmd),
//     GetAllLan6(GetAllLan6Query),
//     GetLan6ById(GetLan6ByIdQuery),
//     GetLan6Overlappings(GetLan6OverlappingsQuery),
//     // SNat
//     CreateSNat6(CreateSNat6Cmd),
//     AddSNat6Target(AddSNat6TargetCmd),
//     // UpdateSNat6Target(UpdateSNat6TargetCmd),
//     // Isp
//     CreateIsp(CreateIspCmd),
//     GetIspByName(GetIspByNameQuery),
//     GetIspById(GetIspByIdQuery),
//     IspLinkWentDown(IspLinkWentDownEvent),
//     IspLinkWentUp(IspLinkWentUpEvent),
//     IspLinkPrefixChanged(IspLinkPrefixChangedEvent),
// }

// // SNat

// impl From<CreateSNat6Cmd> for AppMessage {
//     fn from(value: CreateSNat6Cmd) -> Self {
//         AppMessage::CreateSNat6(value)
//     }
// }

// impl From<AddSNat6TargetCmd> for AppMessage {
//     fn from(value: AddSNat6TargetCmd) -> Self {
//         AppMessage::AddSNat6Target(value)
//     }
// }

// // impl From<UpdateSNat6TargetCmd> for AppMessage {
// //     fn from(value: UpdateSNat6TargetCmd) -> Self {
// //         AppMessage::UpdateSNat6Target(value)
// //     }
// // }

// // Lan

// impl From<CreateLan6Cmd> for AppMessage {
//     fn from(value: CreateLan6Cmd) -> Self {
//         AppMessage::CreateLan6(value)
//     }
// }

// impl From<RenameLan6Cmd> for AppMessage {
//     fn from(value: RenameLan6Cmd) -> Self {
//         AppMessage::RenameLan6(value)
//     }
// }

// impl From<SetLan6PrefixCmd> for AppMessage {
//     fn from(value: SetLan6PrefixCmd) -> Self {
//         AppMessage::SetLan6Prefix(value)
//     }
// }

// impl From<DeleteLan6Cmd> for AppMessage {
//     fn from(value: DeleteLan6Cmd) -> Self {
//         AppMessage::DeleteLan6(value)
//     }
// }

// impl From<GetLan6OverlappingsQuery> for AppMessage {
//     fn from(value: GetLan6OverlappingsQuery) -> Self {
//         AppMessage::GetLan6Overlappings(value)
//     }
// }

// impl From<GetAllLan6Query> for AppMessage {
//     fn from(value: GetAllLan6Query) -> Self {
//         AppMessage::GetAllLan6(value)
//     }
// }

// impl From<GetLan6ByIdQuery> for AppMessage {
//     fn from(value: GetLan6ByIdQuery) -> Self {
//         AppMessage::GetLan6ById(value)
//     }
// }

// // ISP

// impl From<InitializeCmd> for AppMessage {
//     fn from(value: InitializeCmd) -> Self {
//         AppMessage::Initialize(value)
//     }
// }

// impl From<IspLinkWentUpEvent> for AppMessage {
//     fn from(e: IspLinkWentUpEvent) -> Self {
//         AppMessage::IspLinkWentUp(e)
//     }
// }

// impl From<IspLinkWentDownEvent> for AppMessage {
//     fn from(e: IspLinkWentDownEvent) -> Self {
//         AppMessage::IspLinkWentDown(e)
//     }
// }

// // Isp

// impl From<GetIspByNameQuery> for AppMessage {
//     fn from(query: GetIspByNameQuery) -> Self {
//         AppMessage::GetIspByName(query)
//     }
// }

// impl From<GetIspByIdQuery> for AppMessage {
//     fn from(query: GetIspByIdQuery) -> Self {
//         AppMessage::GetIspById(query)
//     }
// }

// impl From<CreateIspCmd> for AppMessage {
//     fn from(cmd: CreateIspCmd) -> Self {
//         AppMessage::CreateIsp(cmd)
//     }
// }