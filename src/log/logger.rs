pub mod logger {
    use actix_web::web;
    use bson::to_document;
    use chrono::Utc;
    use mongodb::Database;

    use crate::ml::network::Network;
    use crate::models::{FeedLog, Job, NetworkLog, UserPreferences};

    // TODO: Add exceptions and fg_in_timestamp
    pub(crate) async fn log_output(db: web::Data<Database>, status: i32, pref: Option<UserPreferences>, sorted_slice: Vec<Job>) -> Result<(), mongodb::error::Error> {
        let log = FeedLog { status, pref, sorted_slice, exceptions: None, timestamp_fg_in: None, timestamp_fg_out: Utc::now().timestamp() };
        db.collection("test-log").insert_one(to_document(&log)?, None).await?;
        Ok(())
    }

    pub(crate) async fn network_snapshot(db: web::Data<Database>, _network: Network<'static>) -> Result<(), mongodb::error::Error> {
        let log = NetworkLog { timestamp: Utc::now().timestamp() };
        db.collection("test-log").insert_one(to_document(&log)?, None).await?;
        Ok(())
    }
}