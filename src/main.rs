pub use self::job_slicer::Job;
mod job_slicer;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, patch};



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/tree")]
async fn build_slice(jobs: web::Json<serde_json::Value>) -> impl Responder {
    // Deserialize the JSON data into a vector of Job objects
    let job_vec: Vec<Job> = serde_json::from_value(jobs.into_inner()).unwrap();

    // Do something with job_vec vector
    println(job_vec);
    // job_slicer.initialize(job_vec);
    HttpResponse::Ok().body("message: Slice was reset and overwritten successfully.")
}
/*
#[patch("/tree")]
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
    job_slicer::main();

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

