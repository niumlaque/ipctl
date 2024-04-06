# Inter-Process Control
This crate provides functionality to send and receive simple messages using inter-process communication (gRPC).

Of course, when creating a product, you should design it properly without relying on such crates.  
However, defining protobufs and implementing communication for every PoC application is cumbersome, and SIGNAL wasn't satisfying.  
So, I wanted a simple feature that only exchanges strings.

## Usage
Basically, you will run `ipctl::Server` in your application to listen, and use `ipctl` to send strings.

### Build the ipctl Client
Clone the repository and build as follows:
```sh
$ git clone https://github.com/niumlaque/ipctl.git
$ cd ipctl
$ cargo build --release --bin ipctl
```

### Build a Sample using ipctl Server
Add the following dependencies to your Cargo.toml:
```toml
[dependencies]
ipctl = { git = "https://github.com/niumlaque/ipctl", branch = "master" }
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
```

Then, execute the following code:
```rs
#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:60001".parse().unwrap();
    ipctl::Server::new(|x| {
        format!("Received: {x}")
    })
    .serve(addr)
    .await
    .unwrap();
}
```
```sh
$ cargo run
```

Afterwards, you can use the previously built ipctl Client for text communication.
```sh
$ ipctl "Hello World"
Received: Hello World
```

### Other examples
[This code](/examples/tracing-loglevel/src/main.rs) is a sample that changes the log level at runtime.  
When you execute the following command, logs will be output every 5 seconds:
```sh
$ cargo run --bin tracing-loglevel
```
You can open a new terminal and execute the following command to confirm that the log level has been changed:
```sh
$ cargo run --bin ipctl DEBUG
```
