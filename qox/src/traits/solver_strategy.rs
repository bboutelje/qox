pub trait SolverStrategy<T, M, L> {
    fn solve_stage(
        &self,
        operator: &L,
        rhs: &[T],
        coeff: T,
        next_t: T,
        mesher: &M,
        dest: &mut [T],
        z_buffer: &mut [T],
    );
}
