use std::net::UdpSocket;
use std::io;

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?; // Bind to a local address and port
    println!("UDP server listening on 127.0.0.1:8080");

    let mut buf = [0; 1024]; // Buffer for incoming data

    loop {
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)?; // Receive data
        let received_data = String::from_utf8_lossy(&buf[..number_of_bytes]);
        println!("Received '{}' from {}", received_data, src_addr);

        // Echo the received data back to the sender
        socket.send_to(&buf[..number_of_bytes], src_addr)?;
    }
}