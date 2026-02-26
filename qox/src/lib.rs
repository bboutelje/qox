pub mod instruments;
pub mod evaluators;
pub mod real;
pub mod market;
pub mod traits;
pub mod core;
pub mod math;
pub mod solvers;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Days(pub i64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Years(pub f64);

