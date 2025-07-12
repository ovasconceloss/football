use sqlx::{SqlitePool, Error};
use crate::core::save::model::Save;

pub struct SaveRepository {
  connection: SqlitePool
}

impl SaveRepository {
  pub fn new(connection: SqlitePool) -> Self {
    SaveRepository { connection: connection }
  }

  pub async fn insert(&self, save: &mut Save) -> Result<(), Error> {
    let query = "INSERT INTO saves (uuid, name, created_at) VALUES (?, ?, ?)";
    let _result = sqlx::query(&query)
      .bind(&save.uuid)
      .bind(&save.name)
      .bind(&save.created_at)
      .execute(&self.connection)
      .await?;

    Ok(())
  }
}