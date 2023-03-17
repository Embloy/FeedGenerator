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


pub fn parse_jobs_from_json(json: &str) -> Result<Job> {
    let job: Job = serde_json::from_str(json)?;
    Ok(job)
}

// pub fn parse_jobs_from_json(json: &str) -> Result<Vec<Job>> {
//     println!("STARTED PARSING");
//     // let data: Value = serde_json::from_str(json)?;
//     // let feed = &data["feed"];
//     // let jobs: Vec<Job> = serde_json::from_value(feed.to_owned())?;
//     let jobs: Vec<Job> = serde_json::from_str(json)?;
//     Ok(jobs)
// }
//
