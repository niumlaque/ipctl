# Inter-Process Control
This crate provides functionality to send and receive simple messages using inter-process communication (gRPC).

Of course, when creating a product, you should design it properly without relying on such crates.  
However, defining protobufs and implementing communication for every PoC application is cumbersome, and SIGNAL wasn't satisfying.  
So, I wanted a simple feature that only exchanges strings.

## Usage example
Basically, you will run `ipctl::Server` in your application to listen, and use `ipctl` to send strings.

[This code](/examples/tracing-loglevel/src/main.rs) is a sample that changes the log level at runtime.  
When you execute the following command, logs will be output every 5 seconds:
```sh
$ cargo run --bin tracing-loglevel
```
You can open a new terminal and execute the following command to confirm that the log level has been changed:
```sh
$ cargo run --bin ipctl DEBUG
```
