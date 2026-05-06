use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
use crate::db::movies::{MovieRepository, SeaOrmMovieRepository};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use sea_orm::Database;
mod models;
mod db;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env variables
    dotenv().ok();
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let port: u16 = env::var("BACKEND_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    // Database connection pool
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // Movie repository instance
    let movie_repo: Arc<dyn MovieRepository> = Arc::new(SeaOrmMovieRepository { db: db.clone() });

    println!("Server running on http://localhost:{}", port);

    // Actix-web server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![header::ACCEPT, header::CONTENT_TYPE])
            .max_age(3600);
    // App instance with CORS and route configuration
        App::new()
            .app_data(web::Data::new(movie_repo.clone()))
            .wrap(cors)
            .service(handlers::movies_handler::get_all)
            .service(handlers::movies_handler::get_by_id)
            .service(handlers::movies_handler::create)
            .service(handlers::movies_handler::update)
            .service(handlers::movies_handler::delete)
            
    
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}