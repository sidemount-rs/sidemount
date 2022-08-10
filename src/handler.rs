use std::future::Future;

use async_trait::async_trait;

use crate::{Request, Response};

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn call(&self, req: Request) -> Response;
}

#[async_trait]
impl<F, Fut> Handler for F
where
    F: Send + Sync + 'static + Fn(Request) -> Fut,
    Fut: Future<Output = Response> + Send,
{
    async fn call(&self, req: Request) -> Response {
        (self)(req).await
    }
}

#[async_trait]
impl<A, B, T, Fut, Fut2> Handler for (A, B)
where
    A: Send + Sync + 'static + Fn(Request) -> Fut,
    B: Send + Sync + 'static + Fn(T) -> Fut2,
    Fut: Future<Output = T> + Send,
    Fut2: Future<Output = Response> + Send,
    T: Send,
{
    async fn call(&self, req: Request) -> Response {
        let (a, b) = *self;
        let res = (a)(req).await;
        (b)(res).await
    }
}

#[async_trait]
impl<A, B, C, T, T2, Fut, Fut2, Fut3> Handler for (A, B, C)
where
    A: Send + Sync + 'static + Fn(Request) -> Fut,
    B: Send + Sync + 'static + Fn(T) -> Fut2,
    C: Send + Sync + 'static + Fn(T2) -> Fut3,
    Fut: Future<Output = T> + Send,
    Fut2: Future<Output = T2> + Send,
    Fut3: Future<Output = Response> + Send,
    T: Send,
    T2: Send,
{
    async fn call(&self, req: Request) -> Response {
        let (a, b, c) = *self;
        let res = (a)(req).await;
        let res = (b)(res).await;
        (c)(res).await
    }
}

#[async_trait]
impl<A, B, C, D, T, T2, T3, Fut, Fut2, Fut3, Fut4> Handler for (A, B, C, D)
where
    A: Send + Sync + 'static + Fn(Request) -> Fut,
    B: Send + Sync + 'static + Fn(T) -> Fut2,
    C: Send + Sync + 'static + Fn(T2) -> Fut3,
    D: Send + Sync + 'static + Fn(T3) -> Fut4,
    Fut: Future<Output = T> + Send,
    Fut2: Future<Output = T2> + Send,
    Fut3: Future<Output = T3> + Send,
    Fut4: Future<Output = Response> + Send,
    T: Send,
    T2: Send,
    T3: Send,
{
    async fn call(&self, req: Request) -> Response {
        let (a, b, c, d) = *self;
        let res = (a)(req).await;
        let res = (b)(res).await;
        let res = (c)(res).await;
        (d)(res).await
    }
}

#[async_trait]
impl<A, B, C, D, E, T, T2, T3, T4, Fut, Fut2, Fut3, Fut4, Fut5> Handler for (A, B, C, D, E)
where
    A: Send + Sync + 'static + Fn(Request) -> Fut,
    B: Send + Sync + 'static + Fn(T) -> Fut2,
    C: Send + Sync + 'static + Fn(T2) -> Fut3,
    D: Send + Sync + 'static + Fn(T3) -> Fut4,
    E: Send + Sync + 'static + Fn(T4) -> Fut5,
    Fut: Future<Output = T> + Send,
    Fut2: Future<Output = T2> + Send,
    Fut3: Future<Output = T3> + Send,
    Fut4: Future<Output = T4> + Send,
    Fut5: Future<Output = Response> + Send,
    T: Send,
    T2: Send,
    T3: Send,
    T4: Send,
{
    async fn call(&self, req: Request) -> Response {
        let (a, b, c, d, e) = *self;
        let res = (a)(req).await;
        let res = (b)(res).await;
        let res = (c)(res).await;
        let res = (d)(res).await;
        (e)(res).await
    }
}

#[async_trait]
impl<A, B, C, D, E, F, T, T2, T3, T4, T5, Fut, Fut2, Fut3, Fut4, Fut5, Fut6> Handler
    for (A, B, C, D, E, F)
where
    A: Send + Sync + 'static + Fn(Request) -> Fut,
    B: Send + Sync + 'static + Fn(T) -> Fut2,
    C: Send + Sync + 'static + Fn(T2) -> Fut3,
    D: Send + Sync + 'static + Fn(T3) -> Fut4,
    E: Send + Sync + 'static + Fn(T4) -> Fut5,
    F: Send + Sync + 'static + Fn(T5) -> Fut6,
    Fut: Future<Output = T> + Send,
    Fut2: Future<Output = T2> + Send,
    Fut3: Future<Output = T3> + Send,
    Fut4: Future<Output = T4> + Send,
    Fut5: Future<Output = T5> + Send,
    Fut6: Future<Output = Response> + Send,
    T: Send,
    T2: Send,
    T3: Send,
    T4: Send,
    T5: Send,
{
    async fn call(&self, req: Request) -> Response {
        let (a, b, c, d, e, f) = *self;
        let res = (a)(req).await;
        let res = (b)(res).await;
        let res = (c)(res).await;
        let res = (d)(res).await;
        let res = (e)(res).await;
        (f)(res).await
    }
}
