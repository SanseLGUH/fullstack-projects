use actix_web::{
    post, get, web, Responder, HttpResponse
};
use std::path::PathBuf;

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct UploadMeta {
    name: String,
    file: Value
}

#[post("/upload/metadata")]
pub async fn upload(payload: web::Json<UploadMeta>) -> impl Responder {
    let filepath: PathBuf = format!("./manifest/").into();
    

    HttpResponse::ServiceUnavailable()
        .body(format!("you sended: {:?}", payload))
}

#[get("/manifest")]
pub async fn manifest() -> Result<impl Responder, std::io::Error> {
    let response = std::fs::read_dir(format!(".\\manifest"))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    Ok(web::Json(response))
} 

#[get("/manifests/{meta}")]
pub async fn public_access(path: web::Path<String>) -> Result<impl Responder, std::io::Error> {
    let filename = path.into_inner();
    let filepath: PathBuf = format!("./manifest/{}", filename).into();

    let json_string = std::fs::read_to_string(filepath)?;
    let response: Value = serde_json::from_str(&json_string)?;

    Ok(web::Json(response))
}