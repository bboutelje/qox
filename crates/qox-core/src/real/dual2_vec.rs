use std::{cmp::Ordering, ops::{Add, Div, Mul, Neg, Sub}};
use crate::traits::real::Real;
use num_dual::{Derivative, DualNum, DualStruct, Dual2Vec};
use nalgebra::Const;

#[derive(Debug, Clone, Copy)]
pub struct Dual2Vec64<const N: usize>(pub Dual2Vec<f64, f64, Const<N>>);

impl<const N: usize> Real for Dual2Vec64<N> {
    fn from_f64(v: f64) -> Self {
        Dual2Vec64(Dual2Vec::from_re(v))
    }

    fn to_f64(&self) -> f64 {
        self.0.re()
    }

    fn exp(&self) -> Self {
        Dual2Vec64(self.0.exp())
    }

    fn ln(&self) -> Self {
        Dual2Vec64(self.0.ln())
    }

    fn sqrt(&self) -> Self {
        Dual2Vec64(self.0.sqrt())
    }

    fn powi(&self, n: i32) -> Self {
        Dual2Vec64(self.0.powi(n))
    }

    fn powf(&self, n: &Self) -> Self {
        // a^b = exp(b * ln(a))
        // We use references throughout to avoid cloning the Hessian matrices
        let ln_base = self.ln();
        let exponent_ln_base = n * &ln_base;
        exponent_ln_base.exp()
    }

    fn norm_cdf(&self) -> Self {
        let x = self.0.re();
        
        let cdf_val = 0.5 * (1.0 + libm::erf(x / std::f64::consts::SQRT_2));
        let pdf_val = (-0.5 * x * x).exp() / (2.0 * std::f64::consts::PI).sqrt();
        let pdf_deriv = -x * pdf_val;

        let v1 = self.0.v1.unwrap_generic(nalgebra::U1, Const::<N>);
        let v2 = self.0.v2.unwrap_generic(Const::<N>, Const::<N>);

        let new_v1 = v1.map(|u_prime| u_prime * pdf_val);

        let mut new_v2 = v2.map(|u_double_prime| u_double_prime * pdf_val);
        
        for i in 0..N {
            for j in 0..N {
                new_v2[(i, j)] += pdf_deriv * v1[i] * v1[j];
            }
        }

        Dual2Vec64(Dual2Vec::new(
            cdf_val, 
            Derivative::some(new_v1), 
            Derivative::some(new_v2)
        ))
    }
}

// --- Implement HRTB Operators (Owned OP Reference) ---

impl<'a, const N: usize> Add<&'a Dual2Vec64<N>> for Dual2Vec64<N> {
    type Output = Self;
    fn add(self, rhs: &'a Self) -> Self { Dual2Vec64(self.0 + rhs.0) }
}

impl<'a, const N: usize> Sub<&'a Dual2Vec64<N>> for Dual2Vec64<N> {
    type Output = Self;
    fn sub(self, rhs: &'a Self) -> Self { Dual2Vec64(self.0 - rhs.0) }
}

impl<'a, const N: usize> Mul<&'a Dual2Vec64<N>> for Dual2Vec64<N> {
    type Output = Self;
    fn mul(self, rhs: &'a Self) -> Self { Dual2Vec64(self.0 * rhs.0) }
}

impl<'a, const N: usize> Div<&'a Dual2Vec64<N>> for Dual2Vec64<N> {
    type Output = Self;
    fn div(self, rhs: &'a Self) -> Self { Dual2Vec64(self.0 / rhs.0) }
}

// --- Standard Trait Implementations ---

impl<const N: usize> From<f64> for Dual2Vec64<N> {
    fn from(v: f64) -> Self { Self::from_f64(v) }
}

impl<const N: usize> PartialEq for Dual2Vec64<N> {
    fn eq(&self, other: &Self) -> bool { self.0.re() == other.0.re() }
}

impl<const N: usize> PartialOrd for Dual2Vec64<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.re().partial_cmp(&other.0.re())
    }
}

impl<const N: usize> Neg for Dual2Vec64<N> {
    type Output = Self;
    fn neg(self) -> Self { Dual2Vec64(-self.0) }
}

// --- Reference on Reference Operators ---

impl<'a, 'b, const N: usize> Mul<&'b Dual2Vec64<N>> for &'a Dual2Vec64<N> {
    type Output = Dual2Vec64<N>;
    fn mul(self, rhs: &'b Dual2Vec64<N>) -> Dual2Vec64<N> {
        Dual2Vec64(self.0 * rhs.0)
    }
}

impl<'a, 'b, const N: usize> Add<&'b Dual2Vec64<N>> for &'a Dual2Vec64<N> {
    type Output = Dual2Vec64<N>;
    fn add(self, rhs: &'b Dual2Vec64<N>) -> Dual2Vec64<N> {
        Dual2Vec64(self.0 + rhs.0)
    }
}

impl<'a, 'b, const N: usize> Sub<&'b Dual2Vec64<N>> for &'a Dual2Vec64<N> {
    type Output = Dual2Vec64<N>;
    fn sub(self, rhs: &'b Dual2Vec64<N>) -> Dual2Vec64<N> {
        Dual2Vec64(self.0 - rhs.0)
    }
}

impl<'a, 'b, const N: usize> Div<&'b Dual2Vec64<N>> for &'a Dual2Vec64<N> {
    type Output = Dual2Vec64<N>;
    fn div(self, rhs: &'b Dual2Vec64<N>) -> Dual2Vec64<N> {
        Dual2Vec64(self.0 / rhs.0)
    }
}

impl<'a, const N: usize> Neg for &'a Dual2Vec64<N> {
    type Output = Dual2Vec64<N>;
    fn neg(self) -> Dual2Vec64<N> {
        Dual2Vec64(-self.0)
    }
}