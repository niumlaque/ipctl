use crate::error::Result;
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
