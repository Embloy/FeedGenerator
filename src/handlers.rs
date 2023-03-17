use std::env;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use base64::decode;
use crate::models::{Job, parse_jobs_from_json};

#[get("/")]
pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}

// TODO: update: request body list of jobs and not single job
#[post("/feed")]
pub async fn load_feed(slice: web::Json<Job>) -> impl Responder {
    println!("STARTED FEED");
    let json_str = serde_json::to_string(&slice).unwrap();
    println!("slice = {}", json_str);

    let _jobs = match parse_jobs_from_json(&json_str) {
        Ok(jobs) => jobs,
        Err(e) => {
            eprintln!("Error parsing jobs from feed JSON: {}", e);
            return HttpResponse::BadRequest().finish();
        }
    };

    // TODO: call Ranker
    // TODO: call Logger
    // TODO: respond with result

    HttpResponse::Created().finish()
}

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