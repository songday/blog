use std::time::SystemTime;

use serde::{Serialize, Deserialize};

use crate::db::{self, DataSource};
use crate::result::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Blog {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub created_at: u64,
}

impl Blog {
    pub async fn save(&mut self, d: &DataSource) -> Result<usize> {
        self.id = db::sled_id(d).await?;
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
        self.created_at = now.unwrap().as_secs();

        let serialized = serde_json::to_string(self).unwrap();

        let b: Option<Blog> = db::sled_get(d, 100).await?;
        db::sled_save(d, self.id, &serialized).await
    }
}