//! Structures to interface the exchange.

#![allow(clippy::large_enum_variant)]

pub mod address;
pub mod decimals;
pub mod error;
pub mod event;
#[cfg(feature = "fixtures")]
pub mod fixtures;
mod macros;
pub mod message;
#[cfg(feature = "schema")]
pub mod schema;
pub mod string;
pub mod time;
pub mod transaction;
pub mod types;
