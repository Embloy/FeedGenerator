// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::{test, web, App};
//     use crate::handlers::hello;
//
//     #[actix_web::test]
//     async fn test_index_get() {
//         let app = test::init_service(
//             App::new()
//                 .route("/", web::get().to(hello)),
//         )
//             .await;
//         let req = test::TestRequest::get().uri("/").to_request();
//
//         assert!(true);
//     }
// }
