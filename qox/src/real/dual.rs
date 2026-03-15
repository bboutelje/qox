use std::ops::{Add, Div, Mul, Neg, Sub, SubAssign};

use crate::traits::real::Real;

//#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Dual {
    pub val: f64,  // The value: f(x)
    pub grad: f64, // The derivative: f'(x)
}

impl Dual {
    #[inline]
    #[allow(dead_code)]
    fn val(&self) -> f64 {
        self.val
    }

    #[inline]
    pub fn new(val: f64, grad: f64) -> Self {
        Self { val, grad }
    }

    #[inline]
    /// Seed a variable for differentiation (dx/dx = 1)
    pub fn var(val: f64) -> Self {
        Self { val, grad: 1.0 }
    }

    #[inline]
    pub fn constant(val: f64) -> Self {
        Self { val, grad: 0.0 }
    }
}

impl Real for Dual {
    #[inline]
    fn from_f64(v: f64) -> Self {
        Dual::constant(v)
    }
    #[inline]
    fn scalar(self) -> f64 {
        self.val
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        if self.val >= other.val { self } else { other }
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        if self.val <= other.val { self } else { other }
    }

    #[inline]
    fn abs(self) -> Self {
        if self.val >= 0.0 {
            Dual::new(self.val, self.grad)
        } else {
            Dual::new(-self.val, -self.grad)
        }
    }

    #[inline]
    fn exp(self) -> Self {
        let res = self.val.exp();
        Dual::new(res, self.grad * res)
    }

    #[inline]
    fn ln(self) -> Self {
        Dual::new(self.val.ln(), self.grad / self.val)
    }

    #[inline]
    fn sqrt(self) -> Self {
        let res = self.val.sqrt();
        Dual::new(res, self.grad / (2.0 * res))
    }

    #[inline]
    fn powi(self, n: i32) -> Self {
        let val = self.val.powi(n);
        let grad = (n as f64) * self.val.powi(n - 1) * self.grad;
        Dual::new(val, grad)
    }

    #[inline]
    fn powf(self, n: Self) -> Self {
        let val = self.val.powf(n.val);
        let grad = val * (n.grad * self.val.ln() + n.val * self.grad / self.val);
        Dual::new(val, grad)
    }

    #[inline]
    fn norm_cdf(self) -> Self {
        todo!("")
        // let val = 0.5 * (1.0 + (self.val / 2.0f64.sqrt()).erf());
        // // Derivative of CDF is the PDF (Standard Normal)
        // let pdf = (-0.5 * self.val * self.val).exp() / (2.0 * std::f64::consts::PI).sqrt();
        // Dual::new(val, self.der * pdf)
    }

    #[inline]
    fn zero() -> Self {
        Self::constant(0.0)
    }

    #[inline]
    fn one() -> Self {
        Self::constant(1.0)
    }
}

// Implementation for: Dual / Dual
impl Div<Dual> for Dual {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        let inv_val = 1.0 / rhs.val; // One division
        let inv_v2 = inv_val * inv_val; // Multiplication
        Self {
            val: self.val * inv_val,
            grad: (self.grad * rhs.val - self.val * rhs.grad) * inv_v2,
        }
    }
}

// Implementation for: Dual * Dual
impl Mul<Dual> for Dual {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            val: self.val * rhs.val,
            grad: self.grad * rhs.val + self.val * rhs.grad,
        }
    }
}

// Implementation for: Dual + Dual
impl Add<Dual> for Dual {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            val: self.val + rhs.val,
            grad: self.grad + rhs.grad,
        }
    }
}

use std::ops::AddAssign;

// Allows: a += b
impl AddAssign for Dual {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.val += rhs.val;
        self.grad += rhs.grad;
    }
}

impl SubAssign for Dual {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.val -= rhs.val;
        self.grad -= rhs.grad;
    }
}

// Allows: a += &b
impl AddAssign<&Dual> for Dual {
    #[inline]
    fn add_assign(&mut self, rhs: &Dual) {
        self.val += rhs.val;
        self.grad += rhs.grad;
    }
}

impl SubAssign<&Dual> for Dual {
    #[inline]
    fn sub_assign(&mut self, rhs: &Dual) {
        self.val -= rhs.val;
        self.grad -= rhs.grad;
    }
}

// Implementation for: Dual - Dual
impl Sub<Dual> for Dual {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            val: self.val - rhs.val,
            grad: self.grad - rhs.grad,
        }
    }
}

impl Neg for Dual {
    type Output = Dual;
    #[inline]
    fn neg(self) -> Self::Output {
        Dual::new(-self.val, -self.grad)
    }
}

impl From<f64> for Dual {
    #[inline]
    fn from(v: f64) -> Self {
        Dual::constant(v)
    }
}
