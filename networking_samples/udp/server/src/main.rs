use std::net::UdpSocket;
use std::io;

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?; 
    println!("UDP server listening on 127.0.0.1:8080");

    let mut buf = [0; 1024]; 
    loop {
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)?; 
        let received_data = String::from_utf8_lossy(&buf[..number_of_bytes]);
        println!("Received '{}' from {}", received_data, src_addr);

        socket.send_to(&buf[..number_of_bytes], src_addr)?;
    }
}