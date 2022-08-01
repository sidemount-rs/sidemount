use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use hyper::server::conn::Http;
use hyper::service::Service;
use hyper::Body;
use tokio::net::{TcpListener, ToSocketAddrs};

use crate::{Handler, Request, Response, Route, RouteResult, Router};

pub struct Server {
    router: Arc<Router>,
}

impl Server {
    /// Creates a new server and default router.
    pub fn new() -> Self {
        Server {
            router: Arc::new(Router::new()),
        }
    }

    /// Creates a new node node or returns a mutable reference to an existing one.
    pub fn at(&mut self, path: &str) -> &mut Route {
        let router =
            Arc::get_mut(&mut self.router).expect("Cannot mount router after binding to listener");
        router.at(path)
    }

    /// Mounts a handler implementation as middleware to be optionally executed with
    /// each of the routes once a route has been found.
    pub fn mount(&mut self, handler: impl Handler) {
        let router =
            Arc::get_mut(&mut self.router).expect("Cannot mount router after binding to listener");
        router.mount(handler);
    }

    /// Routes a path on the router to an existing router implementation.
    pub fn route(&mut self, path: &str, router: Router) {
        let rt =
            Arc::get_mut(&mut self.router).expect("Cannot mount router after binding to listener");
        rt.route(path, router);
    }

    /// Executes a listener on a given listener type.
    pub async fn listen<T: ToSocketAddrs>(
        self,
        addr: T,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let listener = TcpListener::bind(addr).await?;
        loop {
            let (stream, _) = listener.accept().await?;

            let server = Server {
                router: self.router.clone(),
            };
            tokio::task::spawn(async move {
                if let Err(err) = Http::new().serve_connection(stream, server).await {
                    eprintln!("Failed to serve connection: {:?}", err);
                }
            });
        }
    }
}

impl Service<Request> for Server {
    type Response = Response;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let path = req.uri().path();
        let res = match self.router.find(path, req.method().into()) {
            RouteResult::Found(r) => {
                r.call();
                hyper::Response::builder()
                    .status(200)
                    .body(Body::empty())
                    .unwrap()
            }
            RouteResult::NotFound => hyper::Response::builder()
                .status(hyper::StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap(),
            RouteResult::MethodNotAllowed => hyper::Response::builder()
                .status(hyper::StatusCode::METHOD_NOT_ALLOWED)
                .body(Body::empty())
                .unwrap(),
        };

        Box::pin(async { Ok(res) })
    }
}
