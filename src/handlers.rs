use axum::{extract, http};
use serde::Deserialize;
use sqlx::{PgPool, pool};

#[derive(Debug, Deserialize)]
pub struct CreateQuote {
    book: String,
    quote: String,
}

pub async fn health() -> http::StatusCode {
    http::StatusCode::OK
}

pub async fn create_quote(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<CreateQuote>,) -> http::StatusCode {
        println!("{:?}", payload);
        http::StatusCode::CREATED
    }