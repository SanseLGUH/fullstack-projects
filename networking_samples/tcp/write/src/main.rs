use std::{thread, time::Duration, io::{Write, Result}, net::{TcpListener, TcpStream}};
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, Default)]
struct Position {
    y: u32,
    x: u32
}

impl Position {
    fn random() -> Self {
        Self {
            y: rand::random::<u32>(),
            x: rand::random::<u32>()
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    loop {
        let pos = Position::random();
        let encoded = bincode::serialize(&pos).unwrap();

        stream.write_all(&encoded)?;

        thread::sleep(Duration::from_secs(1));
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4462")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected");
                thread::spawn(|| {
                    
                    match handle_client(stream) {
                        Ok(_) => {},
                        Err(e) => {
                            match e.kind() {
                                std::io::ErrorKind::ConnectionReset => { println!("Client disconnected"); },
                                _ => {}
                            }
                        }                    
                    }    

                    // if let Err(e) = handle_client(stream) {
                    //     eprintln!("Error handling client: {:?}", e);
                    // }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
