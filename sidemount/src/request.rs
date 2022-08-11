use std::collections::HashMap;

use crate::{http, Method};

pub struct Request {
    req: http::Request,
    params: HashMap<String, String>,
}

impl Request {
    pub fn new(req: http::Request, params: HashMap<String, String>) -> Self {
        Self { req, params }
    }

    pub fn method(&self) -> &Method {
        self.req.method()
    }

    pub fn path(&self) -> &str {
        self.req.uri().path()
    }

    pub fn param(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }
}
