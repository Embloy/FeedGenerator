////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////MAIN////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

use std::env;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

// use openssl::ssl::{Ssl, SslAcceptor, SslFiletype, SslMethod};

mod handlers;
mod auth;
mod models;
mod ranker;
mod logger;
mod meta;
mod t_score;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");
    dotenv().ok(); // Load the .env file
    println!("Loading dotenv ...");

    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new() // Define routes
            .service(handlers::hello)
            .service(handlers::load_feed)
            .service(handlers::basic_auth)
    })
        // Bind the server to a socket using OpenSSL as the TLS implementation.
        //.bind_openssl(env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string()), builder)?
        .bind(env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".to_string()))?
        .run()
        .await
}
