use chrono::prelude::*;
use actix_web::{
    web::{Data, Json}, HttpResponse, Responder, get, post
};
use tokio::sync::Mutex;

use crate::{payloads::PasswordAuth, timer::{DEFIANT_ACC, CHANNELS, TimerRequest}};

#[post("/set_timer")]
pub async fn timer_new(payload: Json<TimerRequest>, time_vec: Data<Mutex< Vec<TimerRequest> >>) -> impl Responder {


    HttpResponse::Ok()
}

#[get("/timer/update")]
pub async fn update_chrono_timer(
    agreed_time: Data<Mutex< Vec<DateTime<Utc>> >>, req: actix_web::HttpRequest
    ) -> impl Responder {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());


    HttpResponse::Ok()
}

#[get("/timer/agreed_time")]
pub async fn current_agreed(agreed_time: Data<Mutex<Vec< TimerRequest >>>) -> impl Responder {
    let agreed_time = agreed_time.lock().await;

    format!("{:?}", agreed_time)
}

#[get("/discord/account/DEFIANT")]
pub async fn discord_token() -> impl Responder {
    HttpResponse::Ok()
        .body(DEFIANT_ACC)
}

