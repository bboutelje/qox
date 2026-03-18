pub mod bdf2;
pub mod crank_nicolson;
pub mod dimsim2;
pub mod glm;
pub mod implicit_euler;
pub mod sdirk22;

use crate::{
    methods::time_stepping::glm::{GlmTableau, GlmWorkspace, InputVector},
    types::Real,
};

pub trait TimeStepper<T: Real, const S: usize, const R: usize> {
    fn tableau(&self) -> &GlmTableau<T, S, R>;

    fn prepare_stage_rhs(
        &self,
        stage_idx: usize,
        state: &InputVector<T>,
        stages: &[T],
        l_stages: &[T],
        dt: T,
        rhs_out: &mut [T],
    );

    fn finalize_step(&self, state: &mut InputVector<T>, ws: &GlmWorkspace<T>, dt: T);
}
