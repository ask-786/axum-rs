use std::sync::Arc;

use mongodb::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
}
