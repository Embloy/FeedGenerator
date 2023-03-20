////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////MODELS///////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    job_id: i32,
    pub(crate) job_type: String,
    job_status: i32,
    status: String,
    user_id: i32,
    duration: i32,
    code_lang: Option<String>,
    title: String,
    position: Option<String>,
    description: String,
    pub(crate) key_skills: Option<String>,
    pub(crate) salary: Option<f64>,
    pub(crate) currency: Option<String>,
    image_url: Option<String>,
    pub(crate) start_slot: String,
    longitude: f64,
    latitude: f64,
    country_code: Option<String>,
    postal_code: Option<String>,
    city: Option<String>,
    address: Option<String>,
    pub(crate) view_count: i32,
    created_at: String,
    updated_at: String,
    pub(crate) applications_count: i32,
    job_notifications: Option<String>,
    pub(crate) employer_rating: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserPreferences {
    pub(crate) job_type: Option<HashMap<i32, i32>>,
    pub(crate) key_skills: Option<String>,
    pub(crate) salary_range: Option<(f64, f64)>,
    pub(crate) spontaneity: Option<(String, String)>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedRequest {
    pub pref: Option<UserPreferences>,
    pub slice: Vec<Job>,
}
