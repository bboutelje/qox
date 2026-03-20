use crate::{
    methods::{
        complementarity::ComplementaritySolver, constraints::Constraint,
        finite_difference::meshers::Mesher1d, linear_operators::LinearOperator,
        linear_operators::tridiagonal_operator::TridiagonalOperator,
    },
    types::Real,
};

pub struct BrennanSchwartz;

impl BrennanSchwartz {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl<T, M, C> ComplementaritySolver<T, M, TridiagonalOperator<T>, C> for BrennanSchwartz
where
    T: Real,
    M: Mesher1d<T>,
    C: Constraint<T, M>,
{
    fn solve(
        &self,
        op: &TridiagonalOperator<T>,
        rhs: &[T],
        kappa: T,
        constraint: &C,
        mesher: &M,
        x: &mut [T],
    ) {
        let n = op.size();

        // Use pre-allocated buffers if performance is a concern
        let mut d_star = vec![T::zero(); n];
        let mut rhs_star = vec![T::zero(); n];

        // 1. Forward Elimination
        // We transform the system (I - kappa * L)x = rhs
        d_star[0] = T::one() - kappa * op.diag[0];
        rhs_star[0] = rhs[0];

        for i in 1..n {
            let a_i = -kappa * op.lower[i];
            let d_i = T::one() - kappa * op.diag[i];
            let c_prev = -kappa * op.upper[i - 1];

            let m = a_i / d_star[i - 1];
            d_star[i] = d_i - m * c_prev;
            rhs_star[i] = rhs[i] - m * rhs_star[i - 1];
        }

        // 2. Backward Substitution with Brennan-Schwartz Constraint
        // The core of the method: project against the constraint at each step
        let g_last = constraint.lower_bound(n - 1, mesher);
        x[n - 1] = (rhs_star[n - 1] / d_star[n - 1]).max(g_last);

        for i in (0..n - 1).rev() {
            let c_i = -kappa * op.upper[i];
            let g_i = constraint.lower_bound(i, mesher);

            // Standard back-substitution
            let val = (rhs_star[i] - c_i * x[i + 1]) / d_star[i];

            // Apply the American/Complementarity constraint
            x[i] = val.max(g_i);
        }
    }
}
