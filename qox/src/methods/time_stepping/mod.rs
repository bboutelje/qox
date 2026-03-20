pub mod bdf2;
pub mod butcher_jackiewicz2;
pub mod crank_nicolson;
pub mod glm;
pub mod implicit_euler;
pub mod input_vectors;
pub mod sdirk22;

use crate::{
    methods::time_stepping::{
        glm::{GlmTableau, GlmWorkspace},
        input_vectors::InputVector,
    },
    types::Real,
};

pub trait TimeStepper<T: Real, IV: InputVector<T>, const S: usize, const R: usize> {
    fn tableau(&self) -> &GlmTableau<T, S, R>;

    fn prepare_stage_rhs(
        &self,
        stage_idx: usize,
        state: &IV,
        stages: &[T],
        l_stages: &[T],
        dt: T,
        rhs_out: &mut [T],
    );

    fn finalize_step(&self, state: &mut IV, ws: &GlmWorkspace<T>, dt: T);
}
