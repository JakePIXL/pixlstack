use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NewUser {
    pub username: String,
    pub display_name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserInDB {
    pub id: Uuid,
    pub role: Option<String>,
    pub is_verified: Option<bool>,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
