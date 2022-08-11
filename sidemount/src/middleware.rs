use std::sync::Arc;

use async_trait::async_trait;

use crate::{Handler, Request, Response};

#[async_trait]
pub trait Middleware: Send + Sync + 'static {
    async fn handle(&self, req: Request, next: Next) -> Response;
}

pub struct Next {
    middleware: Arc<Vec<Arc<dyn Middleware>>>,
    handler: Arc<dyn Handler>,
    cursor: usize,
}

impl Next {
    pub fn new(middleware: Arc<Vec<Arc<dyn Middleware>>>, handler: Arc<dyn Handler>) -> Self {
        Self {
            middleware,
            handler,
            cursor: 0,
        }
    }

    pub async fn run(mut self, req: Request) -> Response {
        if let Some(mid) = (*self.middleware).get(self.cursor) {
            self.cursor += 1;
            mid.to_owned().handle(req, self).await
        } else {
            self.handler.call(req).await
        }
    }
}
