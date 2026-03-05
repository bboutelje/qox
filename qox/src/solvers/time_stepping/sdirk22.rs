use crate::{solvers::time_stepping::glm::{GlmState, GlmTableau, GlmWorkspace}, traits::{real::Real, time_stepper::TimeStepper}};

pub struct Sdirk22<T: Real> {
    tableau: GlmTableau<T, 2, 1>,
}

impl<T: Real> Sdirk22<T> {
    pub fn new() -> Self {
        let zero = T::zero();
        let one = T::one();
        let two = T::from_f64(2.0);
        let gamma = one - one / two.sqrt(); // 1 - 1/sqrt(2)
        let a21 = one - two * gamma;

        Self {
            tableau: GlmTableau {
                // A: [[gamma, 0], [1-2*gamma, gamma]]
                a: [[gamma, zero], 
                    [a21,   gamma]],
                // U: [[1], [1]] -> Both stages start from y_n
                u: [[one], [one]],
                // B: [[1-gamma, gamma]] -> Combination for y_{n+1}
                b: [[one - gamma, gamma]],
                // V: [[1]] -> History update
                v: [[one]],
                // C: [gamma, 1] -> Stage times
                c: [gamma, one],
            },
        }
    }
}

impl<T: Real> TimeStepper<T, 2, 1> for Sdirk22<T> {
    fn tableau(&self) -> &GlmTableau<T, 2, 1> { &self.tableau }

    fn prepare_stage_rhs(
        &self,
        stage_idx: usize,
        state: &GlmState<T>,
        _stages: &[T],
        l_stages: &[T], // This is key for SDIRK
        dt: T,
        rhs_out: &mut [T],
    ) {
        let n = state.n;
        let y_n = state.step_slice(0);

        if stage_idx == 0 {
            // Stage 1 RHS: y_n
            rhs_out.copy_from_slice(y_n);
        } else {
            // Stage 2 RHS: y_n + dt * a21 * L(Y1)
            let a21 = self.tableau.a[1][0];
            let l_y1 = &l_stages[0..n]; // First stage operator result

            for i in 0..n {
                rhs_out[i] = y_n[i] + dt * a21 * l_y1[i];
            }
        }
    }


    // Remove 'mut' from 'ws'
    fn finalize_step(&self, state: &mut GlmState<T>, ws: &GlmWorkspace<T>, dt: T) {
        let n = state.n;
        
        // These are immutable borrows of the workspace buffers
        let l_y1 = &ws.l_stages[0..n];
        let l_y2 = &ws.l_stages[n..2 * n];
        
        let b1 = self.tableau.b[0][0]; 
        let b2 = self.tableau.b[0][1]; 

        // Update state.items in place
        for i in 0..n {
            state.items[i] = state.items[i] + dt * (b1 * l_y1[i] + b2 * l_y2[i]);
        }
    }
}