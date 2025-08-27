use std::net::UdpSocket;
use std::io;

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?; // Bind to a random available local port
    let server_addr = "127.0.0.1:8080"; // Server address and port

    let message = "Hello UDP Server!";
    socket.send_to(message.as_bytes(), server_addr)?; // Send data to the server
    println!("Sent '{}' to {}", message, server_addr);

    let mut buf = [0; 1024]; // Buffer for incoming data
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)?; // Receive data from the server
    let received_data = String::from_utf8_lossy(&buf[..number_of_bytes]);
    println!("Received echo '{}' from {}", received_data, src_addr);

    Ok(())
}