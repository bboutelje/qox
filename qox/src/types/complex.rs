use num_complex::Complex;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexWrapper(pub Complex<f64>);

impl PartialOrd for ComplexWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.norm().partial_cmp(&other.0.norm())
    }
}

impl From<f64> for ComplexWrapper {
    fn from(val: f64) -> Self {
        ComplexWrapper(Complex::new(val, 0.0))
    }
}

impl Real for ComplexWrapper {
    fn from_f64(val: f64) -> Self {
        ComplexWrapper(Complex::new(val, 0.0))
    }

    fn scalar(self) -> f64 {
        self.0.re
    }

    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }

    fn min(self, other: Self) -> Self {
        if self <= other { self } else { other }
    }

    fn abs(self) -> Self {
        ComplexWrapper(Complex::new(self.0.norm(), 0.0))
    }

    fn exp(self) -> Self {
        ComplexWrapper(self.0.exp())
    }

    fn ln(self) -> Self {
        ComplexWrapper(self.0.ln())
    }

    fn sqrt(self) -> Self {
        ComplexWrapper(self.0.sqrt())
    }

    fn powi(self, n: i32) -> Self {
        ComplexWrapper(self.0.powi(n))
    }

    fn powf(self, n: Self) -> Self {
        ComplexWrapper(self.0.powc(n.0))
    }

    fn norm_cdf(self) -> Self {
        let val = 0.5 * (1.0 + libm::erf(self.0.re / std::f64::consts::SQRT_2));
        ComplexWrapper(Complex::new(val, 0.0))
    }
}

use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

use crate::types::Real;

// --- Unary Negation ---
impl Neg for ComplexWrapper {
    type Output = Self;
    fn neg(self) -> Self {
        ComplexWrapper(-self.0)
    }
}

// --- Arithmetic Operators ---
impl Add for ComplexWrapper {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        ComplexWrapper(self.0 + other.0)
    }
}

impl AddAssign for ComplexWrapper {
    fn add_assign(&mut self, other: Self) {
        // Update the internal state directly
        self.0 += other.0;
    }
}

impl SubAssign for ComplexWrapper {
    fn sub_assign(&mut self, other: Self) {
        // Update the internal state directly
        self.0 -= other.0;
    }
}

impl Sub for ComplexWrapper {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        ComplexWrapper(self.0 - other.0)
    }
}

impl Mul for ComplexWrapper {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        ComplexWrapper(self.0 * other.0)
    }
}

impl Div for ComplexWrapper {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        ComplexWrapper(self.0 / other.0)
    }
}
