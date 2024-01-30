use actix_web::{get, web, Responder};

#[get("/health")]
pub async fn health() -> impl Responder {
    web::Json("OK".to_string())
}
