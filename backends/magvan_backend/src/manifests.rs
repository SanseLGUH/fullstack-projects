use actix_web::{
    get, web, Responder
};
use std::path::PathBuf;

use serde_json::Value;

#[get("/manifests")]
pub async fn endpoints() -> Result<impl Responder, std::io::Error> {
    let mut response = std::fs::read_dir(format!(".\\manifests"))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    Ok(web::Json(response))
} 

#[get("/manifests/{maniormeta}")]
pub async fn public_access(path: web::Path<String>) -> Result<impl Responder, std::io::Error> {
    let filename = path.into_inner();
    let filepath: PathBuf = format!("./manifests/{}", filename).into();

    let json_string = std::fs::read_to_string(filepath)?;
    let response: Value = serde_json::from_str(&json_string)?;

    Ok(web::Json(response))
}