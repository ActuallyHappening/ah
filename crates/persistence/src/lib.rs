pub mod prelude;

pub use persistence::*;
mod persistence;

pub mod errors;
pub mod sidbo;

// re-export for macros
pub use surrealdb;
