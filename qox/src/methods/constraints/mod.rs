use crate::{methods::finite_difference::meshers::Mesher1d, types::Real};

pub mod american;
pub mod none;

pub trait Constraint<T: Real, M: Mesher1d<T>> {
    fn apply(&self, price: &mut [T], mesher: &M);
    fn lower_bound(&self, i: usize, mesher: &M) -> T;
}
