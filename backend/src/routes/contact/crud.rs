use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::domain::NewContactRequest;

pub async fn create_contact_request(
    pool: web::Data<PgPool>,
    data: web::Form<NewContactRequest>,
) -> HttpResponse {
    match insert_contact_request(data.clone(), &pool).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "message": e.to_string()
        })),
    }
}

#[tracing::instrument(
    name = "Inserting new contact request into the database",
    skip(new_contact_request, pool)
)]
async fn insert_contact_request(
    new_contact_request: NewContactRequest,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO contact_requests (id, name, company_name, email, message, urgency, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        uuid::Uuid::new_v4(),
        new_contact_request.name,
        match new_contact_request.company_name {
            Some(company_name) => company_name,
            None => String::from(""),
        },
        new_contact_request.email,
        new_contact_request.message,
        match new_contact_request.urgency {
            Some(urgency) => {
                match urgency {
                    true => "urgent",
                    false => "not urgent",
                }
            }
            None => "regular",
        },
        chrono::Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
