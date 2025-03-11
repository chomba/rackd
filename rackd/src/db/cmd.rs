pub mod migrations;
pub mod traits;
use std::sync::OnceLock;
use crate::net::wan::query::views::Wan;
use super::util::Projectors;

pub fn projectors() -> &'static Projectors {
    static PROJECTORS: OnceLock<Projectors> = OnceLock::new();
    PROJECTORS.get_or_init(|| {
        let mut projectors = Projectors::new();
        
        projectors.register::<Wan>();
        projectors
    })
}

// pub fn reactors() -> 