////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////API-ENDPOINTS///////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

use std::any::Any;
use std::env;

use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web};
use base64::decode;
use serde::Serialize;

use crate::models::Job;

// Test connection
#[get("/")]
pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}

// Find out if you're authenticated and authorized to access FG-API
#[get("/auth")]
pub async fn basic_auth(req: HttpRequest) -> impl Responder {
    let auth_header = match req.headers().get("Authorization") {
        Some(header) => header,
        None => return HttpResponse::Unauthorized().body("Unauthorized")
    };

    let auth_header_str = match auth_header.to_str() {
        Ok(auth_str) => auth_str,
        Err(_) => return HttpResponse::Unauthorized().body("Unauthorized")
    };

    let encoded_credentials = auth_header_str.replace("Basic ", "");
    let decoded_credentials = match decode(&encoded_credentials) {
        Ok(credentials) => credentials,
        Err(_) => return HttpResponse::Unauthorized().body("Unauthorized")
    };

    let credentials = match String::from_utf8(decoded_credentials) {
        Ok(credentials) => credentials,
        Err(_) => return HttpResponse::Unauthorized().body("Unauthorized")
    };

    if credentials == format!("{}:{}", env::var("API_USER").unwrap_or_default(), env::var("API_PASSWORD").unwrap_or_default()) {
        HttpResponse::Ok().body("Authenticated")
    } else {
        HttpResponse::Unauthorized().body("Unauthorized")
    }
}

#[post("/feed")]
pub async fn load_feed(slice: web::Json<Vec<Job>>, req: HttpRequest) -> impl Responder {
    // Check if user is authorized
    if !is_authorized(&req) {
        return HttpResponse::Unauthorized().finish();
    }

    // Parse request body and rank jobs
    let result = process_feed_request(slice.into_inner());

    // Respond with result as response body
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            eprintln!("Error processing feed request: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////HELPER METHODS///////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

// Parse request body and rank jobs
fn process_feed_request(slice: Vec<Job>) -> Result<(Vec<Job>), Box<dyn std::error::Error>> {
    println!("SLICE = {:?}", slice);

    // TODO: call Ranker
    // TODO: call Logger

    Ok(slice)
}

// Check if user is authenticated and authorized to access FG-API
pub fn is_authorized(req: &HttpRequest) -> bool {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Basic ") {
                let decoded_auth = match decode(auth_str[6..].as_bytes()) {
                    Ok(decoded_auth) => decoded_auth,
                    Err(_) => return false,
                };
                if let Ok(credentials) = std::str::from_utf8(&decoded_auth) {
                    return credentials == format!("{}:{}", env::var("API_USER").unwrap_or_default(), env::var("API_PASSWORD").unwrap_or_default());
                }
            }
        }
    }
    false
}
