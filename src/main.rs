mod config;
mod handlers;
mod models;
mod routes;
mod state;

use std::sync::Arc;

use axum::Router;
use config::db;
use routes::{root::root_routes, users::users_routes};
use state::app_state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db = db::connect_db().await.unwrap();
    println!("Database connected");

    let state = AppState { db: Arc::new(db) };

    let app = Router::new()
        .merge(root_routes())
        .merge(users_routes().with_state(state.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
