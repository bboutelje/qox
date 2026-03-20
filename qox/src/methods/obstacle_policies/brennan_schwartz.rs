use crate::{
    methods::{
        complementarity::{ComplementaritySolver, brennan_schwartz::BrennanSchwartz},
        constraints::Constraint,
        finite_difference::meshers::Mesher1d,
        linear_operators::{LinearOperator, tridiagonal_operator::TridiagonalOperator},
        obstacle_policies::ObstaclePolicy,
    },
    types::Real,
};

pub struct BrennanSchwartzPolicy<C> {
    pub constraint: C,
}
impl<T: Real, M: Mesher1d<T>, C: Constraint<T, M>> ObstaclePolicy<T, M, TridiagonalOperator<T>>
    for BrennanSchwartzPolicy<C>
{
    fn solve_stage(
        &self,
        op: &TridiagonalOperator<T>,
        b: &[T],
        coeff: T,
        t: T,
        mesh: &M,
        dest: &mut [T],
        z: &mut [T],
    ) {
        BrennanSchwartz::new().solve(op, b, coeff, &self.constraint, mesh, dest);
    }

    fn compute_stage_derivative<IC>(
        &self,
        operator: &TridiagonalOperator<T>,
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
