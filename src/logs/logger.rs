use actix_web::web;
use bson::to_document;
use chrono::Utc;
use mongodb::Database;

use crate::controllers::models::{FeedLog, RequestLog, NetworkLog, Job, UserPreferences};
use crate::machine_learning::network::Network;

// TODO: Add exceptions
pub(crate) async fn log_request(db: &Database, status: i32, endpoint: &str) -> Result<(), mongodb::error::Error> {
    let log = RequestLog { status, endpoint, timestamp: Utc::now().timestamp(), exceptions: None };
    db.collection("request-logs").insert_one(to_document(&log)?, None).await?;
    Ok(())
}

pub(crate) async fn log_output(db: &Database, status: i32, pref: Option<UserPreferences>, sorted_slice: Vec<Job>, timestamp_fg_in: i64) -> Result<(), mongodb::error::Error> {
    let log = FeedLog { status, pref, sorted_slice, exceptions: None, timestamp_fg_in, timestamp_fg_out: Utc::now().timestamp() };
    db.collection("output-logs").insert_one(to_document(&log)?, None).await?;
    Ok(())
}

pub(crate) async fn network_snapshot(db: &Database, _network: Network<'static>) -> Result<(), mongodb::error::Error> {
    let log = NetworkLog { timestamp: Utc::now().timestamp() };
    db.collection("network-logs").insert_one(to_document(&log)?, None).await?;
    Ok(())
}
