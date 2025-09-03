use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use crate::actors::system::Rackd;
use crate::{trunk, wan};

pub fn router() -> OpenApiRouter {
    let rackd = Rackd::new("/data/lab/rust/rackd/rackd/tmp/rackd.db").unwrap();
    OpenApiRouter::new()
        .routes(routes!(wan::cmd::create::api::create, wan::query::get_by_key::api::get_wan_by_id))
        .routes(routes!(trunk::cmd::create::api::create))
        .with_state(rackd)
}