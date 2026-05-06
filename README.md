# CC3062 — Proyecto 1 (Backend)

Backend  que expone una **API REST ** para gestionar **Películas**.

Repositorio conn el front-end: <https://github.com/oscarligo/CC3062-P1-FRONTEND.git>

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

