use tide::prelude::*;

// Struct which represents a user. ID optionials since it is not availble during the POST request
#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: Option<u32>,
    first_name: String,
    last_name: String,
    email: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    // init the logger
    femme::start();

    // Init the app itself
    let mut app = tide::new();

    // Add the logger to the middleware
    app.with(tide::log::LogMiddleware::new());

    // Register handlers for the endpoints
    app.at("/health").get(health);
    app.at("/get/user/:id").get(get_user);
    app.at("/post/user").post(post_user);

    // Start the server
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

// Handler function to check the health of the application
async fn health(_req: tide::Request<()>) -> tide::Result<String> {
    return Ok("Ok".to_string());
}

// Handler function to retrieve a user
// TODO: Implement database to actually retrieve a user
async fn get_user(req: tide::Request<()>) -> tide::Result<tide::Body> {
    let user_id: u32 = req.param("id")?.parse()?;

    let user = User {
        id: Some(user_id),
        first_name: "test".to_string(),
        last_name: "test".to_string(),
        email: "test@test.com".to_string(),
    };

    return Ok(tide::Body::from_json(&user)?);
}

// Handler function to create a new user
// TODO: Implement database to actually create a user
async fn post_user(mut req: tide::Request<()>) -> tide::Result {
    let user: User = req.body_json().await?;
    return Ok(format!(
        "Created new user {} {} with email {}",
        user.first_name, user.last_name, user.email
    )
    .into());
}
