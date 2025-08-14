use actix_web::{get, App, HttpServer, HttpResponse};

#[get("/croissant_demo")]
async fn croissant_demo() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../static/index.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(croissant_demo)
    })
    .bind(("127.0.0.1", 4462))?
    .run()
    .await
}
