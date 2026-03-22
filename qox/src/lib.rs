pub mod core;
pub mod evaluators;
pub mod instruments;
pub mod market;
pub mod math;
pub mod methods;
pub mod processes;
pub mod traits;
pub mod types;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Days(pub i64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Years(pub f64);
