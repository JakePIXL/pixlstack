use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NewCategory {
    pub name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CategoryInDB {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}
