use std::fs::File;
use std::io::{self, Read, Write};
use std::net::TcpStream;

pub enum Body {
    File(File),
    Bytes(Vec<u8>),
}

pub struct Response {
    status_code: u16,
    status_text: String,
    pub headers: Vec<(String, String)>,
    body: Option<Body>,
}

impl Response {
    pub fn new(status_code: u16) -> Self {
        let status_text = get_status_text(status_code);
        let headers: Vec<(String, String)> = vec![
            ("Date".to_string(), "Thu, 01 Jan 1970 00:00:00 GMT".to_string()),
            ("Server".to_string(), "RIIR/0.0.0 (Windows NT)".to_string()),
        ];

        Response {
            status_code,
            status_text,
            headers,
            body: None,
        }
    }

    pub fn new_with_body(status_code: u16, body_content: Vec<u8>, content_type: &str) -> Self {
        let body_size = body_content.len();
        let mut new_response = Self::new(status_code);
        new_response.headers.push(("Content-Type".to_string(), content_type.to_string()));
        new_response.headers.push(("Content-Length".to_string(), body_size.to_string()));
        new_response.body = Some(Body::Bytes(body_content));
        new_response
    }

    pub fn new_with_file(status_code: u16, file: File, file_name: &str) -> io::Result<Self> {
        let file_size = file.metadata()?.len();
        let mut new_response = Self::new(status_code);
        let file_extension = file_name.split(".").last().unwrap_or_else(|| "");

        new_response.headers.push(("Content-Type".to_string(), get_content_type(file_extension).to_string()));
        new_response.headers.push(("Content-Length".to_string(), file_size.to_string()));
        new_response.headers.push(("Content-Disposition".to_string(), format!("inline; filename=\"{}\"", file_name)));
        new_response.body = Some(Body::File(file));
        Ok(new_response)
    }

    pub fn craft_response_headers(&self) -> Vec<u8> {
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);
        for header in &self.headers {
            response.push_str(&format!("{}: {}\r\n", header.0, header.1));
        }
        response.push_str("\r\n");

        response.into_bytes()
    }

    pub fn stream_body(&mut self, stream: &mut TcpStream) -> io::Result<()> {
        if let Some(body) = &mut self.body {
            match body {
                Body::File(file) => {
                    let mut buffer = [0; 8192];
                    loop {
                        let bytes_read = file.read(&mut buffer)?;
                        if bytes_read == 0 {
                            break;
                        }
                        stream.write_all(&buffer[..bytes_read])?;
                    }
                }
                Body::Bytes(bytes) => {
                    stream.write_all(bytes)?;
                }
            }
        }
        Ok(())
    }
}

fn get_status_text(status_code: u16) -> String {
    match status_code {
        200 => "OK".to_string(),
        204 => "No Content".to_string(),
        404 => "Not Found".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn get_content_type(extension: &str) -> &str {
    match extension {
        "html" => "text/html",
        "htm" => "text/html",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "txt" => "text/plain",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "mp3" => "audio/mpeg",
        "mp4" => "video/mp4",
        _ => "application/octet-stream",
    }
}
