use std::collections::{HashMap, LinkedList};
use std::env;

use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web};
use base64::decode;
use chrono::Utc;
use serde::Deserialize;
use crate::config::app_state::AppState;

use crate::controllers::models::{CustomBaseError, FeedRequest, Job, UserPreferences};
use crate::logs::logger;
use crate::ranking_algorithms::ranker::generate_job_feed;

// Test connection
#[get("/")]
pub async fn hello(state: web::Data<AppState>) -> HttpResponse {
    // logger::log_request(&state.db, 200, "GET:/root").await.expect("LOGGER TIMEOUT");
    HttpResponse::Ok().body("Hello World")
}

// FG-API endpoint: Returns ranked feed based on given jobs and preferences
#[post("/feed")]
pub async fn load_feed(state: web::Data<AppState>, feed_request: web::Json<FeedRequest>, req: HttpRequest) -> impl Responder {
    logger::log_request(&state.db, 200, "POST:/feed").await.expect("LOGGER TIMEOUT");
    let FeedRequest { pref, slice } = feed_request.into_inner();

    // Check if user is authorized
    if !is_authorized(&req) {
        let base = CustomBaseError {
            error: "ERR_INVALID".to_string(),
            description: "Attribute is malformed or unknown.".to_string(),
        };
        let mut errors = HashMap::new();
        errors.insert("email|password", vec![base]);
        println!("{:?}", errors);
        return HttpResponse::Unauthorized().json(errors);
    }

    // Parse request body and rank jobs
    let result = process_feed_request(state, slice, pref).await;

    // Respond with result as response body
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            eprintln!("Error processing feed request: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


pub(crate) fn deserialize_job_types<'de, D>(deserializer: D) -> Result<LinkedList<(i32, f64)>, D::Error> where D: serde::Deserializer<'de> {
    let job_types_map: HashMap<i32, f64> = HashMap::deserialize(deserializer)?;
    let mut job_types_list = LinkedList::new();
    for (k, v) in job_types_map { job_types_list.push_back((k, v)); }
    Ok(job_types_list)
}

// Parse request body and rank jobs
async fn process_feed_request(state: web::Data<AppState>, slice: Vec<Job>, pref: Option<UserPreferences>) -> Result<Vec<Job>, Box<dyn std::error::Error>> {
    let timestamp_fg_in = Utc::now().timestamp();

    // Rank jobs
    let res: Vec<Job> = generate_job_feed(slice.clone(), pref.clone()).await;

    // Log results
    // logger::log_output(&state.db, 200, pref, res.clone(), timestamp_fg_in).await.expect("LOGGER TIMEOUT");
    // logger::network_snapshot(network).await.expect("LOGGER TIMEOUT");

    Ok(res)
}


// Check if user is authenticated and authorized to access FG-API
fn is_authorized(req: &HttpRequest) -> bool {
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
