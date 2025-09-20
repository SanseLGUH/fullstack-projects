use crate::prelude::*;

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