# Inducción para nuevos desarrolladores

Si llegás al equipo desde otro lenguaje, este es el punto de partida. En el orden:

1. **Instalá este EGM** (el fork con el perfil Rust), no el de Google. Las instrucciones están en
   el [README del repositorio](../../README.md#install).
2. **Leé la guía** [Patrones, arquitectura y stack en Rust](Patrones_y_arquitectura_en_Rust.pdf).
   La Parte 0 tiene el plan de inducción por etapas, con un checkpoint en cada una.
3. **Trabajá sobre el [proyecto de inducción](induccion/README.md)**: el servicio que la guía
   construye módulo a módulo (Cargo, errores, Tokio, `validations_rs`, PostgreSQL, axum, gRPC).
   El primer paso es `cargo check --workspace`.
4. **Hacé el proyecto final** (Parte 8 de la guía) usando EGM, como cualquier funcionalidad real.

Las reglas que la guía enseña son las mismas que aplica el revisor `egm-rust` de EGM: el
[Rust Standard](../../resources/document_standards/rust-standard.md). Si cambia una regla del
equipo, se cambia en el estándar y se actualiza la guía.
