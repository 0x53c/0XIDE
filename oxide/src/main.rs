use actix_web::{web, App, HttpServer, Responder};
use std::env;

async fn init_msg() -> impl Responder {
    let message = env::var("INIT_MESSAGE").unwrap_or("Hello.".to_string());
    format!("{}", message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(init_msg))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}