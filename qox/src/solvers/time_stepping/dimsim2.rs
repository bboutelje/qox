use nalgebra::Complex;

use crate::{
    solvers::time_stepping::glm::{GlmState, GlmTableau, GlmWorkspace},
    traits::{real::Real, time_stepper::TimeStepper},
};

pub struct Dimsim2<T: Real> {
    tableau: GlmTableau<T, 2, 2>,
}

impl<T: Real> Dimsim2<T> {
    pub fn new() -> Self {
        let zero = T::zero();
        let one = T::one();
        let gamma = T::from_f64(0.292893218813452); // L-stable gamma

        let a21 = one - gamma;

        Self {
            tableau: GlmTableau {
                // Stage matrix A
                a: [
                    [gamma, zero],
                    [a21, gamma],
                ],

                // Stage starting combination U (using y_n, h*f_n)
                u: [
                    [one, zero],
                    [one, zero],
                ],

                // Stage derivatives combination B (for final update)
                b: [
                    [one - gamma, gamma],
                    [zero, one],
                ],

                // History propagation V (Type-2, rank 1)
                v: [
                    [one, zero],
                    [one, zero],
                ],

                // Stage time offsets
                c: [
                    gamma,
                    one,
                ],
            },
        }
    }
}

impl<T: Real> TimeStepper<T, 2, 2> for Dimsim2<T> {
    fn tableau(&self) -> &GlmTableau<T, 2, 2> {
        &self.tableau
    }

    fn prepare_stage_rhs(
        &self,
        stage_idx: usize,
        state: &GlmState<T>,
        stages: &[T],
        l_stages: &[T],
        dt: T,
        rhs_out: &mut [T],
    ) {
        let n = state.n;
        let y_n = &state.items[0..n];
        let f_n = &state.items[n..2 * n];

        if stage_idx == 0 {
            // Stage 1:
            // Y1 = y_n + dt * 0.5 * f(y_n)

            let u11 = self.tableau.u[0][0];
            let u12 = self.tableau.u[0][1];

            for i in 0..n {
                rhs_out[i] = u11 * y_n[i] + dt * u12 * f_n[i];
            }

        } else {
            // Stage 2:
            // Y2 = y_n + dt * 0.5 * f(y_n) + dt * a21 * f(Y1)

            let u21 = self.tableau.u[1][0];
            let u22 = self.tableau.u[1][1];
            let a21 = self.tableau.a[1][0];

            let l_y1 = &l_stages[0..n];

            for i in 0..n {
                rhs_out[i] =
                    u21 * y_n[i]
                    + dt * u22 * f_n[i]
                    + dt * a21 * l_y1[i];
            }
        }

    }


    fn finalize_step(&self, state: &mut GlmState<T>, ws: &GlmWorkspace<T>, dt: T) {
        let n = state.n;

        let l_y1 = &ws.l_stages[0..n];
        let l_y2 = &ws.l_stages[n..2 * n];

        for i in 0..n {
            let y_old = state.items[i];
            let f_old = state.items[n + i];

            state.items[i] =
                self.tableau.v[0][0] * y_old +
                dt * self.tableau.v[0][1] * f_old +
                dt * (
                    self.tableau.b[0][0] * l_y1[i] +
                    self.tableau.b[0][1] * l_y2[i]
                );

            state.items[n + i] =
                self.tableau.v[1][0] * y_old +
                self.tableau.v[1][1] * f_old +
                dt * (
                    self.tableau.b[1][0] * l_y1[i] +
                    self.tableau.b[1][1] * l_y2[i]
                );
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
    fn dimsim2_convergence_order() {
        let method = Dimsim2::<f64>::new();
        
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
            state.items[0] = y0;      // y_n
            state.items[1] = f(y0);   // f(y_n) (The second history item for DIMSIM2)

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

use crate::real::complex::ComplexWrapper; // Ensure this is imported

fn stability_function(method: &Dimsim2<ComplexWrapper>, z: ComplexWrapper) -> ComplexWrapper {
    // Dimsim2 uses R=2, N=1
    let mut state = GlmState::<ComplexWrapper>::new(2, 1, ComplexWrapper(Complex::new(0.0, 0.0)));

    // state.items needs to hold y_n and f(y_n)
    state.items[0] = ComplexWrapper(Complex::new(1.0, 0.0));
    state.items[1] = z; // f(y) = z*y => f(1) = z

    // Dimsim2 uses S=2, N=1
    let mut ws = GlmWorkspace::<ComplexWrapper>::new(2, 1);
    let dt = ComplexWrapper(Complex::new(1.0, 0.0));

    crate::solvers::time_stepping::glm::step_for_stability(
        method,
        &mut state,
        &mut ws,
        dt,
        z, // pass the stability parameter
    );

    // The stability function returns the new y_n
    state.items[0]
}

#[test]
fn dimsim_a_stability_test() {
    // Use the wrapper here
    let method = Dimsim2::<ComplexWrapper>::new();

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