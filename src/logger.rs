use std::env;

use bson::to_document;
use chrono::Utc;
use mongodb::{Client, options::ClientOptions};

use crate::models::{FeedLog, Job, UserPreferences};

// todo: add exceptions and fg_in_timestamp
pub(crate) async fn log_output(status: i32, pref: Option<UserPreferences>, unsorted_slice: Vec<Job>, sorted_slice: Vec<Job>) -> Result<(), mongodb::error::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
    let client_options = ClientOptions::parse(db_url).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("logs");
    let collection = db.collection("test-logs");

    let log = FeedLog { status, pref, unsorted_slice, sorted_slice, exceptions: None, timestamp_fg_in: None, timestamp_fg_out: Utc::now().timestamp() };
    let doc = to_document(&log)?;
    collection.insert_one(doc, None).await?;
    // collection.insert_one(doc! { "log": bson::to_bson(&log)? }, None).await?;
    Ok(())
}
