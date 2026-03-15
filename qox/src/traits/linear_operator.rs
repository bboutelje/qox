use crate::traits::{constraint::Constraint, fdm_mesher::Mesher1d, real::Real};

pub trait LinearOperator<T: Real, M: Mesher1d<T>> {
    /// Returns the number of grid nodes (N)
    fn size(&self) -> usize;

    /// Computes out = L(t) * v
    /// Equivalent to the 'function evaluation' f(t, y) in ODE solvers.
    fn apply_into(&self, v: &[T], t: T, out: &mut [T]);

    fn setup_coeff(&self, coeff: T);

    /// Solves (I - coeff * L(t)) * x = b
    /// Writes the result into 'dest'.
    /// This is where the Thomas Algorithm (TDMA) lives.
    fn solve_inverse_into(&self, b: &[T], coeff: T, _t: T, dest: &mut [T], z_buffer: &mut [T]);

    fn solve_psor_into<C>(
        &self,
        b: &[T],
        coeff: T,
        constraint: &C,
        mesher: &M,
        x: &mut [T],
        _z_buffer: &mut [T],
    ) where
        C: Constraint<T, M>;
}
