# CC3062 — Proyecto 1 (Backend)

Backend  que expone una **API REST ** para gestionar **Películas**.

Repositorio conn el front-end: <https://github.com/oscarligo/CC3062-P1-FRONTEND.git>

Proyecto funcionando en internet: <https://rompich.site>

<img width="1440" height="900" alt="Screenshot 2026-05-06 at 11 38 55 PM" src="https://github.com/user-attachments/assets/2e8ca11f-7c0b-4d5f-bcf4-ab7275881485" />


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
- Documentación: Swagger

## Challenges Implementados

- Spec de OpenAPI/Swagger escrita y precisa (el contrato de la API en YAML o JSON)
- Swagger UI corriendo y siendo servido desde el backend (no solo el archivo)	20
- Códigos HTTP correctos en toda la API (201 al crear, 204 al eliminar, 404 si no existe, 400 en input inválido, etc.)
- Validación server-side con respuestas de error en JSON descriptivas

## Reflexión sobre el uso de tecnologías

Sin duda volvería a desarrollar una API en Rust. Las macros han sido una de las características que más me han gustado, ya que permiten escribir un código más limpio y legible. Además, durante el proyecto me animé a utilizar un ORM por primera vez, lo que ayudó a reducir la cantidad de líneas de código, sin quitarle complejidad a las operaciones a la base de datos.

También me resultó muy interesante implementar la documentación con Swagger. Gracias a las macros de Utopia, fue relativamente sencillo hacerlo. En resumen, Rust y los crates que utilicé aportaron complejidad al proyecto, pero manteniendo un código reducido, ordenado y escalable.


---


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

