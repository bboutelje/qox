use crate::{
    methods::{
        complementarity::{ComplementaritySolver, psor::Psor},
        constraints::Constraint,
        finite_difference::meshers::Mesher1d,
        linear_operators::LinearOperator,
        linear_operators::tridiagonal_operator::TridiagonalOperator,
        obstacle_policies::ObstaclePolicy,
    },
    types::Real,
};

pub struct PsorObstaclePolicy<C> {
    pub constraint: C,
    pub psor: Psor,
}

impl<T: Real, M: Mesher1d<T>, C: Constraint<T, M>> ObstaclePolicy<T, M, TridiagonalOperator<T>>
    for PsorObstaclePolicy<C>
{
    fn solve_stage(
        &self,
        op: &TridiagonalOperator<T>,
        b: &[T],
        coeff: T,
        _t: T,
        m: &M,
        dest: &mut [T],
        _z: &mut [T],
    ) {
        self.psor.solve(op, b, coeff, &self.constraint, m, dest);

        //op.solve_psor_into(b, coeff, &self.constraint, m, dest, z);
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
