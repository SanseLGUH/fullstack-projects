mod code_executer;
mod voice_chat;
mod live_stream;

mod postgres;
mod payloads;

mod integration_tests;

use crate::payloads::{
    UploadMetadataRequest, FileUploadForm,
    AdminRequest, WebsitePathParams
};

use actix_web::{
    web::{self, Data, resource}, post,
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
async fn upload_meta(payload: web::Json<UploadMetadataRequest>, key: Data<Uuid>) -> Result<impl Responder, std::io::Error> {
    if key.to_string() != payload.key {
        return Ok( HttpResponse::Unauthorized() );
    }

    let file = File::create( format!( "{ARCHIVE_META}/{}.json", payload.name ) )?;
    let mut writer = BufWriter::new(file);

    serde_json::to_writer(&mut writer, &payload.file)?;
    writer.flush()?;

    Ok( HttpResponse::Ok() )
}

#[post("/upload_file")]
async fn upload_file(
    MultipartForm(form): MultipartForm<FileUploadForm>,
    key: Data<Uuid>,
) -> impl Responder {
    let meta: AdminRequest = match serde_json::from_str(&form.json) {
        Ok(m) => m,
        Err(_) => return HttpResponse::BadRequest().body("Invalid JSON"),
    };

    if meta.key != key.to_string() {
        return HttpResponse::Unauthorized().body("Invalid key");
    }

    let file_name = match &form.file.file_name {
        Some(name) => name,
        None => return HttpResponse::BadRequest().body("Missing file name"),
    };

    let path = format!("{ARCHIVE}/uploaded_files/{}", file_name);
    match form.file.file.persist(path) {
        Ok(_) => HttpResponse::Ok().body("File uploaded successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to save file: {}", e)),
    }
}

async fn website(
    path: web::Path<WebsitePathParams>
) -> Result<impl Responder, std::io::Error> {
    let page = path.page.clone().unwrap_or_else(|| "index".to_string());
    let website = path.website_name.clone().unwrap_or_else(|| "main_page".to_string());
    let file_path = PathBuf::from(format!("{WEBSITES}/{}/{}.html", website, page));
    
    match read_to_string(&file_path) {
        Ok(html) => Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)),
        Err(_) => Ok(HttpResponse::NotFound().body("404 - Page not found")),
    }
}

#[post("reset_key")]
async fn reset_key(payload: web::Json<AdminRequest>, key: Data<Mutex<Uuid>>) -> impl Responder {
    let mut key = key.lock().unwrap();

    if key.to_string() != payload.key {
        return HttpResponse::Unauthorized();
    }

    *key = Uuid::new_v4();

    println!("admin-key: {}", key);

    HttpResponse::Ok()
}

use std::sync::Mutex;
use sqlx::{Error, Connection, PgConnection, FromRow};
use sqlx::postgres::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let admin_key = Data::new(Mutex::new(Uuid::new_v4()));
    // let pool = Data::new(Mutex::new(PgPool::connect("127.0.0.1").await));
    println!("admin-key: {}", admin_key.lock().unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&admin_key))
            
            .service(reset_key)
            
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