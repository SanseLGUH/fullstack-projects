use std::net::TcpStream;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:4462")?;

    loop {

        // Buffer to hold the data
        let mut buffer = [0; 1]; // adjust size as needed

        // Read data from the stream
        let bytes_read = stream.read(&mut buffer)?;

        // Convert buffer to string and print
        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
        
        println!("Received: {}", response);
    }
    Ok(())
}
