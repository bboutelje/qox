use crate::{solvers::time_stepping::glm::{GlmState, GlmTableau, GlmWorkspace}, traits::{real::Real, time_stepper::TimeStepper}};

pub struct DimsimCN<T: Real> {
    tableau: GlmTableau<T, 1, 2>,
}

impl<T: Real> DimsimCN<T> {
    pub fn new() -> Self {
        let zero = T::zero();
        let half = T::from_f64(0.5);
        let one = T::one();

        Self {
            tableau: GlmTableau {
                // A: Stage is implicit with weight 0.5
                a: [[half]],
                // U: Y1 = 1.0 * y_n + 0.5 * dt * f(y_n)
                u: [[one, half]],
                // B: Updates for [y_{n+1}, f(y_{n+1})]
                // Row 0: y_{n+1} = y_n + 0.5*dt*f(y_n) + 0.5*dt*f(Y1)
                // Row 1: f(y_{n+1}) = 0*y_n + 0*f(y_n) + 1.0*f(Y1)
                b: [[half], [one]],
                // V: Transfers history [y_n, f(y_n)]
                v: [[one, half],
                    [zero, zero]],
                // C: Time offset is 1.0 (end of step)
                c: [one],
            },
        }
    }
}

impl<T: Real> TimeStepper<T, 1, 2> for DimsimCN<T> {
    fn tableau(&self) -> &GlmTableau<T, 1, 2> { &self.tableau }

    fn prepare_stage_rhs(
        &self,
        _stage_idx: usize, // stage_idx is always 0 for s=1
        state: &GlmState<T>,
        _stages: &[T],
        _l_stages: &[T],
        dt: T,
        rhs_out: &mut [T],
    ) {
        let n = state.n;
        // items[0..n] is y_n, items[n..2n] is f(y_n)
        let y_n = &state.items[0..n];
        let f_n = &state.items[n..2*n];

        let u11 = self.tableau.u[0][0]; // 1.0
        let u12 = self.tableau.u[0][1]; // 0.5

        // RHS for the implicit solve: Y1 = y_n + dt * 0.5 * f(y_n)
        for i in 0..n {
            rhs_out[i] = u11 * y_n[i] + dt * u12 * f_n[i];
        }
    }

    fn finalize_step(&self, state: &mut GlmState<T>, ws: &GlmWorkspace<T>, dt: T) {
        let n = state.n;
        let l_y1 = &ws.l_stages[0..n]; // f(Y1)
        
        // V coefficients
        let v11 = self.tableau.v[0][0]; // 1.0
        let v12 = self.tableau.v[0][1]; // 0.5
        
        // B coefficients
        let b11 = self.tableau.b[0][0]; // 0.5
        let b21 = self.tableau.b[1][0]; // 1.0

        let (y_hist, f_hist) = state.items.split_at_mut(n);

        for i in 0..n {
            let y_old = y_hist[i];
            let f_old = f_hist[i];

            // y_{n+1} = v11*y_n + dt*v12*f_n + dt*b11*f(Y1)
            y_hist[i] = v11 * y_old + dt * (v12 * f_old + b11 * l_y1[i]);
            
            // f_{n+1} = b21 * f(Y1) (since v21 and v22 are zero)
            f_hist[i] = b21 * l_y1[i];
        }
    }
}