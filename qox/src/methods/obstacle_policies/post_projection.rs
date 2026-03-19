use crate::{
    methods::{
        constraints::Constraint, finite_difference::meshers::Mesher1d,
        linear_operators::LinearOperator, obstacle_policies::ObstaclePolicy,
    },
    types::Real,
};

pub struct PostProjectionPolicy<C> {
    pub constraint: C,
}
impl<T: Real, M: Mesher1d<T>, L: LinearOperator<T>, C: Constraint<T, M>> ObstaclePolicy<T, M, L>
    for PostProjectionPolicy<C>
{
    fn solve_stage(&self, op: &L, b: &[T], coeff: T, t: T, mesh: &M, dest: &mut [T], z: &mut [T]) {
        op.solve_inverse_into(b, coeff, t, dest, z);

        self.constraint.apply(dest, mesh);
    }

    fn compute_stage_derivative<IC>(
        &self,
        operator: &L,
        stage_slice: &[T],
        next_t: T,
        mesher: &M,
        initial_conditions: IC,
        l_stage_slice: &mut [T],
    ) where
        IC: crate::traits::payoff::InitialConditions<T> + Copy,
    {
        operator.apply_into(stage_slice, next_t, l_stage_slice);

        for j in 0..operator.size() {
            let s = mesher.location(j);
            let payoff = initial_conditions.get_value(s);

            if stage_slice[j] <= payoff + T::from_f64(f64::EPSILON) {
                l_stage_slice[j] = T::zero();
            }
        }
    }
}
