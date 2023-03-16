mod job_slicer;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, patch};
use crate::job_slicer::Job;
use mongodb::{bson::doc, options::ClientOptions, Client, Collection};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
/*
#[post("/tree")]
async fn build_slice(jobs: web::Json<Vec<Job>>) -> impl Responder{
    job_slicer.initialize(jobs);
    HttpResponse::Ok().body("message: Slice was reset and overwritten successfully.")
}
/*
#[patch("/tree")]
async fn update_slice{}


#[get("/feed")]
async fn generate_feed{}
*/
*/
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the client options, including the connection string mongodb+srv://<username>:<password>@<cluster-address>/test?retryWrites=true&w=majority
    let client_options = ClientOptions::parse("mongodb+srv://cb:sXURVyMz01m1isjU@efg0.rfbpwns.mongodb.net/test?retryWrites=true&w=majority").await?;

    // Create the client with the specified options
    let client = Client::with_options(client_options)?;

    // Access a database
    let db = client.database("embloy_feedgenerator");

    // Access a collection within the database
    // let collection: Collection<Job> = db.collection("db0");

    // List the collections in the database
    let collection_names = db.list_collection_names(None).await?;

    // Print the collection names
    for name in collection_names {
        println!("{}", name);
    }

    Ok(())

    // Perform operations on the collection
    // let job = Job { id: 1, x: 0.0, y: 0.0 };
    // collection.insert_one(job, None);
}

// TODO:
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     job_slicer::main();
//
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }

