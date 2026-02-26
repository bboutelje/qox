use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::traits::real::Real; // Adjust path to your Real trait

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Gradient<const N: usize> {
    pub val: f64,
    pub grad: [f64; N],
}

impl<const N: usize> Gradient<N> {
    pub fn constant(val: f64) -> Self {
        Self { val, grad: [0.0; N] }
    }

    pub fn var(val: f64, index: usize) -> Self {
        let mut grad = [0.0; N];
        if index < N { grad[index] = 1.0; }
        Self { val, grad }
    }
}

// --- Trait Bounds: Value + &Reference (Required by Real) ---

impl<'a, const N: usize> Add<&'a Gradient<N>> for Gradient<N> {
    type Output = Self;
    fn add(self, rhs: &'a Self) -> Self {
        Self {
            val: self.val + rhs.val,
            grad: std::array::from_fn(|i| self.grad[i] + rhs.grad[i]),
        }
    }
}

impl<'a, const N: usize> Sub<&'a Gradient<N>> for Gradient<N> {
    type Output = Self;
    fn sub(self, rhs: &'a Self) -> Self {
        Self {
            val: self.val - rhs.val,
            grad: std::array::from_fn(|i| self.grad[i] - rhs.grad[i]),
        }
    }
}

impl<'a, const N: usize> Mul<&'a Gradient<N>> for Gradient<N> {
    type Output = Self;
    fn mul(self, rhs: &'a Self) -> Self {
        Self {
            val: self.val * rhs.val,
            grad: std::array::from_fn(|i| self.grad[i] * rhs.val + self.val * rhs.grad[i]),
        }
    }
}

impl<'a, const N: usize> Div<&'a Gradient<N>> for Gradient<N> {
    type Output = Self;
    fn div(self, rhs: &'a Self) -> Self {
        let v2 = rhs.val * rhs.val;
        Self {
            val: self.val / rhs.val,
            grad: std::array::from_fn(|i| (self.grad[i] * rhs.val - self.val * rhs.grad[i]) / v2),
        }
    }
}

impl<const N: usize> Neg for Gradient<N> {
    type Output = Self;
    fn neg(self) -> Self {
        Self { val: -self.val, grad: self.grad.map(|da| -da) }
    }
}

impl<const N: usize> From<f64> for Gradient<N> {
    fn from(v: f64) -> Self { Self::constant(v) }
}

// --- Implementation of the Real Trait ---

impl<const N: usize> Real for Gradient<N> {
    fn from_f64(v: f64) -> Self { Self::constant(v) }
    fn to_f64(&self) -> f64 { self.val }
    fn max(&self, other: &Self) -> Self { if self.val >= other.val { *self } else { *other } }

    fn exp(&self) -> Self {
        let res = self.val.exp();
        Self { val: res, grad: self.grad.map(|da| da * res) }
    }

    fn ln(&self) -> Self {
        Self { val: self.val.ln(), grad: self.grad.map(|da| da / self.val) }
    }

    fn sqrt(&self) -> Self {
        let res = self.val.sqrt();
        Self { val: res, grad: self.grad.map(|da| da / (2.0 * res)) }
    }

    fn powi(&self, n: i32) -> Self {
        let val = self.val.powi(n);
        let factor = (n as f64) * self.val.powi(n - 1);
        Self { val, grad: self.grad.map(|da| da * factor) }
    }

    fn powf(&self, n: &Self) -> Self {
        let val = self.val.powf(n.val);
        Self {
            val,
            grad: std::array::from_fn(|i| val * (n.grad[i] * self.val.ln() + n.val * self.grad[i] / self.val)),
        }
    }

    fn norm_cdf(&self) -> Self {
        let val = 0.5 * (1.0 + erf(self.val / 2.0f64.sqrt()));
        let pdf = (-0.5 * self.val * self.val).exp() / (2.0 * std::f64::consts::PI).sqrt();
        Self { val, grad: self.grad.map(|da| da * pdf) }
    }
}

// Internal helper for norm_cdf
fn erf(x: f64) -> f64 {
    let p = 0.3275911;
    let a = [0.254829592, -0.284496736, 1.421413741, -1.453152027, 1.061405429];
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - ((((a[4]*t + a[3])*t + a[2])*t + a[1])*t + a[0])*t * (-x*x).exp();
    sign * y
}

// Implementation for: &'a Gradient - &'b Gradient
impl<'a, 'b, const N: usize> Sub<&'b Gradient<N>> for &'a Gradient<N> {
    type Output = Gradient<N>;
    fn sub(self, rhs: &'b Gradient<N>) -> Self::Output {
        Gradient {
            val: self.val - rhs.val,
            grad: std::array::from_fn(|i| self.grad[i] - rhs.grad[i]),
        }
    }
}

// Implementation for: &'a Gradient + &'b Gradient
impl<'a, 'b, const N: usize> Add<&'b Gradient<N>> for &'a Gradient<N> {
    type Output = Gradient<N>;
    fn add(self, rhs: &'b Gradient<N>) -> Self::Output {
        Gradient {
            val: self.val + rhs.val,
            grad: std::array::from_fn(|i| self.grad[i] + rhs.grad[i]),
        }
    }
}

// Implementation for: &'a Gradient * &'b Gradient
impl<'a, 'b, const N: usize> Mul<&'b Gradient<N>> for &'a Gradient<N> {
    type Output = Gradient<N>;
    fn mul(self, rhs: &'b Gradient<N>) -> Self::Output {
        Gradient {
            val: self.val * rhs.val,
            grad: std::array::from_fn(|i| self.grad[i] * rhs.val + self.val * rhs.grad[i]),
        }
    }
}

// Implementation for: &'a Gradient / &'b Gradient
impl<'a, 'b, const N: usize> Div<&'b Gradient<N>> for &'a Gradient<N> {
    type Output = Gradient<N>;
    fn div(self, rhs: &'b Gradient<N>) -> Self::Output {
        let v2 = rhs.val * rhs.val;
        Gradient {
            val: self.val / rhs.val,
            grad: std::array::from_fn(|i| (self.grad[i] * rhs.val - self.val * rhs.grad[i]) / v2),
        }
    }
}

// Implementation for: -&Gradient
impl<'a, const N: usize> Neg for &'a Gradient<N> {
    type Output = Gradient<N>;
    fn neg(self) -> Self::Output {
        Gradient {
            val: -self.val,
            grad: self.grad.map(|da| -da),
        }
    }
}