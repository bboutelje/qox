use crate::traits::payoff::InitialConditions;

pub mod brennan_schwartz;
pub mod no_obstacle;
pub mod post_projection;
pub mod psor;

pub trait ObstaclePolicy<T, SG, L> {
    fn solve_stage(
        &self,
        operator: &L,
        rhs: &[T],
        dt: T,
        grid: &SG,
        dest: &mut [T],
        z_buffer: &mut [T],
    );

    fn compute_stage_derivative<IC>(
        &self,
        operator: &L,
        stage_slice: &[T],
        grid: &SG,
        initial_conditions: IC,
        l_stage_slice: &mut [T],
    ) where
        IC: InitialConditions<T> + Copy;
}
