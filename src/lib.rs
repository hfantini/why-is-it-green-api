pub mod adapters;

use axum::{Router, routing::get};

use crate::adapters::inbound::http::healthcheck_handler::health;
use crate::adapters::inbound::http::version_handler::version;

pub fn app() -> Router {
    Router::new()
        .route("/healthcheck", get(health))
        .route("/version", get(version))
}
