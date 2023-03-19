////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////MAIN////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

use std::env;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

mod handlers;
mod auth;
mod models;
mod ranker;
mod logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load the .env file
    HttpServer::new(|| {
        App::new()
            .service(handlers::hello)
            .service(handlers::load_feed)
            .service(handlers::basic_auth)
        // .route("/auth", web::get().to(handlers::basic_auth))
    })
        .bind(env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string()))?
        .run()
        .await
}