use crate::{
    methods::{
        finite_difference::meshers::SpatialGrid,
        linear_operators::LinearOperator,
        step_policy::{StepPolicy, linear_policy::LinearPolicy},
    },
    traits::payoff::InitialConditions,
    types::Real,
};
// Assuming AmericanPolicy is your other variant's type
// use crate::methods::step_policy::american_policy::AmericanPolicy;

pub enum UnifiedPolicy<'a, T, SG, L> {
    Linear(LinearPolicy<'a, T, SG, L>),
    // Replace with your actual American policy type
    //American(AmericanPolicy<'a, T, SG, L>),
}

impl<'a, T, SG, L> StepPolicy<T, SG, L> for UnifiedPolicy<'a, T, SG, L>
where
    T: Real,
    L: LinearOperator<T>,
    SG: SpatialGrid<T>,
{
    fn get_operator(&self) -> &L {
        match self {
            Self::Linear(p) => p.get_operator(),
            //Self::American(p) => p.get_operator(),
        }
    }

    fn solve_stage_into(&self, rhs: &[T], dt: T, grid: &SG, dest: &mut [T], z_buffer: &mut [T]) {
        match self {
            Self::Linear(p) => p.solve_stage_into(rhs, dt, grid, dest, z_buffer),
            //Self::American(p) => p.solve_stage_into(rhs, dt, grid, dest, z_buffer),
        }
    }

    fn compute_stage_derivative<IC>(
        &self,
        stage_slice: &[T],
        grid: &SG,
        initial_conditions: IC,
        l_stage_slice: &mut [T],
    ) where
        IC: InitialConditions<T> + Copy,
    {
        match self {
            Self::Linear(p) => {
                p.compute_stage_derivative(stage_slice, grid, initial_conditions, l_stage_slice)
            } // Self::American(p) => {
              //     p.compute_stage_derivative(stage_slice, grid, initial_conditions, l_stage_slice)
              // }
        }
    }
}
