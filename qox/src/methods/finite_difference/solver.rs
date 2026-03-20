use crate::{
    methods::{
        finite_difference::meshers::Mesher1d,
        linear_operators::LinearOperator,
        obstacle_policies::ObstaclePolicy,
        time_stepping::{
            TimeStepper,
            glm::GlmWorkspace,
            input_vectors::{InputVector, nordsieck_vector::NordsieckVector},
        },
        transforms::Transform,
    },
    processes::FdmProcess,
    traits::payoff::InitialConditions,
    types::Real,
};

pub struct Solver {
    pub config: FdmConfig,
}

#[derive(Debug, Clone, Copy)]
pub struct FdmConfig {
    pub nodes: usize,
    pub time_steps: usize,
}

impl Solver {
    pub fn solve<T, Tr, L, M, P, Step, IC, OP, const S: usize, const R: usize>(
        &self,
        process: P,
        stepper: Step,
        initial_conditions: IC,
        mesher: &M,
        dt: T,
        config: FdmConfig,
        obstacle_policy: OP,
    ) -> NordsieckVector<T>
    where
        T: Real,
        Tr: Transform<T> + Copy,
        M: Mesher1d<T>,
        Step: TimeStepper<T, NordsieckVector<T>, S, R>,
        P: FdmProcess<T, L, M, Tr>,
        L: LinearOperator<T>,
        IC: InitialConditions<T> + Copy,
        OP: ObstaclePolicy<T, M, L>,
    {
        let operator = process.build_operator(&mesher);

        let mut vector = NordsieckVector::<T>::new(R, config.nodes, T::zero());
        let mut workspace = GlmWorkspace::<T>::new(S, config.nodes);

        let initial_v = self.initialize_payoff(initial_conditions, mesher);

        vector.step_slice_mut(0).copy_from_slice(&initial_v);

        let n = self.config.nodes;
        if R > 1 {
            let (y_slice, f_slice) = vector.items.split_at_mut(n);
            operator.apply_into(y_slice, T::zero(), f_slice);
        }

        for _ in 0..config.time_steps {
            let next_t = vector.current_time + dt;

            for i in 0..S {
                stepper.prepare_stage_rhs(
                    i,
                    &vector,
                    &workspace.stages,
                    &workspace.l_stages,
                    dt,
                    &mut workspace.rhs_buffer,
                );

                let stage_coeff = stepper.tableau().a[i][i] * dt;
                operator.setup_coeff(stage_coeff);

                let stage_slice = &mut workspace.stages[i * n..(i + 1) * n];

                obstacle_policy.solve_stage(
                    &operator,
                    &workspace.rhs_buffer,
                    stage_coeff,
                    next_t,
                    &mesher,
                    stage_slice,
                    &mut workspace.z_buffer,
                );

                let l_stage_slice = &mut workspace.l_stages[i * n..(i + 1) * n];
                obstacle_policy.compute_stage_derivative(
                    &operator,
                    stage_slice,
                    next_t,
                    &mesher,
                    initial_conditions,
                    l_stage_slice,
                );
                // operator.apply_into(stage_slice, next_t, l_stage_slice);

                // for j in 0..n {
                //     let s = mesher.location(j);
                //     let payoff = initial_conditions.get_value(s);

                //     // If we are at or below the payoff (for a Put) or above (for a Call),
                //     // the value is 'pinned', so the time derivative f(y) effectively becomes 0.
                //     if stage_slice[j] <= payoff + T::from_f64(f64::EPSILON) {
                //         l_stage_slice[j] = T::zero();
                //     }
                // }
            }

            stepper.finalize_step(&mut vector, &workspace, dt);
            vector.current_time = next_t;

            // if R > 1 {
            //     let (y_slice, f_slice) = vector.items.split_at_mut(n);
            //     operator.apply_into(y_slice, next_t, f_slice);
            // }
        }

        vector
        //self.interpolate(&mesher, vector.step_slice(0), spot)
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

    pub fn interpolate<T, M>(&self, mesher: &M, v: &[T], spot: T) -> T
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
