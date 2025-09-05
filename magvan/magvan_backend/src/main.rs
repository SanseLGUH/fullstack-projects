mod file_management;
mod integration_tests;

use uuid::Uuid;

use actix_web::{
    web::{self, resource}, 
    App, HttpServer, HttpResponse, Responder
};
use std::fs::read_to_string;
use std::path::PathBuf;

async fn website(path: Option<web::Path<(String, Option<String>)>>) -> Result<impl Responder, std::io::Error> {
    let file_path: PathBuf = match path {
        Some(p) => {
            let (main_page, page) = p.into_inner();
            let page = page.unwrap_or_else(|| "index".to_string());
            PathBuf::from(format!("./websites/{}/{}.html", main_page, page))
        },
        None => PathBuf::from("./websites/main_page/index.html"),
    };

    match read_to_string(&file_path) {
        Ok(html) => Ok(
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html)
        ),
        Err(_) => {
            Ok(HttpResponse::NotFound().body("404 - Page not found"))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let admin_key = Uuid::new_v4();

    println!("admin-key: {}", admin_key);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(admin_key.clone()))
            
            .service(
                resource("/").route(web::get().to(website))
            )
            .service(
                resource("/websites/{website_name}").route(web::get().to(website))
            )
            .service(
                resource("/websites/{website_name}/{page}").route(web::get().to(website))
            )
            
            // .service(metadatas::upload)

            .service(file_management::upload)   
            
            .service(
                actix_files::Files::new("/archive-meta", "./manifest")
                    .show_files_listing()
            )
            .service(
                actix_files::Files::new("/archive", "./files")
                    .show_files_listing()
                    .use_last_modified(true)
            )
            
            .service(integration_tests::tests)
    })
    .bind("178.250.187.252:4462")?
    .run()
    .await
}
