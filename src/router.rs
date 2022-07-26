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

pub trait Handler: 'static {
    fn call(&self);
}

/// Represents a router that can build and handle [Route] handler implementations.
pub trait Router<T: Route> {
    fn insert(&mut self, method: Method, path: &str, handler: impl Handler);
    fn at(&mut self, path: &str) -> &mut T;
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
}

impl Router<NodeRoute> for NodeRouter {
    fn at(&mut self, path: &str) -> &mut NodeRoute {
        if let None = self.route.get_mut(path) {
            let node = NodeRoute::default();
            self.route.insert(path, node);
        }

        self.route.get_mut(path).unwrap()
    }

    fn insert(&mut self, method: Method, path: &str, handler: impl Handler) {
        if let Some(node) = self.route.get_mut(path) {
            node.method(method, handler);
        } else {
            let mut node = NodeRoute::default();
            node.method(method, handler);
            self.route.insert(path, node);
        }
    }

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
