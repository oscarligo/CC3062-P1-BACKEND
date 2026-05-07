# CC3062 — Proyecto 1 (Backend)

Backend  que expone una **API REST ** para gestionar **Películas**.

Repositorio conn el front-end: <https://github.com/oscarligo/CC3062-P1-FRONTEND.git>

Proyecto funcionando en internet: <https://rompich.site>

<img width="680" height="340" alt="Screenshot 2026-05-06 at 8 48 28 PM" src="https://github.com/user-attachments/assets/d87a2a59-7469-44cf-92e4-a3fa6a5bca5e" />

### Requisitos para levantar.

1. Docker y Docker Compose
2. Copiar .env.example a un .env local
3. En la raíz del proyecto levantar con:

``` shell
docker compose up --build 
```


## Tecnologías

- Lenguaje: Rust con actix-web
- DBMS: PostgreSQL
- ORM: SeaORM
- Contenedores: Docker + Docker Compose

## Configuración de CORS
Definición: CORS (Cross-Origin Resource Sharing) es una política de seguridad que los navegadores aplican para prevenir que un script en un origen acceda a recursos de otro origen sin permiso explícito.

Configuración: Se implementó un middleware que añade las cabeceras Access-Control-Allow-Origin: * y permite los métodos GET, POST, PUT, DELETE, OPTIONS, garantizando que el cliente de JavaScript vanilla pueda consumir la API mediante fetch().

```rust
let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![header::ACCEPT, header::CONTENT_TYPE])
            .max_age(3600);

```

---

## Estructura del proyecto

Arquitectura por capas: **handlers → repositorio → DB**

```
├── database/                               # DML de la base de datos
│ └── init.sql
├── src/
│ ├── main.rs                               # Servidor HTTP + CORS + rutas
│ ├── handlers/
│ │ └── movies_handler.rs                   # Endpoints (/movies)
│ ├── db/                                   # Repositorios para interactuar con la DB
│ │ └── movies/
│ │     ├── movies_repository.rs
│ │     └── movies_repository_impl.rs
│ └── models/                               # Modelos de datos     
│   └── movies/
│       ├── movies.rs                       # Modelo con sea-orm 
│       └── movies_dto.rs                   # DTOs
├── Dockerfile
├── docker-compose.yml
└── Cargo.toml

```

---

## API REST

Base URL: `http://localhost:${BACKEND_PORT}`

## Swagger / OpenAPI

- Swagger UI: `http://localhost:${BACKEND_PORT}/swagger-ui/`
- OpenAPI JSON: `http://localhost:${BACKEND_PORT}/api-doc/openapi.json`

### GET /movies
Lista todas las peliculas

### GET /movies/{id}
Obtiene una pelicula por ID

### POST /movies
Crea una pelicula

### PUT /movies/{id}
Actualiza una palicula por ID

### DELETE /movies/{id}
Elimina una pelicula por ID

