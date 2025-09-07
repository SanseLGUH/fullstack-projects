use actix_multipart::form::{
    text::Text, tempfile::TempFile, MultipartForm
};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct UploadMeta {
    pub name: String, pub file: Value,
    pub admin_key: String
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(limit = "600MB")]
    pub file: TempFile,
    #[multipart]
    pub json: Text<String>,
}

#[derive(Deserialize)]
pub struct UploadFormMeta {
    pub key: String
}

#[derive(Deserialize)]
pub struct WebsitePath {
    pub website_name: Option<String>,
    pub page: Option<String>,
}