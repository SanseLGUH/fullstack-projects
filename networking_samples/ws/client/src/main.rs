use tungstenite::{connect, Message};

fn main() {
    let (mut socket, response) =
        connect("ws://127.0.0.1:8080/ws/")
        .expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());

    socket
        .write_message(Message::Ping(vec![].into()))
        .expect("Failed to send ping");

    println!("Ping sent!");

    if let Ok(msg) = socket.read_message() {
        println!("Received: {:?}", msg);
    }
}
