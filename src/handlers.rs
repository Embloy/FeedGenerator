////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////API-ENDPOINTS///////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

use std::env;

use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web};
use base64::decode;

use crate::models::{FeedRequest, Job, UserPreferences};
use crate::ranker::generate_job_feed;

// Test connection
#[get("/")]
pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}

// Find out if you're authenticated and authorized to access FG-API
#[get("/auth")]
pub async fn basic_auth(req: HttpRequest) -> impl Responder {
    // Extract the authorization header value and check if it exists
    let auth_header = match req.headers().get("Authorization") {
        Some(header) => header.to_str().unwrap_or_default(),
        None => return HttpResponse::Unauthorized().body("Unauthorized")
    };

    // Decode the base64-encoded credentials
    let decoded_credentials = match base64::decode(auth_header.replace("Basic ", "")) {
        Ok(credentials) => credentials,
        Err(_) => return HttpResponse::Unauthorized().body("Unauthorized")
    };

    // Convert the decoded credentials to a UTF-8 string
    let credentials = match String::from_utf8(decoded_credentials) {
        Ok(credentials) => credentials,
        Err(_) => return HttpResponse::Unauthorized().body("Unauthorized")
    };

    // Check if the credentials match the expected API user and password
    if credentials == format!("{}:{}", env::var("API_USER").unwrap_or_default(), env::var("API_PASSWORD").unwrap_or_default()) {
        HttpResponse::Ok().body("Authenticated")
    } else {
        HttpResponse::Unauthorized().body("Unauthorized")
    }
}

#[post("/feed")]
pub async fn load_feed(feed_request: web::Json<FeedRequest>, req: HttpRequest) -> impl Responder {
    println!("req = {:?}", feed_request);
    let FeedRequest { pref, slice } = feed_request.into_inner();
    println!("pref = {:?}", pref);
    println!("slice = {:?}", slice);

    // Check if user is authorized
    if !is_authorized(&req) {
        return HttpResponse::Unauthorized().finish();
    }

    // Parse request body and rank jobs
    let result = process_feed_request(slice, pref);

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
fn process_feed_request(slice: Vec<Job>, pref: UserPreferences) -> Result<Vec<Job>, Box<dyn std::error::Error>> {
    // TODO: Ranking ...
    let res: Vec<Job> = generate_job_feed(slice, pref);

    // TODO: Logging ...
    Ok(res)
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
