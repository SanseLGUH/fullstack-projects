use std::{thread, time::Duration, io::Write, net::{TcpListener, TcpStream}};

struct Position {

}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    
    

    loop {
        
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
