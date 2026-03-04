
use std::ops::{Neg};
use std::time::Instant;

use crate::solvers::black_scholes::finite_difference::meshing::log::LogMesher1d;
use crate::solvers::black_scholes::finite_difference::meshing::uniform::UniformMesher1d;
use crate::traits::Resolved;
use crate::traits::payoff::{InitialCondition};
use crate::traits::{EvaluationResolver, fdm_1d_mesher::Mesher1d, real::Real};

pub struct Solver {
    pub config: FdmConfig,
}

#[derive(Debug, Clone, Copy)]
pub struct FdmConfig {
    pub nodes: usize,
    pub time_steps: usize,
    pub damping_steps: usize,
}

impl Solver
{
    //type Result = <SReal as EvaluationResolver<RCReal, VSReal>>::Output;
    //#[inline(always)]
    pub fn solve<IC, TReal, SReal, RCReal, VSReal>(
        self, 
        initial_condition: IC,
        time_to_expiry: TReal,
        spot: SReal,
        rate: RCReal,
        vol: VSReal,
        //market: &OptionMarketData<SReal, RC, VS>,
    ) -> Resolved<SReal, RCReal, VSReal>
        where 
            TReal: Real,
            RCReal: Real,
            VSReal: Real,
            SReal: Real + EvaluationResolver<RCReal, VSReal>,
            IC: InitialCondition<Resolved<SReal, RCReal, VSReal>> + Copy,
            Resolved<SReal, RCReal, VSReal>: Real + 
                From<SReal> + 
                From<TReal> + 
                From<RCReal> + 
                From<VSReal> + 
                Neg<Output = Resolved<SReal, RCReal, VSReal>>,
    {
        let s_min = <Resolved<SReal, RCReal, VSReal>>::from_f64(0.01);
        let spot_res: Resolved<SReal, RCReal, VSReal> = spot.clone().into();
        let s_max = spot_res * <Resolved<SReal, RCReal, VSReal>>::from_f64(3.0);
        
        let x_min = s_min.ln();
        let x_max = s_max.ln();

        let uniform_mesher = UniformMesher1d::new(x_min, x_max, self.config.nodes);
        let mesher = LogMesher1d::new(uniform_mesher);

        let dt: Resolved<SReal, RCReal, VSReal> = (time_to_expiry / TReal::from_f64(self.config.time_steps as f64)).into();

        let r: Resolved<SReal, RCReal, VSReal> = rate.into();
        let sigma: Resolved<SReal, RCReal, VSReal> = vol.into();

        let (a, c_prime, m_inv) = self.setup_bs_solver(&mesher, r, sigma, dt);

        let mut v_curr = self.initialize_payoff(initial_condition, &mesher);
        //let mut v_curr = self.initialize_payoff(instrument, &mesher);
        let mut v_next = vec![<Resolved<SReal, RCReal, VSReal>>::zero(); self.config.nodes];
        let mut d_scratch = vec![<Resolved<SReal, RCReal, VSReal>>::zero(); self.config.nodes];

        //let duration = start_time.elapsed();

        //println!("setup took: {:?}", duration);

        //let start_time = Instant::now();
        
        // 3. Time-stepping loop (European)
        for _ in 0..self.config.time_steps {
            Self::solve_step_inplace(
                a,
                &c_prime,
                &m_inv,
                &v_curr,
                &mut v_next,
                &mut d_scratch
            );

            std::mem::swap(&mut v_curr, &mut v_next);
        }


        let s_spot: Resolved<SReal, RCReal, VSReal> = spot.clone().into();
        let res = self.interpolate(&mesher, &v_curr, s_spot);

        res
    }
    
    
    
}

impl Solver {
    fn initialize_payoff<T, IC, M>(&self, initial_condition: IC, mesher: &M) -> Vec<T> 
    where
        T: Real,
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

    fn setup_bs_solver<T: Real, M: Mesher1d<T>>(
        &self, 
        mesher: &M, 
        r: T, 
        sigma: T,
        dt: T
    ) -> (T, Vec<T>, Vec<T>) 
    where T: Real

    {
        let n = mesher.size();
        let h = mesher.h_plus()[1];
        let h2 = h * h;

        // --- Step A: Discretization (The "build_bs_coeffs" part) ---
        let vol_sq = sigma * sigma;
        let drift = r - T::from_f64(0.5) * vol_sq;
        let diffusion = T::from_f64(0.5) * vol_sq;
        let neg_dt = -dt.clone();

        // Local Operator L (Uniform Central Difference)
        let l_row = (diffusion / h2) - (drift / (h * T::from_f64(2.0)));
        let d_row = (diffusion * T::from_f64(-2.0) / h2) - r;
        let u_row = (diffusion / h2) + (drift / (h * T::from_f64(2.0)));

        // Implicit Matrix A = (I - dt*L)
        let a = neg_dt * l_row;
        let b = T::one() - (dt * d_row);
        let c = neg_dt * u_row;

        // --- Step B: Factorization (The "factorize" part) ---
        let mut c_prime = vec![T::zero(); n];
        let mut m_inv = vec![T::zero(); n];

        // Boundary i=0
        m_inv[0] = T::one(); 
        c_prime[0] = T::zero();

        for i in 1..n - 1 {
            let m = b - (a * c_prime[i - 1]);
            let m_i = T::one() / m;
            m_inv[i] = m_i.clone();
            c_prime[i] = c * m_i;
        }
        
        m_inv[n - 1] = T::one();

        (a, c_prime, m_inv)
    }


    pub fn solve_step_inplace<T>(
        a: T,              // Scalar lower diagonal
        c_prime: &[T],      // U matrix super-diagonal
        m_inv: &[T],        // L matrix diagonal (inverted)
        v_curr: &[T],       // Input: Option prices at t
        v_next: &mut [T],   // Output: Option prices at t + dt
        d_scratch: &mut [T], // Forward sweep scratchpad
    ) where
        T: Real
{
        let n = v_curr.len();

        // --- 1. Forward Sweep (Solve L * d = v_curr) ---
        d_scratch[0] = v_curr[0] * m_inv[0]; 
        for i in 1..n {
            // d_i = (v_curr_i - a * d_{i-1}) * m_inv_i
            d_scratch[i] = (v_curr[i] - (a * d_scratch[i-1])) * m_inv[i];
        }

        // --- 2. Back Substitution (Solve U * v_next = d) ---
        v_next[n-1] = d_scratch[n-1].clone();
        for i in (0..n-1).rev() {
            // v_next_i = d_i - c_prime_i * v_next_{i+1}
            v_next[i] = d_scratch[i] - (c_prime[i] * v_next[i+1]);
        }
    }

    fn build_compact_bs_operator<T, M>(
        mesher: &M, 
        r: T, 
        sigma: T, 
        dt: T
    ) -> (T, T, T) 
    where 
        T: Real,
        M: Mesher1d<T>
    {
        //type T = <SReal as EvaluationResolver<RCReal, VSReal>>::Output;
        
        // Since it's a uniform log-mesher, h_plus and h_minus are constant.
        // We can just grab the first one (index 1 is safe for internal nodes).
        let h = mesher.h_plus()[1]; 
        let h2 = (h * h);

        let vol_sq = (sigma * sigma);
        let drift = (r - (T::from_f64(0.5) * vol_sq));
        let diffusion = (T::from_f64(0.5) * vol_sq);
        let neg_dt = -dt;

        // Standard Finite Difference weights for uniform grid:
        // D1 (Central): [-1/2h, 0, 1/2h]
        // D2 (Central): [1/h^2, -2/h^2, 1/h^2]
        
        let d1_l = (-T::from_f64(0.5)) / h;
        let d1_u = T::from_f64(0.5) / h;
        // d1_m is 0 for central difference on uniform grid

        let d2_l = T::from_f64(1.0) / h2;
        let d2_m = T::from_f64(-2.0) / h2;
        let d2_u = T::from_f64(1.0) / h2;

        // Local Operator: L = diffusion*D2 + drift*D1 - r*I
        let l_row = (diffusion * d2_l) + (drift * d1_l);
        let d_row = ((diffusion * d2_m) - r); // drift * 0 suppressed
        let u_row = (diffusion * d2_u) + (drift * d1_u);

        // Implicit Euler: (I - dt*L)
        let a = neg_dt * l_row;
        let b = T::one() + (neg_dt * d_row);
        let c = neg_dt * u_row;

        (a, b, c)
    }

    /// Pre-calculates the internal components of the Thomas algorithm.
    /// Returns (c_prime, m_inv)
    fn factorize_tridiagonal<T>(a: T, b: T, c: T, n: usize) -> (Vec<T>, Vec<T>)
    where
        T: Real,
    {
        let mut c_prime = vec![T::zero(); n];
        let mut m_inv = vec![T::zero(); n];

        // Boundary i=0 (Identity/Dirichlet assumed)
        // b_0 = 1, c_0 = 0 -> m_inv = 1, c_prime = 0
        m_inv[0] = T::one();
        c_prime[0] = T::zero();

        for i in 1..n - 1 {
            let m = b - (a * c_prime[i - 1]);
            let m_i = T::one() / m;
            m_inv[i] = m_i.clone();
            c_prime[i] = c * m_i;
        }

        // Boundary i=n-1
        m_inv[n - 1] = T::one(); 

        (c_prime, m_inv)
    }

    pub fn solve_tridiagonal_compact<T>(
        a: T,
        b: T,
        c: T,
        r: &[T]
    ) -> Vec<T>
    where
        T: Real
    {
        let n = r.len();
        let mut c_prime = vec![T::zero(); n];
        let mut d_prime = vec![T::zero(); n];
        let mut res = vec![T::zero(); n];

        // --- 1. Lower Boundary (i=0) ---
        // Standard Dirichlet: V_0 stays as it is in the payoff
        // Implicitly: b_0 = 1, c_0 = 0
        c_prime[0] = T::zero();
        d_prime[0] = r[0].clone();

        // --- 2. Forward Sweep (Internal Nodes) ---
        // We use the scalars a, b, c for i = 1 to n-2
        for i in 1..n - 1 {
            let m = b - (a * c_prime[i - 1]);
            c_prime[i] = c / m;
            d_prime[i] = (r[i] - (a * d_prime[i - 1])) / m;
        }

        // --- 3. Upper Boundary (i=n-1) ---
        // Standard Dirichlet or Linearity. 
        // If we assume V_{n-1} is fixed (e.g. Call = S-K):
        let b_n = T::one();
        let a_n = T::zero();
        let m_n = b_n - (a_n * c_prime[n - 2]);
        d_prime[n - 1] = (r[n - 1] - (a_n * d_prime[n - 2])) / m_n;

        // --- 4. Back Substitution ---
        res[n - 1] = d_prime[n - 1].clone();
        for i in (0..n - 1).rev() {
            res[i] = d_prime[i] - (c_prime[i] * res[i + 1]);
        }

        res
    }

    fn interpolate<T, M>(&self, mesher: &M, v: &[T], spot: T) -> T 
    where
        T: Real,
        M: Mesher1d<T>
    {
        let target = spot.ln();

        let idx = match mesher.centers().binary_search_by(|val| {
            val.to_f64().partial_cmp(&target.to_f64()).expect("NaN in Grid")
        }) {
            Ok(exact_match) => return v[exact_match].clone(),
            Err(insertion_point) => {
                if insertion_point == 0 { return v[0].clone(); }
                if insertion_point >= mesher.centers().len() { return v[v.len() - 1].clone(); }
                insertion_point - 1
            }
        };

        let x0 = mesher.centers()[idx];
        let x1 = mesher.centers()[idx + 1];
        let v0 = v[idx];
        let v1 = v[idx + 1];

        // Linear interpolation in log-space: 
        // V = V0 + (V1 - V0) * (ln(S) - x0) / (x1 - x0)
        let weight = (target - x0) / (x1 - x0);
        v0 + (weight * (v1 - v0))
    }
}
