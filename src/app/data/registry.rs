use crate::app::{data::migrations::scripts::*, isp::projections::*, snat6::projections::*};
use crate::app::{isp::reactors::*, lan6::projections::*, snat6::reactors::*, wan4::reactors::*};
use super::framework::{migration::*, projection::*, reactor::*};
use lazy_static::lazy_static;
use include_dir::{include_dir, Dir};

pub static SCHEMA_MIGRATIONS: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/app/data/migrations/schema");

lazy_static! {
    pub static ref MIGRATIONS: Migrations = Migrations::new([
        Migration::to("0.1.0").up(up_v0_1_0),
        Migration::to("0.2.0").up(up_v0_2_0)
    ]);

    pub static ref PROJECTIONS: Projections = Projections::new([
        Projection::new("lan", project_lan6),
        Projection::new("lan_descendant", project_lan6_descendants),
        Projection::new("snat", project_snat6),
        Projection::new("snat_target", project_snat6_target),
        Projection::new("isp", project_isp)
    ]);
    
    pub static ref REACTORS: Reactors = Reactors::new(vec![
        snat_react_to_isp_events,
        wan_react_to_isp_events,
        isp_react_to_isp_events
    ]);
}