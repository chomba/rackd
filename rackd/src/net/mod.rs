pub mod wan;
pub mod shared;
use wan::cmd::WanCmd;

pub enum NetCmd {
    Wan(WanCmd)
}