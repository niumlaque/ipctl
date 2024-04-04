use std::str::FromStr;
use std::thread;
use std::time::Duration;
use tracing_subscriber::{filter, fmt, prelude::*, reload};

/// First, start this sample with the following command:
/// ```sh
/// $ cargo run --bin tracing-loglevel
/// ```
/// Then, send a log level from ipctl:
/// ```sh
/// $ cargo run --bin ipctl TRACE
/// ```
/// You can change the log level on-the-fly.
#[tokio::main]
async fn main() {
    let filter = filter::LevelFilter::INFO;
    let (filter, reload_handle) = reload::Layer::new(filter);
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::default())
        .init();

    let addr = "127.0.0.1:60001".parse().unwrap();
    let handler = ipctl::Server::new(move |x: &str| {
        if let Ok(level) = tracing::Level::from_str(x) {
            reload_handle.modify(|y| *y = level.into()).unwrap();
            format!("Log level is changed to {level}")
        } else {
            format!("Failed to convert {x} to log level")
        }
    })
    .spawn_and_serve(addr);

    for _ in 0..10 {
        tracing::trace!("TRACE");
        tracing::debug!("DEBUG");
        tracing::info!("INFO");
        tracing::warn!("WARNING");
        tracing::error!("ERROR");
        thread::sleep(Duration::from_secs(5));
    }

    handler.join().await.unwrap();
}
