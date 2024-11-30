use actix_web::{web, App, HttpServer};
use sea_orm::Database;

use services::todo;

mod entities;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/todos", web::get().to(todo::get_todos))
            .route("/todos", web::post().to(todo::create_todo))
            .route("/todos/{id}", web::get().to(todo::get_todo))
            .route("/todos/{id}", web::put().to(todo::update_todo))
            .route("/todos/{id}", web::delete().to(todo::delete_todo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
