use std::sync::Mutex;
use actix_web::web;
use crate::features::items::models::Item;

pub struct AppState {
    pub items: Mutex<Vec<Item>>,
}

pub fn create_app_state() -> web::Data<AppState> {
    web::Data::new(AppState {
        items: Mutex::new(vec![]),
    })
}  