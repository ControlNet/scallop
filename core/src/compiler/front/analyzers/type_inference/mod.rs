//! # Type inference analysis

mod error;
mod function;
mod local;
mod type_inference;
mod type_set;
mod unification;

use super::super::utils::*;

pub use error::*;
pub use function::*;
pub use local::*;
pub use type_inference::*;
pub use type_set::*;
pub use unification::*;
