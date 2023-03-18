use std::any::Any;
use std::env;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use base64::decode;
use serde::Serialize;
use crate::models::{Job, parse_jobs_from_json};

// Test connection
#[get("/")]
pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}

// Find out if you're authenticated and authorized to access FG-API
pub async fn basic_auth_handler(req: HttpRequest) -> impl Responder {
    let headers = req.headers();

    if let Some(auth_header) = headers.get("Authorization") {
        let auth_header_str = auth_header.to_str().unwrap_or_default();
        if auth_header_str.starts_with("Basic ") {
            let encoded_credentials = auth_header_str.replace("Basic ", "");
            let decoded_credentials = decode(&encoded_credentials).unwrap_or_default();
            let credentials = String::from_utf8(decoded_credentials).unwrap_or_default();

            if credentials == format!("{}:{}", env::var("API_USER").unwrap_or_default(), env::var("API_PASSWORD").unwrap_or_default()) {
                HttpResponse::Ok().body("Authenticated")
            } else {
                HttpResponse::Unauthorized().body("Unauthorized")
            }
        } else {
            HttpResponse::Unauthorized().body("Unauthorized")
        }
    } else {
        HttpResponse::Unauthorized().body("Unauthorized")
    }
}

// // TODO: update: request body list of jobs and not single job
// #[post("/feed")]
// pub async fn load_feed(slice: web::Json<Job>) -> impl Responder {
//     println!("STARTED FEED");
//     let json_str = serde_json::to_string(&slice).unwrap();
//     println!("slice = {}", json_str);
//
//     let _jobs = match parse_jobs_from_json(&json_str) {
//         Ok(jobs) => jobs,
//         Err(e) => {
//             eprintln!("Error parsing jobs from feed JSON: {}", e);
//             return HttpResponse::BadRequest().finish();
//         }
//     };
//
//     // TODO: call Ranker
//     // TODO: call Logger
//     // TODO: respond with result
//
//     HttpResponse::Created().finish()
// }



// TODO: update: request body list of jobs and not single job
#[post("/feed")]
pub async fn load_feed(slice: web::Json<Job>, req: HttpRequest) -> impl Responder {
    // Check if user is authorized
    if !is_authorized(&req) {
        return HttpResponse::Unauthorized().finish();
    }

    // Parse request body and rank jobs
    let result = process_feed_request(slice.into_inner());

    // TODO: respond with result
    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            eprintln!("Error processing feed request: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

fn process_feed_request(slice: Job) -> Result<(), String> {
    println!("STARTED FEED");

    let json_str = serde_json::to_string(&slice).map_err(|e| e.to_string())?;
    println!("PARSED JSON = {}", json_str);

    let jobs = match parse_jobs_from_json(&json_str) {
        Ok(jobs) => jobs,
        Err(e) => return Err(format!("Error parsing jobs from feed JSON: {}", e)),
    };
    println!("PARSED SLICE = {:?}", jobs);

    // TODO: call Ranker
    // TODO: call Logger
    // TODO: respond with result
    Ok(())
}

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
