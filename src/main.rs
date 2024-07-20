use std::fs::File;
use std::io::{BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::request::Request;
use crate::response::Response;

mod response;
mod request;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(|| {
            if let Err(e) = handle_request(stream) {
                eprintln!("Failed to handle request: {}", e);
            }
        });
    }

    Ok(())
}

fn handle_request(mut stream: TcpStream) -> std::io::Result<()> {
    let request = Request::new(&stream)?;

    if request.get_location() == "/favicon.ico" {
        let response = Response::new(204);
        stream.write_all(&response.craft_response_headers())?;
        return Ok(());
    }

    let file = File::open("./cats/cat.jpg")?;
    let mut response = Response::new_with_file(200, "image/jpeg", file, "cat")?;
    stream.write_all(&response.craft_response_headers())?;
    response.stream_body(&mut stream)?;

    Ok(())
}

