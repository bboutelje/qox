use crate::{
    solvers::time_stepping::glm::{GlmState, GlmTableau, GlmWorkspace},
    traits::real::Real,
};

pub trait TimeStepper<T: Real, const S: usize, const R: usize> {
    fn tableau(&self) -> &GlmTableau<T, S, R>;

    fn prepare_stage_rhs(
        &self,
        stage_idx: usize,
        state: &GlmState<T>,
        stages: &[T],
        l_stages: &[T],
        dt: T,
        rhs_out: &mut [T],
    );

    fn finalize_step(&self, state: &mut GlmState<T>, ws: &GlmWorkspace<T>, dt: T);
}
