use crate::prelude::*;

const ARCHIVE_META: &str = "./manifest";
const ARCHIVE: &str = "./files";

#[post("/upload_meta")]
pub async fn upload_meta(payload: web::Json<UploadMetadataRequest>, key: Data<Uuid>) -> Result<impl Responder, std::io::Error> {
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
pub async fn upload_file(
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