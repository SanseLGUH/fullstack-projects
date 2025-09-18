use actix_web::{
    web::{self, Data, resource}, post,
    App, HttpServer, HttpResponse, Responder
};
use uuid::Uuid;
use tokio::sync::Mutex;
use crate::payloads::AdminRequest;

#[post("/reset_key")]
pub async fn reset_key(payload: web::Json<AdminRequest>, key: Data<Mutex<Uuid>>) -> impl Responder {
    let mut key = key.lock().await;

    if key.to_string() != payload.key {
        return HttpResponse::Unauthorized();
    }

    *key = Uuid::new_v4();

    println!("admin-key: {}", key);

    HttpResponse::Ok()
}