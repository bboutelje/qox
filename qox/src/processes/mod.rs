use crate::{
    solvers::{finite_difference::meshers::Mesher1d, linear_operators::LinearOperator},
    traits::transform::Transform,
    types::Real,
};

pub mod black_scholes;

pub trait FdmProcess<T: Real, L: LinearOperator<T, M>, M: Mesher1d<T>, Tr: Transform<T> + Copy> {
    fn transform(&self) -> Tr;
    fn build_operator(&self, mesher: &M) -> L;
}
