use crate::{
    methods::{
        constraints::Constraint, finite_difference::meshers::SpatialGrid,
        linear_operators::LinearOperator,
    },
    types::Real,
};

pub mod brennan_schwartz;
pub mod psor;

pub trait ComplementaritySolver<T, SG, L, C>
where
    T: Real,
    SG: SpatialGrid<T>,
    L: LinearOperator<T>,
    C: Constraint<T, SG>,
{
    fn solve(
        &self,
        operator: &L,
        rhs: &[T],
        kappa: T,
        constraint: &C,
        grid: &SG,
        solution: &mut [T],
    );
}
