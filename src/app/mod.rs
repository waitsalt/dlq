use axum::Router;

use crate::{route, util::logger};

pub fn init() -> Router {
    logger::init();
    route::init()
}
