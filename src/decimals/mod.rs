pub mod fixed_positive_decimal;
pub mod macros;
pub mod ops;
pub mod positive_decimal;
pub mod surrogate_decimal;

pub use fixed_positive_decimal::*;
pub use ops::*;
pub use positive_decimal::*;
pub use surrogate_decimal::*;

/// Direction to round a [`PositiveDecimal`] when quantizing it to
/// [`FIXED_DECIMALS`] places.
///
/// Because the values being rounded are non-negative, [`RoundingMode::Up`] is a
/// ceiling and [`RoundingMode::Down`] is a floor - there is no half-way
/// (banker's) rounding.
///
/// Balances must always round in the exchange's favour, so the direction is
/// picked per field rather than per call site: what the user *owns* rounds
/// [`Down`](RoundingMode::Down) (assets, unsettled profit, trading credits,
/// spot) and what the user *owes* rounds [`Up`](RoundingMode::Up) (liabilities,
/// unrealized loss borrow). The same rule applies in scaled space, where a
/// floor/ceiling survives multiplication by a positive borrow-lend index.
pub enum RoundingMode {
    /// Round away from zero (ceiling). Use for amounts the user owes.
    Up,
    /// Round toward zero (floor). Use for amounts the user owns.
    Down,
}
