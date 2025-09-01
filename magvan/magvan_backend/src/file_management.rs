use actix_files::NamedFile;
use actix_web::{
    http::header::{ContentDisposition, DispositionParam, DispositionType},
    web, get, post, HttpResponse, Responder, Result,
};
use serde::Deserialize;
use actix_multipart::form::{text::Text, tempfile::TempFile, MultipartForm};

#[get("/{folder}/{file}")]
pub async fn download(path: web::Path<(String, String)>) -> Result<NamedFile> {
    let (folder, file) = path.into_inner();
    let file_path = format!("./global_files/{}/{}", folder, file);
    
    Ok(NamedFile::open(file_path)?
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![DispositionParam::Filename(file.into())],
        }))
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "200MB")]
    file: TempFile,
    #[multipart]
    json: Text<String>,
}

#[derive(Deserialize)]
struct UploadMeta {
    name: String,
    key: String,
}

#[post("/upload/file")]
pub async fn upload(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    match serde_json::from_str::<UploadMeta>(&form.json) {
        Ok(meta) if meta.key == "AbsoluteSecretKeyMyMan" => {
            let name = form.file.file_name.unwrap_or("unnamed_file".into());
            let path = format!("./global_files/uploaded_files/{}", name);

            match form.file.file.persist(&path) {
                Ok(_) => HttpResponse::Ok().body(format!("File uploaded successfully to {}", path)),
                Err(e) => HttpResponse::InternalServerError()
                    .body(format!("Failed to save file: {}", e)),
            }
        }
        Ok(_) => HttpResponse::Unauthorized().body("Invalid key"),
        Err(_) => HttpResponse::BadRequest().body("Invalid JSON"),
    }
}
