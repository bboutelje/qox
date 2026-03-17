use crate::{
    solvers::time_stepping::{
        TimeStepper,
        glm::{GlmState, GlmTableau, GlmWorkspace},
    },
    types::Real,
};
pub struct ImplicitEuler<T: Real> {
    tableau: GlmTableau<T, 1, 1>,
}

impl<T: Real> ImplicitEuler<T> {
    pub fn new() -> Self {
        let one = T::one();
        Self {
            tableau: GlmTableau {
                a: [[one]],
                u: [[one]],
                b: [[one]],
                v: [[one]],
                c: [one],
            },
        }
    }
}
impl<T: Real> TimeStepper<T, 1, 1> for ImplicitEuler<T> {
    fn tableau(&self) -> &GlmTableau<T, 1, 1> {
        &self.tableau
    }

    fn prepare_stage_rhs(
        &self,
        _stage_idx: usize,
        state: &GlmState<T>,
        _stages: &[T],
        _l_stages: &[T],
        _dt: T,
        rhs_out: &mut [T],
    ) {
        rhs_out.copy_from_slice(state.step_slice(0));
    }

    fn finalize_step(&self, state: &mut GlmState<T>, ws: &GlmWorkspace<T>, _dt: T) {
        // Move the computed stage 0 from workspace back to state items
        // We use the first N elements of ws.stages (which represents stage 0)
        let n = state.n;
        state.step_slice_mut(0).copy_from_slice(&ws.stages[0..n]);
    }
}
