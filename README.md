
# Rust Async Echo Projects

This repository contains four small Rust-based projects designed to help understand and practice the `async`/`await` concurrency model in Rust. It includes both synchronous and asynchronous echo clients and servers.

---

## 📂 Project Structure

```
.
├── echo-client-std        # A basic synchronous TCP client using Rust std library
├── echo-client-tokio      # An asynchronous TCP client using Tokio
├── pattu                  # A basic synchronous TCP echo server
└── pattu-handler          # An asynchronous server that acts as a proxy to 'pattu'
```

---

## 🧠 Goal

The primary goal of these projects is to get hands-on experience with:
- Writing TCP clients and servers in Rust
- Understanding blocking vs. non-blocking I/O
- Learning how to use `tokio` for asynchronous programming
- Handling concurrent connections with `tokio::spawn`

---

## 📦 Dependencies

- [Tokio](https://crates.io/crates/tokio) (for async projects)
  
You can install dependencies by adding this to your `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

---

## ▶️ How to Run

Make sure you have Rust installed. Then, open separate terminal windows and run the following in order:

### 1. Run the Echo Server (`pattu`)

```bash
cd pattu
cargo run --release -- 500   # Optional delay in ms (default: 0)
```

### 2. Run the Async Echo Handler (`pattu-handler`)

```bash
cd ../pattu-handler
cargo run --release
```

### 3. Run the Async Client (`echo-client-tokio`)

```bash
cd ../echo-client-tokio
cargo run --release
```

### 4. Run the Std Client (`echo-client-std`)

```bash
cd ../echo-client-std
cargo run --release
```

---

## 🧪 Behavior

- `echo-client-std`: Connects directly to `pattu` on port `1234`, sends a message, and receives a response.
- `echo-client-tokio`: Connects to `pattu` on port `8000` asynchronously.
- `pattu`: A basic echo server with optional response delay.
- `pattu-handler`: Acts as a proxy server between client and `pattu`, showcasing nested async TCP calls.

---

## 💡 What You’ll Learn

- Difference between blocking and non-blocking I/O in Rust
- How to use `tokio::spawn` to handle multiple connections
- How to manage TCP streams asynchronously using `AsyncReadExt` and `AsyncWriteExt`
- Basic server-client communication using TCP sockets

---

## 🧹 Cleanup

To clean up all build artifacts:

```bash
cargo clean
```

---

## 🧑‍💻 Author

**Aryan Bhattarai**  
This project was made for learning purposes — feel free to fork and try it yourself!

---

## 📜 License

This project is licensed under the MIT License.
