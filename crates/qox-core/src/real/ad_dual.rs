use std::ops::{Add, Sub, Mul, Div, Neg};

use crate::traits::real::Real;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dual {
    pub val: f64, // The value: f(x)
    pub der: f64, // The derivative: f'(x)
}

impl Dual {
    pub fn new(val: f64, der: f64) -> Self {
        Self { val, der }
    }

    /// Seed a variable for differentiation (dx/dx = 1)
    pub fn var(val: f64) -> Self {
        Self { val, der: 1.0 }
    }

    /// Constant value (dc/dx = 0)
    pub fn constant(val: f64) -> Self {
        Self { val, der: 0.0 }
    }
}

// Implementation for: Dual + &Dual
impl<'a> Add<&'a Dual> for Dual {
    type Output = Dual;
    fn add(self, rhs: &'a Dual) -> Self::Output {
        Dual::new(self.val + rhs.val, self.der + rhs.der)
    }
}

// Implementation for: Dual - &Dual
impl<'a> Sub<&'a Dual> for Dual {
    type Output = Dual;
    fn sub(self, rhs: &'a Dual) -> Self::Output {
        Dual::new(self.val - rhs.val, self.der - rhs.der)
    }
}

// Implementation for: Dual * &Dual (Product Rule)
impl<'a> Mul<&'a Dual> for Dual {
    type Output = Dual;
    fn mul(self, rhs: &'a Dual) -> Self::Output {
        // (uv)' = u'v + uv'
        Dual::new(
            self.val * rhs.val,
            self.der * rhs.val + self.val * rhs.der,
        )
    }
}

// Implementation for: Dual / &Dual (Quotient Rule)
impl<'a> Div<&'a Dual> for Dual {
    type Output = Dual;
    fn div(self, rhs: &'a Dual) -> Self::Output {
        // (u/v)' = (u'v - uv') / v^2
        let v2 = rhs.val * rhs.val;
        Dual::new(
            self.val / rhs.val,
            (self.der * rhs.val - self.val * rhs.der) / v2,
        )
    }
}

impl Neg for Dual {
    type Output = Dual;
    fn neg(self) -> Self::Output {
        Dual::new(-self.val, -self.der)
    }
}

impl From<f64> for Dual {
    fn from(v: f64) -> Self {
        Dual::constant(v)
    }
}

impl Real for Dual {
    fn from_f64(v: f64) -> Self {
        Dual::constant(v)
    }

    fn to_f64(&self) -> f64 {
        self.val
    }

    fn max(&self, other: &Self) -> Self {
        if self.val >= other.val { *self } else { *other }
    }

    fn exp(&self) -> Self {
        let res = self.val.exp();
        Dual::new(res, self.der * res)
    }

    fn ln(&self) -> Self {
        Dual::new(self.val.ln(), self.der / self.val)
    }

    fn sqrt(&self) -> Self {
        let res = self.val.sqrt();
        Dual::new(res, self.der / (2.0 * res))
    }

    fn powi(&self, n: i32) -> Self {
        let val = self.val.powi(n);
        let der = (n as f64) * self.val.powi(n - 1) * self.der;
        Dual::new(val, der)
    }

    fn powf(&self, n: &Self) -> Self {
        let val = self.val.powf(n.val);
        // Generalized power rule: d/dx (f^g) = f^g * (g' ln f + g f' / f)
        let der = val * (n.der * self.val.ln() + n.val * self.der / self.val);
        Dual::new(val, der)
    }

    fn norm_cdf(&self) -> Self {
        todo!("")
        // let val = 0.5 * (1.0 + (self.val / 2.0f64.sqrt()).erf());
        // // Derivative of CDF is the PDF (Standard Normal)
        // let pdf = (-0.5 * self.val * self.val).exp() / (2.0 * std::f64::consts::PI).sqrt();
        // Dual::new(val, self.der * pdf)
    }
}

// If your environment doesn't have erf(), you might use the `libm` crate 
// or a numerical approximation to keep things dependency-free.