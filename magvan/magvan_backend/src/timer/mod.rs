pub const DEFIANT_ACC: &str = "";

pub const CHANNELS: &str = "https://discord.com/api/v9/channels";

pub mod services;
pub mod utils;

use actix_web::web::Data;
use tokio::sync::Mutex;

use crate::{
    payloads::{Message, TimerRequest}, 
    timer::utils::{check_time, send_message}
};

use serde::Deserialize;

pub async fn setup(agreed_time: Data<Mutex< Vec<TimerRequest> >>) {
    let agreed_time = agreed_time.clone();

    tokio::task::spawn(async move {
        loop {
            let agreed_time = agreed_time.lock().await;
            
            for time_payload in &*agreed_time {
                let time = time_payload.scheduled_time.parse::<DateTime<Utc>>().unwrap();
                if check_time(&time) {
                    send_message(&time_payload.auth_token, &time_payload.recipients, Message::new(&time_payload.content)).await;
                }
            }   

            tokio::time::sleep(tokio::time::Duration::from_secs(2000)).await;
        }
    });
}