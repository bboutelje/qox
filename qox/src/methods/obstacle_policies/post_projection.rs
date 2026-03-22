use crate::{
    methods::{
        constraints::Constraint, finite_difference::meshers::SpatialGrid,
        linear_operators::LinearOperator, obstacle_policies::ObstaclePolicy,
    },
    types::Real,
};

pub struct PostProjectionPolicy<C> {
    pub constraint: C,
}
impl<T: Real, SG: SpatialGrid<T>, L: LinearOperator<T>, C: Constraint<T, SG>>
    ObstaclePolicy<T, SG, L> for PostProjectionPolicy<C>
{
    fn solve_stage(&self, op: &L, b: &[T], _dt: T, grid: &SG, dest: &mut [T], z: &mut [T]) {
        op.solve_inverse_into(b, dest, z);

        self.constraint.apply(dest, grid);
    }

    fn compute_stage_derivative<IC>(
        &self,
        operator: &L,
        stage_slice: &[T],
        grid: &SG,
        initial_conditions: IC,
        l_stage_slice: &mut [T],
    ) where
        IC: crate::traits::payoff::InitialConditions<T> + Copy,
    {
        operator.apply_into(stage_slice, l_stage_slice);

        for j in 0..operator.size() {
            let s = grid.location(j);
            let payoff = initial_conditions.get_value(s);

            if stage_slice[j] <= payoff + T::from_f64(f64::EPSILON) {
                l_stage_slice[j] = T::zero();
            }
        }
    }
}
