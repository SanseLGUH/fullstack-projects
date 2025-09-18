mod admin;
mod files;
mod timer;
mod code_executer;
mod voice_chat;
mod live_stream;
mod postgres;
mod payloads;
mod error;
mod integration_tests;

use std::path::PathBuf;
use tokio::sync::Mutex;
use chrono::prelude::*;
use uuid::Uuid;

use actix_web::{
    App, HttpServer, HttpResponse, Responder,
    web::{self, Data, resource}
};

use crate::{
    error::ActixError,
    payloads::{WebsitePathParams, Message, TimerRequest},
    admin::reset_key, 
    files::{upload_file, upload_meta},
    timer::services::*,
};

const WEBHOOK: &str = "https://discord.com/api/webhooks/1414535800036528178/h3KROIDPDoDHyiKtg6sCD1kl0FBEqfcRKSDsio9qhRDMM93DZ6zcJlAfOS3oOpFOvPc7";
const WEBSITES: &str = "./websites";

async fn website(
    path: web::Path<WebsitePathParams>
) -> Result<impl Responder, std::io::Error> {
    let page = path.page.clone().unwrap_or_else(|| "index".to_string());
    let website = path.website_name.clone().unwrap_or_else(|| "main_page".to_string());
    let file_path = PathBuf::from(format!("{WEBSITES}/{}/{}.html", website, page));
    
    match std::fs::read_to_string(&file_path) {
        Ok(html) => Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)),
        Err(_) => Ok(HttpResponse::NotFound().body("404 - Page not found")),
    }
}

#[tokio::main]
async fn main() -> Result<(), ActixError> {
    let random_id = Uuid::new_v4();
    let agreed_time: Data<Mutex<Vec<TimerRequest>>> = Data::new(Mutex::new(Vec::new()));

    let resp = reqwest::Client::new()
        .post(WEBHOOK)
        .json(&Message::new(&random_id.to_string()))
        .send()
        .await?;

    println!(
        "response from [current_time, webhook, postgres]: [ {:?} | {:?} | Unsupported ]", 
        Utc::now(), 
        resp.status()
    );

    let admin_key = Data::new(random_id);

    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::PayloadConfig::new(3_291_456))
            .app_data(Data::clone(&admin_key))
            .app_data(Data::clone(&agreed_time))

            .service(current_agreed)
            .service(update_chrono_timer)
            .service(discord_token)
            .service(reset_key)
            
            .service(resource("/").route(web::get().to(website)))
            .service(resource("/websites/{website_name}").route(web::get().to(website)))
            .service(resource("/websites/{website_name}/{page}").route(web::get().to(website)))

            .service(upload_meta)
            .service(upload_file)

            .service(
                actix_files::Files::new("/archive-meta", "./manifest")
                    .show_files_listing()
            )
            .service(
                actix_files::Files::new("/archive", "./files")
                    .show_files_listing()
                    .use_last_modified(true)
            )

            .service(integration_tests::tests)
    })
    .bind("127.0.0.1:4462")?
    .run()
    .await?)
}
