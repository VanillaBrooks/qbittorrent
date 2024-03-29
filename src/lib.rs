#![doc = include_str!("../README.md")]
#![allow(unused_variables)]

#[macro_use]
pub(crate) mod macros;
pub(crate) mod utils;

mod api;
pub mod data;
mod error;
pub mod queries;
pub mod traits;

#[cfg(test)]
pub(crate) mod tests;

pub use api::Api;
pub use error::Error;
