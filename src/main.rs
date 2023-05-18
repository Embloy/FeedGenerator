pub mod logs;
pub mod controllers;
pub mod machine_learning;
pub mod ranking_algorithms;
pub mod lib;
pub mod config;

use std::env;
// use openssl::ssl::{Ssl, SslAcceptor, SslFiletype, SslMethod};
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use dotenv::dotenv;
use mongodb::{Client, Database};

use controllers::handlers::{hello, load_feed};
use machine_learning::{activations::SIGMOID, network::Network};
use ranking_algorithms::job_type_matrix;
use controllers::handlers;
use config::app_state::AppState;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log::info!("Starting server...");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Loading dotenv: ...");
    dotenv().ok(); // Load the .env file
    let host = env::var("BACKEND_HOST").expect("BACKEND_HOST set in .env file");
    let port = env::var("BACKEND_PORT").expect("BACKEND_PORT set in .env file");
    log::info!("... successful!");

    log::info!("Connecting to DB: ...");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL set in .env file");
    let client = Client::with_uri_str(&db_url).await.expect("to establish connection");
    let db: Database = client.database("logs");
    log::info!("... successful!");

    log::info!("Fetching network: ...");
    // let network = fetch_network();
    let network = 5.0;
    let state = AppState { db, network };
    log::info!("... successful!");


    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();

    log::info!("Building job type matrix: ...");
    job_type_matrix::build().expect("job type matrix build");
    log::info!("... successful!");

    log::info!("Listening on {}:{}", host, port);
    HttpServer::new(move || {
        App::new() // Define routes
            .app_data(Data::new(state.clone()))
            // .data(db.clone())
            .service(hello)
            .service(load_feed)
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
