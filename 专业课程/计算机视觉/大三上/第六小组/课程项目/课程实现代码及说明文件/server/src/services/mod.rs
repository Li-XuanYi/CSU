// This module contains business logic services
// Keep your handlers thin by moving business logic here

use sea_orm::DatabaseConnection;

pub struct ServiceContext {
    pub db: DatabaseConnection,
}

impl ServiceContext {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
