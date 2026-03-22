use crate::{methods::finite_difference::meshers::SpatialGrid, types::Real};

pub mod american;
pub mod none;

pub trait Constraint<T: Real, SG: SpatialGrid<T>> {
    fn apply(&self, price: &mut [T], mesher: &SG);
    fn lower_bound(&self, i: usize, mesher: &SG) -> T;
}
