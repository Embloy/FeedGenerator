////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////API-ENDPOINTS///////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::{HashMap, LinkedList};
use std::env;
use serde::Deserialize;


use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web};
use base64::decode;

use crate::models::{FeedRequest, Job, UserPreferences, CustomBaseError};
use crate::ranker::generate_job_feed;

// Test connection
#[get("/")]
pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}

#[post("/feed")]
pub async fn load_feed(feed_request: web::Json<FeedRequest>, req: HttpRequest) -> impl Responder {
    let FeedRequest { pref, slice } = feed_request.into_inner();

    // Check if user is authorized
    if !is_authorized(&req) {
        let base = CustomBaseError {
            error: "ERR_INVALID".to_string(),
            description: "Attribute is malformed or unknown.".to_string()
        };
        let mut errors = HashMap::new();
        errors.insert("email|password",vec![base]);
        println!("{:?}", errors);
        return HttpResponse::Unauthorized().json(errors);
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

pub(crate) fn deserialize_job_types<'de, D>(deserializer: D) -> Result<LinkedList<(i32, f64)>, D::Error>
    where
        D: serde::Deserializer<'de>,
{
    let job_types_map: std::collections::HashMap<i32, f64> = std::collections::HashMap::deserialize(deserializer)?;
    let mut job_types_list = LinkedList::new();
    for (k, v) in job_types_map {
        job_types_list.push_back((k, v));
    }
    Ok(job_types_list)
}

// Parse request body and rank jobs
fn process_feed_request(slice: Vec<Job>, pref: Option<UserPreferences>) -> Result<Vec<Job>, Box<dyn std::error::Error>> {
    // Ranking ...
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
