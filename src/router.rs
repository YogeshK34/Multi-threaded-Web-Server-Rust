#![allow(dead_code)]
use crate::http::{HttpRequest, HttpMethod};
use crate::response::HttpResponse;
use std::collections::HashMap;

type Handler = Box<dyn Fn(&HttpRequest) -> HttpResponse + Send + Sync>;

pub struct Router {
    routes: HashMap<(HttpMethod, String), Handler>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn get<F>(mut self, path: &str, handler: F) -> Self 
    where 
        F: Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static,
    {
        self.routes.insert(
            (HttpMethod::GET, path.to_string()), 
            Box::new(handler)
        );
        self
    }

    pub fn post<F>(mut self, path: &str, handler: F) -> Self 
    where 
        F: Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static,
    {
        self.routes.insert(
            (HttpMethod::POST, path.to_string()), 
            Box::new(handler)
        );
        self
    }

    pub fn put<F>(mut self, path: &str, handler: F) -> Self 
    where 
        F: Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static,
    {
        self.routes.insert(
            (HttpMethod::PUT, path.to_string()), 
            Box::new(handler)
        );
        self
    }

    pub fn delete<F>(mut self, path: &str, handler: F) -> Self 
    where 
        F: Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static,
    {
        self.routes.insert(
            (HttpMethod::DELETE, path.to_string()), 
            Box::new(handler)
        );
        self
    }

    pub fn handle(&self, request: &HttpRequest) -> HttpResponse {
        let key = (request.method.clone(), request.path.clone());
        
        if let Some(handler) = self.routes.get(&key) {
            handler(request)
        } else {
            HttpResponse::not_found()
                .with_body("<h1>404 - Page Not Found</h1>".to_string())
        }
    }
}
