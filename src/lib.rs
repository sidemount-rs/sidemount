mod node;
mod router;

pub use node::*;
pub use router::*;

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
