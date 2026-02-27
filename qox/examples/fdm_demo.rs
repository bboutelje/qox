use std::time::Instant;

use chrono::{Duration, Utc};
use nalgebra::Const;
use num_dual::Dual2Vec;
use num_dual::DualStruct;
use num_dual::DualVec;
use qox::evaluators::finite_difference::evaluator::Evaluator;
use qox::evaluators::finite_difference::evaluator::FdmConfig;
use qox::real::ad_gradient::Gradient;
use qox::real::dual_vec::DualVec64;
use qox::real::reverse_node::ReverseGradient;
use qox::traits::real::Real;
use qox::{instruments::future_option::{FutureOption, OptionType}, market::{market_data::OptionMarketData, rate_curve::ContinuousRateCurve, vol_surface::FlatVolSurface}};
use qox::traits::pricing_engine::OptionEvaluable;

pub fn main() {

    // let spot = ReverseGradient::from(95.0); 
    // let rate = ReverseGradient::from(0.05);
    // let vol = ReverseGradient::var(0.20);

    // let spot = Gradient::<2>::constant(95.0);
    // let rate = Gradient::<2>::var(0.05, 0);
    // let vol = Gradient::<2>::var(0.20, 1);

    // let spot = DualVec64::<1>(
    //     DualVec::<f64, f64, Const<1>>::from_re(95.0)
    // );

    // let rate = DualVec64::<1>(
    //     DualVec::<f64, f64, Const<1>>::from_re(0.05)//.derivative(0)
    // );

    let vol = DualVec64::<1>(
        DualVec::<f64, f64, Const<1>>::from_re(0.20).derivative(0)
    );

    let spot = 95.0;
    let rate = 0.05;
    //let vol = 0.2;
    // let spot = Dual2Vec64::<3>(Dual2Vec::from_re(95.0));
    // let rate = Dual2Vec64::<3>(Dual2Vec::from_re(0.05));
    // //let vol  = Dual2Vec64::<1>(Dual2Vec::from_re(0.20).derivative(0));
    
    // let vol = Dual2Vec64::<3>(<Dual2Vec<_, _, Const<3>>>::from_re(0.20).derivative(0));
    
    let market = OptionMarketData::new(
        spot,
        ContinuousRateCurve::new(rate),
        FlatVolSurface::new(vol),
    );

    // Initialize FDM Engine
    let evaluator = Evaluator {
        config: FdmConfig {
            nodes: 500,
            damping_steps: 0,
            time_steps: 100,
        }
    };

    let option = FutureOption::new(
        100.0,
        Utc::now() + Duration::days(365),
        OptionType::Call,
    );

    let start = Instant::now();
    // This calls the FDM logic instead of the Black formula
    let mut result_price = evaluator.evaluate(&option, &market);
    let n = 1;

    // for _ in 0..n {
    //     //ReverseGradient::reset_tape();
    //     result_price = engine.price(&option, &market);
    // }
    let duration = start.elapsed();

    //println!("Price: {:.4}", result_price);
    //println!("Price: {:.4}", result_price.val);
    // let vega = result_price.0.eps.unwrap_generic(Const::<2>, nalgebra::U1)[0];
    // println!("Vega:  {:.4}", vega);

    //println!("Vega:  {:.4}", result_price.grad[1]);
    println!("Price: {:.4}", result_price.0.re());
    println!("Time taken: {:?}", duration / n as u32);
    // Note: Even though we focused on price, result_price is a Dual2Vec64.
    // result_price.eps[0] IS your Delta. It's already there!
}