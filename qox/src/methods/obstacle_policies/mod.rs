use crate::traits::payoff::InitialConditions;

pub mod brennan_schwartz;
pub mod no_obstacle;
pub mod post_projection;
pub mod psor;

pub trait ObstaclePolicy<T, M, L> {
    fn solve_stage(
        &self,
        operator: &L,
        rhs: &[T],
        coeff: T,
        next_t: T,
        mesher: &M,
        dest: &mut [T],
        z_buffer: &mut [T],
    );

    fn compute_stage_derivative<IC>(
        &self,
        operator: &L,
        stage_slice: &[T],
        next_t: T,
        mesher: &M,
        initial_conditions: IC,
        l_stage_slice: &mut [T],
    ) where
        IC: InitialConditions<T> + Copy;
}
