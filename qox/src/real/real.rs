use crate::traits::real::Real;

// --- f64 Implementation ---

impl Real for f64 {
    

    fn from_f64(val: f64) -> Self { val }
    
    fn scalar(self) -> f64 { self }

    fn max(self, other: Self) -> Self {
        f64::max(self, other)
    }

    fn min(self, other: Self) -> Self {
        f64::min(self, other)
    }

    fn abs(self) -> Self { self.abs() }

    fn exp(self) -> Self { f64::exp(self) }
    fn ln(self) -> Self { f64::ln(self) }
    fn sqrt(self) -> Self { f64::sqrt(self) }
    fn powi(self, n: i32) -> Self { f64::powi(self, n) }
    fn powf(self, n: Self) -> Self { f64::powf(self, n) }
    
    fn norm_cdf(self) -> Self {
        0.5 * (1.0 + libm::erf(self / std::f64::consts::SQRT_2))
    }
}
