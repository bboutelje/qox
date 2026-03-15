use crate::traits::{
    fdm_mesher::Mesher1d, linear_operator::LinearOperator, real::Real, transform::Transform,
};

pub trait FdmProcess<T: Real, L: LinearOperator<T, M>, M: Mesher1d<T>, Tr: Transform<T> + Copy> {
    fn transform(&self) -> Tr;
    fn build_operator(&self, mesher: &M) -> L;
}
