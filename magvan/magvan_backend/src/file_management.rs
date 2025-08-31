use actix_files::NamedFile;
use actix_web::{
    http::header::{DispositionParam, ContentDisposition, DispositionType}, 
    Responder, web, get, post, Result
};
use std::path::PathBuf;

#[get("/{foldername}/{filename}")]
pub async fn download_files(path: web::Path<(String, String)>) -> Result<NamedFile> {
    let (foldername, filename) = path.into_inner();

    let filepath: PathBuf = format!("./global_files/{}/{}", foldername, filename).into();

    let file = NamedFile::open(filepath)?;
    
    Ok(file.set_content_disposition(ContentDisposition {
        disposition: DispositionType::Attachment,
        parameters: vec![DispositionParam::Filename(filename.into())],
    }))
}

#[post("/{foldername}/{filename}")]
pub async fn upload_file(path: web::Path<(String, String)>) -> Result<impl Responder> {
    Ok(format!("test"))
}