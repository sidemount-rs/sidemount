use std::{collections::HashMap, sync::Arc};

use crate::Method;
use crate::{Handler, Node};

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
}

impl Router {
    /// Creates a new router with the default route and middleware
    pub fn new() -> Self {
        Router {
            route: Node::default(),
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
    pub fn find(
        &self,
        path: &str,
        method: Method,
    ) -> RouteResult<(Arc<dyn Handler>, HashMap<String, String>)> {
        let mut params = HashMap::new();
        if let Some(node) = self.route.get_params(path, &mut params) {
            if let Some(handler) = &node._all {
                RouteResult::Found((handler.clone(), params))
            } else if let Some(handler) = node.methods.get(&method) {
                RouteResult::Found((handler.clone(), params))
            } else {
                RouteResult::MethodNotAllowed
            }
        } else {
            RouteResult::NotFound
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Request, Response};

    async fn test(req: Request) -> Response {
        Response::default()
    }

    async fn tester(req: Request) -> i32 {
        3
    }

    async fn tester2(a: i32) -> Response {
        Response::default()
    }

    async fn test3(a: i32) -> String {
        a.to_string()
    }

    async fn test4(s: String) -> Response {
        Response::default()
    }

    #[test]
    fn test_router() {
        let mut router = Router::new();
        router.at("/foo/bar").get(test);

        router.at("/foo/bar/baz").get((tester, tester2));
        router.at("/bah").get((tester, tester2));
        router.at("/boo").get((tester, test3, test4));

        let mut sub_router = Router::new();
        sub_router.at("/bleh").get(test);
        sub_router.at("/foo/bar").post(test);
        router.route("/hi", sub_router);

        assert!(router.find("/hi/bleh", Method::GET).is_found());
        assert!(router.find("/hi/foo/bar", Method::POST).is_found());
        assert!(router.find("/foo/bar", Method::GET).is_found());
        assert!(router.find("/foo/bar/baz", Method::GET).is_found());
    }
}
