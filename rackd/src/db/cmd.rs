pub mod migrations;
pub mod traits;
use std::sync::OnceLock;
use crate::{net::views::NetworkView, trunk::views::TrunkView, wan::views::WanView};

use super::util::Projectors;

pub fn projectors() -> &'static Projectors {
    static PROJECTORS: OnceLock<Projectors> = OnceLock::new();
    PROJECTORS.get_or_init(|| {
        let mut projectors = Projectors::new();
        
        projectors.register::<NetworkView>();
        projectors.register::<WanView>();
        projectors.register::<TrunkView>();
        projectors
    })
}

// pub fn reactors() -> 