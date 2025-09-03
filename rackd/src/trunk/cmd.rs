use crate::util::actor::Msg;
pub mod create;

#[derive(Debug)]
pub enum TrunkCmd {
    Create(Msg<create::CreateTrunk>)
}