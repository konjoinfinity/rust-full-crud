use actix_web::{get, App, HttpResponse, HttpServer};
use mime::APPLICATION_JSON;
use serde::Serialize;

#[derive(Serialize)]
struct User {
    username: String
}

#[get("/users")]
async fn users() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON.to_string())
        .json(User {username: String::from("Ayushi")})
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new().service(users)
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
