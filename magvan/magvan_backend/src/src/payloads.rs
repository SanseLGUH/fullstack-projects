use actix_multipart::form::{
    text::Text, tempfile::TempFile, MultipartForm
};
use serde::Deserialize;
use serde_json::Value;

// Used in the /upload_meta endpoint
#[derive(Deserialize, Debug)]
pub struct UploadMetadataRequest {
    pub name: String, 
    pub file: Value,
    pub key: String,
}

// Used in the /upload_file endpoint
#[derive(Debug, MultipartForm)]
pub struct FileUploadForm {
    #[multipart(limit = "600MB")]
    pub file: TempFile,
    #[multipart]
    pub json: Text<String>,
}

#[derive(Deserialize)]
pub struct AdminRequest {
    pub key: String
}

// Used to extract optional website path information
#[derive(Deserialize)]
pub struct WebsitePathParams {
    pub website_name: Option<String>,
    pub page: Option<String>,
}
