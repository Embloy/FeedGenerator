use std::collections::LinkedList;
use crate::models::{FeedLog, Job, UserPreferences};
use mongodb::{bson::doc, Client, options::ClientOptions};
use std::error::Error;
use actix_web::http::header::ContentLanguage;
use bson::{bson, Array, Document};
use serde::{Serialize, Deserialize};
use bson::Bson;
use std::env;
use dotenv::dotenv;

// todo: add exceptions and fg_in_timestamp

pub(crate) async fn add_about_fg_ranking(log: FeedLog) {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
    let client_options = ClientOptions::parse(&db_url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db_name = "embloy";
    let coll_name = "embloy_fg_ranking_log";
    let db = client.database(db_name);
    let coll: mongodb::Collection<bson::Document> = db.collection(coll_name);

    // convert sorted slice claims to bson
    let ss_bson_docs: Vec<Document> = log
        .sorted_slice
        .iter()
        .map(|job| {
            doc! {
            "job_id": bson!(job.job_id),
        }
        })
        .collect();
    let ss_bson_array = bson::Array::from(
        ss_bson_docs
            .into_iter()
            .map(|doc| bson::to_bson(&doc).unwrap())
            .collect::<Vec<Bson>>(),
    );

    // -------------------------------

    // convert unsorted slice claims to bson
    let us_bson_docs: Vec<Document> = log
        .unsorted_slice
        .iter()
        .map(|job| {
            doc! {
            "job_id": bson!(job.job_id),
        }
        })
        .collect();
    let us_bson_array = bson::Array::from(
        us_bson_docs
            .into_iter()
            .map(|doc| bson::to_bson(&doc).unwrap())
            .collect::<Vec<Bson>>(),
    );

    // -------------------------------

    // convert job types to bson
    let job_types_doc: Vec<Document> = log.pref.clone().unwrap().job_types.iter().map(|job_type| {
        doc! {
        "type_id": bson!(job_type.0),
        "weight": bson!(job_type.1),
    }
    }).collect();

    let job_types_array = Array::from(
        job_types_doc
            .into_iter()
            .map(|doc| Bson::Document(doc))
            .collect::<Vec<Bson>>(),
    );

    // -------------------------------

    // convert salary range to bson
    let salary_range = log.pref.clone().unwrap().salary_range;
    let salary_range_bson = match salary_range {
        Some((min, max)) => bson!({ "min": min, "max": max }),
        None => bson!({}),
    };
    // -------------------------------

    let bson_doc = doc! {
    "status": bson!(log.status),
    "sorted_slice": ss_bson_array,
    "unsorted_slice": us_bson_array,
    "user_preferences": {
            "job_types": job_types_array,
            "spontaneity": bson!(log.pref.unwrap().spontaneity.unwrap_or(-1.0)),
            "salary_range": salary_range_bson
        },
        "timestamp_fg_out": bson!(log.timestamp_fg_out)
    };




    coll.insert_one(bson_doc, None).await.unwrap();

}