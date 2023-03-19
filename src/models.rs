use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    job_id: i32,
    job_type: String,
    job_status: i32,
    status: String,
    user_id: i32,
    duration: i32,
    code_lang: String,
    title: String,
    position: String,
    description: String,
    key_skills: String,
    salary: i32,
    currency: String,
    image_url: String,
    start_slot: String,
    longitude: f64,
    latitude: f64,
    country_code: String,
    postal_code: String,
    city: String,
    address: String,
    view_count: i32,
    created_at: String,
    updated_at: String,
    applications_count: i32,
    job_notifications: String,
}