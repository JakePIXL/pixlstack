use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::{NewUser, UserInDB},
    passwordless_client::PasswordlessClient,
};

pub async fn register_user(
    db_pool: web::Data<PgPool>,
    passwordless: web::Data<PasswordlessClient>,
    new_user: web::Json<NewUser>,
) -> HttpResponse {
    let user_id = Uuid::new_v4();

    let mut transaction = db_pool.begin().await.unwrap();

    match insert_new_user(&mut transaction, user_id).await {
        Ok(_) => match passwordless.try_register(&new_user, user_id).await {
            Ok(token) => {
                transaction.commit().await.unwrap();
                HttpResponse::Ok().body(token)
            }
            Err(e) => {
                transaction.rollback().await.unwrap();
                HttpResponse::InternalServerError().body(e)
            }
        },
        Err(e) => {
            transaction.rollback().await.unwrap();
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

pub async fn login_user(
    // db_pool: web::Data<PgPool>,
    passwordless: web::Data<PasswordlessClient>,
    token: web::Json<String>,
) -> HttpResponse {
    match passwordless.try_login(&token).await {
        Ok(user_id) => HttpResponse::Ok().json(user_id),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[tracing::instrument(name = "Registering a new user", skip(transaction, user_id))]
async fn insert_new_user(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, role, is_verified, is_active)
        VALUES ($1, 'default', true, false)
        "#,
        user_id
    )
    .execute(transaction)
    .await?;

    Ok(())
}

#[tracing::instrument(name = "Searching for user in database", skip(db_pool, user_id))]
async fn find_user(db_pool: web::Data<PgPool>, user_id: Uuid) -> Result<UserInDB, sqlx::Error> {
    match sqlx::query_as!(
        UserInDB,
        r#"
        SELECT * FROM users WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(db_pool.get_ref())
    .await
    .map_err(|e| {
        tracing::error!("Error while searching for user in database: {}", e);
        e
    })? {
        Some(result) => Ok(result),
        None => Err(sqlx::Error::RowNotFound),
    }
}
