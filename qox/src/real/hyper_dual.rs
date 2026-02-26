// use std::{cmp::Ordering, ops::{Add, Div, Mul, Neg, Sub}};
// use crate::traits::real::Real;
// use num_dual::{DualNum, DualStruct, HyperDual64};

// #[derive(Debug, Clone, Copy)]
// pub struct HyperDual(pub HyperDual64);

// impl Real for HyperDual {
//     fn from_f64(v: f64) -> Self { 
//         HyperDual(HyperDual64::from_re(v))
//     }

//     fn to_f64(&self) -> f64 {
//         self.0.re()
//     }
    
//     fn exp(&self) -> Self { 
//         HyperDual(self.0.exp())
//     }
    
//     fn ln(&self) -> Self { 
//         HyperDual(self.0.ln())
//     }
    
//     fn sqrt(&self) -> Self { 
//         HyperDual(self.0.sqrt())
//     }
    
//     fn powi(&self, n: i32) -> Self { 
//         HyperDual(self.0.powi(n))
//     }

//     fn norm_cdf(&self) -> Self {
//         let x = self.0.re();
        
//         let cdf_val = 0.5 * (1.0 + libm::erf(x / std::f64::consts::SQRT_2));
//         let pdf_val = (-0.5 * x * x).exp() / (2.0 * std::f64::consts::PI).sqrt();
//         let pdf_deriv = -x * pdf_val;
        
//         // For HyperDual64 (second order, single variable), the components are:
//         // f(u) ≈ f(x) + f'(x)ε1 + f'(x)ε2 + f''(x)ε1ε2
//         // where ε1 and ε2 represent the dual parts of the input.
//         HyperDual(HyperDual64::new(
//             cdf_val,
//             pdf_val * self.0.eps1,
//             pdf_val * self.0.eps2,
//             pdf_val * self.0.eps1eps2 + pdf_deriv * self.0.eps1 * self.0.eps2,
//         ))
//     }
// }

// // --- Implement HRTB Operators (Owned OP Reference) ---

// impl<'a> Add<&'a HyperDual> for HyperDual {
//     type Output = Self;
//     fn add(self, rhs: &'a Self) -> Self { HyperDual(self.0 + rhs.0) }
// }

// impl<'a> Sub<&'a HyperDual> for HyperDual {
//     type Output = Self;
//     fn sub(self, rhs: &'a Self) -> Self { HyperDual(self.0 - rhs.0) }
// }

// impl<'a> Mul<&'a HyperDual> for HyperDual {
//     type Output = Self;
//     fn mul(self, rhs: &'a Self) -> Self { HyperDual(self.0 * rhs.0) }
// }

// impl<'a> Div<&'a HyperDual> for HyperDual {
//     type Output = Self;
//     fn div(self, rhs: &'a Self) -> Self { HyperDual(self.0 / rhs.0) }
// }

// // --- Standard Boilerplate ---

// impl From<f64> for HyperDual {
//     fn from(v: f64) -> Self { Self::from_f64(v) }
// }

// impl PartialEq for HyperDual {
//     fn eq(&self, other: &Self) -> bool { self.0.re() == other.0.re() }
// }

// impl PartialOrd for HyperDual {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         self.0.re().partial_cmp(&other.0.re())
//     }
// }

// impl Neg for HyperDual {
//     type Output = Self;
//     fn neg(self) -> Self { HyperDual(-self.0) }
// }