use std::net::TcpStream;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:4462")?;

    loop {
        let mut buffer = [0; 4];

        let bytes_read = stream.read(&mut buffer)?;

        stream.write(b"fds");

        // let response = u32::from_be_bytes(buffer);
        
        println!("Received: {:?}", bytes_read);
    }
    Ok(())
}
