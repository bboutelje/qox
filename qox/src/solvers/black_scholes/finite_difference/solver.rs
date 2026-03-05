use std::ops::Neg;
use crate::solvers::time_stepping::glm::GlmState;
use crate::traits::linear_operator::LinearOperator;
use crate::traits::time_stepper::TimeStepper;
use crate::{solvers::{black_scholes::finite_difference::{meshing::{log::LogMesher1d, uniform::UniformMesher1d}, operator::BsOperator}, time_stepping::glm::GlmWorkspace}, traits::{EvaluationResolver, Resolved, fdm_1d_mesher::Mesher1d, payoff::InitialCondition, real::Real}};

pub struct Solver {
    pub config: FdmConfig,
}

#[derive(Debug, Clone, Copy)]
pub struct FdmConfig {
    pub nodes: usize,
    pub time_steps: usize,
    pub damping_steps: usize,
}

impl Solver {

    pub fn solve<IC, TReal, SReal, RCReal, VSReal, Step, const S: usize, const R: usize>(
        self, 
        stepper: Step, 
        initial_condition: IC,
        time_to_expiry: TReal,
        spot: SReal,
        rate: RCReal,
        vol: VSReal
    ) -> Resolved<SReal, RCReal, VSReal>
    where 
        TReal: Real,
        RCReal: Real,
        VSReal: Real,
        SReal: Real + EvaluationResolver<RCReal, VSReal>,
        IC: InitialCondition<Resolved<SReal, RCReal, VSReal>> + Copy,
        Resolved<SReal, RCReal, VSReal>: Real + From<SReal> + From<TReal> + From<RCReal> + From<VSReal> + Neg<Output = Resolved<SReal, RCReal, VSReal>> + Copy,
        Step: TimeStepper<Resolved<SReal, RCReal, VSReal>, S, R>,
    {
        let zero = Resolved::<SReal, RCReal, VSReal>::zero();
        
        let s_min = Resolved::<SReal, RCReal, VSReal>::from_f64(0.01);
        let s_max = Resolved::<SReal, RCReal, VSReal>::from(spot.clone()) * Resolved::<SReal, RCReal, VSReal>::from_f64(5.0);
        
        let mesher = LogMesher1d::new(UniformMesher1d::new(s_min.ln(), s_max.ln(), self.config.nodes));
        let dt = Resolved::<SReal, RCReal, VSReal>::from(time_to_expiry) / Resolved::<SReal, RCReal, VSReal>::from_f64(self.config.time_steps as f64);
        
        let operator = BsOperator {
            mesher: &mesher,
            r: rate.into(),
            sigma: vol.into(),
            cache: std::cell::RefCell::new(None),
        };

        // Initialize GLM structures with const R and S from the stepper
        let mut state = GlmState::<Resolved<SReal, RCReal, VSReal>>::new(R, self.config.nodes, zero);
        let mut workspace = GlmWorkspace::<Resolved<SReal, RCReal, VSReal>>::new(S, self.config.nodes);

        // Load initial payoff into the state
        let initial_v = self.initialize_payoff(initial_condition, &mesher);
        state.step_slice_mut(0).copy_from_slice(&initial_v);

        if R > 1 {
            let n = self.config.nodes;
            let (y_slice, f_slice) = state.items.split_at_mut(n);
            // Use the operator to compute f_0 = L(y_0)
            operator.apply_into(y_slice, zero,f_slice); 
        }

        let n = self.config.nodes;
        for step in 0..self.config.time_steps {

            // let v = state.step_slice(0);

            // println!(
            //     "step {} | min {:?} max {:?} mid {:?}",
            //     step,
            //     v.iter().cloned().fold(v[0], |a, b| a.min(b)),
            //     v.iter().cloned().fold(v[0], |a, b| a.max(b)),
            //     v[v.len() / 2]
            // );


            let next_t = state.current_time + dt;

            for i in 0..S {
                stepper.prepare_stage_rhs(
                    i,
                    &state,
                    &workspace.stages,
                    &workspace.l_stages,
                    dt,
                    &mut workspace.rhs_buffer
                );

                let stage_coeff = stepper.tableau().a[i][i] * dt;
                operator.setup_coeff(stage_coeff);

                let stage_slice = &mut workspace.stages[i * n .. (i + 1) * n];
                
                operator.solve_inverse_into(
                    &workspace.rhs_buffer, 
                    stage_coeff, 
                    next_t, 
                    stage_slice, 
                    &mut workspace.z_buffer // Pass the pre-allocated temp buffer here
                );

                let l_stage_slice = &mut workspace.l_stages[i*n .. (i+1)*n];
                operator.apply_into(stage_slice, next_t, l_stage_slice);
            }

            stepper.finalize_step(&mut state, &workspace, dt);
            state.current_time = next_t;
        }

        self.interpolate(&mesher, state.step_slice(0), spot.into())
    }
}

impl Solver {
    fn initialize_payoff<T, IC, M>(&self, initial_condition: IC, mesher: &M) -> Vec<T> 
    where
        T: Real + Copy,
        IC: InitialCondition<T> + Copy,
        M: Mesher1d<T>
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
        M: Mesher1d<T>
    {
        let target = spot.ln();
        let centers = mesher.centers();
        
        let idx = match centers.binary_search_by(|val| {
            val.scalar().partial_cmp(&target.scalar()).expect("NaN in Grid")
        }) {
            Ok(exact) => return v[exact].clone(),
            Err(i) => {
                if i == 0 { return v[0].clone(); }
                if i >= centers.len() { return v[v.len() - 1].clone(); }
                i - 1
            }
        };

        let x0 = centers[idx];
        let x1 = centers[idx + 1];
        let weight = (target - x0) / (x1 - x0);
        v[idx] + (weight * (v[idx + 1] - v[idx]))
    }
}