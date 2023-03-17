use std::error::Error;
use std::f32::consts::E;
use std::pin::Pin;

use actix_web::{App, get, HttpResponse, HttpServer, patch, post, Responder, web};
use actix_web::cookie::time::format_description::well_known::iso8601::Config;
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::headers::authorization::Basic;
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::handlers::auth;

use crate::models::Job;

mod job_slicer;
mod models;
mod handlers;
mod errors;
use crate::handlers::auth as other_auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the environment variables from the .env file
    dotenv().ok();

    // Get the address to bind the server to
    let addr = "127.0.0.1:8080";

    // Create the HttpServer and bind it to the address
    HttpServer::new(|| {
        App::new()
            // Use the Auth middleware to protect the hello endpoint
            .service(Auth::new().basic(auth))
            .service(handlers::hello)
            .service(handlers::feed)
    })
        .bind(addr)?
        .run()
        .await
}
// async fn validator(req: ServiceRequest, credentials: Basic) -> Result<ServiceRequest, dyn Error> {
//     let config = req
//         .app_data::<Config>()
//         .map(|data| Pin::new(data).get_ref().clone())
//         .unwrap();
//     // match auth::validate_user(credentials.user_id(), credentials.password()) {
//         // Ok(res) => {
//         //     if res == true {
//         //         Err(AuthenticationError::from(config).into())
//         //     } else {
//         //         Err(AuthenticationError::from(config).into())
//         //     }
//         // }
//         // Err(_) => Err(AuthenticationError::from(config).into()),
//     // }
//     Ok(req)
// }

