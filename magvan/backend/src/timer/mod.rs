pub const CHANNELS: &str = "https://discord.com/api/v9/channels";

pub struct ScheduledTimer {
    pub token: String,
    pub content: String,
    pub recipients: Vec<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>
}

impl ScheduledTimer {
    fn new(t: String, c: String, r: Vec<String>, s_t: DateTime<Utc>, e_t: DateTime<Utc>, ) -> Self {
        Self { 
            token: t, content: c, recipients: c,
            start_time: s_t, end_time: e_t 
        }
    }
}

pub mod services;

use crate::prelude::*;

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


fn check_time(agreed_time: &DateTime<Utc>) -> bool {
    *agreed_time <= Utc::now()
}

async fn send_message(token: &str, to_who: &Vec<String>, payload: Message) {
    let client = Client::new();

    for id in to_who {
        let resp = client
            .post(format!("{}/{}/messages", CHANNELS, id))
            .header("Authorization", token)
            .json(&payload)
            .send().await;

        println!("{:?}",resp );
    }
}
