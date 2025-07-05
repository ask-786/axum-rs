use axum::{
    Router,
    routing::get,
};

use crate::{
    handlers::users::{add_user, get_all_users, get_user_with_id},
    state::app_state::AppState,
};

pub fn users_routes() -> Router<AppState> {
    Router::new().nest(
        "/users",
        Router::new()
            .route("/", get(get_all_users).put(add_user))
            .route("/{id}", get(get_user_with_id)),
    )
}
