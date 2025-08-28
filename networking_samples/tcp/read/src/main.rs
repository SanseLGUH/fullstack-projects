use std::net::TcpStream;
use std::io::{self, Read};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
struct Position {
    y: u32,
    x: u32,
}

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:4462")?;

    loop {
        let mut buffer = [0u8; 8];

        match stream.read_exact(&mut buffer) {
            Ok(_) => {
                match bincode::deserialize::<Position>(&buffer) {
                    Ok(pos) => println!("Received Position: {:?}", pos),
                    Err(e) => eprintln!("Failed to deserialize Position: {}", e),
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::ConnectionReset => {
                eprintln!("Connection reset by server.");
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
