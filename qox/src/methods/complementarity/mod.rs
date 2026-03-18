use crate::{
    methods::{
        constraints::Constraint, finite_difference::meshers::Mesher1d,
        linear_operators_old::LinearOperator,
    },
    types::Real,
};

pub mod brennan_schwartz;
pub mod psor;

pub trait ComplementaritySolver<T, M, Op, C>
where
    T: Real,
    M: Mesher1d<T>,
    Op: LinearOperator<T, M>,
    C: Constraint<T, M>,
{
    fn solve(
        &self,
        operator: &Op,
        rhs: &[T],
        kappa: T,
        constraint: &C,
        mesher: &M,
        solution: &mut [T],
    );
}
