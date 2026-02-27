// use std::ops::{Add, Div, Mul, Sub};
// use crate::solvers::finite_difference::operators::{TridiagonalCoeffs, solve_tridiagonal};
// use crate::market::{market_data::OptionMarketData, vol_surface::VolSurface};
// use crate::solvers::finite_difference::grid::FdmGrid;
// use crate::traits::{instrument::OptionInstrument, pricing_engine::OptionEvaluable, rate_curve::RateCurve, real::Real};

// pub struct Evaluator {
//     pub config: FdmConfig,
// }

// pub struct FdmConfig {
//     pub nodes: usize,
//     pub time_steps: usize,
//     pub damping_steps: usize,
// }

// impl<I, SReal, RcReal, VsReal, RC, VS> OptionEvaluable<I, SReal, RcReal, VsReal, RC, VS> for Evaluator
// where
//     SReal: Real,
//     RcReal: Real,
//     VsReal: Real,
//     I: OptionInstrument<f64>,
//     RC: RateCurve<RcReal>,
//     VS: VolSurface<VsReal>,
//     // for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + 
//     //                Mul<&'a T, Output = T> + Div<&'a T, Output = T> +
//     //                std::ops::Neg<Output = T>
// {
//     fn evaluate(&self, instrument: &I, market: &OptionMarketData<SReal, RcReal, VsReal, RC, VS>) -> T {
        
//         let s_min = T::from_f64(0.01); 
//         let s_max = &market.spot_price * &T::from_f64(3.0); 
        

//         let grid = FdmGrid::new_linear_space(s_min, s_max, self.config.nodes);
//         let mut v = self.initialize_payoff(instrument, &grid);
//         let dt = T::from_f64(instrument.time_to_expiry().to_f64() / self.config.time_steps as f64);

//         // 1. Build the operator ONCE
//         let r = market.rate_curve.zero_rate(&T::zero());
//         let sigma = market.vol_surface.volatility(&T::zero());
//         let coeffs = TridiagonalCoeffs::black_scholes_operator(&grid.centers, &r, &sigma, &dt);

//         // 2. Loop only the solver
//         for _ in 0..self.config.time_steps {
//             // You might need to adjust the boundaries of v here if they are time-dependent
//             v = solve_tridiagonal(&coeffs.a, &coeffs.b, &coeffs.c, &v);
//         }

//         self.interpolate(&grid, &v, &market.spot_price)
//     }
//     // fn price(&self, instrument: &I, market: &OptionMarketData<T, RC, VS>) -> T {
//     //     let s_min = T::from_f64(0.01); 
//     //     let s_max = &market.spot_price * &T::from_f64(3.0); 
        
//     //     // Initialize the actual grid object
//     //     let grid = FdmGrid::new_log_space(s_min, s_max, self.config.nodes);
        
//     //     let mut v = self.initialize_payoff(instrument, &grid);

//     //     let dt = T::from_f64(instrument.time_to_expiry().to_f64() / self.config.time_steps as f64);
//     //     for _ in 0..self.config.time_steps {
//     //         // Pass the grid into the stepping logic
//     //         v = self.step_backwards(&v, market, &grid, &dt);
//     //     }

//     //     self.interpolate(&grid, &v, &market.spot_price)
//     // }
    
//     fn evaluate_all(
//         &self,
//         _instrument: &I,
//         _market: &OptionMarketData<T, RC, VS>,
//     ) -> crate::traits::pricing_engine::OptionEvaluation<f64>
//     where
//         RC: RateCurve<T>,
//         VS: VolSurface<T> {
//         todo!()
//     }
// }

// impl Evaluator {
//     /// 1. Lines up with: self.initialize_payoff(instrument, &grid)
//     fn initialize_payoff<T, I>(
//         &self, 
//         instrument: &I, 
//         grid: &FdmGrid<T>
//     ) -> Vec<T> 
//     where 
//         T: Real, 
//         I: OptionInstrument<T>,
//         for<'a> &'a T: Add<&'a T, Output = T> + 
//                    Sub<&'a T, Output = T> + 
//                    Mul<&'a T, Output = T> + 
//                    Div<&'a T, Output = T> +
//                    std::ops::Neg<Output = T>,
//     {
//         let k = T::from_f64(instrument.strike());
        
//         grid.centers.iter().map(|s| {
//             if instrument.is_call() {
//                 // Call Payoff: max(S - K, 0)
//                 (s - &k).max(&Real::zero())
//             } else {
                
//                 (&k - s).max(&Real::zero())
//             }
//         }).collect::<Vec<T>>()
//     }

//     /// 2. Lines up with: v = self.step_backwards(&v, market, &grid, &dt)
//     fn step_backwards<T, RC, VS>(
//         &self,
//         v_old: &[T],
//         market: &OptionMarketData<T, RC, VS>,
//         grid: &FdmGrid<T>,
//         dt: &T,
//     ) -> Vec<T>
//     where
//         T: Real,
//         RC: RateCurve<T>,
//         VS: VolSurface<T>,
//         for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + 
//                        Mul<&'a T, Output = T> + Div<&'a T, Output = T> +
//                        std::ops::Neg<Output = T>,
//     {
//         // Use t=0 or current model time for sensitivities
//         let r = market.rate_curve.zero_rate(&T::zero());
//         let sigma = market.vol_surface.volatility(&T::zero());
        
//         // Build the system using the grid stencils
//         let (l_1, d_1, u_1) = grid.first_derivative_coeffs();
//         let (l_2, d_2, u_2) = grid.second_derivative_coeffs();

//         let n = grid.centers.len();
//         let mut a = vec![T::zero(); n];
//         let mut b = vec![T::one(); n];
//         let mut c = vec![T::zero(); n];

//         // 1. Pre-calculate values that don't change per grid node
//         let vol_sq = &sigma * &sigma;
//         let half_vol_sq = &T::from_f64(0.5) * &vol_sq;
//         let neg_dt = &-dt;

//         // 2. Single pass to build the tridiagonal system
//         for i in 1..n-1 {
//             let s = &grid.centers[i];
//             let s_sq = s * s;
            
//             // Operator L = 0.5 * sigma^2 * S^2 * D2 + r * S * D1 - r * I
//             let drift = &r * s;
//             let diffusion = &half_vol_sq * &s_sq;

//             // Compute L components for row i
//             let l_row_i = &(&diffusion * &l_2[i]) + &(&drift * &l_1[i]);
//             let d_row_i = &(&(&diffusion * &d_2[i]) + &(&drift * &d_1[i])) - &r;
//             let u_row_i = &(&diffusion * &u_2[i]) + &(&drift * &u_1[i]);

//             // Implicit Euler: [I - dt*L] V_new = V_old
//             // a[i] = -dt * L_lower
//             // b[i] = 1 - dt * L_diag
//             // c[i] = -dt * L_upper
//             a[i] = neg_dt * &l_row_i;
//             b[i] = &T::one() + &(neg_dt * &d_row_i); // 1 - (dt * d_row_i)
//             c[i] = neg_dt * &u_row_i;
//         }
//         // 3. Boundary Conditions (Dirichlet)
//         // At S=0 and S=S_max, we keep the payoff or the discounted boundary
//         // These rows in the matrix remain Identity (b[0]=1, b[n-1]=1) 
//         // which simply passes the boundary value through.

//         solve_tridiagonal(&a, &b, &c, v_old)
//     }


//     fn interpolate<T: Real>(&self, grid: &FdmGrid<T>, v: &[T], spot: &T) -> T 
//     where 
//         for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + 
//                        Mul<&'a T, Output = T> + Div<&'a T, Output = T>
//     {
//         // 1. Find the surrounding nodes S_i and S_{i+1}
//         // Since centers are sorted, binary search is O(log n)
//         let idx = match grid.centers.binary_search_by(|val| {
//             val.to_f64().partial_cmp(&spot.to_f64()).expect("NaN in Grid")
//         }) {
//             Ok(exact_match) => return v[exact_match].clone(),
//             Err(insertion_point) => {
//                 // If the spot is outside the grid, clamp to the nearest boundary
//                 if insertion_point == 0 { return v[0].clone(); }
//                 if insertion_point >= grid.centers.len() { return v[v.len() - 1].clone(); }
//                 insertion_point - 1
//             }
//         };

//         let s0 = &grid.centers[idx];
//         let s1 = &grid.centers[idx + 1];
//         let v0 = &v[idx];
//         let v1 = &v[idx + 1];

//         // 2. Linear Interpolation formula: V = V0 + (V1 - V0) * (S - S0) / (S1 - S0)
//         let weight = (spot - s0) / &(s1 - s0);
//         v0 + &(&weight * &(v1 - v0))
//     }
// }