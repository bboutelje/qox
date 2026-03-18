use crate::{
    methods::{
        complementarity::ComplementaritySolver,
        constraints::Constraint,
        finite_difference::meshers::Mesher1d,
        linear_operators_old::{LinearOperator, tridiagonal_operator::TridiagonalOperator},
    },
    types::Real,
};

pub struct BrennanSchwartz;

impl<T, M, C> ComplementaritySolver<T, M, TridiagonalOperator<T, M>, C> for BrennanSchwartz
where
    T: Real,
    M: Mesher1d<T>,
    C: Constraint<T, M>,
{
    fn solve(
        &self,
        op: &TridiagonalOperator<T, M>,
        rhs: &[T],
        kappa: T,
        constraint: &C,
        mesher: &M,
        x: &mut [T],
    ) {
        let n = op.size();

        // Temporary buffers for the modified coefficients
        // In a production loop, you might want to pass these in via a z_buffer
        let mut d_star = vec![T::zero(); n];
        let mut rhs_star = vec![T::zero(); n];

        // 1. Forward Elimination (Modified for the constraint)
        // A_ii = 1 - kappa * diag[i]
        // A_i,i-1 = -kappa * lower[i]
        // A_i,i+1 = -kappa * upper[i]

        d_star[0] = T::one() - kappa * op.diag[0];
        rhs_star[0] = rhs[0].max(constraint.lower_bound(0, mesher) * d_star[0]);

        for i in 1..n {
            let a_i = -kappa * op.lower[i];
            let d_i = T::one() - kappa * op.diag[i];
            let c_prev = -kappa * op.upper[i - 1];

            // Standard Thomas recurrence for the diagonal
            let m = a_i / d_star[i - 1];
            d_star[i] = d_i - m * c_prev;

            // Update RHS and project against the constraint
            let g_i = constraint.lower_bound(i, mesher);
            rhs_star[i] = (rhs[i] - m * rhs_star[i - 1]).max(g_i * d_star[i]);
        }

        // 2. Backward Substitution
        x[n - 1] = rhs_star[n - 1] / d_star[n - 1];
        for i in (0..n - 1).rev() {
            let c_i = -kappa * op.upper[i];
            x[i] = (rhs_star[i] - c_i * x[i + 1]) / d_star[i];

            // Final check against constraint
            let g_i = constraint.lower_bound(i, mesher);
            if x[i] < g_i {
                x[i] = g_i;
            }
        }
    }
}
