use actix_web::{web, App, HttpServer};
use crate::app_state::AppState;
use crate::features::items::routes as items_routes;

pub async fn run(app_state: web::Data<AppState>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(items_routes::configure)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}  