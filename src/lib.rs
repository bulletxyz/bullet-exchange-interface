//! Structures to interface the exchange.

#![allow(clippy::large_enum_variant)]

pub mod address;
pub mod bounds;
pub mod decimals;
pub mod error;
pub mod event;
mod macros;
pub mod message;
pub mod schema;
pub mod string;
pub mod time;
pub mod transaction;
pub mod types;

/// Re-export SafeVec for convenience.
pub use sov_rollup_interface::common::SafeVec;
