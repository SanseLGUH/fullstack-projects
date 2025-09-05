use actix_web::{
    web, post, HttpResponse, Responder
};
use actix_multipart::form::{
    text::Text, tempfile::TempFile, MultipartForm
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "600MB")]
    file: TempFile,
    #[multipart]
    json: Text<String>,
}

#[derive(Deserialize)]
struct UploadMeta {
    key: String
}

// #[derive(Deserialize, Debug)]
// struct UploadMeta {
//     name: String,
//     file: Value,
//     admin_key: String
// }

// #[post("/upload/metadata")]
// pub async fn upload(payload: web::Json<UploadMeta>, key: web::Data<Uuid>) -> Result<impl Responder, std::io::Error> {
//     if key.to_string() != payload.admin_key {
//         return Ok( HttpResponse::Unauthorized() );
//     }

//     let file = File::create( format!( "./manifest/{}.json", payload.name ) )?;
//     let mut writer = BufWriter::new(file);

//     serde_json::to_writer(&mut writer, &payload.file)?;
//     writer.flush()?;

//     Ok( HttpResponse::Ok() )
// }


#[post("/archive/upload")]
pub async fn upload(
    MultipartForm(form): MultipartForm<UploadForm>,
    admin_key: web::Data<Uuid>,
) -> impl Responder {
    match serde_json::from_str::<UploadMeta>(&form.json) {
        Ok(meta) => {
            if meta.key != admin_key.to_string() {
                return HttpResponse::Unauthorized().body("Invalid key");
            }

            match form.file.file_name {
                Some(file_name) => {
                    match form.file.file.persist(format!("./files/uploaded_files/{}", file_name)) {
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