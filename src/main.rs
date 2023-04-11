use tide::prelude::*;

#[async_std::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());

    app.at("/health").get(health);

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

async fn health(_req: tide::Request<()>) -> tide::Result<String> {
    return Ok("Ok".to_string());
}
