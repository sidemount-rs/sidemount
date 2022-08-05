#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(trait_alias)]
#![feature(trivial_bounds)]

mod node;
mod router;
mod server;

pub use node::Node;
pub use router::{Method, Route, RouteResult, Router};
pub use server::Server;

pub type Request = hyper::Request<hyper::Body>;
pub type Response = hyper::Response<hyper::Body>;
pub trait Handler = Fn<(Request,), Output = Response>;

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
