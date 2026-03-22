use crate::{
    methods::{
        finite_difference::meshers::Mesher1d, linear_operators::LinearOperator,
        obstacle_policies::ObstaclePolicy,
    },
    types::Real,
};

pub struct NoObstaclePolicy;
impl<T: Real, M: Mesher1d<T>, L: LinearOperator<T>> ObstaclePolicy<T, M, L> for NoObstaclePolicy {
    fn solve_stage(&self, op: &L, b: &[T], _dt: T, _mesh: &M, dest: &mut [T], z: &mut [T]) {
        op.solve_inverse_into(b, dest, z);
    }

    fn compute_stage_derivative<IC>(
        &self,
        operator: &L,
        stage_slice: &[T],
        _mesher: &M,
        _initial_conditions: IC,
        l_stage_slice: &mut [T],
    ) where
        IC: crate::traits::payoff::InitialConditions<T> + Copy,
    {
        operator.apply_into(stage_slice, l_stage_slice);
    }
}
