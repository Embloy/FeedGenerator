use std::env;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

// use openssl::ssl::{Ssl, SslAcceptor, SslFiletype, SslMethod};

mod handlers;
mod models;
mod ranker;
mod logger;
mod meta;
mod t_score;
mod job_type_matrix;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");
    dotenv().ok(); // Load the .env file
    println!("Loading dotenv: Successful!");


    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();
    job_type_matrix::build().expect("TODO: panic message");
    HttpServer::new(|| {
        App::new() // Define routes
            .service(handlers::hello)
            .service(handlers::load_feed)
    })
        // Bind the server to a socket using OpenSSL as the TLS implementation.
        //.bind_openssl(env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string()), builder)?
        .bind(env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".to_string()))?
        .run()
        .await
}


/*
IGNORE:
println!("salary in range: {}", (((50.0 - 20.0) / (70.0 - 20.0)) as f64).max(-2.0).min(2.0));
println!("salary > range: {}", (((50.0 - 30.0) / (40.0 - 30.0)) as f64).max(-2.0).min(2.0));
println!("salary < range: {}", (((50.0 - 60.0) / (70.0 - 60.0)) as f64).max(-2.0).min(2.0));
println!("salary = max: {}", (((50.0 - 40.0) / (50.0 - 40.0)) as f64).max(-2.0).min(2.0));
println!("salary = min: {}", (((50.0 - 50.0) / (70.0 - 50.0)) as f64).max(-2.0).min(2.0));
println!("salary << range: {}", (((50.0 - 1000.0) / (1000.1 - 1000.0)) as f64).max(-2.0).min(2.0));
println!("salary >> range: {}", (((5000000.0 - 1.0) / (2.0 - 1.0)) as f64).max(-2.0).min(2.0));
*/

