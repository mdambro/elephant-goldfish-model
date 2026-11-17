# Proyecto de inducción

Servicio de registro de usuarios que acompaña la guía *Patrones, arquitectura y stack en Rust*
(`../Patrones_y_arquitectura_en_Rust.pdf`). Expone el mismo caso de uso por REST (axum) y por gRPC
(tonic), y guarda en PostgreSQL (tokio-postgres con deadpool-postgres).

## Estructura

| Crate | Capa | Qué tiene |
| --- | --- | --- |
| `crates/domain` | Dominio | `entities` (`User`), `value_objects` (`Email`, `UserId`), `errors` (`DomainError`). |
| `crates/application` | Aplicación | `register_user` (comando, error, reglas propias y `RegisterUserUseCase`), `ports` (`UserRepository`), `rules` (reglas compartidas, como `EmailNotTaken`). |
| `crates/proto` | Contrato gRPC | `users.proto` y el código generado. |
| `crates/infrastructure` | Infraestructura | `http` (axum), `grpc` (tonic), `persistence` (PostgreSQL) y `main`. |

Los archivos se agrupan por responsabilidad, no uno por tipo: un módulo reúne lo que cambia junto
(por ejemplo, el comando, el error y el caso de uso de `register_user`) y deja privado lo demás.

## Requisitos

- Rust (con `rustup`), `protoc` y Docker. La sección 0.2 de la guía explica cómo instalarlos.

## Primer paso: compilar

```text
cargo check --workspace
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Las capas `domain` y `application` se verificaron con tests y clippy. `proto` e `infrastructure`
se escribieron contra el código fuente de axum 0.8.9, tonic 0.14.9, tokio-postgres 0.7.18 y
deadpool-postgres 0.14.2: si `cargo check` marca algo, corregirlo es el primer ejercicio.

## Correr el servicio

```text
docker run --name pg -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres
docker exec -i pg psql -U postgres < migrations/0001_create_users.sql
DATABASE_URL=postgres://postgres:postgres@localhost/postgres cargo run -p infrastructure
```

En PowerShell, definí la variable con `$env:DATABASE_URL = "postgres://postgres:postgres@localhost/postgres"`.

REST queda en `http://localhost:8080` y gRPC en `localhost:50051`.

```text
curl -i -X POST localhost:8080/users -H 'content-type: application/json' \
  -d '{"email":"ana@example.com","age":30,"accepted_terms":true}'

grpcurl -plaintext -import-path crates/proto/proto -proto users/v1/users.proto \
  -d '{"email":"luz@example.com","age":25,"accepted_terms":true}' \
  localhost:50051 users.v1.UserService/RegisterUser
```

## Dependencia de `validations_rs`

Mientras la versión 0.3.0 no esté publicada en crates.io, el workspace la toma del repositorio,
fijada a un commit (`rev`), que sigue disponible aunque se borre la rama. Cuando se publique,
cambiá esa línea del `Cargo.toml` del workspace por `validations_rs = "0.3"`.
