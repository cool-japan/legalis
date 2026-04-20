//! Test modules for legalis-i18n
//!
//! Re-exports from the parent crate so test submodules can use `use super::*;`.

#[allow(unused_imports)]
pub use super::*;
#[allow(unused_imports)]
pub use std::collections::HashMap;
#[allow(unused_imports)]
pub use std::sync::{Arc, Mutex};

mod advanced;
mod basic;
