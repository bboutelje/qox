use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

use crate::types::Real; // Adjust path to your Real trait

//#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct DualArray<const N: usize> {
    pub val: f64,
    pub grad: [f64; N],
}

impl<const N: usize> DualArray<N> {
    #[inline]
    fn constant(val: f64) -> Self {
        Self {
            val,
            grad: [0.0; N],
        }
    }

    #[inline]
    pub fn var(val: f64, index: usize) -> Self {
        let mut grad = [0.0; N];
        if index < N {
            grad[index] = 1.0;
        }
        Self { val, grad }
    }
}

// --- Trait Bounds: Value + &Reference (Required by Real) ---

impl<'a, const N: usize> Add<&'a DualArray<N>> for DualArray<N> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &'a Self) -> Self {
        Self {
            val: self.val + rhs.val,
            grad: std::array::from_fn(|i| self.grad[i] + rhs.grad[i]),
        }
    }
}

impl<const N: usize> AddAssign<&DualArray<N>> for DualArray<N> {
    #[inline]
    fn add_assign(&mut self, rhs: &DualArray<N>) {
        self.val += rhs.val;
        for i in 0..N {
            self.grad[i] += rhs.grad[i];
        }
    }
}

impl<const N: usize> SubAssign<&DualArray<N>> for DualArray<N> {
    #[inline]
    fn sub_assign(&mut self, rhs: &DualArray<N>) {
        self.val -= rhs.val;
        for i in 0..N {
            self.grad[i] -= rhs.grad[i];
        }
    }
}

impl<'a, const N: usize> Sub<&'a DualArray<N>> for DualArray<N> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &'a Self) -> Self {
        Self {
            val: self.val - rhs.val,
            grad: std::array::from_fn(|i| self.grad[i] - rhs.grad[i]),
        }
    }
}

impl<'a, const N: usize> Mul<&'a DualArray<N>> for DualArray<N> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &'a Self) -> Self {
        Self {
            val: self.val * rhs.val,
            grad: std::array::from_fn(|i| self.grad[i] * rhs.val + self.val * rhs.grad[i]),
        }
    }
}

impl<'a, const N: usize> Div<&'a DualArray<N>> for DualArray<N> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: &'a Self) -> Self {
        let v2 = rhs.val * rhs.val;
        Self {
            val: self.val / rhs.val,
            grad: std::array::from_fn(|i| (self.grad[i] * rhs.val - self.val * rhs.grad[i]) / v2),
        }
    }
}

impl<const N: usize> Neg for DualArray<N> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            val: -self.val,
            grad: self.grad.map(|da| -da),
        }
    }
}

impl<const N: usize> From<f64> for DualArray<N> {
    #[inline]
    fn from(v: f64) -> Self {
        Self::constant(v)
    }
}

// --- Implementation of the Real Trait ---

impl<const N: usize> Real for DualArray<N> {
    #[inline]
    fn from_f64(v: f64) -> Self {
        Self::constant(v)
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
            self
        } else {
            Self {
                val: -self.val,
                grad: self.grad.map(|da| -da),
            }
        }
    }

    #[inline]
    fn exp(self) -> Self {
        let res = self.val.exp();
        Self {
            val: res,
            grad: self.grad.map(|da| da * res),
        }
    }

    #[inline]
    fn ln(self) -> Self {
        Self {
            val: self.val.ln(),
            grad: self.grad.map(|da| da / self.val),
        }
    }

    #[inline]
    fn sqrt(self) -> Self {
        let res = self.val.sqrt();
        Self {
            val: res,
            grad: self.grad.map(|da| da / (2.0 * res)),
        }
    }

    #[inline]
    fn powi(self, n: i32) -> Self {
        let val = self.val.powi(n);
        let factor = (n as f64) * self.val.powi(n - 1);
        Self {
            val,
            grad: self.grad.map(|da| da * factor),
        }
    }

    #[inline]
    fn powf(self, n: Self) -> Self {
        let val = self.val.powf(n.val);
        Self {
            val,
            grad: std::array::from_fn(|i| {
                val * (n.grad[i] * self.val.ln() + n.val * self.grad[i] / self.val)
            }),
        }
    }

    #[inline]
    fn norm_cdf(self) -> Self {
        let val = 0.5 * (1.0 + erf(self.val / 2.0f64.sqrt()));
        let pdf = (-0.5 * self.val * self.val).exp() / (2.0 * std::f64::consts::PI).sqrt();
        Self {
            val,
            grad: self.grad.map(|da| da * pdf),
        }
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

// Internal helper for norm_cdf
fn erf(x: f64) -> f64 {
    let p = 0.3275911;
    let a = [
        0.254829592,
        -0.284496736,
        1.421413741,
        -1.453152027,
        1.061405429,
    ];
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - ((((a[4] * t + a[3]) * t + a[2]) * t + a[1]) * t + a[0]) * t * (-x * x).exp();
    sign * y
}

// Implementation for: &'a DualArray - &'b DualArray
impl<'a, 'b, const N: usize> Sub<&'b DualArray<N>> for &'a DualArray<N> {
    type Output = DualArray<N>;
    #[inline]
    fn sub(self, rhs: &'b DualArray<N>) -> Self::Output {
        DualArray {
            val: self.val - rhs.val,
            grad: std::array::from_fn(|i| self.grad[i] - rhs.grad[i]),
        }
    }
}

// Implementation for: &'a DualArray + &'b DualArray
impl<'a, 'b, const N: usize> Add<&'b DualArray<N>> for &'a DualArray<N> {
    type Output = DualArray<N>;
    #[inline]
    fn add(self, rhs: &'b DualArray<N>) -> Self::Output {
        DualArray {
            val: self.val + rhs.val,
            grad: std::array::from_fn(|i| self.grad[i] + rhs.grad[i]),
        }
    }
}

// Implementation for: &'a DualArray * &'b DualArray
impl<'a, 'b, const N: usize> Mul<&'b DualArray<N>> for &'a DualArray<N> {
    type Output = DualArray<N>;
    #[inline]
    fn mul(self, rhs: &'b DualArray<N>) -> Self::Output {
        DualArray {
            val: self.val * rhs.val,
            grad: std::array::from_fn(|i| self.grad[i] * rhs.val + self.val * rhs.grad[i]),
        }
    }
}

// Implementation for: &'a DualArray / &'b DualArray
impl<'a, 'b, const N: usize> Div<&'b DualArray<N>> for &'a DualArray<N> {
    type Output = DualArray<N>;
    #[inline]
    fn div(self, rhs: &'b DualArray<N>) -> Self::Output {
        let v2 = rhs.val * rhs.val;
        DualArray {
            val: self.val / rhs.val,
            grad: std::array::from_fn(|i| (self.grad[i] * rhs.val - self.val * rhs.grad[i]) / v2),
        }
    }
}

// Implementation for: -&DualArray
impl<'a, const N: usize> Neg for &'a DualArray<N> {
    type Output = DualArray<N>;
    #[inline]
    fn neg(self) -> Self::Output {
        DualArray {
            val: -self.val,
            grad: self.grad.map(|da| -da),
        }
    }
}

// Implementation for: DualArray / DualArray
impl<const N: usize> Div<DualArray<N>> for DualArray<N> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        self / &rhs // Reuses your DualArray / &DualArray implementation
    }
}

// Implementation for: DualArray * DualArray
impl<const N: usize> Mul<DualArray<N>> for DualArray<N> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        self * &rhs
    }
}

// Implementation for: DualArray + DualArray
impl<const N: usize> Add<DualArray<N>> for DualArray<N> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        self + &rhs
    }
}

// Implementation for: DualArray - DualArray
impl<const N: usize> Sub<DualArray<N>> for DualArray<N> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        self - &rhs
    }
}

// Implementation for: DualArray += DualArray
impl<const N: usize> AddAssign<DualArray<N>> for DualArray<N> {
    #[inline]
    fn add_assign(&mut self, rhs: DualArray<N>) {
        // Delegate to the reference implementation
        *self += &rhs;
    }
}

impl<const N: usize> SubAssign<DualArray<N>> for DualArray<N> {
    #[inline]
    fn sub_assign(&mut self, rhs: DualArray<N>) {
        // Delegate to the reference implementation
        *self -= &rhs;
    }
}
