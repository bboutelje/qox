use crate::traits::{fdm_mesher::Mesher1d, real::Real};

pub trait Constraint<T: Real, M: Mesher1d<T>> {
    fn apply(&self, price: &mut [T], mesher: &M);
    fn lower_bound(&self, i: usize, mesher: &M) -> T;
}
