use crate::traits::{real::Real};

pub trait VolSurface
{
    type T: Real;
    fn volatility(&self, t: &Self::T) -> Self::T;
}