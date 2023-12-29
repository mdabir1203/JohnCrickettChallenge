//Set up the web server and routes.

use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Welcome to uKnowWho Carousel Generator !"
}

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))})   // making a GET request to "/" will call the index function
        .bind("127.0.0.1:8080")? // the ? operator is like your friend who checks if anyone else is trying to build their tower in the same spot
        .run()
        .await
}