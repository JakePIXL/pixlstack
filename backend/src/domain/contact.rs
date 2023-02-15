#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct NewContactRequest {
    pub name: String,
    pub email: String,
    pub company_name: Option<String>,
    pub message: String,
    pub urgency: Option<bool>,
}
