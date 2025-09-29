// ==== Error handling module ====
mod error;

// ==== Admin-related logic (e.g., reset keys, auth) ====
mod admin;

// ==== Request/response payload definitions ====
mod payloads;

// ==== File upload, metadata handling ====
mod files;

// ==== HTML templating or web UI rendering ====
mod html;

// purpose is to use my server like pintest station or to send data trow it.. 
mod background;

// ==== Feature modules (currently disabled) ====
// mod code_executer;   // Code execution sandbox
// mod voice_chat;      // Real-time voice communication
// mod live_stream;     // Live streaming support

// ==== Integration tests module (for end-to-end testing) ====
mod integration_tests;

// ==== Common imports re-exported for convenience ====
mod prelude;

use crate::prelude::*;

#[derive(SmartDefault)]
pub struct BackgroundData {
    #[default(_code = "Data::new( Uuid::new_v4() )")]
    admin_key: Data<Uuid>,
    timers: Data<Mutex<HashMap<String, ScheduledTimer>>>,
    postgres_pool: Data<String>,
    redis_pool: Data<String>
}

#[tokio::main]
async fn main() -> Result<(), ActixError> {
    let background_data = Data::new(Mutex::new( BackgroundData::default() ));

    let b_d_c = background_data.clone();
    tokio::task::spawn( async move { 
        background_main( b_d_c ).await.unwrap(); 
    });

    let timers = background_data.lock().await.timers.clone();

    Ok(HttpServer::new(move || {
         App::new()
            .app_data(web::PayloadConfig::new(3_291_456))
            .app_data(Data::clone(&background_data))
            .app_data(Data::clone(&timers))

            // timer
            .service(timer_new)
            .service(current_agreed)

            // admin key
            .service(reset_key)
            
            // html module: websites 
            .service(resource("/").route(web::get().to(website)))
            .service(resource("/websites/{website_name}").route(web::get().to(website)))
            .service(resource("/websites/{website_name}/{page}").route(web::get().to(website)))

            // file module
            .service(upload_meta)
            .service(upload_file)


            // actix_files + metadatas
            .service(
                actix_files::Files::new("/archive-meta", "./manifest")
                    .show_files_listing()
            )
            .service(
                actix_files::Files::new("/archive", "./files")
                    .show_files_listing()
                    .use_last_modified(true)
            )

            // tests
            .service(integration_tests::state)
    })
    .bind("127.0.0.1:4462")?
    .run()
    .await?)
}
