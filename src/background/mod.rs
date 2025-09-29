pub mod timer;

use crate::prelude::*;

pub async fn background_main(data: Data<Mutex<BackgroundData>>) -> Result<(), ActixError> {
	let mut data = data.lock().await;

	let args: Vec<String> = env::args().collect();

    let webhook = match args.get(1) {
        Some(wh) => wh,
        None => &String::new()
    };

	let resp = match reqwest::Client::new()
        .post( webhook )
        .json( &Message::new(&data.admin_key.to_string()) )
        .send()
        .await {
            Ok(r) => r.status().to_string(),
            Err(_) => String::from("Request Error...")
    };

    println!(
        "response from [current_time, webhook, postgres]: [ {:?} | {} | Unsupported ]", 
        Utc::now(), 
        resp
    );

    let timers = data.timers.clone();

	tokio::task::spawn( async move { SetupTimer(timers).await } );

	loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}