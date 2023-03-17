use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

mod handlers;
mod auth;
mod job_slicer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handlers::hello)
            .route("/basic-auth", web::get().to(handlers::basic_auth_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}