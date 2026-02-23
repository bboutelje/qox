pub mod conventions;
pub mod rate;
pub mod yield_curve;
pub mod interpolate;
pub mod error;
pub mod instruments;
pub mod engines;
pub mod real;
pub mod market;
pub mod tenor;
pub mod traits;
pub mod period;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Days(pub i64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Years(pub f64);

