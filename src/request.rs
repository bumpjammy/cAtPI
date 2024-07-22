use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use crate::response::Response;

pub struct Request {
    stream: TcpStream,
    request_string: String
}

impl Request {
    pub fn new(mut stream: TcpStream) -> std::io::Result<Request> {

        let mut reader = BufReader::new(&mut stream);
        let mut request_string = String::new();
        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 || line == "\r\n" {
                break;
            }
            request_string.push_str(&line);
        }

        Ok(Request {
            stream,
            request_string
        })
    }

    pub fn get_type(&self) -> RequestType {
        let parts: Vec<&str> = self.request_string.split_whitespace().collect();
        match parts.first() {
            Some(value) => RequestType::from_value(value),
            None => RequestType::GET
        }
    }

    pub fn get_location(&self) -> &str {
        let parts: Vec<&str> = self.request_string.split_whitespace().collect();
        match parts.get(1) {
            Some(value) => value,
            None => "404",
        }
    }

    pub fn respond(&mut self, response: &mut Response) -> std::io::Result<()> {
        self.stream.write_all(&response.craft_response_headers())?;
        response.stream_body(&mut self.stream)?;
        Ok(())
    }
}


pub enum RequestType {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl RequestType {
    pub fn value(&self) -> &str {
        match self {
            RequestType::GET => {"GET"}
            RequestType::POST => {"POST"}
            RequestType::PUT => {"PUT"}
            RequestType::PATCH => {"PATCH"}
            RequestType::DELETE => {"DELETE"}
        }
    }

    fn from_value(value: &str) -> Self {
        match value {
            "POST" => { RequestType::POST }
            "PUT" => { RequestType::PUT }
            "PATCH" => { RequestType::PATCH }
            "DELETE" => { RequestType::DELETE }
            _ => { RequestType::GET } // Any unknown defaults to a GET request
        }
    }
}
