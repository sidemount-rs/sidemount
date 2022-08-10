use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use hyper::Body;
use hyper::{server::conn::Http, service::Service};
use tokio::net::{TcpListener, ToSocketAddrs};

use crate::{http, Middleware, Next, Request, Response, Route, RouteResult, Router};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

pub struct Server {
    middleware: Arc<Vec<Arc<dyn Middleware>>>,
    router: Arc<Router>,
}

impl Server {
    /// Creates a new server and default router.
    pub fn new() -> Self {
        Server {
            middleware: Arc::new(Vec::new()),
            router: Arc::new(Router::new()),
        }
    }

    /// Mounts middleware implementation to the server.
    pub fn mount(&mut self, mid: impl Middleware) {
        let middleware = Arc::get_mut(&mut self.middleware)
            .expect("Cannot mount middleware after binding to listener");
        middleware.push(Arc::new(mid));
    }

    /// Creates a new node node or returns a mutable reference to an existing one.
    pub fn at(&mut self, path: &str) -> &mut Route {
        let router =
            Arc::get_mut(&mut self.router).expect("Cannot mount router after binding to listener");
        router.at(path)
    }

    /// Routes a path on the router to an existing router implementation.
    pub fn route(&mut self, path: &str, router: Router) {
        let rt =
            Arc::get_mut(&mut self.router).expect("Cannot mount router after binding to listener");
        rt.route(path, router);
    }

    /// Executes a listener on a given listener type.
    pub async fn listen<T: ToSocketAddrs>(self, addr: T) -> Result<()> {
        let listener = TcpListener::bind(addr).await?;
        loop {
            let (stream, _) = listener.accept().await?;

            let server = Server {
                middleware: self.middleware.clone(),
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

impl Service<http::Request> for Server {
    type Response = http::Response;
    type Error = hyper::Error;
    type Future =
        Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<std::result::Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: http::Request) -> Self::Future {
        let router = self.router.clone();
        let middleware = self.middleware.clone();
        Box::pin(async move {
            let res = match router.find(req.uri().path(), req.method().into()) {
                RouteResult::Found(r) => {
                    let (handler, params) = r;
                    let req = Request::new(req, params);
                    let next = Next::new(middleware, handler);
                    next.run(req).await.into()
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
            Ok(res)
        })
    }
}
