use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Save {
  pub uuid: Uuid,
  pub name: String,
  pub created_at: String,
}

impl Save {
  pub fn new(name: String) -> Self {
    Save { 
      uuid: Uuid::new_v4(), 
      name: name, 
      created_at: chrono::Utc::now().to_rfc3339() 
    }
  }

  pub fn is_name_valid(&self) -> bool {
    !self.name.trim().is_empty() && self.name.len() <= 100
  }
}