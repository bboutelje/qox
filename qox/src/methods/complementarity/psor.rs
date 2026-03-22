use crate::{
    methods::{
        complementarity::ComplementaritySolver,
        constraints::Constraint,
        finite_difference::meshers::SpatialGrid,
        linear_operators::{LinearOperator, tridiagonal_operator::TridiagonalOperator},
    },
    types::Real,
};

pub struct Psor {
    pub omega: f64,
    pub tolerance: f64,
    pub max_iter: usize,
}

impl<T, SG, C> ComplementaritySolver<T, SG, TridiagonalOperator<T>, C> for Psor
where
    T: Real,
    SG: SpatialGrid<T>,
    C: Constraint<T, SG>,
{
    fn solve(
        &self,
        op: &TridiagonalOperator<T>,
        rhs: &[T],
        kappa: T,
        constraint: &C,
        mesher: &SG,
        x: &mut [T],
    ) {
        let n = op.size();
        let w = T::from_f64(self.omega);
        let tol = T::from_f64(self.tolerance);
        let one_minus_w = T::one() - w;

        for _ in 0..self.max_iter {
            let mut max_diff = T::zero();

            for i in 0..n {
                let old_xi = x[i];
                let mut sum = rhs[i];

                // Specialized logic: We know exactly where the neighbors are
                if i > 0 {
                    // A[i, i-1] = -kappa * lower[i]
                    sum -= (-kappa * op.lower[i]) * x[i - 1];
                }
                if i < n - 1 {
                    // A[i, i+1] = -kappa * upper[i]
                    sum -= (-kappa * op.upper[i]) * x[i + 1];
                }

                // A[i, i] = 1 - kappa * diag[i]
                let diag_ii = T::one() - kappa * op.diag[i];

                let x_gs = sum / diag_ii;
                let x_relaxed = one_minus_w * old_xi + w * x_gs;

                // Projection onto the obstacle/constraint
                x[i] = x_relaxed.max(constraint.lower_bound(i, mesher));

                let diff = (x[i] - old_xi).abs();
                if diff > max_diff {
                    max_diff = diff;
                }
            }

            if max_diff < tol {
                break;
            }
        }
    }
}
