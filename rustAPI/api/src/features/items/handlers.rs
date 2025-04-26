use actix_web::{web, HttpResponse, Responder};
use crate::app_state::AppState;
use super::models::Item;

pub async fn get_items(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    HttpResponse::Ok().json(&*items)
}

pub async fn get_item(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let item_id = path.into_inner();
    let items = data.items.lock().unwrap();

    if let Some(item) = items.iter().find(|i| i.id == item_id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}

pub async fn create_item(item: web::Json<Item>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    items.push(item.into_inner());
    HttpResponse::Created().body("Item created")
}  