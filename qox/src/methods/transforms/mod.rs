use crate::types::Real;

pub mod identity;
pub mod log;

pub trait Transform<T: Real> {
    fn to_transform(&self, physical: T) -> T;
    fn to_physical(&self, mesh: T) -> T;
    fn jacobian(&self, xi: T) -> T;
    fn hessian(&self, xi: T) -> T;
}
