use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use std::sync::Arc;
use crate::db::movies::MovieRepository;
use crate::models::movie::Model;
use crate::models::movie::CreateMovieDto;

// GET /movies
#[get("/movies")]
pub async fn get_all(repo: web::Data<Arc<dyn MovieRepository>>) -> impl Responder {
    match repo.get_all().await {
        Ok(movies) => HttpResponse::Ok().json(movies),
        Err(_) => HttpResponse::InternalServerError().json("Error while fetching movies"),
    }
}

// GET /movies/{id}
#[get("/movies/{id}")]
pub async fn get_by_id(
    repo: web::Data<Arc<dyn MovieRepository>>, 
    id: web::Path<i32>
) -> impl Responder {
    match repo.get_by_id(id.into_inner()).await {
        Ok(Some(movie)) => HttpResponse::Ok().json(movie),
        Ok(None) => HttpResponse::NotFound().json("Movie not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// POST /movies
#[post("/movies")]
pub async fn create(
    repo: web::Data<Arc<dyn MovieRepository>>, 
    body: web::Json<CreateMovieDto>
) -> impl Responder {
    let new_movie = Model {
        id: 0, 
        title: body.title.clone(),
        genre: body.genre.clone(),
        poster: body.poster.clone(),
        year: body.year,
        rating: body.rating,
    };

    match repo.create(new_movie).await {
        Ok(movie) => HttpResponse::Created().json(movie),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// PUT /movies/{id}
#[put("/movies/{id}")]
pub async fn update(
    repo: web::Data<Arc<dyn MovieRepository>>,
    id: web::Path<i32>,
    body: web::Json<Model>
) -> impl Responder {
    match repo.update(id.into_inner(), body.into_inner()).await {
        Ok(movie) => HttpResponse::Ok().json(movie),
        Err(_) => HttpResponse::NotFound().json("Error: Movie not found"),
    }
}


// DELETE /movies/{id}
#[delete("/movies/{id}")]
pub async fn delete(
    repo: web::Data<Arc<dyn MovieRepository>>, 
    id: web::Path<i32>
) -> impl Responder {
    match repo.delete(id.into_inner()).await {
        Ok(rows) if rows > 0 => HttpResponse::NoContent().finish(),
        _ => HttpResponse::NotFound().json("Error: Movie not found"),
    }
}