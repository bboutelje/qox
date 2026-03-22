use std::marker::PhantomData;

use crate::{
    methods::{
        finite_difference::meshers::SpatialGrid, linear_operators::LinearOperator,
        step_policy::StepPolicy,
    },
    traits::payoff::InitialConditions,
    types::Real,
};

pub struct LinearPolicy<'a, T, SG, L> {
    pub operator: &'a L,
    _marker: PhantomData<(T, SG)>,
}

impl<'a, T, SG, L> LinearPolicy<'a, T, SG, L> {
    pub fn new(operator: &'a L) -> Self {
        Self {
            operator,
            _marker: PhantomData,
        }
    }
}

impl<'a, T: Real, SG: SpatialGrid<T>, L: LinearOperator<T>> StepPolicy<T, SG, L>
    for LinearPolicy<'a, T, SG, L>
{
    fn solve_stage_into(&self, rhs: &[T], _dt: T, _grid: &SG, dest: &mut [T], z_buffer: &mut [T]) {
        self.operator.solve_inverse_into(rhs, dest, z_buffer);
    }

    fn compute_stage_derivative<IC>(
        &self,
        stage_slice: &[T],
        _grid: &SG,
        _initial_conditions: IC,
        l_stage_slice: &mut [T],
    ) where
        IC: InitialConditions<T> + Copy,
    {
        self.operator.apply_into(stage_slice, l_stage_slice);
    }

    fn get_operator(&self) -> &L {
        &self.operator
    }
}
