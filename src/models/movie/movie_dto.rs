use serde::Deserialize;


#[derive(Deserialize)]
pub struct CreateMovieDto {
    pub title: String,
    pub genre: String,
    pub poster: String,
    pub year: i32,
    pub rating: f32,
}

#[derive(Deserialize)]
pub struct UpdateMovieDto {
    pub title: Option<String>,
    pub rating: Option<f32>,
}