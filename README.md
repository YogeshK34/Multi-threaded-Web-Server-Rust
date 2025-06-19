# Multi-threaded Web Server with HTTP Router in Rust

A high-performance, multi-threaded web server implementation built from scratch in Rust, featuring a custom thread pool and a complete HTTP routing system with support for multiple HTTP methods and JSON APIs.

## 🚀 Features

- **Custom Thread Pool**: Built-in thread pool implementation for efficient resource management
- **HTTP Router**: Full-featured routing system supporting GET, POST, PUT, DELETE methods
- **JSON API Support**: Built-in JSON response handling with proper Content-Type headers
- **Request Parsing**: Complete HTTP request parser with headers and body extraction
- **Concurrent Request Handling**: Multiple requests processed simultaneously
- **Memory Safe**: Leverages Rust's ownership system for thread-safe operations
- **Low-Level Implementation**: Built from scratch using Arc, Mutex, and channels

## 🏗️ Architecture

### HTTP Router System
- **Method-based Routing**: Clean separation of GET, POST, PUT, DELETE endpoints
- **Request Parsing**: Full HTTP/1.1 request parsing with headers and body
- **Response Builder**: Fluent API for building HTTP responses with proper status codes
- **JSON Support**: Built-in JSON response handling with appropriate headers

### Thread Pool Design
- **Worker Threads**: Pre-allocated threads that wait for incoming jobs
- **Job Queue**: Channel-based system for distributing work to available threads
- **Type Erasure**: Uses `Box<dyn FnOnce() + Send + 'static>` for flexible job handling
- **Thread Safety**: Arc and Mutex for safe shared state across threads

### Key Components
- **`ThreadPool`**: Main coordinator managing worker threads
- **`Router`**: HTTP routing system with method-based endpoint registration
- **`HttpRequest`**: Complete HTTP request representation with parsing
- **`HttpResponse`**: Response builder with status codes and headers
- **`Worker`**: Individual thread wrapper with unique ID
- **`Job`**: Type alias for closures that can be executed by workers

## 🛠️ Technical Implementation

### Core Technologies
- **Rust Standard Library**: `std::thread`, `std::sync`, `std::collections`
- **Concurrency Primitives**: Arc (Atomic Reference Counter), Mutex (Mutual Exclusion)
- **Message Passing**: `mpsc::channel` for job distribution
- **Generic Programming**: Flexible closure handling with trait bounds
- **HTTP Protocol**: Custom HTTP/1.1 implementation

### Request Flow
1. TCP connection accepted by main thread
2. Raw HTTP request parsed into `HttpRequest` struct
3. Router matches request method and path to registered handler
4. Handler executed in worker thread from thread pool
5. `HttpResponse` built and sent back to client

## 📁 Project Structure

```
src/
├── main.rs          # HTTP server, routing setup, and connection handling
├── lib.rs           # ThreadPool implementation
├── http.rs          # HTTP request parsing and method definitions
├── response.rs      # HTTP response builder
├── router.rs        # HTTP routing system
├── hello.html       # Success response page
└── error.html       # 404 error page
```

## 🌐 API Endpoints

### Web Pages
- **GET /** - Main welcome page (serves hello.html)
- **GET /sleep** - Slow endpoint (5-second delay) for testing concurrency

### Health Check
- **GET /api/health** - Server health status
  ```json
  {"status": "healthy", "server": "rust-web-server"}
  ```

### User Management API
- **GET /api/users** - Get all users
  ```json
  [
    {"id": 1, "name": "Alice", "email": "alice@example.com"},
    {"id": 2, "name": "Bob", "email": "bob@example.com"}
  ]
  ```

- **POST /api/users** - Create a new user
  ```bash
  curl -X POST http://127.0.0.1:7878/api/users \\
    -H "Content-Type: application/json" \\
    -d '{"name": "Charlie", "email": "charlie@example.com"}'
  ```
  Response:
  ```json
  {"message": "User created successfully", "id": 3}
  ```

- **PUT /api/users/1** - Update user with ID 1
  ```bash
  curl -X PUT http://127.0.0.1:7878/api/users/1 \\
    -H "Content-Type: application/json" \\
    -d '{"name": "Alice Updated", "email": "alice.new@example.com"}'
  ```
  Response:
  ```json
  {"message": "User updated successfully"}
  ```

- **DELETE /api/users/1** - Delete user with ID 1
  ```bash
  curl -X DELETE http://127.0.0.1:7878/api/users/1
  ```
  Response:
  ```json
  {"message": "User deleted successfully"}
  ```

## 🚦 Getting Started

### Prerequisites
- Rust 1.70+ installed
- Cargo package manager

### Installation & Running

1. **Clone the repository**
   ```bash
   git clone https://github.com/YogeshK34/Multi-threaded-Web-Server-Rust.git
   cd Multi-threaded-Web-Server-Rust
   ```

2. **Build the project**
   ```bash
   cargo build --release
   ```

3. **Run the server**
   ```bash
   cargo run
   ```

4. **Test the endpoints**
   
   **Web Interface:**
   - `http://127.0.0.1:7878` - Main page
   - `http://127.0.0.1:7878/sleep` - Slow request test

   **API Testing:**
   ```bash
   # Health check
   curl http://127.0.0.1:7878/api/health
   
   # Get users
   curl http://127.0.0.1:7878/api/users
   
   # Create user
   curl -X POST http://127.0.0.1:7878/api/users \\
     -H "Content-Type: application/json" \\
     -d '{"name": "Test User", "email": "test@example.com"}'
   
   # Update user
   curl -X PUT http://127.0.0.1:7878/api/users/1 \\
     -H "Content-Type: application/json" \\
     -d '{"name": "Updated User", "email": "updated@example.com"}'
   
   # Delete user
   curl -X DELETE http://127.0.0.1:7878/api/users/1
   ```

## 🧪 Usage Example

```rust
use multi_threaded_web_server::ThreadPool;
use std::sync::Arc;

fn main() {
    // Create a thread pool with 4 worker threads
    let pool = ThreadPool::new(4);
    
    // Create router with endpoints
    let router = Arc::new(Router::new()
        .get("/", |_req| {
            HttpResponse::ok()
                .with_body("<h1>Hello, World!</h1>".to_string())
        })
        .post("/api/data", |req| {
            println!("Received: {}", req.body);
            HttpResponse::json(201, "CREATED")
                .with_body(r#"{"status": "created"}"#.to_string())
        }));
    
    // Handle requests concurrently
    for stream in listener.incoming() {
        let router_clone = Arc::clone(&router);
        pool.execute(move || {
            handle_connection(stream.unwrap(), &router_clone);
        });
    }
}
```

## 🔧 Configuration

### Server Configuration
The server runs on `127.0.0.1:7878` by default. To modify:

1. Edit `src/main.rs`
2. Change the listener address:
   ```rust
   let listener = TcpListener::bind("127.0.0.1:YOUR_PORT").unwrap();
   ```

### Thread Pool Size
Modify the thread pool size in `main.rs`:
```rust
let pool = ThreadPool::new(8); // 8 worker threads
```

## 📊 Performance Characteristics

- **Thread Pool Size**: Configurable (default: 4 threads)
- **Memory Usage**: Minimal heap allocation with efficient Arc/Mutex usage
- **Latency**: Low latency due to pre-allocated threads
- **Throughput**: High concurrent request handling capability
- **HTTP Methods**: Full support for GET, POST, PUT, DELETE
- **Request Size**: Configurable buffer size (default: 1024 bytes)

## 🧠 Learning Outcomes

This project demonstrates:
- **Systems Programming**: Low-level thread management and synchronization
- **HTTP Protocol**: Complete HTTP/1.1 request/response handling
- **Rust Ownership**: Safe concurrent programming without data races
- **Design Patterns**: Thread pool and router patterns implementation
- **Network Programming**: TCP socket handling and HTTP parsing
- **Type System**: Advanced generics, trait bounds, and type erasure
- **API Design**: RESTful API design and JSON handling

## 🚀 Advanced Features

### HTTP Router Features
- Method-based routing (GET, POST, PUT, DELETE)
- Request header parsing
- Request body extraction
- JSON response building
- Proper HTTP status codes
- Content-Type handling

### Concurrency Features
- Thread-safe request handling
- Shared router state with Arc
- Worker thread management
- Job queue with channels
- Type-erased closures

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📚 References

- [The Rust Programming Language Book - Chapter 20](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- [HTTP/1.1 Specification - RFC 7230](https://tools.ietf.org/html/rfc7230)
- [Rust std::thread documentation](https://doc.rust-lang.org/std/thread/)
- [Rust std::sync documentation](https://doc.rust-lang.org/std/sync/)
- [Rust std::collections::HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 Author

**Yogesh Khutwad** - [GitHub](https://github.com/YogeshK34)

---

⭐ **Star this repository if you found it helpful!**

## 🔮 Future Enhancements

- [ ] Path parameters support (`/api/users/:id`)
- [ ] Query parameter parsing
- [ ] Middleware system (logging, authentication, CORS)
- [ ] Database integration
- [ ] WebSocket support
- [ ] HTTPS/TLS support
- [ ] Request/Response compression
- [ ] Rate limiting
- [ ] Static file serving
- [ ] Template engine integration
```