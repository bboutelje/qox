use crate::traits::real::Real;

pub trait Transform<T: Real> {
    fn to_mesh(&self, physical: T) -> T;
    fn to_physical(&self, mesh: T) -> T;
    fn jacobian(&self, xi: T) -> T;
    fn hessian(&self, xi: T) -> T;
}