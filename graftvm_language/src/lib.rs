//! Parlance — a minimal functional front-end language for GraftVM.
//!
//! Expression hierarchy (lowest to highest precedence):
//!
//! 1. **atomic** — literals (`42`, `3.14`, `true`), variable identifiers
//! 2. **application** — `f x y` (left-associative function call)
//! 3. **infix** — `a + b` desugars to `((declared_expr a) b)`
//!
//! # Example
//!
//! ```text
//! ;; declare an infix operator
//! (infix + 3)
//!
//! ;; function definition
//! (def add x y
//!   (+ x y))
//!
//! ;; expression
//! (+ 1 2)
//! ```

pub mod lexer;
pub mod parser;
pub mod lower;

pub use lexer::Token;
pub use parser::{Expr, parse};
pub use lower::lower;
