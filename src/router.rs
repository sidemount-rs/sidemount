use std::{collections::HashMap, sync::Arc};

use crate::node::Node;

#[derive(PartialEq, Eq, Hash)]
pub enum Method {
    /// The GET method requests transfer of a current selected representation for the
    /// target resource. GET is the primary mechanism of information retrieval and the
    /// focus of almost all performance optimizations. Hence, when people speak of
    /// retrieving some identifiable information via HTTP, they are generally referring
    /// to making a GET request.
    /// [ref](https://httpwg.org/specs/rfc7231.html#GET)
    GET,
    /// The POST method requests that the target resource process the representation
    /// enclosed in the request according to the resource's own specific semantics.
    /// For example, POST is used for the following functions (among others):
    ///
    /// * Providing a block of data, such as the fields entered into an HTML form, to a data-handling process;
    /// * Posting a message to a bulletin board, newsgroup, mailing list, blog, or similar group of articles;
    /// * Creating a new resource that has yet to be identified by the origin server; and
    /// * Appending data to a resource's existing representation(s).
    ///
    /// An origin server indicates response semantics by choosing an appropriate status
    /// code depending on the result of processing the POST request; almost all of the
    /// status codes defined by this specification might be received in a response to
    /// POST (the exceptions being 206 (Partial Content), 304 (Not Modified), and
    /// 416 (Range Not Satisfiable)).
    /// [ref](https://httpwg.org/specs/rfc7231.html#POST)
    POST,
    PUT,
    DELETE,
    UNSUPPORTED,
}

impl From<&str> for Method {
    fn from(method: &str) -> Self {
        match method {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            _ => Method::UNSUPPORTED,
        }
    }
}

pub enum RouteResult<T> {
    NotFound,
    MethodNotAllowed,
    Found(T),
}

impl<T> RouteResult<T> {
    pub fn is_found(&self) -> bool {
        match self {
            RouteResult::Found(_) => true,
            _ => false,
        }
    }

    pub fn is_not_allowed(&self) -> bool {
        match self {
            RouteResult::MethodNotAllowed => true,
            _ => false,
        }
    }
}

pub trait Handler: 'static {
    fn call(&self);
}

/// Represents a router that can build and handle [Route] handler implementations.
pub trait Router<T: Route> {
    fn at(&mut self, path: &str) -> &mut T;
    fn mount(&mut self, handler: impl Handler);
    fn route(&mut self, path: &str, router: Self);
    fn insert(&mut self, method: Method, path: &str, handler: impl Handler);
    fn find(&self, path: &str, method: Method) -> RouteResult<&dyn Handler>;
}

/// Represents a route builder that keys off of HTTP methods.
pub trait Route {
    fn method(&mut self, method: Method, handler: impl Handler);
    fn all(&mut self, handler: impl Handler);
    fn get(&mut self, handler: impl Handler);
    fn post(&mut self, handler: impl Handler);
    fn put(&mut self, handler: impl Handler);
    fn delete(&mut self, handler: impl Handler);
}

#[derive(Default)]
pub struct NodeRoute {
    methods: HashMap<Method, Arc<dyn Handler>>,
    _all: Option<Arc<dyn Handler>>,
}

impl Route for NodeRoute {
    fn method(&mut self, method: Method, handler: impl Handler) {
        self.methods.insert(method, Arc::new(handler));
        self._all = None;
    }
    fn all(&mut self, handler: impl Handler) {
        self.methods.clear();
        self._all = Some(Arc::new(handler));
    }
    fn get(&mut self, handler: impl Handler) {
        self.method(Method::GET, handler);
    }
    fn post(&mut self, handler: impl Handler) {
        self.method(Method::POST, handler);
    }
    fn put(&mut self, handler: impl Handler) {
        self.method(Method::PUT, handler);
    }
    fn delete(&mut self, handler: impl Handler) {
        self.method(Method::DELETE, handler);
    }
}

pub struct NodeRouter {
    route: Node<NodeRoute>,
    middleware: Arc<Vec<Arc<dyn Handler>>>,
}

impl NodeRouter {
    pub fn new() -> Self {
        NodeRouter {
            route: Node::default(),
            middleware: Arc::new(Vec::new()),
        }
    }
}

impl Router<NodeRoute> for NodeRouter {
    /// Creates a new node route or returns a mutable reference to an existing one.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// fn test() {}
    ///
    /// let mut router = NodeRouter::new();
    /// router.at("/foo").get(test);
    /// ```
    fn at(&mut self, path: &str) -> &mut NodeRoute {
        if let None = self.route.get_mut(path) {
            let node = NodeRoute::default();
            self.route.insert(path, node);
        }

        self.route.get_mut(path).unwrap()
    }

    /// Mounts a handler implementation as middleware to be optionally executed with
    /// each of the routes once a route has been found.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// fn test() {}
    /// fn test2() {}
    /// fn index() {}
    ///
    /// let mut router = NodeRouter::new();
    /// router.mount((test, test2));
    /// router.at("/foo").get(index);
    /// ```
    fn mount(&mut self, handler: impl Handler) {
        if let Some(mid) = Arc::get_mut(&mut self.middleware) {
            mid.push(Arc::new(handler));
        }
    }

    /// Inserts a route handler for the given path and HTTP method
    ///
    /// ## Examples
    /// ```rust
    /// fn test() {}
    /// fn index() {}
    ///
    /// let mut router = NodeRouter::new();
    /// router.insert(Method::GET, "/foo/bar", (test, index));
    /// ```
    fn insert(&mut self, method: Method, path: &str, handler: impl Handler) {
        if let Some(node) = self.route.get_mut(path) {
            node.method(method, handler);
        } else {
            let mut node = NodeRoute::default();
            node.method(method, handler);
            self.route.insert(path, node);
        }
    }

    /// Routes a path on the router to an existing router implementation.
    ///
    /// ## Examples
    /// ```rust
    /// fn security() {}
    /// fn settings() {}
    /// fn authenticated() {}
    ///
    /// let mut router = NodeRouter::new();
    ///
    /// let mut manager = NodeRouter::new();
    /// manager.mount(authenticated);
    /// manager.at("/settings").get(settings);
    /// manager.at("/security").get(security);
    ///
    /// router.at("/admin").route(manager);
    /// ```
    fn route(&mut self, path: &str, router: NodeRouter) {
        self.route.insert_node(path, router.route);
    }

    /// Finds a route result along the given path and method.
    ///
    /// ## Examples
    /// ```rust
    /// fn index() {}
    /// fn foo() {}
    ///
    /// let mut router = NodeRouter::new();
    /// router.at("/foo/bar").get(index);
    /// router.at("/foo").get(foo);
    ///
    /// assert!(!router.find("/foo/bar/bas", Method::GET).is_found());
    /// assert!(router.find("/foo/bar", Method::GET).is_found());
    /// assert!(router.find("/foo", Method::GET).is_found());
    /// assert!(router.find("/foo", Method::POST).is_not_allowed());
    /// ```
    fn find(&self, path: &str, method: Method) -> RouteResult<&dyn Handler> {
        if let Some(node) = self.route.get(path) {
            if let Some(handler) = &node._all {
                RouteResult::Found(&**handler)
            } else if let Some(handler) = node.methods.get(&method) {
                RouteResult::Found(&**handler)
            } else {
                RouteResult::MethodNotAllowed
            }
        } else {
            RouteResult::NotFound
        }
    }
}

/// Default handler implementation for a function
///
/// ## Examples
/// ```rust
/// fn assert_impl_handler(f: impl Handler) {}
///
/// fn test() {}
/// fn test2() {}
///
/// assert_impl_handler(test);
/// assert_impl_handler(test2);
/// ```
impl<Func> Handler for Func
where
    Func: Fn() + 'static,
{
    fn call(&self) {
        (self)();
    }
}

/// Default handler implementation for a tuple of handlers.
///
/// ## Examples
/// ```rust
/// fn assert_impl_handler(f: impl Handler) {}
///
/// fn test() {}
/// fn test2() {}
///
/// assert_impl_handler((test, test2));
/// ```
impl<A, B> Handler for (A, B)
where
    A: Handler,
    B: Handler,
{
    fn call(&self) {
        self.0.call();
        self.1.call();
    }
}

/// Default handler implementation for an Arc handler.
///
/// ## Examples
/// ```rust
/// fn assert_impl_handler(f: impl Handler) {}
///
/// fn test() {}
///
/// assert_impl_handler(Arc::new(test));
/// ```
impl Handler for Arc<dyn Handler> {
    fn call(&self) {
        self.as_ref().call();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tester() {}

    fn tester2() {}

    #[test]
    fn test_router() {
        let mut router = NodeRouter::new();
        router.mount((tester, tester2));
        router.mount(tester2);
        router.at("/foo/bar").get(tester);
        router.at("/foo/bar/baz").get((tester, tester2));

        let mut sub_router = NodeRouter::new();
        sub_router.at("/bleh").get(tester);
        sub_router.at("/foo/bar").post(tester);
        router.route("/hi", sub_router);

        assert!(router.find("/hi/bleh", Method::GET).is_found());
        assert!(router.find("/hi/foo/bar", Method::POST).is_found());
        assert!(router.find("/foo/bar", Method::GET).is_found());
        assert!(router.find("/foo/bar/baz", Method::GET).is_found());
    }
}
