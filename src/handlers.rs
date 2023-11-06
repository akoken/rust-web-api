use axum::http;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateQuote {
    book: String,
    quote: String,
}

pub async fn health() -> http::StatusCode {
    http::StatusCode::OK
}

pub async fn create_quote(
    axum::Json(payload): axum::Json<CreateQuote>,) -> http::StatusCode {
        println!("{:?}", payload);
        http::StatusCode::CREATED
    }