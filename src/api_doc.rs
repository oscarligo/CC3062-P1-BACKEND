use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "CC3062 Movies API",
        version = "0.1.0"
    ),
    paths(
        crate::handlers::movies_handler::get_all,
        crate::handlers::movies_handler::get_by_id,
        crate::handlers::movies_handler::create,
        crate::handlers::movies_handler::update,
        crate::handlers::movies_handler::delete,
    ),
    components(
        schemas(
            crate::models::movie::Model,
            crate::models::movie::CreateMovieDto
        )
    ),
    tags(
        (name = "Movies", description = "Operaciones sobre películas")
    )
)]
pub struct ApiDoc;
