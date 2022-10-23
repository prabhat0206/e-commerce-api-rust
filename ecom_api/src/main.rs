use actix_web::{web, App, HttpServer};
mod controller;
mod database;
mod serializers;
mod utilities;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = database::Database::init().await;
    let db_data = web::Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .configure(controller::init_routes)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
