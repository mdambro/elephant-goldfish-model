//! Domain layer: entities, value objects and domain errors. No I/O.
//!
//! One module per responsibility, not one file per type: a module groups the types that change
//! together and keeps everything else private.

pub mod entities;
pub mod errors;
pub mod value_objects;
