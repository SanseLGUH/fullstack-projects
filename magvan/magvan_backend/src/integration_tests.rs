use actix_web::{web, get, Responder};

#[get("/test")]
pub async fn tests() -> impl Responder {
    format!("Hello!")
}