mod file_management;
mod integration_tests;
mod manifests;

use actix_web::{
    App, HttpServer
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(manifests::endpoints)
            .service(manifests::public_access)
            .service(file_management::download_files)
    })
    .bind("127.0.0.1:4462")?
    .run()
    .await
}
