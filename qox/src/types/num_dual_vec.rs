use crate::types::Real;
use nalgebra::ComplexField;
use nalgebra::Const;
use num_dual::{Derivative, DualStruct, DualVec};
use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct NumDualVec<const N: usize>(pub DualVec<f64, f64, Const<N>>);

impl<const N: usize> NumDualVec<N> {
    /// Creates a constant value with zero gradients.
    #[inline]
    pub fn constant(val: f64) -> Self {
        Self(DualVec::from_re(val))
    }

    /// Creates a variable where the gradient is 1.0 at the specified index
    /// and 0.0 elsewhere.
    #[inline]
    pub fn var(val: f64, index: usize) -> Self {
        let mut v = DualVec::from_re(val);
        if index < N {
            // Create a derivative vector where only 'index' is 1.0
            let mut deriv = nalgebra::SVector::<f64, N>::zeros();
            deriv[index] = 1.0;
            v.eps = Derivative::some(deriv);
        }
        Self(v)
    }
}

impl<const N: usize> Real for NumDualVec<N> {
    fn from_f64(v: f64) -> Self {
        NumDualVec(DualVec::from_re(v))
    }

    fn scalar(self) -> f64 {
        self.0.re()
    }

    fn max(self, other: Self) -> Self {
        if self.0.re() >= other.0.re() {
            self
        } else {
            other
        }
    }

    fn min(self, other: Self) -> Self {
        if self.0.re() <= other.0.re() {
            self
        } else {
            other
        }
    }

    #[inline]
    fn abs(self) -> Self {
        NumDualVec(self.0.abs())
    }

    fn exp(self) -> Self {
        NumDualVec(self.0.exp())
    }

    fn ln(self) -> Self {
        NumDualVec(self.0.ln())
    }

    fn sqrt(self) -> Self {
        NumDualVec(self.0.sqrt())
    }

    fn powi(self, n: i32) -> Self {
        NumDualVec(self.0.powi(n))
    }

    fn powf(self, n: Self) -> Self {
        let ln_base = self.ln();
        let exponent_ln_base = n * ln_base;
        exponent_ln_base.exp()
    }

    fn norm_cdf(self) -> Self {
        let x = self.0.re();
        let cdf_val = 0.5 * (1.0 + libm::erf(x / std::f64::consts::SQRT_2));
        let pdf_val = (-0.5 * x * x).exp() / (2.0 * std::f64::consts::PI).sqrt();

        let new_matrix = self
            .0
            .eps
            .unwrap_generic(Const::<N>, nalgebra::U1)
            .map(|component| component * pdf_val);

        NumDualVec(DualVec::new(cdf_val, Derivative::some(new_matrix)))
    }

    fn zero() -> Self {
        Self::from_f64(0.0)
    }

    fn one() -> Self {
        Self::from_f64(1.0)
    }
}

// Implementation for: NumDualVec + NumDualVec
impl<const N: usize> Add<NumDualVec<N>> for NumDualVec<N> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        NumDualVec(self.0 + rhs.0)
    }
}

impl<const N: usize> std::ops::AddAssign<NumDualVec<N>> for NumDualVec<N> {
    #[inline]
    fn add_assign(&mut self, rhs: NumDualVec<N>) {
        self.0 += rhs.0;
    }
}

impl<const N: usize> std::ops::SubAssign<NumDualVec<N>> for NumDualVec<N> {
    #[inline]
    fn sub_assign(&mut self, rhs: NumDualVec<N>) {
        self.0 -= rhs.0;
    }
}

// Implementation for: NumDualVec - NumDualVec
impl<const N: usize> Sub<NumDualVec<N>> for NumDualVec<N> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        NumDualVec(self.0 - rhs.0)
    }
}

// Implementation for: NumDualVec * NumDualVec
impl<const N: usize> Mul<NumDualVec<N>> for NumDualVec<N> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        NumDualVec(self.0 * rhs.0)
    }
}

// Implementation for: NumDualVec / NumDualVec
impl<const N: usize> Div<NumDualVec<N>> for NumDualVec<N> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        NumDualVec(self.0 / rhs.0)
    }
}

// Implementation for: -NumDualVec
impl<const N: usize> Neg for NumDualVec<N> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        NumDualVec(-self.0)
    }
}

// Implementation for: f64 -> NumDualVec
impl<const N: usize> From<f64> for NumDualVec<N> {
    #[inline]
    fn from(v: f64) -> Self {
        Self::from_f64(v)
    }
}

impl<const N: usize> PartialEq for NumDualVec<N> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.re() == other.0.re()
    }
}

impl<const N: usize> PartialOrd for NumDualVec<N> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.re().partial_cmp(&other.0.re())
    }
}

// Implementation for: &NumDualVec - &NumDualVec
impl<'a, 'b, const N: usize> Sub<&'b NumDualVec<N>> for &'a NumDualVec<N> {
    type Output = NumDualVec<N>;

    #[inline]
    fn sub(self, rhs: &'b NumDualVec<N>) -> Self::Output {
        NumDualVec(self.0 - rhs.0)
    }
}

// You typically want to provide the others (Add, Mul, Div) as well:
impl<'a, 'b, const N: usize> Add<&'b NumDualVec<N>> for &'a NumDualVec<N> {
    type Output = NumDualVec<N>;
    #[inline]
    fn add(self, rhs: &'b NumDualVec<N>) -> Self::Output {
        NumDualVec(self.0 + rhs.0)
    }
}

impl<'a, 'b, const N: usize> Mul<&'b NumDualVec<N>> for &'a NumDualVec<N> {
    type Output = NumDualVec<N>;
    #[inline]
    fn mul(self, rhs: &'b NumDualVec<N>) -> Self::Output {
        NumDualVec(self.0 * rhs.0)
    }
}

impl<'a, 'b, const N: usize> Div<&'b NumDualVec<N>> for &'a NumDualVec<N> {
    type Output = NumDualVec<N>;
    #[inline]
    fn div(self, rhs: &'b NumDualVec<N>) -> Self::Output {
        NumDualVec(self.0 / rhs.0)
    }
}

// Implementation for: -&NumDualVec
impl<'a, const N: usize> Neg for &'a NumDualVec<N> {
    type Output = NumDualVec<N>;

    #[inline]
    fn neg(self) -> Self::Output {
        NumDualVec(-self.0)
    }
}
