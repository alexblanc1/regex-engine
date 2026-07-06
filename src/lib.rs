//! regex-engine — a regular-expression engine based on Brzozowski derivatives.
//!
//! See `README.md` for the overall design and roadmap.

pub mod ast;
pub mod derivative;
pub mod parser;
pub mod dfa;
pub mod search;

pub use ast::Reg;
pub use derivative::{derivative, eta, matches, nullable};
