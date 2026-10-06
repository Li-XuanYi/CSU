use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Get database URL from environment
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    // Connect to database
    let db = sea_orm::Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    println!("Running migrations...");
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    println!("Migrations completed successfully!");
}
