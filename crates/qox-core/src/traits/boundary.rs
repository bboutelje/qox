use crate::traits::real::Real;

pub trait BoundaryCondition<T: Real> {
    fn apply_lower(&self, t: &T) -> T;
    fn apply_upper(&self, t: &T, s_max: &T) -> T;
}