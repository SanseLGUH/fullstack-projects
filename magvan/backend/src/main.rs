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

// ==== Timer-related services and scheduling ====
mod timer;

// ==== Feature modules (currently disabled) ====
// mod code_executer;   // Code execution sandbox
// mod voice_chat;      // Real-time voice communication
// mod live_stream;     // Live streaming support

// ==== Integration tests module (for end-to-end testing) ====
mod integration_tests;

// ==== Common imports re-exported for convenience ====
mod prelude;

use crate::prelude::*;

#[tokio::main]
async fn main() -> Result<(), ActixError> {
    // Collect command-line arguments (e.g., [discord_webhook, ...])
    let args: Vec<String> = env::args().collect();

    // Generate a random admin key for secure access control
    let admin_key = Data::new(Uuid::new_v4());

    // Shared timer storage: holds timers scheduled by users during the session
    // Wrapped in a Mutex for safe concurrent access across threads
    let agreed_times: Data<Mutex<HashMap<String, ScheduledTimer>>> = Data::new(Mutex::new(HashMap::new()));

    // Setup recurring or scheduled tasks based on the agreed_time list (uncomment when needed)
    // SetupTimer(Data::clone(&agreed_times)).await;

    // Connect to a Redis Cluster for storing timers persistently
    // These timers are stored per-user for retrieval and verification purposes
    // Format: key (user ID or token) → value (e.g., { started_time, target_time, ... })
    // Make sure Redis is running with cluster support: `redis-server --cluster-enabled yes`
    let redis_client = ClusterClient::new(vec!["redis://127.0.0.1/"]);

    // Create a shared, asynchronous Redis connection wrapped in a Mutex
    let redis_con: Data<Mutex<ClusterConnection>> = Data::new(Mutex::new(
        redis_client.get_async_connection().await?
    ));

    let resp = reqwest::Client::new()
        .post(&args[1])
        .json(&Message::new(&admin_key.to_string()))
        .send()
        .await?;

    println!(
        "response from [current_time, webhook, postgres]: [ {:?} | {:?} | Unsupported ]", 
        Utc::now(), 
        resp.status()
    );

    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::PayloadConfig::new(3_291_456))
            .app_data(Data::clone(&admin_key))
            .app_data(Data::clone(&agreed_times))
            .app_data(Data::clone(&redis_con))

            // timer
            .service(current_agreed)
            .service(update_chrono_timer)

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
            .service(integration_tests::tests)
    })
    .bind("127.0.0.1:4462")?
    .run()
    .await?)
}
