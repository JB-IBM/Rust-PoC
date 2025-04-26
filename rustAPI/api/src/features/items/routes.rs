use actix_web::web;
use super::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/items")
            .route("", web::get().to(handlers::get_items))
            .route("/{id}", web::get().to(handlers::get_item))
            .route("", web::post().to(handlers::create_item))
    );
}