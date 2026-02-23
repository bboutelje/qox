use crate::traits::real::Real;

pub trait RateCurve<T: Real> {
    fn zero_rate(&self, t: &T) -> T;
    fn discount_factor(&self, t: &T) -> T;
}