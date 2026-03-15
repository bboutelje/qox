use crate::traits::{
    constraint::Constraint, fdm_mesher::Mesher1d, linear_operator::LinearOperator, real::Real,
    solver_strategy::SolverStrategy,
};

pub struct Unconstrained;
impl<T: Real, M: Mesher1d<T>, L: LinearOperator<T, M>> SolverStrategy<T, M, L> for Unconstrained {
    fn solve_stage(&self, op: &L, b: &[T], coeff: T, _t: T, _m: &M, dest: &mut [T], z: &mut [T]) {
        op.solve_inverse_into(b, coeff, _t, dest, z);
    }
}

pub struct Constrained<C> {
    pub constraint: C,
}
impl<T: Real, M: Mesher1d<T>, L: LinearOperator<T, M>, C: Constraint<T, M>> SolverStrategy<T, M, L>
    for Constrained<C>
{
    fn solve_stage(&self, op: &L, b: &[T], coeff: T, _t: T, m: &M, dest: &mut [T], z: &mut [T]) {
        op.solve_psor_into(b, coeff, &self.constraint, m, dest, z);
    }
}
