use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use std::sync::Arc;
use crate::db::movies::MovieRepository;
use crate::models::movie::Model;
use crate::models::movie::CreateMovieDto;

// GET /movies
#[utoipa::path(
    get,
    path = "/movies",
    tag = "Movies",
    responses(
        (status = 200, description = "List movies", body = [Model]),
        (status = 500, description = "Error while fetching movies", body = String)
    )
)]
#[get("/movies")]
pub async fn get_all(repo: web::Data<Arc<dyn MovieRepository>>) -> impl Responder {
    match repo.get_all().await {
        Ok(movies) => HttpResponse::Ok().json(movies),
        Err(_) => HttpResponse::InternalServerError().json("Error while fetching movies"),
    }
}

// GET /movies/{id}
#[utoipa::path(
    get,
    path = "/movies/{id}",
    tag = "Movies",
    params(
        ("id" = i32, Path, description = "Movie id")
    ),
    responses(
        (status = 200, description = "Movie found", body = Model),
        (status = 404, description = "Movie not found", body = String),
        (status = 500, description = "Internal server error")
    )
)]
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
#[utoipa::path(
    post,
    path = "/movies",
    tag = "Movies",
    request_body = CreateMovieDto,
    responses(
        (status = 201, description = "Movie created", body = Model),
        (status = 500, description = "Internal server error", body = String)
    )
)]
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
#[utoipa::path(
    put,
    path = "/movies/{id}",
    tag = "Movies",
    params(
        ("id" = i32, Path, description = "Movie id")
    ),
    request_body = CreateMovieDto,
    responses(
        (status = 200, description = "Movie updated", body = Model),
        (status = 404, description = "Movie not found", body = String)
    )
)]
#[put("/movies/{id}")]
pub async fn update(
    repo: web::Data<Arc<dyn MovieRepository>>,
    id: web::Path<i32>,
    body: web::Json<CreateMovieDto>
) -> impl Responder {
    let id = id.into_inner();
    let updated_movie = Model {
        id,
        title: body.title.clone(),
        genre: body.genre.clone(),
        poster: body.poster.clone(),
        year: body.year,
        rating: body.rating,
    };

    match repo.update(id, updated_movie).await {
        Ok(movie) => HttpResponse::Ok().json(movie),
        Err(_) => HttpResponse::NotFound().json("Error: Movie not found"),
    }
}


// DELETE /movies/{id}
#[utoipa::path(
    delete,
    path = "/movies/{id}",
    tag = "Movies",
    params(
        ("id" = i32, Path, description = "Movie id")
    ),
    responses(
        (status = 204, description = "Movie deleted"),
        (status = 404, description = "Movie not found", body = String)
    )
)]
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