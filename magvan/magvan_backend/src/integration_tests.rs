use std::sync::Mutex;
use chrono::prelude::*;
use actix_web::{web::Data, get, Responder, HttpResponse, HttpRequest};

#[get("/tests")]
pub async fn tests(req: HttpRequest, agreed_time: Data<Mutex<DateTime<Utc>>>) -> impl Responder {
    if let Some(val) = req.peer_addr() {
        println!("Address {:?}", val.ip());
    };
    HttpResponse::Ok()
        .body( agreed_time.lock().unwrap().to_string() )
}