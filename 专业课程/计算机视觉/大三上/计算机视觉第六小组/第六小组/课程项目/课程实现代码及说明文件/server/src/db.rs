use crate::config::DatabaseConfig;
use crate::error::Result;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

/// Initialize database connection pool
pub async fn init_db_pool(config: &DatabaseConfig) -> Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(&config.url);

    opt.max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(3600))
        .sqlx_logging(true);

    let db = Database::connect(opt).await?;

    tracing::info!("Database connection pool initialized successfully");

    Ok(db)
}
