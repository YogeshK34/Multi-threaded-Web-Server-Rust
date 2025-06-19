#![allow(dead_code)]
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl HttpMethod {
    pub fn from_str(method: &str) -> Option<HttpMethod> {
        match method {
            "GET" => Some(HttpMethod::GET),
            "POST" => Some(HttpMethod::POST),
            "PUT" => Some(HttpMethod::PUT),
            "DELETE" => Some(HttpMethod::DELETE),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    pub fn parse(request: &str) -> Option<HttpRequest> {
        let lines: Vec<&str> = request.split("\r\n").collect();
        if lines.is_empty() {
            return None;
        }

        // Parse request line: "GET /path HTTP/1.1"
        let request_line_parts: Vec<&str> = lines[0].split_whitespace().collect();
        if request_line_parts.len() != 3 {
            return None;
        }

        let method = HttpMethod::from_str(request_line_parts[0])?;
        let path = request_line_parts[1].to_string();

        // Parse headers
        let mut headers = HashMap::new();
        let mut body_start = 0;

        for (i, line) in lines.iter().enumerate().skip(1) {
            if line.is_empty() {
                body_start = i + 1;
                break;
            }
            
            if let Some(colon_pos) = line.find(':') {
                let key = line[..colon_pos].trim().to_lowercase();
                let value = line[colon_pos + 1..].trim().to_string();
                headers.insert(key, value);
            }
        }

        // Parse body (for POST requests)
        let body = if body_start < lines.len() {
            lines[body_start..].join("\r\n")
        } else {
            String::new()
        };

        Some(HttpRequest {
            method,
            path,
            headers,
            body,
        })
    }
}
