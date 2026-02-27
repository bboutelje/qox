// use std::ops::{Add, Div, Mul, Sub};

// use crate::market::market_data::OptionMarketData;
// use crate::market::vol_surface::VolSurface;
// use crate::real::dual2_vec::Dual2Vec64;
// use crate::traits::pricing_engine::{OptionEvaluable, OptionEvaluation};
// use crate::traits::instrument::OptionInstrument;
// use crate::traits::rate_curve::RateCurve;
// use crate::traits::real::Real;
// use num_dual::{DualStruct};
// use crate::real::dual_vec::DualVec64;

// pub struct BlackEngine;
// impl BlackEngine {
//     fn compute<T, I, RC, VS>(
//         &self,
//         instrument: &I,
//         market: &OptionMarketData<T, RC, VS>,
//     ) -> T
//     where
//         T: Real, 
//         I: OptionInstrument<T>,
//         RC: RateCurve<T>,
//         VS: VolSurface<T>,
//         for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + 
//                     Mul<&'a T, Output = T> + Div<&'a T, Output = T>,
//     {
//         let f = &market.spot_price;
//         let k = T::from_f64(instrument.strike().to_f64());
//         let t = T::from_f64(instrument.time_to_expiry().to_f64());

//         let sigma = market.vol_surface.volatility(&t);
//         let df = market.rate_curve.discount_factor(&t);
//         let half = T::from_f64(0.5);

//         let sqrt_t = t.sqrt(); 
//         let vol_std = &sigma * &sqrt_t; 

//         let ln_part = (f / &k).ln();
//         let sig_sq = sigma.powi(2);
//         let drift = &(&half * &sig_sq) * &t;

//         let d1 = &(ln_part + &drift) / &vol_std;
//         let d2 = &d1 - &vol_std;

//         if instrument.is_call() {
//             let n_d1 = d1.norm_cdf(); // Owned T
//             let n_d2 = d2.norm_cdf(); // Owned T
            
//             // Pattern: &(Result of Term 1 - Result of Term 2)
//             // We wrap the multiplications in parentheses and borrow them
//             let payoff = &(f * &n_d1) - &(&k * &n_d2);
            
//             &df * &payoff
//         } else {
//             let n_nd1 = (-d1).norm_cdf(); // Owned T
//             let n_nd2 = (-d2).norm_cdf(); // Owned T
            
//             let payoff = &(&k * &n_nd2) - &(f * &n_nd1);
            
//             &df * &payoff
//         }
//     }
// }

// // impl<I, RC, VS> OptionEvaluable<I, HyperDual, RC, VS> for BlackEngine
// // where
// //     I: OptionInstrument<HyperDual>,
// //     RC: RateCurve<HyperDual>,
// //     VS: VolSurface<HyperDual>
// // {
    
// //     fn price(
// //         &self,
// //         instrument: &I,
// //         market: &OptionMarketData<HyperDual, RC, VS>,
// //     ) -> HyperDual {
// //         let f = market.spot_price.clone();
// //         let k = HyperDual::from_f64(instrument.strike().to_f64());
// //         let t = HyperDual::from_f64(instrument.time_to_expiry().to_f64());
// //         let sigma = HyperDual(market.vol_surface.volatility(t).0);
// //         let df = HyperDual(market.rate_curve.discount_factor(t).0);
// //         let is_call = instrument.is_call();

// //         let sqrt_t = t.sqrt();
// //         let half = HyperDual::from_f64(0.5);

// //         let d1 = ((f / k).ln() + half * sigma.powi(2) * t) / (sigma * sqrt_t);
// //         let d2 = d1 - (sigma * sqrt_t);

// //         if is_call {
// //             df * (f * d1.norm_cdf() - k * d2.norm_cdf())
// //         } else {
// //             df * (k * (-d2).norm_cdf() - f * (-d1).norm_cdf())
// //         }
// //     }


// //     fn price_and_greeks(
// //         &self,
// //         instrument: &I,
// //         market: &OptionMarketData<HyperDual, RC, VS>,
// //     ) -> OptionEvaluation<f64>
// //     where
// //         RC: RateCurve<HyperDual>,
// //         VS: VolSurface<HyperDual>,
// //     {
        
// //         let price_hyper = self.price(instrument, market);

// //         OptionEvaluation {
// //             price: price_hyper.0.re(),
// //             delta: price_hyper.0.eps1,
// //             gamma: price_hyper.0.eps1eps2,
// //             vega: 0.0,
// //             theta: 0.0,
// //             rho: 0.0,
// //         }
// //     }
// // }

// impl<I, RC, VS, const N: usize> OptionEvaluable<I, DualVec64<N>, RC, VS> for BlackEngine
// where
//     I: OptionInstrument<DualVec64<N>>,
//     RC: RateCurve<DualVec64<N>>,
//     VS: VolSurface<DualVec64<N>>,
//     for<'a> &'a DualVec64<N>: Add<&'a DualVec64<N>, Output = DualVec64<N>> + 
//                              Sub<&'a DualVec64<N>, Output = DualVec64<N>> + 
//                              Mul<&'a DualVec64<N>, Output = DualVec64<N>> + 
//                              Div<&'a DualVec64<N>, Output = DualVec64<N>>,
// {
//     fn evaluate(
//         &self,
//         instrument: &I,
//         market: &OptionMarketData<DualVec64<N>, RC, VS>,
//     ) -> DualVec64<N> {
//         self.compute(instrument, market)
//     }

//     fn evaluate_all(
//             &self,
//             instrument: &I,
//             market: &OptionMarketData<DualVec64<N>, RC, VS>,
//         ) -> OptionEvaluation<f64>
//         where
//             RC: RateCurve<DualVec64<N>>,
//             VS: VolSurface<DualVec64<N>> {
        
//         let res = self.compute(instrument, market);
       
//         let eps = res.0.eps.unwrap_generic(nalgebra::Const::<N>, nalgebra::U1);

//         OptionEvaluation {
//             price: res.0.re(),
//             delta: eps[0],
//             gamma: 0.0,          // DualVec64 doesn't provide 2nd derivatives by default
//             vega:  if N > 2 { eps[2] } else { 0.0 },
//             theta: 0.0,
//             rho: if N > 1 { eps[1] } else { 0.0 },
//         }
//     }
// }

// impl<I, RC, VS, const N: usize> OptionEvaluable<I, Dual2Vec64<N>, RC, VS> for BlackEngine
// where
//     I: OptionInstrument<Dual2Vec64<N>>,
//     RC: RateCurve<Dual2Vec64<N>>,
//     VS: VolSurface<Dual2Vec64<N>>,
//     for<'a> &'a Dual2Vec64<N>: Add<&'a Dual2Vec64<N>, Output = Dual2Vec64<N>> + 
//                              Sub<&'a Dual2Vec64<N>, Output = Dual2Vec64<N>> + 
//                              Mul<&'a Dual2Vec64<N>, Output = Dual2Vec64<N>> + 
//                              Div<&'a Dual2Vec64<N>, Output = Dual2Vec64<N>>,
// {
//     fn evaluate(
//             &self,
//             instrument: &I,
//             market: &OptionMarketData<Dual2Vec64<N>, RC, VS>,
//         ) -> Dual2Vec64<N> {
//         self.compute(instrument, market)
//     }

//     fn evaluate_all(
//             &self,
//             instrument: &I,
//             market: &OptionMarketData<Dual2Vec64<N>, RC, VS>,
//         ) -> OptionEvaluation<f64>
//         where
//             RC: RateCurve<Dual2Vec64<N>>,
//             VS: VolSurface<Dual2Vec64<N>> {
        
    
//         let res = self.compute(instrument, market);
        
//         // v1: Gradient vector [dS, dr, dSigma, ...]
//         let v1 = res.0.v1.unwrap_generic(nalgebra::U1, nalgebra::Const::<N>);
        
//         // v2: Hessian matrix [ (dS,dS), (dS,dr), (dS,dSigma) ... ]
//         let v2 = res.0.v2.unwrap_generic(nalgebra::Const::<N>, nalgebra::Const::<N>);

//         OptionEvaluation {
//             price: res.0.re(),
//             // First-order Greeks
//             delta: v1[0],
//             vega:  if N > 2 { v1[2] } else { 0.0 },
//             rho:   if N > 1 { v1[1] } else { 0.0 },
//             theta: 0.0, // Usually handled by t-sensitivity if t is a dual
            
//             // Second-order Greeks (The "Single Pass" advantage)
//             gamma: v2[(0, 0)], 
            
//             // Optional: You could extend your OptionEvaluation struct to include these:
//             // vanna: if N > 2 { v2[(0, 2)] } else { 0.0 }, // d^2V / (dS dSigma)
//             // volga: if N > 2 { v2[(2, 2)] } else { 0.0 }, // d^2V / (dSigma^2)
//         }
//     }
// }