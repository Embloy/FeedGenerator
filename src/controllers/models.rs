use std::collections::LinkedList;

use serde::{Deserialize, Serialize};

use crate::controllers::handlers::deserialize_job_types;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    pub job_id: i32,
    pub job_type_value: i32,
    pub job_type: String,
    pub job_status: i32,
    pub status: String,
    pub user_id: i32,
    pub duration: i32,
    pub code_lang: Option<String>,
    pub title: String,
    pub position: Option<String>,
    pub description:Option<String>,
    pub key_skills: Option<String>,
    pub salary: Option<f64>,
    pub currency: Option<String>,
    pub euro_salary: Option<f64>,
    pub image_url: Option<String>,
    pub start_slot: String,
    pub longitude: f64,
    pub latitude: f64,
    pub country_code: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
    pub view_count: i32,
    pub created_at: String,
    pub updated_at: String,
    pub applications_count: i32,
    pub job_notifications: Option<String>,
    pub employer_rating: Option<i32>,
    pub boost: Option<i32>,
    pub relevance_score: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserPreferences {
    pub id: Option<i32>,
    #[serde(deserialize_with = "deserialize_job_types")]
    pub job_types: LinkedList<(i32, f64)>,
    pub key_skills: Option<String>,
    pub salary_range: Option<(f64, f64)>,
    pub spontaneity: Option<f64>,
    pub num_jobs_done: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedRequest {
    pub pref: Option<UserPreferences>,
    pub slice: Vec<Job>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestLog<'a> {
    pub status: i32,
    pub endpoint: &'a str,
    pub timestamp: i64,
    pub exceptions: Option<Vec<CustomBaseError>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedLog {
    pub status: i32,
    pub timestamp_fg_in: i64,
    pub timestamp_fg_out: i64,
    pub pref: Option<UserPreferences>,
    pub sorted_slice: Vec<Job>,
    pub exceptions: Option<Vec<CustomBaseError>>,
}

#[derive(Serialize)]
pub struct NetworkLog {
    pub timestamp: i64,
    // pub network: Network<'static>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomBaseError {
    pub error: String,
    pub description: String,
}
