use std::fs::File;
use std::io::{BufRead, Write};
use std::net::{TcpListener};
use std::{thread};
use std::path::{Path, PathBuf};
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

    let (file, file_name) = safe_open(request.get_location())?;

    let mut response = Response::new_with_file(200, file, file_name.as_str())?;
    request.respond(&mut response)?;

    Ok(())
}

fn safe_open(request_location: &str) -> std::io::Result<(File, String)> {
    let mut file_name = request_location.replacen("/", "", 1);

    if file_name.is_empty() {
        file_name = "index.html".to_string();
    }

    let mut path = PathBuf::from("./public");
    path.push(&file_name);

    if let Ok(canonicalized_path) = path.canonicalize() {
        let public_path = Path::new("./public").canonicalize()?;
        if canonicalized_path.starts_with(public_path) {
            let file = File::open(canonicalized_path)?;
            return Ok((file, file_name.split("/").last().unwrap_or("").to_string()))
        }
    }

    Ok((File::open("./cats/404.png")?, "404.png".to_string()))
}