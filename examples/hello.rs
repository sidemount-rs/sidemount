use sidemount::{Request, Response, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = sidemount::new();
    app.at("/foo").get(hello);
    app.listen("127.0.0.1:7000").await?;

    Ok(())
}

async fn hello(req: Request) -> Response {
    Response::default()
}
