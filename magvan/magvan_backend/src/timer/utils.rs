use actix_web::{web::Data, get, HttpResponse, Responder};

use reqwest::Client;
use chrono::prelude::*;

use crate::{
	timer::CHANNELS, payloads::Message
};

pub fn check_time(agreed_time: &DateTime<Utc>) -> bool {
	*agreed_time <= Utc::now()
}

pub async fn send_message(token: &str, to_who: &Vec<String>, payload: Message) {
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

