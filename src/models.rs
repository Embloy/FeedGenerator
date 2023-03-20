use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    job_id: i32,
    pub(crate) job_type: String,
    job_status: i32,
    status: String,
    user_id: i32,
    duration: i32,
    code_lang: String,
    title: String,
    position: String,
    description: String,
    pub(crate) key_skills: String,
    pub(crate) salary: f64,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPreferences {
    pub(crate) job_type: HashMap<i32, i32>,
    // pub(crate) key_skills: String,
    pub(crate) salary_range: (f64, f64),
    pub(crate) spontaneity: (f64, f64),

}

// ER -> 0.2
// Salary Range -> 0.2
// #App/#V -> 0.5
// Sp -> 0.1


#[derive(Debug, Serialize, Deserialize)]
pub struct FeedRequest {
    pub pref: UserPreferences,
    pub slice: Vec<Job>,
}
