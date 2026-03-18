use crate::{
    methods::{
        finite_difference::{free_boundary::FreeBoundaryStrategy, meshers::Mesher1d},
        linear_operators_old::LinearOperator,
    },
    types::Real,
};

pub struct Unconstrained;
impl<T: Real, M: Mesher1d<T>, L: LinearOperator<T, M>> FreeBoundaryStrategy<T, M, L>
    for Unconstrained
{
    fn solve_stage(&self, op: &L, b: &[T], coeff: T, t: T, _m: &M, dest: &mut [T], z: &mut [T]) {
        op.solve_inverse_into(b, coeff, t, dest, z);
    }

    fn compute_stage_derivative<IC>(
        &self,
        operator: &L,
        stage_slice: &[T],
        next_t: T,
        _mesher: &M,
        _initial_conditions: IC,
        l_stage_slice: &mut [T],
    ) where
        IC: crate::traits::payoff::InitialConditions<T> + Copy,
    {
        operator.apply_into(stage_slice, next_t, l_stage_slice);
    }
}
