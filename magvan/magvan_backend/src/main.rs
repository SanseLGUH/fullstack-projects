mod file_management;
mod integration_tests;
mod metadatas;
mod websocket;

use actix_web::{
    web, App, HttpServer, HttpResponse, Responder
};

#[actix_web::get("/websites/{website_name}")]
async fn websites(path: web::Path<String>) -> impl Responder  {
    

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(metadatas::manifest)
            .service(metadatas::public_access)
            .service(metadatas::upload)
            .service(file_management::download)
            .service(file_management::upload)
            .route("/ws", web::get().to(websocket::with_any::index))
    })
    .bind("127.0.0.1:4462")?
    .run()
    .await
}
