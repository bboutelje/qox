use std::{ops::{Add, Div, Mul, Neg, Sub}};
use crate::traits::real::Real;
use num_dual::{Derivative, DualNum, DualStruct, DualVec};
use nalgebra::Const;

#[derive(Debug, Clone, Copy)]
pub struct DualVec64<const N: usize>(pub DualVec<f64, f64, Const<N>>);

impl<const N: usize> Real for DualVec64<N> {
    fn from_f64(v: f64) -> Self {
        DualVec64(DualVec::from_re(v))
    }

    fn to_f64(&self) -> f64 {
        self.0.re()
    }

    fn max(&self, other: &Self) -> Self {
        if self.0.re() >= other.0.re() {
            *self
        } else {
            *other
        }
    }

    fn exp(&self) -> Self {
        DualVec64(self.0.exp())
    }

    fn ln(&self) -> Self {
        DualVec64(self.0.ln())
    }

    fn sqrt(&self) -> Self {
        DualVec64(self.0.sqrt())
    }

    fn powi(&self, n: i32) -> Self {
        DualVec64(self.0.powi(n))
    }

    fn powf(&self, n: &Self) -> Self {
        let ln_base = self.ln();
        let exponent_ln_base = n * &ln_base;
        exponent_ln_base.exp()
    }

    fn norm_cdf(&self) -> Self {
        let x = self.0.re();
        let cdf_val = 0.5 * (1.0 + libm::erf(x / std::f64::consts::SQRT_2));
        let pdf_val = (-0.5 * x * x).exp() / (2.0 * std::f64::consts::PI).sqrt();

        let new_matrix = self.0.eps
            .unwrap_generic(Const::<N>, nalgebra::U1)
            .map(|component| component * pdf_val);

        DualVec64(DualVec::new(cdf_val, Derivative::some(new_matrix)))
    }
}

// --- Implement Reference Operators (The "Magic" part) ---

impl<'a, const N: usize> Add<&'a DualVec64<N>> for DualVec64<N> {
    type Output = Self;
    fn add(self, rhs: &'a Self) -> Self { DualVec64(self.0 + rhs.0) }
}

impl<'a, const N: usize> Sub<&'a DualVec64<N>> for DualVec64<N> {
    type Output = Self;
    fn sub(self, rhs: &'a Self) -> Self { DualVec64(self.0 - rhs.0) }
}

impl<'a, const N: usize> Mul<&'a DualVec64<N>> for DualVec64<N> {
    type Output = Self;
    fn mul(self, rhs: &'a Self) -> Self { DualVec64(self.0 * rhs.0) }
}

impl<'a, const N: usize> Div<&'a DualVec64<N>> for DualVec64<N> {
    type Output = Self;
    fn div(self, rhs: &'a Self) -> Self { DualVec64(self.0 / rhs.0) }
}

// --- Standard Boilerplate ---

impl<const N: usize> From<f64> for DualVec64<N> {
    fn from(v: f64) -> Self { Self::from_f64(v) }
}

impl<const N: usize> PartialEq for DualVec64<N> {
    fn eq(&self, other: &Self) -> bool { self.0.re() == other.0.re() }
}

impl<const N: usize> Neg for DualVec64<N> {
    type Output = Self;
    fn neg(self) -> Self { DualVec64(-self.0) }
}

// Reference * Reference
impl<'a, 'b, const N: usize> Mul<&'b DualVec64<N>> for &'a DualVec64<N> {
    type Output = DualVec64<N>;
    fn mul(self, rhs: &'b DualVec64<N>) -> DualVec64<N> {
        DualVec64(self.0 * rhs.0)
    }
}

// Reference + Reference (You'll need this for other math)
impl<'a, 'b, const N: usize> Add<&'b DualVec64<N>> for &'a DualVec64<N> {
    type Output = DualVec64<N>;
    fn add(self, rhs: &'b DualVec64<N>) -> DualVec64<N> {
        DualVec64(self.0 + rhs.0)
    }
}

// Reference - Reference
impl<'a, 'b, const N: usize> Sub<&'b DualVec64<N>> for &'a DualVec64<N> {
    type Output = DualVec64<N>;
    fn sub(self, rhs: &'b DualVec64<N>) -> DualVec64<N> {
        DualVec64(self.0 - rhs.0)
    }
}

// Reference / Reference
impl<'a, 'b, const N: usize> Div<&'b DualVec64<N>> for &'a DualVec64<N> {
    type Output = DualVec64<N>;
    fn div(self, rhs: &'b DualVec64<N>) -> DualVec64<N> {
        DualVec64(self.0 / rhs.0)
    }
}

// Reference Negation (Needed for exp(-r*t))
impl<'a, const N: usize> Neg for &'a DualVec64<N> {
    type Output = DualVec64<N>;
    fn neg(self) -> DualVec64<N> {
        DualVec64(-self.0)
    }
}