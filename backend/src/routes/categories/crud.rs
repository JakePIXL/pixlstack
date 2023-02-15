use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{CategoryInDB, NewCategory};

pub async fn create_category(
    pool: web::Data<PgPool>,
    new_category: web::Either<web::Json<NewCategory>, web::Form<NewCategory>>,
) -> HttpResponse {
    let new_category = match new_category {
        web::Either::Left(json) => json.into_inner(),
        web::Either::Right(form) => form.into_inner(),
    };

    let category_id = match insert_category(&pool, new_category).await {
        Ok(category_id) => category_id,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    HttpResponse::Created().json(category_id.to_string())
}

pub async fn get_category(pool: web::Data<PgPool>, slug: web::Path<String>) -> HttpResponse {
    match sqlx::query_as!(
        CategoryInDB,
        "
        SELECT * FROM categories
        WHERE slug = $1
        ",
        slug.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(category) => {
            if category.is_none() {
                return HttpResponse::NotFound().finish();
            }
            HttpResponse::Ok().json(category.unwrap())
        }
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[tracing::instrument(name = "Saving new category in the database", skip(new_category, pool))]
async fn insert_category(pool: &PgPool, new_category: NewCategory) -> Result<Uuid, sqlx::Error> {
    let category_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO categories (id, name, slug)
        VALUES ($1, $2, $3)
        "#,
        category_id,
        new_category.name.clone(),
        slug::slugify(new_category.name)
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(category_id)
}
