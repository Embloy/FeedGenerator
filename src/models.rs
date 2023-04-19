use std::collections::LinkedList;

use serde::{Deserialize, Serialize};

use crate::handlers::deserialize_job_types;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    pub(crate) job_id: i32,
    pub(crate) job_type_value: i32,
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
    pub(crate) euro_salary: Option<f64>,
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
    pub(crate) employer_rating: Option<i32>,
    pub(crate) boost: Option<i32>,
    pub(crate) relevance_score: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserPreferences {
    #[serde(deserialize_with = "deserialize_job_types")]
    pub(crate) job_types: LinkedList<(i32, f64)>,
    pub(crate) key_skills: Option<String>,
    pub(crate) salary_range: Option<(f64, f64)>,
    pub(crate) spontaneity: Option<f64>,
    pub(crate) num_jobs_done: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedRequest {
    pub pref: Option<UserPreferences>,
    pub slice: Vec<Job>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedLog {
    pub status: i32,
    pub timestamp_fg_in: Option<i64>,
    pub timestamp_fg_out: Option<i64>,
    pub pref: Option<UserPreferences>,
    pub unsorted_slice: Vec<Job>,
    pub sorted_slice: Vec<Job>,
    pub exceptions: Option<Vec<CustomBaseError>>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CustomBaseError {
    pub error: String,
    pub description: String,
}
