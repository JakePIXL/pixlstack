use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

mod crud;
pub use crud::*;

use crate::domain::ArticleInDB;

#[derive(serde::Deserialize)]
pub struct ArticlesQuery {
    pub limit: Option<i64>,
    // pub offset: Option<i64>,
    pub page: Option<i64>,
}

pub async fn get_articles(
    pool: web::Data<PgPool>,
    query: web::Query<ArticlesQuery>,
) -> impl Responder {
    let limit = query.limit.unwrap_or(6);
    // let offset = query.offset.unwrap_or(0);
    let page = query.page.unwrap_or(1);

    let offset = if page <= 1 { 0 } else { (page - 1) * limit };

    let count = sqlx::query!("SELECT COUNT(*) FROM articles;")
        .fetch_one(pool.get_ref())
        .await
        .unwrap()
        .count
        .unwrap();

    let pages = (count as f64 / limit as f64).ceil() as i64;

    let articles = match sqlx::query_as!(
        ArticleInDB,
        "
        SELECT articles.*, categories.name AS category_name, categories.slug AS category_slug
        FROM articles
        JOIN categories
        ON articles.category_id = categories.id
        ORDER BY articles.created_at DESC
        LIMIT $1 OFFSET $2;
        ",
        limit,
        offset
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(articles) => articles,
        Err(_) => {
            return HttpResponse::NotFound().finish();
        }
    };

    if count == 0 {
        return HttpResponse::NotFound().finish();
    }

    HttpResponse::Ok().json(serde_json::json!({
        "count": count,
        "pages": pages,
        "articles": articles,
    }))
}

pub async fn get_article(pool: web::Data<PgPool>, slug: web::Path<String>) -> impl Responder {
    let article = match sqlx::query_as!(
        ArticleInDB,
        "
        SELECT articles.*, categories.name AS category_name, categories.slug AS category_slug
        FROM articles
        JOIN categories
        ON articles.category_id = categories.id
        WHERE articles.slug = $1;
        ",
        slug.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(article) => {
            if article.is_none() {
                return HttpResponse::NotFound().finish();
            }
            article.unwrap()
        }
        Err(_) => {
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().json(article)
}
