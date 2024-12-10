// #![doc = include_str!("../../README.md")]
//! Rust bindings to the [Frankfurter API](https://github.com/lineofflight/frankfurter)
//!
//! ## Usage
//!
//! ```rust
#![doc = include_str!("../examples/basic.rs")]
//! ```

pub mod api;
mod data;
mod error;

// RE-EXPORTS
pub use chrono;
pub use data::*;
pub use error::*;
pub use reqwest;
pub use serde_json;
pub use url;
