use actix_web::{get, web, App, HttpServer, Responder};
use std::str::FromStr;
use tracing_subscriber::{filter, fmt, prelude::*, reload};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let msg = format!("Hello {}!", name);
    tracing::trace!(msg);
    tracing::debug!(msg);
    tracing::info!(msg);
    tracing::warn!(msg);
    tracing::error!(msg);
    msg
}

/// First, start this sample with the following command:
/// ```sh
/// $ cargo run --run with-actix-web
/// ```
/// Then, send a log level from ipctl:
/// ```sh
/// $ cargo run --bin ipctl TRACE
/// ```
/// You can confirm that the log level has changed by executing the following command.
/// ```sh
/// $ curl 127.0.0.1:8080/hello/john
/// ```
#[actix_web::main]
async fn main() {
    let filter = filter::LevelFilter::INFO;
    let (filter, reload_handle) = reload::Layer::new(filter);
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::default())
        .init();

    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let ipctl_handler = actix_web::rt::spawn(async move {
        let addr = "127.0.0.1:60001".parse().unwrap();
        ipctl::Server::new(move |x: &str| {
            if let Ok(level) = tracing::Level::from_str(x) {
                reload_handle.modify(|y| *y = level.into()).unwrap();
                format!("Log level is changed to {level}")
            } else {
                format!("Failed to convert {x} to log level")
            }
        })
        .serve_with_signal(addr, async {
            let _ = rx.await;
        })
        .await
    });

    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await
        .unwrap();
    tx.send(()).unwrap();
    ipctl_handler.await.unwrap().unwrap();
}
