use std::env;
use std::vec::Vec;

use actix_web::{App, Error, get, HttpResponse, HttpServer, patch, post, Responder, web};
use futures::TryFutureExt;
use r2d2::Pool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

// Handler for POST /feed
#[post("/feed")]
pub(crate) async fn feed() -> impl Responder {
    HttpResponse::Ok().body("LOADING FEED!")
}

// Handler for GET /
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

// Handler for the authentication endpoint
#[get("/auth")]
pub async fn auth() -> impl Responder {
    // Extract the expected username and password from the environment variables
    let expected_username = env::var("API_USER").unwrap_or_else(|_| "API_USER".into());
    let expected_password = env::var("API_PASSWORD").unwrap_or_else(|_| "API_PASSWORD".into());

    <HttpResponse as Into<T>>::into(HttpResponse::Unauthorized()
        .header("WWW-Authenticate", r#"Basic realm="Restricted""#)
        .body("Authentication required"))
        .into()
        .with_secure(|guard| guard.basic_auth(|user, password| {
            if user == expected_username && password == Some(expected_password.as_str()) {
                Ok(())
            } else {
                Err(())
            }
        }))
}