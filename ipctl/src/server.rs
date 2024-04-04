use crate::error::Result;
use std::future::Future;
use std::net::SocketAddr;
use tonic::{Request, Response};

/// Provides functionality to asynchronously wait for receiving a value
/// and call the specified callback function.
///
/// ```no_run
/// use ipctl;
///
/// #[tokio::main]
/// async fn main() {
///     let addr = "127.0.0.1:60001".parse();
///     ipctl::Server::new(|x| format!("received: {x}"))
///         .serve(addr)
///         .await
///         .unwrap();
/// }
/// ```
pub struct Server<F: Fn(&str) -> String + 'static + Send + Sync> {
    /// The function called then receiving a value.
    callback: F,
}

impl<F: Fn(&str) -> String + 'static + Send + Sync> Server<F> {
    /// Create a new `Server` with a callback function.
    pub fn new(callback: F) -> Self {
        Self { callback }
    }

    /// Listens for incoming connections and
    /// prepares the receiver-side program to accept commands from other applications.
    pub async fn serve(self, addr: SocketAddr) -> Result<()> {
        Ok(tonic::transport::Server::builder()
            .add_service(crate::ControlServer::new(Inner {
                callback: self.callback,
            }))
            .serve(addr)
            .await?)
    }

    /// Listens for incoming connections and
    /// prepares the receiver-side program to accept commands from other applications.
    /// Stops waiting upon receiving a signal.
    pub async fn serve_with_signal<Signal: Future<Output = ()>>(
        self,
        addr: SocketAddr,
        signal: Signal,
    ) -> Result<()> {
        Ok(tonic::transport::Server::builder()
            .add_service(crate::ControlServer::new(Inner {
                callback: self.callback,
            }))
            .serve_with_shutdown(addr, signal)
            .await?)
    }

    /// Spawns a Tokio thread to listen for incoming connections and
    /// prepares the receiver-side program to accept commands from other applications.
    #[cfg(feature = "tokio")]
    pub fn spawn_and_serve(self, addr: SocketAddr) -> JoinHandler {
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let handler = tokio::spawn(async move {
            self.serve_with_signal(addr, async {
                let _ = rx.await;
            })
            .await
        });

        JoinHandler::new(handler, tx)
    }
}

/// A handler for managing the shutdown of the Tokio thread spawned for serving
/// communication between processes.
#[cfg(feature = "tokio")]
pub struct JoinHandler {
    inner: tokio::task::JoinHandle<Result<()>>,
    sender: tokio::sync::oneshot::Sender<()>,
}

#[cfg(feature = "tokio")]
impl JoinHandler {
    fn new(
        inner: tokio::task::JoinHandle<Result<()>>,
        sender: tokio::sync::oneshot::Sender<()>,
    ) -> Self {
        Self { inner, sender }
    }

    /// Sends a shutdown signal to the Tokio thread and waits until it stops.
    pub async fn join(self) -> Result<()> {
        // Sends shutdown signal to the Tokio thread.
        self.sender.send(()).map_err(|_| super::Error::Channel)?;
        // Waits until the Tokio thread stops.
        self.inner.await.map_err(|e| super::Error::JoinError(e))??;
        Ok(())
    }
}

struct Inner<F: Fn(&str) -> String + 'static + Send + Sync> {
    callback: F,
}

#[tonic::async_trait]
impl<F: Fn(&str) -> String + 'static + Send + Sync> crate::Control for Inner<F> {
    async fn call(
        &self,
        req: Request<crate::Request>,
    ) -> std::result::Result<Response<crate::Response>, tonic::Status> {
        let req = req.into_inner();
        let ret = (self.callback)(&req.value);
        Ok(Response::new(crate::Response { value: ret }))
    }
}
