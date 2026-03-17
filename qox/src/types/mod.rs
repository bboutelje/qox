pub mod complex;
pub mod dual;
pub mod dual2_vec;
pub mod dual_array;
pub mod f64;
pub mod hyper_dual;
pub mod num_dual_vec;
pub mod reverse_node;

use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

pub trait Real:
    Sized
    + Copy
    + Debug
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Neg<Output = Self>
    + PartialOrd
    + PartialEq
    + From<f64>
{
    fn from_f64(v: f64) -> Self;
    fn scalar(self) -> f64; // No longer &self

    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;

    fn abs(self) -> Self;

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
