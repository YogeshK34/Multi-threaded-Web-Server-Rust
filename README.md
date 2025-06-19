# Multi-threaded Web Server in Rust

A high-performance, multi-threaded web server implementation built from scratch in Rust, featuring a custom thread pool for efficient request handling.

## 🚀 Features

- **Custom Thread Pool**: Built-in thread pool implementation for efficient resource management
- **Concurrent Request Handling**: Multiple requests processed simultaneously
- **Memory Safe**: Leverages Rust's ownership system for thread-safe operations
- **Low-Level Implementation**: Built from scratch using Arc, Mutex, and channels
- **HTTP/1.1 Support**: Basic HTTP request/response handling

## 🏗️ Architecture

### Thread Pool Design
- **Worker Threads**: Pre-allocated threads that wait for incoming jobs
- **Job Queue**: Channel-based system for distributing work to available threads
- **Type Erasure**: Uses `Box<dyn FnOnce() + Send + 'static>` for flexible job handling
- **Thread Safety**: Arc and Mutex for safe shared state across threads

### Key Components
- `ThreadPool`: Main coordinator managing worker threads
- `Worker`: Individual thread wrapper with unique ID
- `Job`: Type alias for closures that can be executed by workers
- Channel-based communication between main thread and workers

## 🛠️ Technical Implementation

### Core Technologies
- **Rust Standard Library**: `std::thread`, `std::sync`
- **Concurrency Primitives**: Arc (Atomic Reference Counter), Mutex (Mutual Exclusion)
- **Message Passing**: `mpsc::channel` for job distribution
- **Generic Programming**: Flexible closure handling with trait bounds

### Thread Pool Flow
1. ThreadPool creates N worker threads on initialization
2. Each worker waits on a shared receiver for incoming jobs
3. `execute()` method converts closures to jobs and sends them via channel
4. First available worker picks up the job and executes it
5. Worker returns to waiting state for next job

## 📁 Project Structure

```
src/
├── main.rs          # HTTP server and request handling
├── lib.rs           # ThreadPool implementation
├── hello.html       # Success response page
└── error.html       # 404 error page
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

4. **Test the server**
   Open your browser and navigate to:
   - `http://127.0.0.1:7878` - Main page
   - `http://127.0.0.1:7878/sleep` - Simulated slow request
   - `http://127.0.0.1:7878/anything` - 404 error page

## 🧪 Usage Example

```rust
use multi_threaded_web_server::ThreadPool;

fn main() {
    // Create a thread pool with 4 worker threads
    let pool = ThreadPool::new(4);
    
    // Execute concurrent tasks
    for i in 0..10 {
        pool.execute(move || {
            println!("Task {} executed by worker thread", i);
        });
    }
}
```

## 🔧 Configuration

The server runs on `127.0.0.1:7878` by default. To modify:

1. Edit `src/main.rs`
2. Change the listener address:
   ```rust
   let listener = TcpListener::bind("127.0.0.1:YOUR_PORT").unwrap();
   ```

## 📊 Performance Characteristics

- **Thread Pool Size**: Configurable (default: 4 threads)
- **Memory Usage**: Minimal heap allocation with efficient Arc/Mutex usage
- **Latency**: Low latency due to pre-allocated threads
- **Throughput**: High concurrent request handling capability

## 🧠 Learning Outcomes

This project demonstrates:
- **Systems Programming**: Low-level thread management and synchronization
- **Rust Ownership**: Safe concurrent programming without data races
- **Design Patterns**: Thread pool pattern implementation
- **Network Programming**: HTTP protocol handling
- **Type System**: Advanced generics and trait bounds

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📚 References

- [The Rust Programming Language Book - Chapter 20](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- [Rust std::thread documentation](https://doc.rust-lang.org/std/thread/)
- [Rust std::sync documentation](https://doc.rust-lang.org/std/sync/)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 Author

**Yogesh Khutwad** - [GitHub](https://github.com/YogeshK34)

---

⭐ **Star this repository if you found it helpful!**
```

This README covers all the important aspects of your ThreadPool implementation and provides a professional overview of your project! Just copy and paste it into your repository. 🚀

