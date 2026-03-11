

pub trait RateCurve<T> {
    fn zero_rate(&self, t: T) -> T;
    fn discount_factor(&self, t: T) -> T;
}