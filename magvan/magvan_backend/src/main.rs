mod code_executer;
mod voice_chat;
mod live_stream;

mod postgres;
mod payloads;

mod integration_tests;

use crate::payloads::{
    UploadMeta, UploadForm,
    UploadFormMeta, WebsitePath
};

use actix_web::{
    web::{self, resource}, post,
    App, HttpServer, HttpResponse, Responder
};
use actix_multipart::form::MultipartForm;
use std::io::{BufWriter, Write};
use std::fs::{read_to_string, File};
use std::path::PathBuf;

use uuid::Uuid;

const WEBSITES: &str = "./websites";
const ARCHIVE_META: &str = "./manifest";
const ARCHIVE: &str = "./files";

#[post("/upload_meta")]
pub async fn upload_meta(payload: web::Json<UploadMeta>, key: web::Data<Uuid>) -> Result<impl Responder, std::io::Error> {
    if key.to_string() != payload.admin_key {
        return Ok( HttpResponse::Unauthorized() );
    }

    let file = File::create( format!( "{ARCHIVE_META}/{}.json", payload.name ) )?;
    let mut writer = BufWriter::new(file);

    serde_json::to_writer(&mut writer, &payload.file)?;
    writer.flush()?;

    Ok( HttpResponse::Ok() )
}

#[post("/upload-file")]
pub async fn upload_file(
    MultipartForm(form): MultipartForm<UploadForm>,
    admin_key: web::Data<Uuid>,
) -> impl Responder {
    match serde_json::from_str::<UploadFormMeta>(&form.json) {
        Ok(meta) => {
            if meta.key != admin_key.to_string() {
                return HttpResponse::Unauthorized().body("Invalid key");
            }

            match form.file.file_name {
                Some(file_name) => {
                    match form.file.file.persist(format!("{ARCHIVE}/uploaded_files/{}", file_name)) {
                        Ok(_) => HttpResponse::Ok()
                            .body(format!("File uploaded successfully")),
                        Err(e) => HttpResponse::InternalServerError()
                            .body(format!("Failed to save file: {}", e)),
                    }
                }
                None => {
                    return HttpResponse::BadRequest().body("Missing file name");
                }
            }            
        }
        Err(_) => HttpResponse::BadRequest().body("Invalid JSON"),
    }
}

async fn website(
    path: web::Path<WebsitePath>
) -> Result<impl Responder, std::io::Error> {
    let page = path.page.clone().unwrap_or_else(|| "index".to_string());
    let website = path.website_name.clone().unwrap_or_else(|| "main_page".to_string());
    let file_path = PathBuf::from(format!("{WEBSITES}/{}/{}.html", website, page));
    
    match read_to_string(&file_path) {
        Ok(html) => Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)),
        Err(_) => Ok(HttpResponse::NotFound().body("404 - Page not found")),
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
                resource("/")
                    .route(web::get().to(website))
            )
            .service(
                resource("/websites/{website_name}")
                    .route(web::get().to(website))
            )
            .service(
                resource("/websites/{website_name}/{page}")
                    .route(web::get().to(website))
            )

            .service(upload_meta)

            .service(upload_file)   
            
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
    .bind("0.0.0.0:4462")?
    .run()
    .await
}
 