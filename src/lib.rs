pub mod adapters;

use axum::{Router, routing::get};

use crate::adapters::inbound::http::healthcheck_handler::health;

pub fn app() -> Router {
    Router::new().route("/healthcheck", get(health))
}
