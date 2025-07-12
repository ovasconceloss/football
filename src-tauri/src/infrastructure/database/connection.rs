use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn create_connection() -> SqlitePool {
  SqlitePoolOptions::new()
    .max_connections(1)
    .connect("sqlite://savegame.db")
    .await
    .expect("An error occurred while connecting to the database.")
}