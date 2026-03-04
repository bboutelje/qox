use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Real: 
    Sized + 
    Copy + 
    Add<Self, Output = Self> + 
    Sub<Self, Output = Self> + 
    Mul<Self, Output = Self> + 
    Div<Self, Output = Self> +
    Neg<Output = Self> +
    PartialOrd +
    PartialEq +
    From<f64>
{
    fn from_f64(v: f64) -> Self;
    fn to_f64(self) -> f64; // No longer &self

    fn max(self, other: Self) -> Self; // No longer &self or &Self

    fn exp(self) -> Self;
    fn ln(self) -> Self;
    fn sqrt(self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn powf(self, n: Self) -> Self; // No longer &Self
    fn norm_cdf(self) -> Self;

    fn zero() -> Self {
        Self::from_f64(0.0)
    }

    fn one() -> Self {
        Self::from_f64(1.0)
    }
}