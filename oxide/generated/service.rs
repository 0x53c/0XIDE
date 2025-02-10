use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
strict serviceRequest {
 //TODO: add req fields here <>?
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
HttpServer::new(|| {
   App::new()
     .service(web::resource("/service")
       .route(web::post().to(service))
 }).bind("127.0.0.1:8080")?
  . run().await
}
