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
    /// The PUT method requests that the state of the target resource be created or
    /// replaced with the state defined by the representation enclosed in the request
    /// message payload. A successful PUT of a given representation would suggest that
    /// a subsequent GET on that same target resource will result in an equivalent
    /// representation being sent in a 200 (OK) response. However, there is no guarantee
    /// that such a state change will be observable, since the target resource might be
    /// acted upon by other user agents in parallel, or might be subject to dynamic
    /// processing by the origin server, before any subsequent GET is received. A
    /// successful response only implies that the user agent's intent was achieved at
    /// the time of its processing by the origin server.
    /// [ref](https://httpwg.org/specs/rfc7231.html#PUT)
    PUT,
    /// The DELETE method requests that the origin server remove the association between
    /// the target resource and its current functionality. In effect, this method is
    /// similar to the rm command in UNIX: it expresses a deletion operation on the URI
    /// mapping of the origin server rather than an expectation that the previously
    /// associated information be deleted.
    /// [ref](https://httpwg.org/specs/rfc7231.html#DELETE)
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

impl From<&hyper::Method> for Method {
    fn from(m: &hyper::Method) -> Self {
        match m {
            &hyper::Method::GET => Method::GET,
            &hyper::Method::POST => Method::POST,
            &hyper::Method::PUT => Method::PUT,
            &hyper::Method::DELETE => Method::DELETE,
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

pub trait Handler: Sync + Send + 'static {
    fn call(&self);
}

/// Represents a route builder that keys off of HTTP methods.
#[derive(Default)]
pub struct Route {
    methods: HashMap<Method, Arc<dyn Handler>>,
    _all: Option<Arc<dyn Handler>>,
}

impl Route {
    /// Inserts a handler implementation on the given HTTP method.
    pub fn method(&mut self, method: Method, handler: impl Handler) {
        self.methods.insert(method, Arc::new(handler));
    }
    /// Inserts a handler implementation on the all HTTP methods.
    pub fn all(&mut self, handler: impl Handler) {
        self._all = Some(Arc::new(handler));
    }
    /// Inserts a handler implementation on the GET HTTP method.
    pub fn get(&mut self, handler: impl Handler) {
        self.method(Method::GET, handler);
    }
    /// Inserts a handler implementation on the POST HTTP method.
    pub fn post(&mut self, handler: impl Handler) {
        self.method(Method::POST, handler);
    }
    /// Inserts a handler implementation on the PUT HTTP method.
    pub fn put(&mut self, handler: impl Handler) {
        self.method(Method::PUT, handler);
    }
    /// Inserts a handler implementation on the DELETE HTTP method.
    pub fn delete(&mut self, handler: impl Handler) {
        self.method(Method::DELETE, handler);
    }
}

/// Represents a router that can build and handle [Route] handler implementations.
pub struct Router {
    route: Node<Route>,
    middleware: Arc<Vec<Arc<dyn Handler>>>,
}

impl Router {
    /// Creates a new router with the default route and middleware
    pub fn new() -> Self {
        Router {
            route: Node::default(),
            middleware: Arc::new(Vec::new()),
        }
    }

    /// Creates a new node route or returns a mutable reference to an existing one.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use sidemount::*;
    ///
    /// fn test() {}
    ///
    /// let mut router = Router::new();
    /// router.at("/foo").get(test);
    /// ```
    pub fn at(&mut self, path: &str) -> &mut Route {
        if let None = self.route.get_mut(path) {
            let node = Route::default();
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
    /// use sidemount::*;
    ///
    /// fn test() {}
    /// fn test2() {}
    /// fn index() {}
    ///
    /// let mut router = Router::new();
    /// router.mount((test, test2));
    /// router.at("/foo").get(index);
    /// ```
    pub fn mount(&mut self, handler: impl Handler) {
        if let Some(mid) = Arc::get_mut(&mut self.middleware) {
            mid.push(Arc::new(handler));
        }
    }

    /// Inserts a route handler for the given path and HTTP method
    ///
    /// ## Examples
    /// ```rust
    /// use sidemount::*;
    ///
    /// fn test() {}
    /// fn index() {}
    ///
    /// let mut router = Router::new();
    /// router.insert(Method::GET, "/foo/bar", (test, index));
    /// ```
    pub fn insert(&mut self, method: Method, path: &str, handler: impl Handler) {
        if let Some(node) = self.route.get_mut(path) {
            node.method(method, handler);
        } else {
            let mut node = Route::default();
            node.method(method, handler);
            self.route.insert(path, node);
        }
    }

    /// Routes a path on the router to an existing router implementation.
    ///
    /// ## Examples
    /// ```rust
    /// use sidemount::*;
    ///
    /// fn security() {}
    /// fn settings() {}
    /// fn authenticated() {}
    ///
    /// let mut router = Router::new();
    ///
    /// let mut manager = Router::new();
    /// manager.mount(authenticated);
    /// manager.at("/settings").get(settings);
    /// manager.at("/security").get(security);
    ///
    /// router.route("/admin", manager);
    /// ```
    pub fn route(&mut self, path: &str, router: Router) {
        self.route.insert_node(path, router.route);
    }

    /// Finds a route result along the given path and method.
    ///
    /// ## Examples
    /// ```rust
    /// use sidemount::*;
    ///
    /// fn index() {}
    /// fn foo() {}
    ///
    /// let mut router = Router::new();
    /// router.at("/foo/bar").get(index);
    /// router.at("/foo").get(foo);
    ///
    /// assert!(!router.find("/foo/bar/bas", Method::GET).is_found());
    /// assert!(router.find("/foo/bar", Method::GET).is_found());
    /// assert!(router.find("/foo", Method::GET).is_found());
    /// assert!(router.find("/foo", Method::POST).is_not_allowed());
    /// ```
    pub fn find(&self, path: &str, method: Method) -> RouteResult<&dyn Handler> {
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
/// use sidemount::Handler;
///
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
    Func: Fn() + Send + Sync + 'static,
{
    fn call(&self) {
        (self)();
    }
}

/// Default handler implementation for a tuple of handlers.
///
/// ## Examples
/// ```rust
/// use sidemount::Handler;
///
/// fn assert_impl_handler(f: impl Handler) {}
///
/// fn test() {}
/// fn test2() {}
///
/// assert_impl_handler((test, test2));
/// ```
macro_rules! ary {
    ($($name:ident)+) => (
        impl<$($name),*> Handler for ($($name,)*)
            where $($name: Handler),*
        {
            #[allow(non_snake_case)]
            fn call(&self) {
                let ($(ref $name,)*) = *self;
                $(
                    $name.call();
                )*
            }
        }
    );
}

ary! { A B }
ary! { A B C }
ary! { A B C D }
ary! { A B C D E }
ary! { A B C D E F }
ary! { A B C D E F G }
ary! { A B C D E F G H }
ary! { A B C D E F G H I }
ary! { A B C D E F G H I J }
ary! { A B C D E F G H I J K }
ary! { A B C D E F G H I J K L }
ary! { A B C D E F G H I J K L M }
ary! { A B C D E F G H I J K L M N }
ary! { A B C D E F G H I J K L M N O }
ary! { A B C D E F G H I J K L M N O P }
ary! { A B C D E F G H I J K L M N O P Q }
ary! { A B C D E F G H I J K L M N O P Q R }
ary! { A B C D E F G H I J K L M N O P Q R S }
ary! { A B C D E F G H I J K L M N O P Q R S T }
ary! { A B C D E F G H I J K L M N O P Q R S T U }
ary! { A B C D E F G H I J K L M N O P Q R S T U V }
ary! { A B C D E F G H I J K L M N O P Q R S T U V W }
ary! { A B C D E F G H I J K L M N O P Q R S T U V W X }
ary! { A B C D E F G H I J K L M N O P Q R S T U V W X Y }
ary! { A B C D E F G H I J K L M N O P Q R S T U V W X Y Z }

/// Default handler implementation for an Arc handler.
///
/// ## Examples
/// ```rust
/// use std::sync::Arc;
/// use sidemount::Handler;
///
/// fn assert_impl_handler(f: impl Handler) {}
///
/// fn test() {}
///
/// assert_impl_handler(Arc::new(test));
/// ```
impl<T> Handler for Arc<T>
where
    T: Handler,
{
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
        let mut router = Router::new();
        router.mount((tester, tester2));
        router.mount(tester2);
        router.at("/foo/bar").get(tester);
        router.at("/foo/bar/baz").get((tester, tester2));
        router.at("/bah").get((tester, Arc::new(tester2)));

        let mut sub_router = Router::new();
        sub_router.at("/bleh").get(tester);
        sub_router.at("/foo/bar").post(tester);
        router.route("/hi", sub_router);

        assert!(router.find("/hi/bleh", Method::GET).is_found());
        assert!(router.find("/hi/foo/bar", Method::POST).is_found());
        assert!(router.find("/foo/bar", Method::GET).is_found());
        assert!(router.find("/foo/bar/baz", Method::GET).is_found());
    }
}
