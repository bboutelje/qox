use crate::{
    methods::{
        constraints::Constraint, finite_difference::meshers::Mesher1d,
        linear_operators::LinearOperator,
    },
    types::Real,
};

pub mod brennan_schwartz;
pub mod psor;

pub trait ComplementaritySolver<T, M, L, C>
where
    T: Real,
    M: Mesher1d<T>,
    L: LinearOperator<T>,
    C: Constraint<T, M>,
{
    fn solve(
        &self,
        operator: &L,
        rhs: &[T],
        kappa: T,
        constraint: &C,
        mesher: &M,
        solution: &mut [T],
    );
}
