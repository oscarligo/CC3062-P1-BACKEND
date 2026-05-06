use serde::{Serialize, Deserialize};

/*
Movies model and DTO  definitions
*/


// Movie model
#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    pub id: i32,
    pub titulo: String,
    pub temporadas: i32,
    pub genero: String,
}

// DTO for creating a new movie
#[derive(Deserialize)]
pub struct CreateMovie {
    pub titulo: String,
    pub temporadas: i32,
    pub genero: String,
}