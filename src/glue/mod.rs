//! Code for putting together compilation steps.

mod pipeline;
pub use self::pipeline::pipeline;

mod interpret;
pub use self::interpret::{interpret_pipeline, interpret_statement, interpret};

mod transpile;
pub use self::transpile::transpile;
