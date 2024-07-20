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
        let request = Request::new(stream)?;
        thread::spawn(|| {
            if let Err(e) = handle_request(request) {
                eprintln!("Failed to handle request: {}", e);
            }
        });
    }

    Ok(())
}

fn handle_request(mut request: Request) -> std::io::Result<()> {
    if request.get_location() == "/favicon.ico" {
        let mut response = Response::new(204);
        request.respond(&mut response)?;
        return Ok(());
    }

    let file = File::open("./cats/cat.jpg")?;
    let mut response = Response::new_with_file(200, "image/jpeg", file, "cat")?;
    request.respond(&mut response)?;

    Ok(())
}

