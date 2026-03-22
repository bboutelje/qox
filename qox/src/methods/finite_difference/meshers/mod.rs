use crate::types::Real;

pub mod concentrating;
pub mod log;
pub mod uniform;
pub mod uniform_old;

pub trait SpatialGrid<T: Real> {
    fn size(&self) -> usize;
    fn location(&self, index: usize) -> T;
}

pub trait Mesher1d<T: Real>: SpatialGrid<T> {
    fn centers(&self) -> &[T];
    fn h_plus(&self) -> &[T];
    fn h_minus(&self) -> &[T];
}
