// use chrono::{Duration, Utc};
// use num_dual::{Dual2Vec};
// use qox::evaluators::analytic::black::BlackEngine;
// use qox::instruments::future_option::{FutureOption, OptionType};
// use qox::market::{market_data::OptionMarketData, rate_curve::ContinuousRateCurve, vol_surface::FlatVolSurface};
// use qox::real::dual2_vec::Dual2Vec64;
// use qox::traits::pricing_engine::OptionEvaluable;

// pub fn main() {

//     // let spot = HyperDual(HyperDual64::new(100.0, 1.0, 1.0, 0.0));      // only eps1
//     // let rate = HyperDual(HyperDual64::new(0.05, 0.0, 1.0, 0.0));       // only eps2
//     // let vol = HyperDual(HyperDual64::new(0.20, 0.0, 0.0, 1.0)); 
    
//     // If N = 3, you are tracking 3 variables (e.g., spot, rate, vol)
//     // let spot = DualVec64::<3>(DualVec::new(100.0, Derivative::new(Some(Vector3::new(1.0, 0.0, 0.0)))));
//     // let rate = DualVec64::<3>(DualVec::new(0.05,  Derivative::new(Some(Vector3::new(0.0, 1.0, 0.0)))));
//     // let vol  = DualVec64::<3>(DualVec::new(0.20,  Derivative::new(Some(Vector3::new(0.0, 0.0, 1.0)))));

//     // Requires: use num_dual::DualNum;
//     // Indices: 0 = Spot, 1 = Rate, 2 = Vol
//     let spot = Dual2Vec64::<3>(Dual2Vec::<f64, f64, nalgebra::Const<3>>::from_re(100.0).derivative(0));
//     let rate = Dual2Vec64::<3>(Dual2Vec::<f64, f64, nalgebra::Const<3>>::from_re(0.05).derivative(1));
//     let vol  = Dual2Vec64::<3>(Dual2Vec::<f64, f64, nalgebra::Const<3>>::from_re(0.20).derivative(2));

//     let rate_curve = ContinuousRateCurve::new(rate);
//     let vol_surface = FlatVolSurface::new(vol);


//     let market = OptionMarketData::new(
//         spot,
//         rate_curve,
//         vol_surface,
//     );
    
//     let engine = BlackEngine;

//     let option = FutureOption::new(
//         100.0,
//         Utc::now() + Duration::days(365),
//         OptionType::Call,
//     );
    
//     let result = engine.evaluate_all(&option, &market);
    
//     println!("Black Put Price: {:.4}", result.price);
//     println!("Delta: {:.4}", result.delta);
//     println!("Gamma: {:.4}", result.gamma);
//     println!("Vega: {:.4}", result.vega);
//     println!("Theta: {:.4}", result.theta);
//     println!("Rho: {:.4}", result.rho);
// }

pub fn main(){}