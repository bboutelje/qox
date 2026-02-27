use crate::traits::real::Real;

// pub trait RateCurve<T: Real> {
//     fn zero_rate(&self, t: &T) -> T;
//     fn discount_factor(&self, t: &T) -> T;
// }


pub trait RateCurve {
    /// The numeric type used by this curve (e.g., f64, Dual)
    type T: Real; 

    fn zero_rate(&self, t: &Self::T) -> Self::T;
    fn discount_factor(&self, t: &Self::T) -> Self::T;
}