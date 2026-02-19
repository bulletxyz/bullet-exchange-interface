#[macro_export]
macro_rules! pos_dec {
    ($($args:tt)+) => {{
        let decimal = rust_decimal_macros::dec!($($args)+);
        PositiveDecimal::new(decimal).expect("Invalid positive decimal literal")
    }};
}

#[macro_export]
macro_rules! fixed_pos_dec {
    ($($args:tt)+) => {{
        let decimal = pos_dec!($($args)+);
        FixedPositiveDecimal::new(decimal, RoundingMode::Down).expect("Invalid fixed positive decimal literal")
    }};
}

#[macro_export]
macro_rules! surr_dec {
    ($($args:tt)+) => {{
        let decimal = rust_decimal_macros::dec!($($args)+);
        SurrogateDecimal::from(decimal)
    }};
}
