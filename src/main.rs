use std::env;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use mongodb::{Client, Database};

//use activations::SIGMOID;
//use network::Network;

// use openssl::ssl::{Ssl, SslAcceptor, SslFiletype, SslMethod};

mod handlers;
mod models;
mod ranker;
mod meta;
mod t_score;
mod job_type_matrix;

use crate::ml::*;
//pub mod activations;
//pub mod matrix;
//pub mod network;

mod log {
    pub mod logger; // Include the logger module
}

use log::logger::*; // Use the logger module

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");
    print!("Loading dotenv: ...");
    dotenv().ok(); // Load the .env file
    println!(" successful!");

    print!("Connecting to DB: ...");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
    let client = Client::with_uri_str(&db_url).await.expect("Failed to connect to database");
    let db: Database = client.database("log");
    println!(" successfully connected to database!");

    print!("Fetching network: ...");
    let _network = fetch_network();
    println!(" successful!");

    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();
    job_type_matrix::build().expect("ERROR BUILDING JOB TYPE MATRIX");
    HttpServer::new(move || {
        App::new() // Define routes
            .data(db.clone())
            .service(handlers::hello)
            .service(handlers::load_feed)
    })
        // Bind the server to a socket using OpenSSL as the TLS implementation.
        //.bind_openssl(env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string()), builder)?
        .bind(env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".to_string()))?
        .run()
        .await
}

// Get current state of network when starting server
fn fetch_network() -> Network<'static> {
    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];
    let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];
    let mut network = Network::new(vec![2, 3, 1], 0.5, SIGMOID);
    network.train(inputs, targets, 1000);
//    println!("{:?}", network.feed_forward(vec![0.0, 0.0]));
//    println!("{:?}", network.feed_forward(vec![0.0, 1.0]));
//    println!("{:?}", network.feed_forward(vec![1.0, 0.0]));
//    println!("{:?}", network.feed_forward(vec![1.0, 1.0]));
//    println!("{:?}", network.feed_forward(vec![0.5, 1.0]));
    network
}
