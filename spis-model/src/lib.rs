use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Image {
    pub uuid: String,
    pub image: String,
    pub thumbnail: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}