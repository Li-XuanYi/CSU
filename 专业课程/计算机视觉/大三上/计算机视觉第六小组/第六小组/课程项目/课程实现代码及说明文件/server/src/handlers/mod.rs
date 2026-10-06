use sea_orm::DatabaseConnection;

pub mod albums;
pub mod docs;
pub mod health;
pub mod photos;
pub mod processing;
pub mod users;

/// Application state containing shared resources
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
