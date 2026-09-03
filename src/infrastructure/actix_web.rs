use actix_web::{web, App};
use crate::application::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/loans", web::post().to(handlers::create_loan))
       .route("/loans/{id}", web::get().to(handlers::get_loan))
       .route("/loans/{id}", web::put().to(handlers::update_loan))
       .route("/loans/{id}", web::delete().to(handlers::delete_loan));
}