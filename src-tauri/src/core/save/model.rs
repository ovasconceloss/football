use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Save {
  pub uuid: Uuid,
  pub name: String,
  pub created_at: String,
}