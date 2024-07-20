use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub struct Request {
    request_string: String
}

impl Request {
    pub fn new(stream: &TcpStream) -> std::io::Result<Self> {

        let mut reader = BufReader::new(stream);
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
            request_string
        })
    }

    pub fn get_type(&self) -> RequestType {
        let parts: Vec<&str> = self.request_string.split_whitespace().collect();
        match parts.get(0) {
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