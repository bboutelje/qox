use crate::{
    solvers::time_stepping::glm::{GlmState, GlmTableau, GlmWorkspace}, 
    traits::{real::Real, time_stepper::TimeStepper}
};

pub struct Bdf2<T: Real> {
    tableau: GlmTableau<T, 1, 2>,
}

impl<T: Real> Bdf2<T> {
    pub fn new() -> Self {
        let zero = T::zero();
        let one = T::one();
        let two = T::from_f64(2.0);
        let three = T::from_f64(3.0);
        let four = T::from_f64(4.0);

        let gamma = two / three; // 2/3
        let u0 = four / three;    // 4/3
        let u1 = -one / three;   // -1/3

        Self {
            tableau: GlmTableau {
                // A: [[T; S]; S] -> [[T; 1]; 1]
                a: [[gamma]], 
                
                // U: [[T; R]; S] -> [[T; 2]; 1] 
                // One row (stage), two columns (history)
                u: [[u0, u1]], 

                // B: [[T; S]; R] -> [[T; 1]; 2]
                // Two rows (history), one column (stage)
                b: [[gamma], [zero]], 

                // V: [[T; R]; R] -> [[T; 2]; 2]
                // Two rows, two columns
                v: [[u0, u1], 
                    [one, zero]], 

                // C: [T; S] -> [T; 1]
                c: [one],
            },
        }
    }
}

impl<T: Real> TimeStepper<T, 1, 2> for Bdf2<T> {
    fn tableau(&self) -> &GlmTableau<T, 1, 2> { &self.tableau }

    fn prepare_stage_rhs(
        &self,
        _stage_idx: usize,
        state: &GlmState<T>,
        _stages: &[T],
        _l_stages: &[T],
        _dt: T,
        rhs_out: &mut [T],
    ) {
        let y_n = state.step_slice(0);
        let y_nm1 = state.step_slice(1);
        let u0 = self.tableau.u[0][0]; // 4/3
        let u1 = self.tableau.u[0][1]; // -1/3

        for i in 0..state.n {
            rhs_out[i] = u0 * y_n[i] + u1 * y_nm1[i];
        }
    }

    fn finalize_step(&self, state: &mut GlmState<T>, ws: &GlmWorkspace<T>, _dt: T) {
        let n = state.n;
        
        // 1. Shift history: Move y_n (at 0..n) into the y_n-1 slot (at n)
        // copy_within(source_range, destination_index)
        state.items.copy_within(0..n, n); 

        // 2. Update current: Move the newly solved stage 0 into the y_n slot (at 0..n)
        let stage_0 = &ws.stages[0..n];
        state.step_slice_mut(0).copy_from_slice(stage_0);
    }
}