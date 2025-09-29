pub const CHANNELS: &str = "https://discord.com/api/v9/channels";


// curl -X POST -H "Content-Type: application/json" -d "{\"auth_token\":\"\",\"content\":\"your_content\",\"scheduled_time\":\"2025-09-21T10:00:00Z\",\"recipients\":[\"1407203780843409543\"],\"password\":\"your_password\"}" http://127.0.0.1/set_timer
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimerRequest {
    pub auth_token: String,
    pub content: String,
    pub scheduled_time: String,
    pub recipients: Vec<String>,
    pub password: String
}

#[derive(Debug, Clone)]
pub struct ScheduledTimer {
    pub token: String,
    pub content: String,
    pub recipients: Vec<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>
}

impl ScheduledTimer {
    fn from_timerequest(t_r: TimerRequest, s_t: DateTime<Utc>) -> chrono::format::ParseResult<Self> {
        Ok( Self { 
            token: t_r.auth_token,
            content: t_r.content,
            recipients: t_r.recipients,
            end_time: t_r.scheduled_time.parse::<DateTime<Utc>>()?,
            start_time: s_t
        } )
    }
}

pub mod services;

use crate::prelude::*;

pub async fn setup(timer_map_data: Data<Mutex<HashMap<String, ScheduledTimer>>>) {
    // TODO: This implementation is limited to around 20 concurrent timers
    // due to how many tasks and locks are being spawned each loop iteration.
    // Consider using more scalable synchronization primitives like AtomicCell,
    // DashMap, or a message-passing architecture with Tokio mpsc channels.
    
    loop {
        let timer_map_lock = timer_map_data.lock().await;
        for (timer_id, timer) in timer_map_lock.clone().into_iter() {
            let timer_map_inner = timer_map_data.clone();
            tokio::task::spawn(async move {
                if timer.end_time <= Utc::now() {
                    let mut timer_map = timer_map_inner.lock().await;
                    send_message(
                        &timer.token,
                        &timer.recipients,
                        Message::new(&timer.content)
                    ).await;
                    
                    timer_map.remove(&timer_id);
                }
            });
        }
        tokio::time::sleep(std::time::Duration::from_secs(100)).await;
    }
}

async fn send_message(token: &str, to_who: &Vec<String>, payload: Message) {
    let client = Client::new();

    for id in to_who {
        client
            .post(format!("{}/{}/messages", CHANNELS, id))
            .header("Authorization", token)
            .json(&payload)
            .send().await;


        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }

    // i need to make response for every person who send message 
    // this will need rest api or something
}
