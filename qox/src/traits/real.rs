use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Real: 
    Sized + Clone +
    // This part is the "magic". It says: 
    // "You can use operators on references of any lifetime 'a"
    for<'a> Add<&'a Self, Output = Self> + 
    for<'a> Sub<&'a Self, Output = Self> + 
    for<'a> Mul<&'a Self, Output = Self> + 
    for<'a> Div<&'a Self, Output = Self> +
    Neg<Output = Self> +
    PartialEq +
    From<f64>
{
    fn from_f64(v: f64) -> Self;
    fn to_f64(&self) -> f64;
    fn from_real<R: Real>(v: R) -> Self {
        Self::from_f64(v.to_f64())
    }


    fn max(&self, other: &Self) -> Self;

    fn exp(&self) -> Self;
    fn ln(&self) -> Self;
    fn sqrt(&self) -> Self;
    fn powi(&self, n: i32) -> Self;
    fn powf(&self, n: &Self) -> Self;
    fn norm_cdf(&self) -> Self;
    fn zero() -> Self {
        Self::from_f64(0.0)
    }

    fn one() -> Self {
        Self::from_f64(1.0)
    }
}