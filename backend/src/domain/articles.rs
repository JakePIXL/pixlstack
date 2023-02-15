use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct NewArticle {
    pub title: String,
    pub category: Option<String>,
    pub excerpt: String,
    pub body: String,
    pub published: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ArticleInDB {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub category_id: Option<Uuid>,
    pub category_name: String,
    pub category_slug: String,
    pub excerpt: String,
    pub content: String,
    pub is_published: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
