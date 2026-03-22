use crate::traits::payoff::InitialConditions;

pub mod linear_policy;
pub mod unified_policy;

pub trait StepPolicy<T, SG, L> {
    fn get_operator(&self) -> &L;

    fn solve_stage_into(&self, rhs: &[T], dt: T, grid: &SG, dest: &mut [T], z_buffer: &mut [T]);

    fn compute_stage_derivative<IC>(
        &self,
        stage_slice: &[T],
        grid: &SG,
        initial_conditions: IC,
        l_stage_slice: &mut [T],
    ) where
        IC: InitialConditions<T> + Copy;
}
