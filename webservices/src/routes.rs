use super::handlers::*;
use actix_web::{web, App, HttpServer};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_checkout_handler));
}
