use crate::handlers::root::root_handler;
use axum::{Router, routing::get};

pub fn root_routes() -> Router {
    Router::new().route("/", get(root_handler))
}
