// ==== Actix Web & Multipart ====
pub use actix_web::{
    App, HttpServer, HttpResponse, Responder, HttpRequest,
    web::{self, Data, Json, resource},
    get, post,
};
pub use actix_multipart::form::{
    MultipartForm, text::Text, tempfile::TempFile,
};

// ==== Redis ====
pub use redis::{
    AsyncCommands,
    cluster::ClusterClient,
    cluster_async::ClusterConnection,
};

// ==== Tokio ====
pub use tokio::sync::Mutex;

// ==== Chrono ====
pub use chrono::prelude::*;

// ==== UUID ====
pub use uuid::Uuid;

// ==== Serde ====
pub use serde::{Deserialize, Serialize};
pub use serde_json::Value;

// ==== Std Lib ====
pub use std::{
    env,
    fs::{read_to_string, File},
    io::{BufWriter, Write},
    path::PathBuf,
    collections::HashMap, sync::Arc,
};

// ==== Reqwest ====
pub use reqwest::Client;

// ==== Crate Internals ====
pub use crate::{
    error::ActixError,
    payloads::{AdminRequest, WebsitePathParams, Message, TimerRequest, FileUploadForm, UploadMetadataRequest},
    admin::reset_key,
    files::{upload_file, upload_meta},
    html::website,
    timer::{setup as SetupTimer, ScheduledTimer, services::*},
};
