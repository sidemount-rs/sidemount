#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut app = sidemount::new();
    app.at("/foo").get(hello);
    app.listen("127.0.0.1:7000").await?;

    Ok(())
}

fn hello() {
    println!("hello world!");
}
