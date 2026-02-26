use std::ops::{Add, Div, Mul, Sub};

use crate::{market::{market_data::OptionMarketData, vol_surface::VolSurface}, solvers::fdm::{meshing::{log::FdmLog1dMesher, uniform::Uniform1dMesher}, operators_log::{TridiagonalCoeffs, solve_tridiagonal}}, traits::{fdm_1d_mesher::Fdm1dMesher, instrument::OptionInstrument, pricing_engine::OptionEvaluable, rate_curve::RateCurve, real::Real}};

pub struct Evaluator {
    pub config: FdmConfig,
}

pub struct FdmConfig {
    pub nodes: usize,
    pub time_steps: usize,
    pub damping_steps: usize,
}


impl<I, RC, VS, T> OptionEvaluable<I, T, RC, VS> for Evaluator
where
    T: Real + std::fmt::Debug,
    I: OptionInstrument<T>,
    RC: RateCurve<T>,
    VS: VolSurface<T>,
    for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + 
                   Mul<&'a T, Output = T> + Div<&'a T, Output = T> +
                   std::ops::Neg<Output = T>

{
    fn evaluate(&self, instrument: &I, market: &OptionMarketData<T, RC, VS>) -> T {
        // 1. Boundaries in Log Space
        let s_min = T::from_f64(0.01); 
        let s_max = &market.spot_price * &T::from_f64(5.0); // Slightly wider for log-space
        
        let x_min = s_min.ln();
        let x_max = s_max.ln();

        // 2. Build the Uniform Mesher in Log Space
        let uniform_mesher = Uniform1dMesher::new(x_min, x_max, self.config.nodes);
        let mesher = FdmLog1dMesher::new(uniform_mesher);

        let mut v = self.initialize_payoff(instrument, &mesher);
        
        let dt = T::from_f64(instrument.time_to_expiry().to_f64() / self.config.time_steps as f64);

        println!("time to expiry {:.6}", instrument.time_to_expiry().to_f64());

        let r = market.rate_curve.zero_rate(&T::zero());
        let sigma = market.vol_surface.volatility(&T::zero());
        let coeffs = TridiagonalCoeffs::black_scholes_log_operator(&mesher, &r, &sigma, &dt);

        for _ in 0..self.config.time_steps {
            v = solve_tridiagonal(&coeffs.a, &coeffs.b, &coeffs.c, &v);
        }

        self.interpolate(&mesher, &v, &market.spot_price)
    }
    
    fn evaluate_all(
        &self,
        _instrument: &I,
        _market: &OptionMarketData<T, RC, VS>,
    ) -> crate::traits::pricing_engine::OptionEvaluation<f64>
    where
        RC: RateCurve<T>,
        VS: VolSurface<T> {
        todo!()
    }
}

impl Evaluator {
    fn initialize_payoff<T, I, M>(&self, instrument: &I, mesher: &M) -> Vec<T> 
    where 
        T: Real, 
        I: OptionInstrument<T>,
        M: Fdm1dMesher<T>,
        for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + 
                       Mul<&'a T, Output = T> + Div<&'a T, Output = T>
    {
        let k = T::from_f64(instrument.strike());
        (0..mesher.size()).map(|i| {
            let s = mesher.location(i);
            if instrument.is_call() {
                (s - &k).max(&T::zero())
            } else {
                (&k - s).max(&T::zero())
            }
        }).collect()
    }

    fn step_backwards<T, RC, VS, M>(
        &self,
        v_old: &[T],
        market: &OptionMarketData<T, RC, VS>,
        mesher: &M,
        dt: &T,
    ) -> Vec<T>
    where
        T: Real,
        RC: RateCurve<T>,
        VS: VolSurface<T>,
        M: Fdm1dMesher<T>,
        for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + 
                       Mul<&'a T, Output = T> + Div<&'a T, Output = T> +
                       std::ops::Neg<Output = T>,
    {
        let r = market.rate_curve.zero_rate(&T::zero());
        let sigma = market.vol_surface.volatility(&T::zero());
        
        let (l_1, d_1, u_1) = self.first_derivative_coeffs(mesher);
        let (l_2, d_2, u_2) = self.second_derivative_coeffs(mesher);

        let n = mesher.centers().len();
        let mut a = vec![T::zero(); n];
        let mut b = vec![T::one(); n];
        let mut c = vec![T::zero(); n];

        // CONSTANT COEFFICIENTS: Calculated once outside the loop
        let vol_sq = &sigma * &sigma;
        let half_vol_sq = &T::from_f64(0.5) * &vol_sq;
        
        // Log-space drift: (r - 0.5 * sigma^2)
        let drift = &r - &half_vol_sq;
        let diffusion = half_vol_sq;
        let neg_dt = &-dt;

        for i in 1..n-1 {
            // L = diffusion * D2 + drift * D1 - r * I
            // No 's' or 's_sq' terms here!
            let l_row = &(&diffusion * &l_2[i]) + &(&drift * &l_1[i]);
            let d_row = &(&(&diffusion * &d_2[i]) + &(&drift * &d_1[i])) - &r;
            let u_row = &(&diffusion * &u_2[i]) + &(&drift * &u_1[i]);

            a[i] = neg_dt * &l_row;
            b[i] = &T::one() + &(neg_dt * &d_row);
            c[i] = neg_dt * &u_row;
        }

        solve_tridiagonal(&a, &b, &c, v_old)
    }

    fn first_derivative_coeffs<T, M>(&self, mesher: &M) -> (Vec<T>, Vec<T>, Vec<T>)
    where
        T: Real,
        M: Fdm1dMesher<T>,
        for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + 
                       Mul<&'a T, Output = T> + Div<&'a T, Output = T>,
    {
        let n = mesher.size();
        let mut l = vec![T::zero(); n];
        let mut d = vec![T::zero(); n];
        let mut u = vec![T::zero(); n];

        let hp = mesher.h_plus();
        let hm = mesher.h_minus();

        for i in 1..n-1 {
            let h_p = &hp[i];
            let h_m = &hm[i];
            let sum_h = h_p + h_m;

            // Non-uniform weights
            l[i] = &(&T::zero() - h_p) / &(h_m * &sum_h);
            d[i] = &(h_p - h_m) / &(h_p * h_m);
            u[i] = h_m / &(h_p * &sum_h);
        }
        (l, d, u)
    }

    fn second_derivative_coeffs<T, M>(&self, mesher: &M) -> (Vec<T>, Vec<T>, Vec<T>)
    where
        T: Real,
        M: Fdm1dMesher<T>,
        for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + 
                       Mul<&'a T, Output = T> + Div<&'a T, Output = T>,
    {
        let n = mesher.size();
        let mut l = vec![T::zero(); n];
        let mut d = vec![T::zero(); n];
        let mut u = vec![T::zero(); n];

        let hp = mesher.h_plus();
        let hm = mesher.h_minus();

        for i in 1..n-1 {
            let h_p = &hp[i];
            let h_m = &hm[i];
            let sum_h = h_p + h_m;

            // Standard second derivative finite difference for non-uniform grid
            l[i] = &T::from_f64(2.0) / &(h_m * &sum_h);
            d[i] = &T::from_f64(-2.0) / &(h_p * h_m);
            u[i] = &T::from_f64(2.0) / &(h_p * &sum_h);
        }
        (l, d, u)
    }


    fn interpolate<T, M>(&self, mesher: &M, v: &[T], spot: &T) -> T 
    where
        T: Real,
        M: Fdm1dMesher<T>,
        for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + 
                    Mul<&'a T, Output = T> + Div<&'a T, Output = T>
    {
        // CRITICAL CHANGE: Calculate the log-spot for searching the log-grid
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

        let x0 = &mesher.centers()[idx];
        let x1 = &mesher.centers()[idx + 1];
        let v0 = &v[idx];
        let v1 = &v[idx + 1];

        // Linear interpolation in log-space: 
        // V = V0 + (V1 - V0) * (ln(S) - x0) / (x1 - x0)
        let weight = (&target - x0) / &(x1 - x0);
        v0 + &(&weight * &(v1 - v0))
    }
}
