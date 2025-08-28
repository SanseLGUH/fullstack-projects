use std::net::UdpSocket;
use std::io;

use std::thread::sleep;
use std::time::Duration;

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?; 
    let server_addr = "127.0.0.1:8080";

    loop {
        sleep(Duration::from_secs(1));

        let message = "Hello UDP Server!";
        socket.send_to(message.as_bytes(), server_addr)?; 
        println!("Sent '{}' to {}", message, server_addr);

        let mut buf = [0; 1024]; 
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)?;
        let received_data = String::from_utf8_lossy(&buf[..number_of_bytes]);
        println!("Received echo '{}' from {}", received_data, src_addr);        
    }

    Ok(())
}