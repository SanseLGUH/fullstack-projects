use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::thread;
use std::time::Duration;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    loop {
        // Send a simple message every second
        let message = b"Hello from server!\n";
        stream.write_all(message)?;
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
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
