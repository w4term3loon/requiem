use core::fmt;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

struct Socket<'a> {
    host: &'a str,
    port: u16,
}

impl<'a> fmt::Display for Socket<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

fn eat(socket: &Socket) -> std::io::Result<()> {
    let mut stream: TcpStream =
        TcpStream::connect(format!("{}", socket))?;

    // Authentication
    let auth = b"x77\n";
    stream.write_all(auth)?;
    stream.flush()?;
    println!("Sent authentication");

    // Read data from the server
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Received: {}", message);

    // macOS workaround for multiple shutdowns
    match stream.shutdown(Shutdown::Both) {
        Ok(_) => (),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotConnected => return Ok(()),
            _ => return Err(e)
        }
    }
    Ok(())
}

fn serve(socket: &Socket) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("{}", socket))?;
    println!("Server listening on {}", socket);

    // Accept incoming connections (blocking)
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("connection accepted on {}", socket);

                // Authenticate
                let mut buffer = [0; 1024];
                let bytes_read = stream.read(&mut buffer)?;
                let auth = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Authentication: {}", auth);

                // Payload
                stream.write_all(b"payload from the server\n")?;
                stream.flush()?;
                println!("Sent payload!");
            }
            Err(e) => eprintln!("connection failed {}", e),
        }
    }
    Ok(())
}

fn main() {
    let socket: Socket = Socket {
        host: "localhost",
        port: 8080,
    };

    match eat(&socket) {
        Ok(_) => (),
        Err(_) => serve(&socket).expect("wut"),
    }
}
