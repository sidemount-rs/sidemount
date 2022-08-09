#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(trait_alias)]

mod func;
mod node;
mod request;
mod response;
mod router;
mod server;

use std::future::Future;

use async_trait::async_trait;

pub use node::Node;
pub use request::Request;
pub use response::Response;
pub use router::{Route, RouteResult, Router};
pub use server::Server;

pub mod http {
    pub type Request = hyper::Request<hyper::Body>;
    pub type Response = hyper::Response<hyper::Body>;
    pub type Method = hyper::Method;
}
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub type Method = http::Method;

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn call(&self, req: Request) -> Response;
}

#[async_trait]
impl<F, Fut> Handler for F
where
    F: Send + Sync + 'static + Fn(Request) -> Fut,
    Fut: Future<Output = Response> + Send,
{
    async fn call(&self, req: Request) -> Response {
        (self)(req).await
    }
}

/// Creates a new server to process requests on a protocol.
///
/// ## Examples
/// ```ignore
/// fn index() {}
/// fn authorize() {}
///
/// #[tokio::main]
/// async fn main() {
///     let mut app = sidemount::new();
///     app.at("/foo").get(index);
///
///     app.listen("127.0.0.1:7000").await
/// }
/// ```
pub fn new() -> Server {
    Server::new()
}

/// Creates a new [Router] implementation using the default
/// radix tree node router with support for mounting middleware.
///
/// ## Examples
/// ```rust
/// fn index() {}
///
/// let mut router = sidemount::router();
/// router.at("/foo").get(index);
///
/// assert!(router.find("/foo", sidemount::Method::GET).is_found());
/// ```
pub fn router() -> Router {
    Router::new()
}
