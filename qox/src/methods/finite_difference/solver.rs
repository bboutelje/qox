use crate::{
    methods::{
        finite_difference::meshers::{Mesher1d, SpatialGrid},
        linear_operators::LinearOperator,
        step_policy::StepPolicy,
        time_stepping::{
            TimeStepper,
            glm::GlmWorkspace,
            input_vectors::{InputVector, nordsieck_vector::NordsieckVector},
        },
    },
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
    pub fn solve<T, L, SG, Step, IC, SP, const S: usize, const R: usize>(
        &self,
        stepper: Step,
        initial_conditions: IC,
        grid: &SG,
        dt: T,
        config: FdmConfig,
        step_policy: &SP,
    ) -> NordsieckVector<T>
    where
        T: Real,
        SG: SpatialGrid<T>,
        Step: TimeStepper<T, NordsieckVector<T>, S, R>,
        L: LinearOperator<T>,
        IC: InitialConditions<T> + Copy,
        SP: StepPolicy<T, SG, L>,
    {
        let mut vector = NordsieckVector::<T>::new(R, config.nodes, T::zero());
        let mut workspace = GlmWorkspace::<T>::new(S, config.nodes);

        let initial_v = self.initialize_payoff(initial_conditions, grid);

        vector.step_slice_mut(0).copy_from_slice(&initial_v);

        let n = self.config.nodes;
        if R > 1 {
            let (y_slice, f_slice) = vector.items.split_at_mut(n);
            step_policy.get_operator().apply_into(y_slice, f_slice);
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
                step_policy.get_operator().setup_coeff(stage_coeff);

                let stage_slice = &mut workspace.stages[i * n..(i + 1) * n];

                step_policy.solve_stage_into(
                    &workspace.rhs_buffer,
                    dt,
                    &grid,
                    stage_slice,
                    &mut workspace.z_buffer,
                );

                let l_stage_slice = &mut workspace.l_stages[i * n..(i + 1) * n];
                step_policy.compute_stage_derivative(
                    stage_slice,
                    &grid,
                    initial_conditions,
                    l_stage_slice,
                );
            }

            stepper.finalize_step(&mut vector, &workspace, dt);
            vector.current_time = next_t;
        }

        vector
        //self.interpolate(&mesher, vector.step_slice(0), spot)
    }

    fn initialize_payoff<T, IC, SG>(&self, initial_condition: IC, spatial_grid: &SG) -> Vec<T>
    where
        T: Real,
        IC: InitialConditions<T> + Copy,
        SG: SpatialGrid<T>,
    {
        (0..spatial_grid.size())
            .map(|i| {
                let s = spatial_grid.location(i);
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
