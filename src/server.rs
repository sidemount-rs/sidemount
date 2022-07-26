use crate::{Handler, Route, Router};

pub struct Server {
    router: Router,
}

impl Server {
    /// Creates a new server and default router.
    pub fn new() -> Self {
        Server {
            router: Router::new(),
        }
    }

    /// Creates a new node node or returns a mutable reference to an existing one.
    pub fn at(&mut self, path: &str) -> &mut Route {
        self.router.at(path)
    }

    /// Mounts a handler implementation as middleware to be optionally executed with
    /// each of the routes once a route has been found.
    pub fn mount(&mut self, handler: impl Handler) {
        self.router.mount(handler);
    }

    /// Routes a path on the router to an existing router implementation.
    pub fn route(&mut self, path: &str, router: Router) {
        self.router.route(path, router);
    }
}
