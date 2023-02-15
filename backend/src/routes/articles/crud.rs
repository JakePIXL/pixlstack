use actix_web::{web, HttpResponse};
use chrono::Utc;
use slug::slugify;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::NewArticle;

pub async fn create_article(
    pool: web::Data<PgPool>,
    data: web::Either<web::Json<NewArticle>, web::Form<NewArticle>>,
) -> HttpResponse {
    let article = match data {
        web::Either::Left(json) => json.into_inner(),
        web::Either::Right(form) => form.into_inner(),
    };

    if let Ok(item) = sqlx::query!(
        "SELECT * FROM articles WHERE slug = $1",
        slugify(article.title.clone())
    )
    .fetch_optional(pool.as_ref())
    .await
    {
        if item.is_some() {
            return HttpResponse::BadRequest().body("Article already exists");
        }
    }

    let article_id = match insert_article(&pool, article).await {
        Ok(article_id) => article_id,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    HttpResponse::Created().json(article_id.to_string())
}

#[tracing::instrument(name = "Saving new article in the database", skip(new_article, pool))]
async fn insert_article(pool: &PgPool, new_article: NewArticle) -> Result<Uuid, sqlx::Error> {
    let article_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO articles (id, title, slug, excerpt, content, category_id, is_published, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        article_id,
        new_article.title.clone(),
        slugify(new_article.title),
        new_article.excerpt,
        new_article.body,
        Uuid::parse_str(&new_article.category.unwrap_or("".to_string())).unwrap_or(Uuid::nil()),
        new_article.published,
        Utc::now(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(article_id)
}
