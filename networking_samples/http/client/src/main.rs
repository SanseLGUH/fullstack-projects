use reqwest::Client;

async fn recv_hello(client: &Client) -> String {
    let resp = client.get("http://127.0.0.1:8080/")
        .send().await.unwrap()
        .text().await.unwrap();

    resp
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    println!("{}", recv_hello(&client).await);
}