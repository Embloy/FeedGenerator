use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: u32,
    pub user_id: u32,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct NewJob<'a> {
    pub id: &'a u32,
    pub user_id: &'a u32,
    pub title: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
