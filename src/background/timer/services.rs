use crate::prelude::*;

#[post("/set_timer")]
pub async fn timer_new(payload: Json<TimerRequest>, times: Data<Mutex<HashMap<String, ScheduledTimer>>>) -> impl Responder {
    let mut timer = times.lock().await;

    if payload.recipients.len() >= 25 {
        return HttpResponse::TooManyRequests() 
            .body("Recipients reached limit! limit is 25..");
    }

    if timer.len() >= 20 {
        return HttpResponse::TooManyRequests()
            .body("Max timers is reached! 20/20");
    }

    let resp = Client::new()
        .get("https://discord.com/api/v9/users/@me")
        .header("Authorization", &payload.auth_token)
        .send().await.unwrap();

    if resp.status() == StatusCode::from_u16(401).unwrap() {
        return HttpResponse::Unauthorized()
            .body("Invalid discord token");
    }

    match ScheduledTimer::from_timerequest(payload.clone(), Utc::now()) {
        Ok(parsed) => {
            timer.insert( 
                payload.password.clone(),
                parsed
            );

            HttpResponse::Ok()
                .body("Timer started")
        }
        Err(e) => HttpResponse::BadRequest().body( format!("{:?}", e) )
    }
}

#[get("/timers")]
pub async fn current_agreed(
    agreed_time: Data<Mutex<HashMap<String, ScheduledTimer>>>,
    req: HttpRequest,
    key: Data<Mutex<Uuid>>
) -> impl Responder {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    if auth_header.is_none() {
        return HttpResponse::BadRequest()
            .body("Missing Authorization header");
    }

    let key = key.lock().await;

    if auth_header.unwrap() != key.to_string() {
        return HttpResponse::Unauthorized()
            .body("Invalid admin key!");
    }

    let agreed_time = agreed_time.lock().await;

    HttpResponse::Ok()
        .body( format!("{:?}", agreed_time) )
}