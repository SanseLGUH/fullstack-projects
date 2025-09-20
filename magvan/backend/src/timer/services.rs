use crate::prelude::*;

#[post("/set_timer")]
pub async fn timer_new(payload: Json<TimerRequest>, times: Data<Mutex<HashMap<ScheduledTimer>>>) -> impl Responder {
    let mut timer = times.lock().await;

    timer.insert( 
        "key",
        payload.clone() 
    );

    Json(payload)
}

#[get("/timer/update")]
pub async fn update_chrono_timer(
    timers: Data<Mutex<HashMap<ScheduledTimer>>>, req: actix_web::HttpRequest
    ) -> impl Responder {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());



    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{ content: "something" }"#)
}

#[get("/timer/agreed_time")]
pub async fn current_agreed(agreed_time: Data<Mutex<HashMap<ScheduledTimer>>>) -> impl Responder {
    let agreed_time = agreed_time.lock().await;




    format!("{:?}", agreed_time)
}