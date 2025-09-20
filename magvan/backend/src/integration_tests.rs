use crate::prelude::*;

#[get("/tests")]
pub async fn tests(req: HttpRequest, agreed_time: Data<Mutex<DateTime<Utc>>>) -> impl Responder {
    if let Some(val) = req.peer_addr() {
        println!("Address {:?}", val.ip());
    };
    HttpResponse::Ok()
        .body( agreed_time.lock().await.to_string() )
}