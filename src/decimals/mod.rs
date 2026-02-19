pub mod fixed_positive_decimal;
pub mod macros;
pub mod ops;
pub mod positive_decimal;
pub mod surrogate_decimal;

pub use fixed_positive_decimal::*;
pub use ops::*;
pub use positive_decimal::*;
pub use surrogate_decimal::*;

pub enum RoundingMode {
    Up,
    Down,
}
