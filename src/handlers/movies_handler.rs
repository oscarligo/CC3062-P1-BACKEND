use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use std::sync::Arc;
use crate::db::movies::MovieRepository;
use crate::entities::movie::Model;
use crate::dto::CreateMovie; // Tu DTO de entrada

// GET /series
#[get("/series")]
pub async fn get_all(repo: web::Data<Arc<dyn MovieRepository>>) -> impl Responder {
    match repo.get_all().await {
        Ok(movies) => HttpResponse::Ok().json(movies),
        Err(_) => HttpResponse::InternalServerError().json("Error al obtener películas"),
    }
}

// GET /series/{id}
#[get("/series/{id}")]
pub async fn get_by_id(
    repo: web::Data<Arc<dyn MovieRepository>>, 
    id: web::Path<i32>
) -> impl Responder {
    match repo.get_by_id(id.into_inner()).await {
        Ok(Some(movie)) => HttpResponse::Ok().json(movie),
        Ok(None) => HttpResponse::NotFound().json("Película no encontrada"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// POST /series
#[post("/series")]
pub async fn create(
    repo: web::Data<Arc<dyn MovieRepository>>, 
    body: web::Json<CreateMovie>
) -> impl Responder {
    // Mapeamos el DTO al Modelo de la entidad
    let new_movie = Model {
        id: 0, // Sea-ORM ignorará esto al insertar si es ActiveModel
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

// PUT /series/{id}
#[put("/series/{id}")]
pub async fn update(
    repo: web::Data<Arc<dyn MovieRepository>>,
    id: web::Path<i32>,
    body: web::Json<Model>
) -> impl Responder {
    match repo.update(id.into_inner(), body.into_inner()).await {
        Ok(movie) => HttpResponse::Ok().json(movie),
        Err(_) => HttpResponse::NotFound().json("No se pudo actualizar la película"),
    }
}


// DELETE /series/{id}
#[delete("/series/{id}")]
pub async fn delete(
    repo: web::Data<Arc<dyn MovieRepository>>, 
    id: web::Path<i32>
) -> impl Responder {
    match repo.delete(id.into_inner()).await {
        Ok(rows) if rows > 0 => HttpResponse::NoContent().finish(),
        _ => HttpResponse::NotFound().json("La película no existe"),
    }
}