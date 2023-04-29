use actix_web::web;
use bson::to_document;
use chrono::Utc;
use mongodb::Database;

use crate::models::{FeedLog, Job, UserPreferences};

// Todo: add exceptions and fg_in_timestamp
pub(crate) async fn log_output(db: web::Data<Database>, status: i32, pref: Option<UserPreferences>, unsorted_slice: Vec<Job>, sorted_slice: Vec<Job>) -> Result<(), mongodb::error::Error> {
    let log = FeedLog { status, pref, unsorted_slice, sorted_slice, exceptions: None, timestamp_fg_in: None, timestamp_fg_out: Utc::now().timestamp() };
    db.collection("test-logs").insert_one(to_document(&log)?, None).await?;
    Ok(())
}
