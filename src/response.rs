#![allow(dead_code)]
use std::collections::HashMap;

pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    pub fn new(status_code: u16, status_text: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html".to_string());
        
        HttpResponse {
            status_code,
            status_text: status_text.to_string(),
            headers,
            body: String::new(),
        }
    }

    pub fn ok() -> Self {
        Self::new(200, "OK")
    }

    pub fn not_found() -> Self {
        Self::new(404, "NOT FOUND")
    }

    pub fn bad_request() -> Self {
        Self::new(400, "BAD REQUEST")
    }

    pub fn json(status_code: u16, status_text: &str) -> Self {
        let mut response = Self::new(status_code, status_text);
        response.headers.insert("Content-Type".to_string(), "application/json".to_string());
        response
    }

    pub fn with_body(mut self, body: String) -> Self {
        self.body = body.clone();
        self.headers.insert("Content-Length".to_string(), body.len().to_string());
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn to_string(&self) -> String {
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);
        
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        
        response.push_str("\r\n");
        response.push_str(&self.body);
        response
    }
}
