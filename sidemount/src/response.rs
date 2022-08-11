use crate::http;

#[derive(Default)]
pub struct Response {
    res: http::Response,
}

impl From<Response> for http::Response {
    fn from(res: Response) -> Self {
        res.res
    }
}
