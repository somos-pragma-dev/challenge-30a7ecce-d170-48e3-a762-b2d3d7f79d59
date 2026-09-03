mod application;
mod domain;
mod infrastructure;

use actix_web::{web, App, HttpServer};
use infrastructure::actix_web::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(configure_routes)
    })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}