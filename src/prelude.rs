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
pub use smart_default::SmartDefault;

// ==== Reqwest ====
pub use reqwest::{Client, StatusCode};

// ==== Crate Internals ====
pub use crate::{
    BackgroundData,
    error::ActixError,
    payloads::{AdminRequest, WebsitePathParams, Message, FileUploadForm, UploadMetadataRequest},
    admin::reset_key,
    files::{upload_file, upload_meta},
    html::website,
    background::{ 
        background_main,
        timer::{setup as SetupTimer, ScheduledTimer, TimerRequest, services::*}
    },
};
