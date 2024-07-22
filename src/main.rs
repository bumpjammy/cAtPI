#![allow(dead_code)]

use std::fs::File;
use std::net::TcpListener;
use std::{fs, thread};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::request::{Request, RequestType};
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
    let (mut response, file, file_name);

    if matches!(request.get_type(), RequestType::GET) {
        if request.get_location() == "/cat" {
            (file, file_name) = get_random_kitty()?;
        } else {
            (file, file_name) = safe_open(request.get_location())?;
        }
        response = Response::new_with_file(200, file, file_name.as_str())?;
        request.respond(&mut response)?;
    }

    Ok(())
}

fn safe_open(request_location: &str) -> std::io::Result<(File, String)> {
    let mut file_name = request_location.replacen('/', "", 1);

    if file_name.is_empty() {
        file_name = "index.html".to_string();
    }

    let mut path = PathBuf::from("./public");
    path.push(&file_name);

    if let Ok(canonicalized_path) = path.canonicalize() {
        let public_path = Path::new("./public").canonicalize()?;
        if canonicalized_path.starts_with(public_path) {
            let file = File::open(canonicalized_path)?;
            return Ok((file, file_name.split('/').last().unwrap_or("").to_string()))
        }
    }

    Ok((File::open("./public/404.png")?, "404.png".to_string()))
}

fn get_random_kitty() -> std::io::Result<(File, String)>{
    let mut cats = Vec::new();

    let paths = fs::read_dir("./public/cats")?;
    for path in paths {
        let file_name = path?.file_name();
        if let Some(file_name) = file_name.to_str() {
            if file_name.contains("README") { continue }
            cats.push(file_name.to_string());
        }
    }

    let rand_num = basic_get_rand(cats.len());
    if let Some(the_chosen_one) = cats.get(rand_num) {
        safe_open(format!("/cats/{}", the_chosen_one).as_str())
    } else {
        safe_open("/404.png")
    }
}

fn basic_get_rand(max: usize) -> usize {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    time.as_nanos() as usize % max
}
