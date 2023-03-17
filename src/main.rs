mod job_slicer;

use mongodb::{Client, options::ClientOptions, Collection, bson, options::InsertOneOptions};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, patch};
use crate::job_slicer::{Job, JobSlicer};
use serde::{Serialize, Deserialize};
use serde_json::json;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/tree")]
async fn build_slice(jobs: web::Json<serde_json::Value>) -> impl Responder {
    // Deserialize the JSON data into a vector of Job objects
    let job_vec: Vec<Job> = serde_json::from_value(jobs.into_inner()).unwrap();

    // Do something with job_vec vector
    // println!("{}", job_vec);
    // job_slicer.initialize(job_vec);
    HttpResponse::Ok().body("message: Slice was reset and overwritten successfully.")
}
/*#[patch("/tree")]
async fn update_slice{}


#[get("/feed")]
async fn generate_feed{}
*/
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("STARTED MAIN");
    job_slicer::main();

    println!("CONNECTING TO DATABASE");
    // Create a MongoDB client instance
    let client = Client::with_uri_str("mongodb+srv://cb:sXURVyMz01m1isjU@efg0.rfbpwns.mongodb.net/test?retryWrites=true&w=majority").await;

    // Access the database and collection
    let db = client.expect("REASON").database("embloy_feedgenerator");
    let _collection: Collection<Job> = db.collection("db0");

    // The code below successfully makes a db entry!

    // ==== TEST ==== //
    // let job = Job { id: 7, x: 0.0, y: 0.0 };
    // let mut job_vector: Vec<Job> = Vec::new();
    // job_vector.push(job);
    // let options = InsertOneOptions::default();
    // _collection.insert_one(job, options).await.expect("Failed to insert document");;
    // ==== TEST ==== //

    // Do whatever you need to do with the database and collection
    // List the collections in the database
    match db.list_collection_names(None).await {
        Ok(collection_names) => {
            // Print the collection names
            for name in collection_names {
                println!("CONNECTED TO DATABASE: collection = [{}]", name);
            }
        }
        Err(e) => {
            // Print the error message
            eprintln!("Error listing collections: {}", e);
        }
    }



    println!("STARTED SERVER");
    // Start the HTTP server
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}