use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::thread;
use std::time::Duration;
// use std::sync::Arc;

mod http;
mod response;
mod router;

use http::HttpRequest;
use response::HttpResponse;
use router::Router;
use multi_threaded_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // Create router with all your endpoints
    let router = std::sync::Arc::new(Router::new()
        .get("/", |_req| {
            let contents = fs::read_to_string("hello.html").unwrap_or_else(|_| {
                "<h1>Welcome to Rust Web Server!</h1>".to_string()
            });
            HttpResponse::ok().with_body(contents)
        })
        .get("/sleep", |_req| {
            thread::sleep(Duration::from_secs(5));
            let contents = "<h1>Slow response completed!</h1>".to_string();
            HttpResponse::ok().with_body(contents)
        })
        .get("/api/health", |_req| {
            HttpResponse::json(200, "OK")
                .with_body(r#"{"status": "healthy", "server": "rust-web-server"}"#.to_string())
        })
        .get("/api/users", |_req| {
            let users = r#"[
                {"id": 1, "name": "Alice", "email": "alice@example.com"},
                {"id": 2, "name": "Bob", "email": "bob@example.com"}
            ]"#;
            HttpResponse::json(200, "OK").with_body(users.to_string())
        })
        .post("/api/users", |req| {
            println!("Received POST data: {}", req.body);
            let response = r#"{"message": "User created successfully", "id": 3}"#;
            HttpResponse::json(201, "CREATED").with_body(response.to_string())
        })
        .put("/api/users/1", |req| {
            println!("Updating user 1 with data: {}", req.body);
            let response = r#"{"message": "User updated successfully"}"#;
            HttpResponse::json(200, "OK").with_body(response.to_string())
        })
        .delete("/api/users/1", |_req| {
            let response = r#"{"message": "User deleted successfully"}"#;
            HttpResponse::json(200, "OK").with_body(response.to_string())
        }));

    println!("Server running on http://127.0.0.1:7878");
    println!("Available endpoints:");
    println!("  GET  /");
    println!("  GET  /sleep");
    println!("  GET  /api/health");
    println!("  GET  /api/users");
    println!("  POST /api/users");
    println!("  PUT  /api/users/1");
    println!("  DELETE /api/users/1");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let router_clone = std::sync::Arc::clone(&router);
        
        pool.execute(move || {
            handle_connection(stream, &router_clone);
        });
    }
}

fn handle_connection(mut stream: TcpStream, router: &Router) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request_string = String::from_utf8_lossy(&buffer[..]);
    
    if let Some(request) = HttpRequest::parse(&request_string) {
        let response = router.handle(&request);
        stream.write(response.to_string().as_bytes()).unwrap();
    } else {
        let response = HttpResponse::bad_request()
            .with_body("<h1>400 - Bad Request</h1>".to_string());
        stream.write(response.to_string().as_bytes()).unwrap();
    }
    
    stream.flush().unwrap();
}
