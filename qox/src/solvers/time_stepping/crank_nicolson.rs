use crate::{
    solvers::time_stepping::{
        TimeStepper,
        glm::{GlmState, GlmTableau, GlmWorkspace},
    },
    types::{Real, complex::ComplexWrapper},
};
use nalgebra::Complex;

pub struct CrankNicolson<T: Real> {
    tableau: GlmTableau<T, 1, 2>,
}

impl<T: Real> CrankNicolson<T> {
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
                v: [[one, half], [zero, zero]],
                // C: Time offset is 1.0 (end of step)
                c: [one],
            },
        }
    }
}

impl<T: Real> TimeStepper<T, 1, 2> for CrankNicolson<T> {
    fn tableau(&self) -> &GlmTableau<T, 1, 2> {
        &self.tableau
    }

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
        let f_n = &state.items[n..2 * n];

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

#[cfg(test)]
mod tests {
    use super::*;
    // Note: Ensure GlmState and GlmWorkspace are accessible or imported

    fn exact_solution(t: f64) -> f64 {
        (-t).exp()
    }

    fn f(y: f64) -> f64 {
        -y
    }

    #[test]
    fn crank_nicolson_convergence_order() {
        let method = CrankNicolson::<f64>::new();

        let t_final = 1.0;
        let exact = exact_solution(t_final);

        let steps = [2usize, 4, 8, 16, 32];
        let mut prev_error: Option<f64> = None;

        for &n_steps in &steps {
            let dt = t_final / n_steps as f64;

            // 1. Fix GlmState::new: Needs (r, n, current_time)
            // r=2 (from Dimsim2<T, 2, 2>), n=1 (scalar problem), t=0.0
            let mut state = GlmState::<f64>::new(2, 1, 0.0);

            // Initial conditions
            let y0 = 1.0;
            state.items[0] = y0; // y_n
            state.items[1] = f(y0); // f(y_n) (The second history item for DIMSIM2)

            // 2. Fix GlmWorkspace::new: Needs (s, n)
            // s=2 (stages), n=1 (nodes)
            let mut ws = GlmWorkspace::<f64>::new(2, 1);

            for _ in 0..n_steps {
                // Note: GLM step now updates state.current_time internally
                // inside the test loop in dimsim2.rs
                crate::solvers::time_stepping::glm::step(
                    &method,
                    &mut state,
                    &mut ws,
                    dt,
                    |y: &[f64], out: &mut [f64]| {
                        out[0] = -y[0];
                    },
                );
            }

            let y_final = state.items[0];
            let error = (y_final - exact).abs();

            println!("N = {:>3}, error = {:.8e}", n_steps, error);

            if let Some(prev) = prev_error {
                let order = (prev / error).log2();
                println!("observed order ≈ {:.4}", order);
            }

            prev_error = Some(error);
        }
    }
}

#[allow(dead_code)]
fn stability_function(method: &CrankNicolson<ComplexWrapper>, z: ComplexWrapper) -> ComplexWrapper {
    // Dimsim2 uses R=2, N=1
    let mut state = GlmState::<ComplexWrapper>::new(2, 1, ComplexWrapper(Complex::new(0.0, 0.0)));

    // state.items needs to hold y_n and f(y_n)
    state.items[0] = ComplexWrapper(Complex::new(1.0, 0.0));
    state.items[1] = z; // f(y) = z*y => f(1) = z

    // Dimsim2 uses S=2, N=1
    let mut ws = GlmWorkspace::<ComplexWrapper>::new(2, 1);
    let dt = ComplexWrapper(Complex::new(1.0, 0.0));

    crate::solvers::time_stepping::glm::step_for_stability(
        method, &mut state, &mut ws, dt, z, // pass the stability parameter
    );

    // The stability function returns the new y_n
    state.items[0]
}

#[test]
fn crank_nicolson_a_stability_test() {
    // Use the wrapper here
    let method = CrankNicolson::<ComplexWrapper>::new();

    for re in (-100..0).map(|x| x as f64 * 0.1) {
        for im in (-50..50).map(|x| x as f64 * 0.1) {
            let z = ComplexWrapper(Complex::new(re, im));
            let r = stability_function(&method, z);

            // Access the inner complex for the norm calculation
            assert!(
                r.0.norm() <= 1.0 + 1e-8,
                "unstable at z={:?}, |R|={}",
                z,
                r.0.norm()
            );
        }
    }
}
