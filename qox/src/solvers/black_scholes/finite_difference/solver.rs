use crate::{
    solvers::{
        black_scholes::finite_difference::solver_old::FdmConfig,
        time_stepping::glm::{GlmState, GlmWorkspace},
    },
    traits::{
        fdm_mesher::Mesher1d, fdm_process::FdmProcess, linear_operator::LinearOperator,
        payoff::InitialConditions, real::Real, solver_strategy::SolverStrategy,
        time_stepper::TimeStepper, transform::Transform,
    },
};

pub struct Solver {
    pub config: FdmConfig,
}

impl Solver {
    pub fn solve<T, Tr, L, M, P, Step, IC, Strategy, const S: usize, const R: usize>(
        &self,
        process: P,
        stepper: Step,
        initial_conditions: IC,
        mesher: M,
        dt: T,
        config: FdmConfig,
        spot: T,
        strategy: Strategy,
    ) -> T
    where
        T: Real,
        Tr: Transform<T> + Copy,
        M: Mesher1d<T>,
        Step: TimeStepper<T, S, R>,
        P: FdmProcess<T, L, M, Tr>,
        L: LinearOperator<T, M>,
        IC: InitialConditions<T> + Copy,
        Strategy: SolverStrategy<T, M, L>,
    {
        let operator = process.build_operator(&mesher);

        let mut state = GlmState::<T>::new(R, config.nodes, T::zero());
        let mut workspace = GlmWorkspace::<T>::new(S, config.nodes);

        let initial_v = self.initialize_payoff(initial_conditions, &mesher);

        state.step_slice_mut(0).copy_from_slice(&initial_v);

        let n = self.config.nodes;
        if R > 1 {
            let (y_slice, f_slice) = state.items.split_at_mut(n);
            operator.apply_into(y_slice, T::zero(), f_slice);
        }

        for _ in 0..config.time_steps {
            let next_t = state.current_time + dt;

            for i in 0..S {
                stepper.prepare_stage_rhs(
                    i,
                    &state,
                    &workspace.stages,
                    &workspace.l_stages,
                    dt,
                    &mut workspace.rhs_buffer,
                );

                let stage_coeff = stepper.tableau().a[i][i] * dt;
                operator.setup_coeff(stage_coeff);

                let stage_slice = &mut workspace.stages[i * n..(i + 1) * n];

                strategy.solve_stage(
                    &operator,
                    &workspace.rhs_buffer,
                    stage_coeff,
                    next_t,
                    &mesher,
                    stage_slice,
                    &mut workspace.z_buffer,
                );

                let l_stage_slice = &mut workspace.l_stages[i * n..(i + 1) * n];
                operator.apply_into(stage_slice, next_t, l_stage_slice);
            }

            stepper.finalize_step(&mut state, &workspace, dt);
            state.current_time = next_t;

            // if R > 1 {
            //     let (y_slice, f_slice) = state.items.split_at_mut(n);
            //     operator.apply_into(y_slice, next_t, f_slice);
            // }
        }

        self.interpolate(&mesher, state.step_slice(0), spot)
    }

    fn initialize_payoff<T, IC, M>(&self, initial_condition: IC, mesher: &M) -> Vec<T>
    where
        T: Real,
        IC: InitialConditions<T> + Copy,
        M: Mesher1d<T>,
    {
        (0..mesher.size())
            .map(|i| {
                let s = mesher.location(i);
                initial_condition.get_value(s)
            })
            .collect()
    }

    fn interpolate<T, M>(&self, mesher: &M, v: &[T], spot: T) -> T
    where
        T: Real,
        M: Mesher1d<T>,
    {
        let target = spot.ln();
        let centers = mesher.centers();

        let idx = match centers.binary_search_by(|val| {
            val.scalar()
                .partial_cmp(&target.scalar())
                .expect("NaN in Grid")
        }) {
            Ok(exact) => return v[exact].clone(),
            Err(i) => {
                if i == 0 {
                    return v[0].clone();
                }
                if i >= centers.len() {
                    return v[v.len() - 1].clone();
                }
                i - 1
            }
        };

        let x0 = centers[idx];
        let x1 = centers[idx + 1];
        let weight = (target - x0) / (x1 - x0);
        v[idx] + (weight * (v[idx + 1] - v[idx]))
    }
}
