use crate::{
    methods::time_stepping::{
        TimeStepper,
        glm::{GlmTableau, GlmWorkspace},
        input_vectors::{InputVector, nordsieck_vector::NordsieckVector},
    },
    types::Real,
};

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
                a: [[gamma, zero], [a21, gamma]],
                u: [[one], [one]],
                b: [[one - gamma, gamma]],
                v: [[one]],
                c: [gamma, one],
            },
        }
    }
}

impl<T: Real> TimeStepper<T, NordsieckVector<T>, 2, 1> for Sdirk22<T> {
    fn tableau(&self) -> &GlmTableau<T, 2, 1> {
        &self.tableau
    }

    fn prepare_stage_rhs(
        &self,
        stage_idx: usize,
        state: &NordsieckVector<T>,
        _stages: &[T],
        l_stages: &[T],
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

    fn finalize_step(&self, state: &mut NordsieckVector<T>, ws: &GlmWorkspace<T>, dt: T) {
        let n = state.n;

        let l_y1 = &ws.l_stages[0..n];
        let l_y2 = &ws.l_stages[n..2 * n];

        let b1 = self.tableau.b[0][0];
        let b2 = self.tableau.b[0][1];

        for i in 0..n {
            state.items[i] = state.items[i] + dt * (b1 * l_y1[i] + b2 * l_y2[i]);
        }
    }
}
