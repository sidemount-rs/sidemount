use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use hyper::Body;
use hyper::{server::conn::Http, service::Service};
use tokio::net::{TcpListener, ToSocketAddrs};

use crate::{Request, Response, Route, RouteResult, Router};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

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
    type Future =
        Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<std::result::Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let router = self.router.clone();
        Box::pin(async move {
            let res = match router.find(req.uri().path(), req.method().into()) {
                RouteResult::Found(r) => {
                    let (handler, params) = r;
                    handler.call(Request::new(req, params)).await
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
