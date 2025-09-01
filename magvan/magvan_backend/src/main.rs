mod file_management;
mod integration_tests;
mod manifests;
mod websocket;

use actix_web::{
    App, HttpServer, HttpResponse, Responder
};

#[actix_web::get("/websites/{website_name}")]
async fn websites(path: actix_web::web::Path<String>) -> impl Responder  {
    

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(manifests::endpoints)
            .service(manifests::public_access)
            .service(file_management::download_files)
            .service(file_management::upload_file)
            .route("/ws", web::get().to(websocket::with_any::index))
    })
    .bind("127.0.0.1:4462")?
    .run()
    .await
}
