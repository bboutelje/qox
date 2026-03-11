
pub trait VolSurface<T>
{
    fn volatility(&self, strike: f64, t: T) -> T;
}