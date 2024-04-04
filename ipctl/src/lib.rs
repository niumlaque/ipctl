/// This crate provides functionality to send and receive simple commands
/// to other applications using Inter-Process Communication (IPC).
/// Using this crate facilitates interaction between applications.
/// The sending application specifies a simple command and sends it to the receiving application.
/// The receiving application then executes the appropriate processing based on the received command.
///
/// # Usage
/// Add dependencies to your `Cargo.toml`
/// ```toml
/// ipctl = "0.1"
/// tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
/// ```
///
/// # Examples
/// TODO:
pub mod error;
mod proto;
mod server;

pub use error::Error;
pub use proto::ipctl::control_client::ControlClient;
pub use proto::ipctl::control_server::{Control, ControlServer};
pub use proto::ipctl::Request;
pub use proto::ipctl::Response;
#[cfg(feature = "tokio")]
pub use server::JoinHandler;
pub use server::Server;
